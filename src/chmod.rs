use super::metadata::Metadata;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn build_package(
    metadata_path: &str,
    metadata: &Metadata,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building Package: {}", metadata.name);

    // Use a 'bundle' directory to avoid conflicts with source files/directories of the same name
    let base_dir = PathBuf::from(&metadata.name);

    // Clean up previous bundle if it exists
    if base_dir.exists() {
        fs::remove_dir_all(&base_dir)?;
    }
    fs::create_dir_all(&base_dir)?;

    // -------- Process install lines --------
    let metadata_file = File::open(metadata_path)?;
    let reader = BufReader::new(metadata_file);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("install") {
            // Example: install -m755 "twincan" to "/usr/bin/"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            }

            // Mode
            let mode_str = if parts[1].starts_with("-m") {
                parts[1].trim_start_matches("-m")
            } else {
                parts[1]
            };
            let mode = u32::from_str_radix(mode_str, 8)?;

            // Source file with variable expansion
            let source_file_name = parts[2].trim_matches('"').to_string();
            //source_file_name = source_file_name.replace("$name", &metadata.name);
            let source_dir = Path::new(&metadata.sources[0]);
            let source_path = source_dir.join(&source_file_name);

            if !source_path.exists() {
                println!("Source not found: {:?}", source_path);
                continue;
            }

            if source_path.is_dir() {
                println!("Source is a directory, not a file: {:?}", source_path);
                continue;
            }

            // Destination with variable expansion
            let dest_path = parts[4].trim_matches('"').to_string();
            //dest_path_str = dest_path_str.replace("$name", &metadata.name);
            let dest_path_trimmed = dest_path.trim_start_matches('/');

            let mut full_dest = base_dir.join(dest_path_trimmed);

            // If destination is directory → append filename
            if dest_path.ends_with('/') || !Path::new(&dest_path).extension().is_some() {
                full_dest = full_dest.join(&source_file_name);
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
        }
    }

    println!("Build Complete at {:?}", base_dir);
    Ok(())
}
