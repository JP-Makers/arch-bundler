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
builddate = 1771092125
packager = John Doe <john@example.org>
size = 181849963
arch = any
license = GPL-3.0-or-later
depend = bash
"#,
        metadata.name,
        metadata.name,
        metadata.version,
        metadata.release,
        metadata.description,
        metadata.url
    )?;
    Ok(())
}
