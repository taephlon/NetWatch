use serde::Serialize;

#[derive(Serialize)]
pub struct ConnectionRow {
    pub id: i64,

    pub timestamp: i64,

    pub src_ip: String,
    pub dst_ip: String,

    pub src_port: u16,
    pub dst_port: u16,

    pub old_state: u32,
    pub new_state: u32,
}

#[derive(Serialize)]
pub struct Stats {
    pub total_connections: u64,
    pub unique_destinations: u64,
    pub unique_ports: u64,
}
