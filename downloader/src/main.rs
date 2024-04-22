use reqwest::Client;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::copy;

#[derive(Debug, Deserialize, Serialize)]
struct Link {
    zip: String,
    xml: String,
}

/// MAKE SURE YOU CHANGE YOUR DEFAULT DOWNLOAD LOCATION OR YOUR DOWNLOAD FOLDER WILL NEVER BE THE SAME

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://e4ftl01.cr.usgs.gov/ASTT/ASTGTM.003/2000.03.01/";
    const JSON_PATH: &str = "C:\\test\\missing_links.json";

    let file = std::fs::File::open(JSON_PATH)?;
    let links: Vec<String> = serde_json::from_reader(file)?;

    let total_items = links.len();
    let time_per_item_secs = 0.5; // 5 seconds per item

    let total_time_secs = total_items as f64 * time_per_item_secs;

    println!(
        "Total time for processing all items: {} seconds",
        total_time_secs
    );

    let mut remaining_time_secs = total_time_secs;

    for (index, link) in links.iter().enumerate() {
        let zip_url = format!("{}{}", BASE_URL, link);

        // Open the URL or handle the error
        if let Err(err) = webbrowser::open(&zip_url) {
            eprintln!("Failed to open link {}: {}", &zip_url, err);
        }

        println!("Processed link {} of {}", index + 1, total_items);

        tokio::time::sleep(tokio::time::Duration::from_secs_f64(time_per_item_secs)).await;

        remaining_time_secs -= time_per_item_secs;
        println!("Time remaining: {} minutes", remaining_time_secs / 60.);
    }

    Ok(())
}
