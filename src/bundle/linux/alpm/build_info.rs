use crate::metadata;
use alpm_buildinfo::BuildInfoV2;
use alpm_types::{BuildDate, FromOffsetDateTime, MetadataFileName};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use time::OffsetDateTime;

pub fn get_build_date() -> u64 {
    let now = OffsetDateTime::now_utc();
    // BuildDate might be i64 internally, cast to u64 for compatibility with the rest of the code
    i64::from(BuildDate::from_offset_datetime(now)) as u64
}

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
packager = {} <{}>
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
        metadata.pkgbuild_sha256sum,
        metadata.maintainer,
        metadata.email,
        get_build_date(),
        buildenv_str
    );

    let buildinfo = BuildInfoV2::from_str(&buildinfo_data)?;

    write!(file, "{}", buildinfo)?;

    Ok(())
}
