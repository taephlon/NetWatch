use anyhow::Result;
use libbpf_rs::{ObjectBuilder, RingBufferBuilder, MapCore};
use std::path::Path;
use std::net::Ipv4Addr;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use axum::{
    Router,
    routing::get,
    extract::{State, ws::{WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    extract::ws::Message,
    Json,
};

use tower_http::services::ServeDir;
use state::AppState;

mod db;
mod models;
mod api;
mod state;
mod websocket;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct ConnEvent {
    pid: u32,

    saddr: u32,
    daddr: u32,

    sport: u16,
    dport: u16,

    family: u16,
    protocol: u16,

    oldstate: u32,
    newstate: u32,

    comm: [u8; 16],
    } 

fn bytes_to_string(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len()); 
    String::from_utf8_lossy(&bytes[..len]).to_string() 
}

async fn connections_handler(
    State(state): State<AppState>,
) -> Json<Vec<api::ConnectionRow>> {
    let conn = state.db.lock().unwrap();

    Json(db::get_connections(&conn).unwrap_or_default())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.tx))
}

async fn handle_socket(
    mut socket: WebSocket,
    tx: broadcast::Sender<String>,
) {
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                if let Ok(text) = msg {
                    if socket.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let obj_path = Path::new("ebpf/connect.bpf.o");

    let (tx, _) = broadcast::channel::<String>(1000);
    let tx_ringbuf = tx.clone();

    let open_obj = ObjectBuilder::default().open_file(obj_path)?;
    let mut obj = open_obj.load()?;

    let mut links = Vec::new();
    for prog in obj.progs_mut() {
        links.push(prog.attach()?);
    }

    let db_conn = Arc::new(Mutex::new(db::init_db()?));

    let state = AppState {
        db: db_conn.clone(),
        tx: tx.clone(),
    };

    let mut ringbuf = RingBufferBuilder::new();

    let events_map = obj
        .maps()
        .find(|m| m.name() == "events")
        .expect("events map not found");

    let tx_ring = tx.clone();
    let db_clone = db_conn.clone();

    ringbuf.add(&events_map, move |data: &[u8]| {
        if data.len() < std::mem::size_of::<ConnEvent>() {
            return 0;
        }

        let event = unsafe {
            &*(data.as_ptr() as *const ConnEvent)
        };

        let src = Ipv4Addr::from(event.saddr.to_be());
        let dst = Ipv4Addr::from(event.daddr.to_be());
        
        let ws_event = models::WsConnectionEvent {
    pid: event.pid,

    src_ip: src.to_string(),
    dst_ip: dst.to_string(),

    src_port: event.sport,
    dst_port: event.dport,

    old_state: event.oldstate,
    new_state: event.newstate,
};

        let msg = format!(
            "PID={} {}:{} -> {}:{}",
            event.pid,
            src,
            event.sport,
            dst,
            event.dport
        );

        let _ = tx_ring.send(msg);

        let record = models::Connection {
            timestamp: Utc::now().timestamp(),
            src_ip: src.to_string(),
            dst_ip: dst.to_string(),
            src_port: event.sport,
            dst_port: event.dport,
            old_state: event.oldstate,
            new_state: event.newstate,
        };

        if let Ok(conn) = db_clone.lock() {
            let _ = db::insert_connection(&conn, &record);
        }
        
        if let Ok(json) = serde_json::to_string(&ws_event) {
    let _ = tx_ringbuf.send(json);
}
        
        0
    })?;

    let ringbuf = ringbuf.build()?;

    let app = Router::new()
        .route("/connections", get(connections_handler))
        .route("/ws", get(websocket::ws_handler))
        .fallback_service(ServeDir::new("web"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    println!("NetWatch running...");

    loop {
        ringbuf.poll(std::time::Duration::from_millis(100))?;
    }
}
