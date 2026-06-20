pub fn classify_ip(ip: &str) -> (String, String, i32) {

    if ip.starts_with("185.") {
        return (
            "Suspicious".into(),
            "Local IOC Feed".into(),
            70,
        );
    }

    if ip.starts_with("45.") {
        return (
            "Known Scanner".into(),
            "Abuse Feed".into(),
            90,
        );
    }

    (
        "Clean".into(),
        "None".into(),
        0,
    )
}
