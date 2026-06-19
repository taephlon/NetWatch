pub fn classify(
    hostname: &str,
    port: u16,
) -> (u8, String) {

    if port == 23 {
        return (
            90,
            "TELNET".into(),
        );
    }

    if hostname.contains("tor") {
        return (
            80,
            "TOR".into(),
        );
    }

    if hostname.contains(".ru") {
        return (
            60,
            "HIGH_RISK_TLD".into(),
        );
    }

    (
        0,
        "NORMAL".into(),
    )
}
