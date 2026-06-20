use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Connection {
    pub pid: u32,

    pub process_name: String,
    pub executable: String,


    pub timestamp: i64,

    pub src_ip: String,
    pub dst_ip: String,

    pub src_port: u16,
    pub dst_port: u16,

    pub old_state: u32,
    pub new_state: u32,

    pub hostname: String,

    pub risk_score: u8,
    pub threat_label: String,
    
    pub threat_source: String,
    pub threat_confidence: u8,

    pub country: String,
    pub city: String,

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
