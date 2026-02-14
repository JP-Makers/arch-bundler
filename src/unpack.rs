use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use liblzma::read::XzDecoder;
use std::fs::File;
use std::io;
use std::path::Path;
use tar::Archive;
use zstd::stream::read::Decoder as ZstdDecoder;

pub fn unpack_source(path: &str, dest: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    println!("Unpacking {}...", path);

    match extension {
        "gz" => {
            let decoder = GzDecoder::new(file);
            let mut archive = Archive::new(decoder);
            archive.unpack(dest)?;
        }
        "xz" => {
            let decoder = XzDecoder::new(file);
            let mut archive = Archive::new(decoder);
            archive.unpack(dest)?;
        }
        "bz2" => {
            let decoder = BzDecoder::new(file);
            let mut archive = Archive::new(decoder);
            archive.unpack(dest)?;
        }
        "zst" => {
            let decoder = ZstdDecoder::new(file)?;
            let mut archive = Archive::new(decoder);
            archive.unpack(dest)?;
        }
        _ => {
            if path.ends_with(".tar") {
                let mut archive = Archive::new(file);
                archive.unpack(dest)?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unsupported extension: {}", extension),
                ));
            }
        }
    }

    Ok(())
}
