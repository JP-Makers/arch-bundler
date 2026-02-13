use std::path::PathBuf;
mod chmod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define source directory here
    let source_dir = PathBuf::from("source");

    // Metadata file path
    let metadata_path = "metadata";

    chmod::build_package(metadata_path, source_dir)?;

    Ok(())
}
