pub fn classify(
    hostname: &str,
    port: u16,
) -> (u8, String, String, u8) {

    if port == 23 {
        return (
            90,
            "TELNET".into(),
            "PORT RULE".into(),
            95,
        );
    }

    if hostname.contains("tor") {
        return (
            80,
            "TOR".into(),
            "HOSTNAME_RULE".into(),
            99,
        );
    }

    if hostname.contains(".ru") {
        return (
            60,
            "HIGH_RISK_TLD".into(),
            "HOSTNAME_RULE".into(),
            99,
        );
    }

    (
        0,
        "NORMAL".into(),
        "NONE".into(),
        100,
    )
}
