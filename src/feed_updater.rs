use std::{
    fs,
    time::Duration,
};

pub async fn update_feed() {

    let url =
        "https://raw.githubusercontent.com/stamparm/ipsum/master/ipsum.txt";

    println!(
        "[FEED] Downloading..."
    );

    let response =
        reqwest::get(url).await;

    match response {

        Ok(resp) => {

            match resp.text().await {

                Ok(text) => {

                    let mut output =
                        String::new();

                    for line in text.lines() {

                        if line.starts_with('#') {
                            continue;
                        }

                        let parts:
                            Vec<&str> =
                                line.split_whitespace()
                                    .collect();

                        if let Some(ip) =
                            parts.first()
                        {
                            output.push_str(ip);
                            output.push('\n');
                        }
                    }

                    let _ =
                        fs::write(
                            "data/malware_ips.txt",
                            output,
                        );

                    println!(
                        "[FEED] Updated"
                    );
                }

                Err(err) => {
                    println!(
                        "[FEED] {}",
                        err
                    );
                }
            }
        }

        Err(err) => {
            println!(
                "[FEED] {}",
                err
            );
        }
    }
}

pub async fn scheduler() {

    loop {

        update_feed().await;

        tokio::time::sleep(
            Duration::from_secs(
                60 * 60
            )
        )
        .await;
    }
}
