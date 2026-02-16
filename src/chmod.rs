use super::metadata;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn chmod_package(pkg_dir: &str, is_appimage: bool) -> Result<(), Box<dyn std::error::Error>> {
    let metadata_path = "metadata";
    let metadata = metadata::extract_metadata(metadata_path)?;
    println!("Building Package: {}", metadata.name);

    // Use a 'bundle' directory to avoid conflicts with source files/directories of the same name
    let base_dir = PathBuf::from(pkg_dir);

    // Clean up previous bundle if it exists
    if base_dir.exists() {
        fs::remove_dir_all(&base_dir)?;
    }
    fs::create_dir_all(&base_dir)?;

    // -------- Process instructions --------
    for line in &metadata.package_instructions {
        process_install_line(line, &base_dir, &metadata.name, false)?;
    }

    if is_appimage {
        for line in &metadata.appimage_icon_instructions {
            process_install_line(line, &base_dir, &metadata.name, true)?;
        }
        for line in &metadata.appimage_desktop_instructions {
            process_install_line(line, &base_dir, &metadata.name, true)?;
        }
    }

    println!("Build Complete at {:?}", base_dir);
    Ok(())
}

fn process_install_line(
    line: &str,
    base_dir: &Path,
    metadata_name: &str,
    default_to_root: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let trimmed = line.trim();
    if !trimmed.starts_with("install") {
        return Ok(());
    }

    // Example: install -m755 "twincan" to "/usr/bin/"
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.len() < 3 {
        return Ok(());
    }

    // Mode
    let mut current_idx = 1;
    let mode_str = if parts[current_idx].starts_with("-m") {
        let m = parts[current_idx].trim_start_matches("-m");
        current_idx += 1;
        m
    } else {
        "644" // Default mode if not specified
    };
    let mode = u32::from_str_radix(mode_str, 8)?;

    // Source file
    if current_idx >= parts.len() {
        return Ok(());
    }
    let source_file_name = parts[current_idx].trim_matches('"').to_string();
    current_idx += 1;

    let source_dir = Path::new(metadata_name);
    let source_path = source_dir.join(&source_file_name);

    if !source_path.exists() {
        return Err(format!("Source not found: {:?}", source_path).into());
    }

    if source_path.is_dir() {
        return Err(format!("Source is a directory, not a file: {:?}", source_path).into());
    }

    // Destination
    let dest_path_str = if current_idx < parts.len() && parts[current_idx] == "to" {
        current_idx += 1;
        if current_idx < parts.len() {
            parts[current_idx].trim_matches('"').to_string()
        } else {
            "/".to_string()
        }
    } else if default_to_root {
        "/".to_string()
    } else {
        return Ok(()); // Skip if no destination and not defaulting to root
    };

    let dest_path_trimmed = dest_path_str.trim_start_matches('/');
    let mut full_dest = base_dir.join(dest_path_trimmed);

    // If destination is directory → append filename
    if dest_path_str.ends_with('/') || !Path::new(&dest_path_str).extension().is_some() {
        if let Some(file_name) = Path::new(&source_file_name).file_name() {
            full_dest = full_dest.join(file_name);
        }
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

    Ok(())
}
