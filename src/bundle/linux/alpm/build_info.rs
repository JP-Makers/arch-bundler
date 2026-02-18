use crate::metadata;
use alpm_buildinfo::BuildInfoV2;
use alpm_types::MetadataFileName;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

pub const BUILD_DATE: u64 = 1771092125;

pub fn create_build_info(input_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = input_path.as_ref();
    let buildinfo_path = input_path.join(MetadataFileName::BuildInfo.as_ref());
    let mut file = File::create(&buildinfo_path)?;

    let metadata = metadata::extract_metadata("metadata")?;

    let mut buildenv_str = String::new();
    for opt in &metadata.alpm_build_env {
        buildenv_str.push_str(&format!("buildenv = {}\n", opt));
    }

    let buildinfo_data = format!(
        r#"format = 2
pkgname = {}
pkgbase = {}
pkgver = {}-{}
pkgarch = {}
pkgbuild_sha256sum = {}
packager = Antigravity <antigravity@google.com>
builddate = {}
builddir = /build
startdir = /startdir/
buildtool = devtools
buildtoolver = 1:1.2.1-1-any
{}
installed = bar-1.2.3-1-any
"#,
        metadata.name,
        metadata.name,
        metadata.version,
        metadata.release,
        metadata.arch.first().unwrap_or(&"any".to_string()),
        metadata.sha256sums.first().unwrap_or(
            &"b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c".to_string()
        ),
        BUILD_DATE,
        buildenv_str
    );

    let buildinfo = BuildInfoV2::from_str(&buildinfo_data)?;

    write!(file, "{}", buildinfo)?;

    Ok(())
}
