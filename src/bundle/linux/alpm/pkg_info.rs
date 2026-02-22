use crate::metadata;
use alpm_types::MetadataFileName;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn create_package_info(input_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = input_path.as_ref();
    let mut file = File::create(&input_path.join(MetadataFileName::PackageInfo.as_ref()))?;
    let metadata = metadata::extract_metadata("metadata")?;
    write!(
        file,
        r#"pkgname = {}
pkgbase = {}
xdata = pkgtype=pkg
pkgver = {}-{}
pkgdesc = {}
url = {}
builddate = {}
packager = {} <{}>
size = 1024
arch = {}
license = {}
conflict = {}
provides = {}
depend = {}
"#,
        metadata.name,
        metadata.name,
        metadata.version,
        metadata.release,
        metadata.description,
        metadata.url,
        super::build_info::get_build_date(),
        metadata.maintainer,
        metadata.email,
        metadata.arch.first().unwrap_or(&"any".to_string()),
        metadata.license,
        metadata.conflicts[0],
        metadata.provides[0],
        metadata.alpm_depends.join("\ndepend = ")
    )?;
    Ok(())
}
