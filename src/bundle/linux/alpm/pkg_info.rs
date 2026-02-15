use crate::metadata;
use alpm_types::MetadataFileName;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::build_info::BUILD_DATE;

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
packager = Antigravity <antigravity@google.com>
size = 1024
arch = {}
license = {}
depend = {}
"#,
        metadata.name,
        metadata.name,
        metadata.version,
        metadata.release,
        metadata.description,
        metadata.url,
        BUILD_DATE,
        metadata.arch.first().unwrap_or(&"any".to_string()),
        metadata.license,
        metadata.depends.join("\ndepend = ")
    )?;
    Ok(())
}
