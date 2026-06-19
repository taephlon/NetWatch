use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Connection {
    pub pid: u32,

    pub timestamp: i64,

    pub src_ip: String,
    pub dst_ip: String,

    pub src_port: u16,
    pub dst_port: u16,

    pub old_state: u32,
    pub new_state: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsConnectionEvent {
    pub pid: u32,

    pub src_ip: String,
    pub dst_ip: String,

    pub src_port: u16,
    pub dst_port: u16,

    pub old_state: u32,
    pub new_state: u32,
}
