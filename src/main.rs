use anyhow::Result;
use libbpf_rs::{MapCore, ObjectBuilder, RingBufferBuilder};

use std::{
    net::Ipv4Addr,
    path::Path,
    sync::{Arc, Mutex},
};

use chrono::Utc;
use tokio::sync::broadcast;

use axum::{
    routing::get,
    Router,
};

use tower_http::services::ServeDir;

mod db;
mod api;
mod state;
mod models;
mod websocket;
mod dns;
mod process;
mod threat;

use state::AppState;

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

async fn connections_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::Json<Vec<api::ConnectionRow>> {
    let conn = state.db.lock().unwrap();

    axum::Json(
        db::get_connections(&conn)
            .unwrap_or_default()
    )
}

#[tokio::main]
async fn main() -> Result<()> {

    let obj_path =
        Path::new("ebpf/connect.bpf.o");

    let open_obj =
        ObjectBuilder::default()
            .open_file(obj_path)?;

    let mut obj =
        open_obj.load()?;

    let mut links = Vec::new();

    for prog in obj.progs_mut() {
        links.push(
            prog.attach()?
        );
    }

    let db_conn =
        Arc::new(
            Mutex::new(
                db::init_db()?
            )
        );

    let (tx, _) =
        broadcast::channel::<String>(1000);

    let state = AppState {
        db: db_conn.clone(),
        tx: tx.clone(),
    };

    let mut ringbuf_builder =
        RingBufferBuilder::new();

    let events_map = obj
        .maps()
        .find(|m| m.name() == "events")
        .expect("events map not found");

    let tx_ring = tx.clone();
    let db_clone = db_conn.clone();

    ringbuf_builder.add(
        &events_map,
        move |data: &[u8]| {

            if data.len()
                < std::mem::size_of::<ConnEvent>()
            {
                return 0;
            }

            let event = unsafe {
                &*(data.as_ptr()
                    as *const ConnEvent)
            };

            let src =
                Ipv4Addr::from(
                    event.saddr.to_be()
                );

            let dst =
                Ipv4Addr::from(
                    event.daddr.to_be()
                );

            let hostname =
                dns::reverse_lookup(
                     &dst.to_string()
             );
    
            let (risk_score, threat_label) =
                threat::classify(
                  &hostname,
                    event.dport,
                );

            let process_name =
                process::process_name(event.pid);

            let executable =
                process::executable(event.pid);

            let dst =
                Ipv4Addr::from(event.daddr.to_be());

            let record =
                models::Connection {

                    pid: event.pid,

                    process_name,
                    executable,

                    timestamp:
                        Utc::now()
                            .timestamp(),

                    src_ip:
                        src.to_string(),

                    dst_ip:
                        dst.to_string(),

                    src_port:
                        event.sport,

                    dst_port:
                        event.dport,

                    old_state:
                        event.oldstate,

                    new_state:
                        event.newstate,

                    hostname,

                    risk_score,
                    threat_label
                };

            if let Ok(conn)
                = db_clone.lock()
            {
                let _ =
                    db::insert_connection(
                        &conn,
                        &record,
                    );
            }

            let ws_event =
                api::ConnectionRow {

                    id: 0,

                    pid: record.pid,

                    process_name: record.process_name.clone(),
                    executable: record.executable.clone(),

                    timestamp:
                        record.timestamp,

                    src_ip:
                        record.src_ip.clone(),

                    dst_ip:
                        record.dst_ip.clone(),

                    src_port:
                        record.src_port,

                    dst_port:
                        record.dst_port,

                    old_state:
                        record.old_state,

                    new_state:
                        record.new_state,

                    hostname: 
                        record.hostname.clone(),

                    risk_score: record.risk_score,
                    threat_label: record.threat_label.clone(),

                };

            if let Ok(json)
                = serde_json::to_string(
                    &ws_event
                )
            {
                let _ =
                    tx_ring.send(json);
            }

            0
        },
    )?;

    let ringbuf =
        ringbuf_builder.build()?;

    let app =
        Router::new()
            .route(
                "/connections",
                get(connections_handler),
            )
            .route(
                "/ws",
                get(
                    websocket::ws_handler
                ),
            )
            .fallback_service(
                ServeDir::new("web")
            )
            .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(
            "0.0.0.0:3000",
        )
        .await?;

    tokio::spawn(async move {

        axum::serve(
            listener,
            app,
        )
        .await
        .unwrap();

    });

    println!(
        "NetWatch running on :3000"
    );

    loop {

        ringbuf.poll(
            std::time::Duration::from_millis(
                100
            )
        )?;

    }
}
