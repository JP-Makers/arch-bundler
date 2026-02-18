use crate::chmod::chmod_package;
use crate::metadata;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tar::Builder;

pub fn deb_build() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = metadata::extract_metadata("metadata")?;
    let deb_dir = format!("{}.deb", metadata.name);
    let base_dir = PathBuf::from(&deb_dir);

    // Prepare files in temporary directory using chmod_package logic
    chmod_package(&deb_dir, false)?;

    // Create DEBIAN directory for control file
    let debian_dir = base_dir.join("DEBIAN");
    fs::create_dir_all(&debian_dir)?;

    // Create control file
    let control_path = debian_dir.join("control");
    let mut control_file = File::create(control_path)?;

    let architecture = if metadata.arch.contains(&"x86_64".to_string()) {
        "amd64"
    } else {
        "all"
    };

    writeln!(control_file, "Package: {}", metadata.name)?;
    writeln!(
        control_file,
        "Version: {}-{}",
        metadata.version, metadata.release
    )?;
    writeln!(control_file, "Section: utils")?;
    writeln!(control_file, "Priority: optional")?;
    writeln!(control_file, "Architecture: {}", architecture)?;
    if !metadata.deb_depends.is_empty() {
        writeln!(control_file, "Depends: {}", metadata.deb_depends.join(", "))?;
    }
    writeln!(
        control_file,
        "Maintainer: {} <{}>",
        metadata.maintainer, metadata.email
    )?;
    writeln!(control_file, "Description: {}", metadata.description)?;
    writeln!(control_file, "Homepage: {}", metadata.url)?;

    // Create debian-binary
    let debian_binary_path = base_dir.join("debian-binary");
    let mut debian_binary = File::create(&debian_binary_path)?;
    write!(debian_binary, "2.0\n")?;

    // Create data.tar.gz
    let data_tar_gz_path = base_dir.join("data.tar.gz");
    let data_tar_gz = File::create(&data_tar_gz_path)?;
    let enc = GzEncoder::new(data_tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    // Add all files from base_dir except DEBIAN, debian-binary, and the archives themselves
    for entry in walkdir::WalkDir::new(&base_dir) {
        let entry = entry?;
        let path = entry.path();
        let rel_path = path.strip_prefix(&base_dir)?;

        if rel_path.starts_with("DEBIAN")
            || rel_path == Path::new("debian-binary")
            || rel_path == Path::new("data.tar.gz")
            || rel_path == Path::new("control.tar.gz")
            || rel_path.as_os_str().is_empty()
        {
            continue;
        }

        if path.is_file() {
            tar.append_path_with_name(path, rel_path)?;
        } else if path.is_dir() {
            tar.append_dir(rel_path, path)?;
        }
    }
    tar.finish()?;
    let enc = tar.into_inner()?;
    enc.finish()?;

    // Create control.tar.gz
    let control_tar_gz_path = base_dir.join("control.tar.gz");
    let control_tar_gz = File::create(&control_tar_gz_path)?;
    let enc = GzEncoder::new(control_tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_path_with_name(debian_dir.join("control"), "control")?;
    tar.finish()?;
    let enc = tar.into_inner()?;
    enc.finish()?;

    // Combine into .deb using ar
    let output_path = std::env::current_dir()?.join("output");
    if !output_path.exists() {
        fs::create_dir_all(&output_path)?;
    }
    let deb_file_path = output_path.join(format!(
        "{}_{}-{}_{}.deb",
        metadata.name, metadata.version, metadata.release, architecture
    ));
    let deb_file = File::create(&deb_file_path)?;
    let mut ar = ar::Builder::new(deb_file);

    // Add files in order: debian-binary, control.tar.gz, data.tar.gz
    add_file_to_ar(&mut ar, &debian_binary_path, "debian-binary")?;
    add_file_to_ar(&mut ar, &control_tar_gz_path, "control.tar.gz")?;
    add_file_to_ar(&mut ar, &data_tar_gz_path, "data.tar.gz")?;

    println!(".deb package created successfully!");
    println!("  Location: {}", deb_file_path.display());

    // Cleanup
    //fs::remove_dir_all(&base_dir)?;

    Ok(())
}

fn add_file_to_ar<W: Write>(ar: &mut ar::Builder<W>, path: &Path, name: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    let header = ar::Header::from_metadata(name.as_bytes().to_vec(), &metadata);
    ar.append(&header, &mut file)
}
