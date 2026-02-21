use crate::chmod::chmod_package;
use crate::metadata;
use std::fs::{self, File};
use std::path::PathBuf;

pub fn rpm_build() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = metadata::extract_metadata("metadata")?;
    let rpm_dir = format!("{}.rpm", metadata.name);
    let base_dir = PathBuf::from(&rpm_dir);

    // Prepare files in temporary directory
    chmod_package(&rpm_dir, false)?;

    let architecture = if metadata.arch.contains(&"x86_64".to_string()) {
        "x86_64"
    } else {
        "noarch"
    };

    let mut builder = rpm::PackageBuilder::new(
        &metadata.name,
        &metadata.version,
        &metadata.license,
        architecture,
        &metadata.description,
    );

    builder = builder.release(&metadata.release);

    // Add dependencies
    for dep in &metadata.rpm_depends {
        builder = builder.requires(rpm::Dependency::any(dep));
    }

    // Add provides
    for prov in &metadata.provides {
        builder = builder.provides(rpm::Dependency::any(prov));
    }

    // Add conflicts
    for conf in &metadata.conflicts {
        builder = builder.conflicts(rpm::Dependency::any(conf));
    }

    // Add files
    for entry in walkdir::WalkDir::new(&base_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(&base_dir)?;
            let target_path = format!("/{}", rel_path.to_string_lossy());

            // Get mode and add file
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let mode = fs::metadata(path)?.mode();
                builder = builder
                    .with_file(path, rpm::FileOptions::new(&target_path).mode(mode as i32))?;
            }
            #[cfg(not(unix))]
            {
                builder = builder.with_file(path, rpm::FileOptions::new(&target_path))?;
            }
        }
    }

    let package = builder.build()?;

    // Create output directory
    let output_path = std::env::current_dir()?.join("output");
    if !output_path.exists() {
        fs::create_dir_all(&output_path)?;
    }

    let rpm_file_path = output_path.join(format!(
        "{}-{}-{}.{}.rpm",
        metadata.name, metadata.version, metadata.release, architecture
    ));

    let mut f = File::create(&rpm_file_path)?;
    package.write(&mut f)?;

    println!(".rpm package created successfully!");
    println!("  Location: {}", rpm_file_path.display());

    // Cleanup
    //if base_dir.exists() {
    //  fs::remove_dir_all(&base_dir)?;
    //}

    Ok(())
}
