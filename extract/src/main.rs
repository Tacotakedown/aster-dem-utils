use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;
use zip::read::ZipArchive;

fn main() -> std::io::Result<()> {
    let input_dir = "C:\\test\\out";
    let output_dir = "E:\\unzipped";
    let start_time = Instant::now();

    let entries = std::fs::read_dir(input_dir)?;

    let total_zip_files = entries.filter_map(|entry| entry.ok()).count();

    let entries = std::fs::read_dir(input_dir)?;

    for (entry_idx, entry) in entries.enumerate() {
        let entry = entry?;
        let file_path = entry.path();

        let file = File::open(&file_path)?;
        let mut archive = ZipArchive::new(file)?;

        let archive_len = archive.len();

        for i in 0..archive_len {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            if file_name.ends_with(".tif") || file_name.ends_with(".tiff") {
                print!(
                    "\rUnzipping file: {} Progress: {}/{}",
                    file_name,
                    entry_idx + 1,
                    total_zip_files
                );
                io::stdout().flush()?; // Flush the stdout to ensure the output is printed immediately

                let output_path = format!("{}/{}", output_dir, file_name);

                std::fs::create_dir_all(
                    output_path
                        .rsplit('/')
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join("/"),
                )?;

                let mut output_file = File::create(output_path)?;
                std::io::copy(&mut file, &mut output_file)?;
            }
        }
    }

    println!("Complete, time taken: {:?}", start_time.elapsed());

    Ok(())
}
