use crate::metadata;
use alpm_types::MetadataFileName;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn create_build_info(input_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = input_path.as_ref();
    let mut file = File::create(&input_path.join(MetadataFileName::BuildInfo.as_ref()))?;
    let metadata = metadata::extract_metadata("metadata")?;
    write!(
        file,
        r#"format = 2
builddate = 1771092125
builddir = /build
startdir = /startdir/
buildtool = devtools
buildtoolver = 1:1.2.1-1-any
installed = other-example-1.2.3-1-any
packager = John Doe <john@example.org>
pkgarch = any
pkgbase = {}
pkgbuild_sha256sum = b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
pkgname = {}
pkgver = {}-{}
"#,
        metadata.name, metadata.name, metadata.version, metadata.release
    )?;
    Ok(())
}
