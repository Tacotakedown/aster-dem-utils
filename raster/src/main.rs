use geotiff::TIFF;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::{self, Write};

// Define a struct to store elevation data
#[derive(Debug, Clone)]
struct ElevationData {
    lon: f64,
    lat: f64,
    elevation: usize,
}

#[derive(Debug)]
struct Coordinates {
    lon: f64,
    lat: f64,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.lon == other.lon && self.lat == other.lat
    }
}

// Implement Eq for Coordinates by delegating to PartialEq
impl Eq for Coordinates {}

impl std::hash::Hash for Coordinates {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

impl Coordinates {
    // Custom hash function for Coordinates
    fn hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.lon.to_bits().hash(&mut hasher);
        self.lat.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

// Implementing Hash for Coordinates based on the custom hash function

// Function to parse dataset and write combined data to binary file
fn parse_and_write_dataset(
    input_folder: &str,
    output_file: &str,
    resolution: f64,
) -> Result<(), io::Error> {
    // Create the output binary file
    let mut output_file = File::create(output_file)?;

    // Track processed coordinates to avoid duplicate entries
    let mut processed_coords = HashSet::new();

    // Open the output binary file for writing
    for entry in std::fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // Parse the longitude and latitude from the file name
        let (lon, lat) = parse_lon_lat_from_filename(file_name);

        // Check if coordinates are already processed
        let coords = Coordinates { lon, lat };
        if processed_coords.contains(&coords) {
            continue; // Skip duplicate coordinates
        }

        // Open the TIFF file
        let tiff = TIFF::open(&path.to_string_lossy());
        let tiff = tiff.expect("failed to extract tiff from box");
        // Extract elevation data based on resolution
        let elevation_data = extract_elevation_data(&tiff, resolution, lon, lat).unwrap();
        // Write elevation data to the binary file
        for data_point in elevation_data {
            writeln!(
                output_file,
                "{},{},{}",
                data_point.lon, data_point.lat, data_point.elevation
            )?;
        }

        // Mark coordinates as processed
        processed_coords.insert(coords);
    }

    Ok(())
}

// Function to parse longitude and latitude from file name
fn parse_lon_lat_from_filename(file_name: &str) -> (f64, f64) {
    // Implement logic to extract lon and lat from file name
    // Example implementation:
    let lon = file_name[13..16].parse::<f64>().unwrap_or(0.0);
    let lat = file_name[17..20].parse::<f64>().unwrap_or(0.0);
    (lon, lat)
}

// Function to extract elevation data based on resolution
fn extract_elevation_data(
    tiff: &TIFF,
    resolution: f64,
    lon: f64,
    lat: f64,
) -> Option<Vec<ElevationData>> {
    // Calculate number of samples and lines based on resolution
    let samples = (1.0 / resolution).round() as usize;
    let lines = (1.0 / resolution).round() as usize;

    let mut elevation_data = vec![];

    // Loop through coordinates based on resolution and extract elevation data
    for i in 0..samples {
        for j in 0..lines {
            let lon_offset = i as f64 * resolution;
            let lat_offset = j as f64 * resolution;
            if let elevation =
                tiff.get_value_at((lon + lon_offset) as usize, (lat + lat_offset) as usize)
            {
                elevation_data.push(ElevationData {
                    lon: lon + lon_offset,
                    lat: lat + lat_offset,
                    elevation,
                });
            }
        }
    }

    if elevation_data.is_empty() {
        None
    } else {
        Some(elevation_data)
    }
}

fn main() {
    let input_folder = "E:\\unzipped";
    let output_file = "combined_data.bin";
    let resolution = 1.; // Adjust resolution as needed (in degrees)

    if let Err(err) = parse_and_write_dataset(input_folder, output_file, resolution) {
        eprintln!("Error: {}", err);
    } else {
        println!(
            "Dataset parsed and combined data written to {}",
            output_file
        );
    }
}
