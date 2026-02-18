use crate::chmod::chmod_package;
use crate::metadata;
use flate2::Compression;
use flate2::write::GzEncoder;
use md5::{Digest, Md5};
use std::fs::{self, File};
use std::io::{self, Read, Write};
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
    writeln!(control_file, "Architecture: {}", architecture)?;
    writeln!(
        control_file,
        "Maintainer: {} <{}>",
        metadata.maintainer, metadata.email
    )?;
    if !metadata.deb_depends.is_empty() {
        writeln!(control_file, "Depends: {}", metadata.deb_depends.join(", "))?;
    }
    if !metadata.conflicts.is_empty() {
        writeln!(control_file, "Conflicts: {}", metadata.conflicts.join(", "))?;
    }
    if !metadata.provides.is_empty() {
        writeln!(control_file, "Provides: {}", metadata.provides.join(", "))?;
    }
    writeln!(control_file, "Section: utils")?;
    writeln!(control_file, "Priority: optional")?;
    writeln!(control_file, "Homepage: {}", metadata.url)?;
    writeln!(control_file, "Description: {}", metadata.description)?;

    // Create postrm script
    let postrm_path = debian_dir.join("postrm");
    let mut postrm_file = File::create(&postrm_path)?;
    write!(
        postrm_file,
        "#!/bin/sh\nset -e\nif [ \"$1\" = \"remove\" ] || [ \"$1\" = \"purge\" ]; then\n    apt-get autoremove -y 2>/dev/null || true\nfi\n"
    )?;
    // Make postrm executable (chmod 755)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&postrm_path, fs::Permissions::from_mode(0o755))?;
    }

    // Create debian-binary
    let debian_binary_path = base_dir.join("debian-binary");
    let mut debian_binary = File::create(&debian_binary_path)?;
    write!(debian_binary, "2.0\n")?;

    // Collect data files and compute md5sums
    let mut md5sums_entries: Vec<(String, String)> = Vec::new();
    let mut data_files: Vec<(PathBuf, PathBuf)> = Vec::new(); // (abs_path, rel_path)

    for entry in walkdir::WalkDir::new(&base_dir) {
        let entry = entry?;
        let path = entry.path().to_path_buf();
        let rel_path = path.strip_prefix(&base_dir)?.to_path_buf();

        if rel_path.starts_with("DEBIAN")
            || rel_path == Path::new("debian-binary")
            || rel_path == Path::new("data.tar.gz")
            || rel_path == Path::new("control.tar.gz")
            || rel_path.as_os_str().is_empty()
        {
            continue;
        }

        if path.is_file() {
            // Compute MD5
            let mut file_data = Vec::new();
            File::open(&path)?.read_to_end(&mut file_data)?;
            let mut hasher = Md5::new();
            hasher.update(&file_data);
            let digest = hasher.finalize();
            let rel_str = rel_path.to_string_lossy().into_owned();
            md5sums_entries.push((format!("{:x}", digest), rel_str.clone()));
            data_files.push((path, rel_path));
        } else if path.is_dir() {
            data_files.push((path, rel_path));
        }
    }

    // Create md5sums file in DEBIAN/
    let md5sums_path = debian_dir.join("md5sums");
    let mut md5sums_file = File::create(&md5sums_path)?;
    for (hash, rel) in &md5sums_entries {
        writeln!(md5sums_file, "{}  {}", hash, rel)?;
    }

    // Create data.tar.gz
    let data_tar_gz_path = base_dir.join("data.tar.gz");
    let data_tar_gz = File::create(&data_tar_gz_path)?;
    let enc = GzEncoder::new(data_tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    for (abs_path, rel_path) in &data_files {
        if abs_path.is_file() {
            tar.append_path_with_name(abs_path, rel_path)?;
        } else if abs_path.is_dir() {
            tar.append_dir(rel_path, abs_path)?;
        }
    }
    tar.finish()?;
    let enc = tar.into_inner()?;
    enc.finish()?;

    // Create control.tar.gz (includes control, md5sums, postrm)
    let control_tar_gz_path = base_dir.join("control.tar.gz");
    let control_tar_gz = File::create(&control_tar_gz_path)?;
    let enc = GzEncoder::new(control_tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_path_with_name(debian_dir.join("control"), "control")?;
    tar.append_path_with_name(&md5sums_path, "md5sums")?;
    tar.append_path_with_name(&postrm_path, "postrm")?;
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
