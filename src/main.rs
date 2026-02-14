mod bundle;
mod chmod;
mod metadata;

use crate::bundle::linux::alpm::alpm_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Metadata file path
    let metadata_path = "metadata";
    metadata::extract_metadata(metadata_path)?;

    // Extract all metadata
    let metadata = metadata::extract_all_metadata(metadata_path)?;

    // Display metadata (optional, can be called via metadata::extract_metadata if intended)
    // metadata::extract_metadata(metadata_path)?;

    // Build package
    chmod::build_package(metadata_path, &metadata)?;

    alpm_build();

    Ok(())
}
