

#[derive(Debug, Clone)]
pub struct Connection {
    pub timestamp: i64,

    pub src_ip: String,
    pub dst_ip: String,

    pub src_port: u16,
    pub dst_port: u16,

    pub old_state: u32,
    pub new_state: u32,
}
