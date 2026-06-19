use std::fs;

pub fn process_name(pid: u32) -> String {
    fs::read_to_string(
        format!("/proc/{}/comm", pid)
    )
    .unwrap_or_else(|_| "unknown".into())
    .trim()
    .to_string()
}

pub fn executable(pid: u32) -> String {
    std::fs::read_link(
        format!("/proc/{}/exe", pid)
    )
    .map(|p| p.display().to_string())
    .unwrap_or_else(|_| "unknown".into())
}
