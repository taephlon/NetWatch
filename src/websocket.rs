use axum::{
    extract::{
        State,
        ws::{
            Message,
            WebSocket,
            WebSocketUpgrade,
        },
    },
    response::IntoResponse,
};

use futures_util::SinkExt;

use tokio::sync::broadcast;

use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {

    ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            state.tx,
        )
    })
}

async fn handle_socket(
    mut socket: WebSocket,
    tx: broadcast::Sender<String>,
) {

    let mut rx = tx.subscribe();

    while let Ok(msg) = rx.recv().await {

        if socket
            .send(Message::Text(msg.into()))
            .await
            .is_err()
        {
            break;
        }
    }
}
