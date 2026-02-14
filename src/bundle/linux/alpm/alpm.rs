use super::build_info::create_build_info;
use super::pkg_info::create_package_info;

use alpm_compress::compression::CompressionSettings;
use alpm_mtree::create_mtree_v2_from_input_dir;
use alpm_package::{InputDir, OutputDir, Package, PackageCreationConfig, PackageInput};

pub fn alpm_build() -> testresult::TestResult {
    // Create a temporary directory for input files only
    let input_dir = InputDir::new(std::env::current_dir()?.join("pkg"))?;

    // Use a permanent output directory in the current working directory
    let output_path = std::env::current_dir()?.join("output");
    let output_dir = OutputDir::new(output_path)?;

    // Create a valid, but minimal BUILDINFOv1 file.
    create_build_info(&input_dir)?;

    // Create a valid, but minimal PKGINFOv1 file.
    create_package_info(&input_dir)?;

    // Create a valid ALPM-MTREEv1 file from the input directory.
    create_mtree_v2_from_input_dir(&input_dir)?;

    // Create PackageInput and PackageCreationConfig.
    let package_input: PackageInput = input_dir.try_into()?;
    let config =
        PackageCreationConfig::new(package_input, output_dir, CompressionSettings::default())?;
    // Create package file.
    Package::try_from(&config)?;

    println!("✓ Package created successfully!");
    println!("  Location: {}/output/", std::env::current_dir()?.display());
    Ok(())
}
