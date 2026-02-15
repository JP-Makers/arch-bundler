mod bundle;
mod checksum;
mod chmod;
mod clean;
mod clone;
mod metadata;
mod unpack;

pub use bundle::linux::alpm::alpm_build;
pub use checksum::verify_checksum;
pub use chmod::chmod_package;
pub use clone::fetch_source;
pub use metadata::Metadata;
pub use unpack::unpack_source;

pub async fn build_package() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = metadata::extract_metadata("metadata")?;

    for source in &metadata.sources {
        let filename = fetch_source(source).await?;

        // Verify checksum BEFORE unpacking
        verify_checksum(&metadata, &filename)?;

        // If it's an archive, unpack it
        if filename.ends_with(".tar.gz")
            || filename.ends_with(".tar.xz")
            || filename.ends_with(".tar.bz2")
            || filename.ends_with(".tar.zst")
            || filename.ends_with(".tar")
        {
            unpack_source(&filename, &metadata.name)?;
            clean::remove_source(&filename)?;
        }
    }

    chmod_package("metadata", &metadata)?;
    alpm_build()?;
    // clean::remove_source("pkg")?;
    Ok(())
}
