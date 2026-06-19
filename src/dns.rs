use dns_lookup::lookup_addr;
use std::net::IpAddr;

pub fn reverse_lookup(ip: &str) -> String {

    if let Ok(addr) = ip.parse::<IpAddr>() {

        if let Ok(host) =
            lookup_addr(&addr)
        {
            return host;
        }
    }

    ip.to_string()
}
