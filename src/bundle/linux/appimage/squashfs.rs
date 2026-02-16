use backhand::{
    FilesystemCompressor, FilesystemWriter, NodeHeader,
    compression::{CompressionOptions, Compressor, Zstd},
};
use std::{fs, path::Path};
use walkdir::WalkDir;

pub fn squashfs_build(app_dir: &str, output_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(app_dir).exists() {
        return Err(format!("Directory {} not found", app_dir).into());
    }

    let mut fs_writer = FilesystemWriter::default();

    // Configure Zstd level 20
    fs_writer.set_compressor(FilesystemCompressor::new(
        Compressor::Zstd,
        Some(CompressionOptions::Zstd(Zstd {
            compression_level: 20,
        })),
    )?);

    // Set root permissions
    fs_writer.set_root_uid(0);
    fs_writer.set_root_gid(0);
    fs_writer.set_root_mode(0o755);

    println!(
        "Converting {} to SquashFS (level 20, 755 permissions)...",
        app_dir
    );

    // Metadata: root-owned, 755 permissions
    let header = NodeHeader::new(0o755, 0, 0, 0);

    for entry in WalkDir::new(app_dir).min_depth(1) {
        let entry = entry?;
        let rel_path = entry.path().strip_prefix(app_dir)?;

        if entry.file_type().is_dir() {
            fs_writer.push_dir(rel_path, header)?;
        } else if entry.file_type().is_file() {
            fs_writer.push_file(fs::File::open(entry.path())?, rel_path, header)?;
        } else if entry.file_type().is_symlink() {
            fs_writer.push_symlink(fs::read_link(entry.path())?, rel_path, header)?;
        }
    }

    fs_writer.write(&mut fs::File::create(format!("{}.squashfs", output_name))?)?;

    println!("Created {}.squashfs successfully!", output_name);
    Ok(())
}
