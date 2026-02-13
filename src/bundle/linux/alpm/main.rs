use std::fs::{File, Permissions, create_dir_all};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use alpm_compress::compression::CompressionSettings;
use alpm_mtree::create_mtree_v2_from_input_dir;
use alpm_package::{InputDir, OutputDir, Package, PackageCreationConfig, PackageInput};
use alpm_types::MetadataFileName;
use tempfile::TempDir;

fn main() -> testresult::TestResult {
    // Create a temporary directory for input files only
    let temp_dir = TempDir::new()?;
    let input_dir = temp_dir.path().join("input");
    create_dir_all(&input_dir)?;
    let input_dir = InputDir::new(input_dir)?;

    // Use a permanent output directory in the current working directory
    let output_path = std::env::current_dir()?.join("output");
    let output_dir = OutputDir::new(output_path)?;

    // Create a valid, but minimal BUILDINFOv2 file.
    let mut file = File::create(&input_dir.join(MetadataFileName::BuildInfo.as_ref()))?;
    write!(
        file,
        r#"
format = 2
builddate = 1
builddir = /build
startdir = /startdir/
buildtool = devtools
buildtoolver = 1:1.2.1-1-any
installed = other-example-1.2.3-1-any
packager = John Doe <john@example.org>
pkgarch = any
pkgbase = example
pkgbuild_sha256sum = b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
pkgname = example
pkgver = 1:1.0.0-1
"#
    )?;

    // Create a valid, but minimal PKGINFOv2 file.
    let mut file = File::create(&input_dir.join(MetadataFileName::PackageInfo.as_ref()))?;
    write!(
        file,
        r#"
pkgname = example
pkgbase = example
xdata = pkgtype=pkg
pkgver = 1:1.0.0-1
pkgdesc = A project that returns true
url = https://example.org/
builddate = 1
packager = John Doe <john@example.org>
size = 181849963
arch = any
license = GPL-3.0-or-later
depend = bash
"#
    )?;

    // Create a dummy script as package data.
    create_dir_all(&input_dir.join("usr/bin"))?;
    let mut file = File::create(&input_dir.join("usr/bin/example"))?;
    write!(
        file,
        r#"!/bin/bash
true
"#
    )?;
    file.set_permissions(Permissions::from_mode(0o755))?;

    // Create a valid ALPM-MTREEv2 file from the input directory.
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
