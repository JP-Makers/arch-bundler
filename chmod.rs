use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

pub fn build_package(
    metadata_path: &str,
    source_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {

    let metadata_file = File::open(metadata_path)?;
    let reader = BufReader::new(metadata_file);

    let mut package_name = String::new();

    // -------- Extract package name --------
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("name:") {
            package_name = trimmed
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
        }
    }

    if package_name.is_empty() {
        return Err("Package name not found".into());
    }

    println!("Package: {}", package_name);

    let base_dir = PathBuf::from(&package_name);
    fs::create_dir_all(&base_dir)?;

    // -------- Process install lines --------
    let metadata_file = File::open(metadata_path)?;
    let reader = BufReader::new(metadata_file);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("install") {
            // install -m755 "twincan" to "/usr/bin"

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            }

            // Mode
            let mode_str = parts[1].trim_start_matches("-m");
            let mode = u32::from_str_radix(mode_str, 8)?;

            // Source file
            let source_file_name = parts[2].trim_matches('"');
            let source_path = source_dir.join(source_file_name);

            if !source_path.exists() {
                println!("⚠ Source not found: {:?}", source_path);
                continue;
            }

            // Destination
            let mut dest_path = parts[4].trim_matches('"');
            dest_path = dest_path.trim_start_matches('/');

            let mut full_dest = base_dir.join(dest_path);

            // If destination is directory → append filename
            if dest_path.ends_with('/') || !Path::new(dest_path).extension().is_some() {
                full_dest = full_dest.join(source_file_name);
            }

            println!("Installing:");
            println!("  Mode: {}", mode_str);
            println!("  Source: {:?}", source_path);
            println!("  Dest: {:?}", full_dest);

            // Create parent directories
            if let Some(parent) = full_dest.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy
            fs::copy(&source_path, &full_dest)?;

            // Set permissions
            let mut perms = fs::metadata(&full_dest)?.permissions();
            perms.set_mode(mode);
            fs::set_permissions(&full_dest, perms)?;

            println!("  ✔ Installed\n");
        }
    }

    println!("Build Complete ✅");

    Ok(())
}
