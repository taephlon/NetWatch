use maxminddb::Reader;
use std::net::IpAddr;

pub struct GeoInfo {
    pub country: String,
    pub city: String,
}

pub fn lookup(ip: &str) -> GeoInfo {
    let reader =
        Reader::open_readfile(
            "data/GeoLite2-City.mmdb"
        );

    if reader.is_err() {
        return GeoInfo {
            country: "Unknown".into(),
            city: "Unknown".into(),
        };
    }

    let reader = reader.unwrap();

    let ip: IpAddr =
        match ip.parse() {
            Ok(ip) => ip,
            Err(_) => {
                return GeoInfo {
                    country: "Unknown".into(),
                    city: "Unknown".into(),
                }
            }
        };

    let result:
        Result<
            maxminddb::geoip2::City,
            _
        > = reader.lookup(ip);

    match result {
        Ok(city) => {
        let country = city.country
        .and_then(|c| c.names)
        .and_then(|n| n.get("en").copied())
        .unwrap_or("Unknown")
        .to_string();

        let city_name = city.city
        .and_then(|c| c.names)
        .and_then(|n| n.get("en").copied())
        .unwrap_or("Unknown")
        .to_string();

    GeoInfo {
        country,
        city: city_name,
    }
}
        Err(_) => GeoInfo {
            country: "Unknown".into(),
            city: "Unknown".into(),
        },
    }
}
