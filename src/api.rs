use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionRow {
    pub id: i64,

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
}
