use std::collections::HashSet;

pub struct ThreatFeed {
    pub bad_ips: HashSet<String>,
}

impl ThreatFeed {

    pub fn reload(
        &mut self
    ) {
        println!("Threat feed reloaded");
    }

    pub fn new() -> Self {
        let mut ips = HashSet::new();

        ips.insert("1.1.1.1".to_string());
        ips.insert("8.8.8.8".to_string());

        println!(
            "Loaded {} threat indicators",
            ips.len()
        );

        Self {
            bad_ips: ips,
        }
    }

    pub fn check_ip(
        &self,
        ip: &str,
    ) -> bool {
        let hit =
            self.bad_ips.contains(ip);

        if hit {
            println!(
                "[THREAT FEED] Match found: {}",
                ip
            );
        }

        hit
    }
}
