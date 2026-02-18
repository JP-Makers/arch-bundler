use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::os::unix::fs::PermissionsExt;

use crate::bundle::linux::appimage::squashfs::squashfs_build;
use crate::chmod::chmod_package;
use crate::metadata;

static RUNTIME: &[u8] = include_bytes!("apprun/runtime-x86_64");

pub fn appimage_build() -> Result<(), Box<dyn std::error::Error>> {
    //Extract metadata
    let metadata = metadata::extract_metadata("metadata")?;

    //Check if metadata is valid
    if metadata.appimage_exec.is_empty() {
        return Err("appimage_exec not found".into());
    }

    if metadata.appimage_desktop_instructions.is_empty() {
        return Err("appimage_desktop not found".into());
    }

    if metadata.appimage_icon_instructions.is_empty() {
        return Err("appimage_icon not found".into());
    }

    //Create AppDir
    let base_dir = format!("{}.AppDir", metadata.name);
    chmod_package(&base_dir, true)?;

    //Create output directory
    let output_dir = std::env::current_dir()?.join("output");
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }

    let base_name = format!(
        "{}-{}-{}-{}",
        metadata.name, metadata.version, metadata.release, metadata.arch[0]
    );

    //Add AppRun
    let apprun_path = std::path::Path::new(&base_dir).join("AppRun");
    let mut file = File::create(&apprun_path)?;

    let content = format!(
        "#!/bin/bash\nHERE=\"$(dirname \"$(readlink -f \"$0\")\")\"\nexec \"$HERE{}\" \"$@\"\n",
        metadata.appimage_exec
    );

    file.write_all(content.as_bytes())?;

    #[cfg(unix)]
    file.set_permissions(std::fs::Permissions::from_mode(0o755))?;

    //Squashfs build
    squashfs_build(&base_dir, &base_name)?;

    //Remove AppDir
    //std::fs::remove_dir_all(&base_dir)?;

    let squashfs_path = format!("{}.squashfs", base_name);
    let mut squashfs_data = BufReader::new(File::open(&squashfs_path)?);

    //Create AppImage with 755 permission
    let mut output_file = File::create(output_dir.join(format!("{}.AppImage", base_name)))?;

    #[cfg(unix)]
    output_file.set_permissions(std::fs::Permissions::from_mode(0o755))?;

    let mut out_writer = BufWriter::new(&mut output_file);
    out_writer.write_all(RUNTIME)?;
    std::io::copy(&mut squashfs_data, &mut out_writer)?;

    //Remove squashfs file
    //std::fs::remove_file(&squashfs_path)?;

    Ok(())
}
