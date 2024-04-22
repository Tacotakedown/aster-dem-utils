use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Link {
    zip: String,
    xml: String,
}

fn main() {
    const JSON_PATH: &str = "C:\\test\\download_links.json";

    let file = std::fs::File::open(JSON_PATH).expect("failed to open file");
    let links: Vec<Link> = serde_json::from_reader(file).expect("failed to deserialize");

    let directory_path = "E:\\out";

    let zip_files = fs::read_dir(directory_path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().into_string().unwrap())
        .filter(|file_name| file_name.ends_with(".zip"))
        .collect::<Vec<_>>();

    let missing_files: Vec<_> = links
        .par_iter() // Parallel iterator
        .filter_map(|link| {
            if !zip_files.contains(&link.zip) {
                Some(link.zip.clone())
            } else {
                None
            }
        })
        .collect();

    write_missing_files_to_json("missing_files.json", &missing_files);
}
fn write_missing_files_to_json(file_path: &str, missing_files: &[String]) {
    let json_data = serde_json::to_string_pretty(missing_files).unwrap();

    fs::write(file_path, json_data).unwrap_or_else(|err| {
        eprintln!("Error writing JSON file: {}", err);
        std::process::exit(1);
    });

    println!("Missing files written to {}", file_path);
}
