#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConnEvent {
    pub pid: u32,

    pub saddr: u32,
    pub daddr: u32,

    pub sport: u16,
    pub dport: u16,

    pub family: u16,
    pub protocol: u16,

    pub oldstate: u32,
    pub newstate: u32,

    pub comm: [u8; 16],
}
