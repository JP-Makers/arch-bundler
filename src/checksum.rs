use super::metadata::Metadata;
use blake2::{Blake2b512, Digest};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fs::File;
use std::io::{self, Read};

fn compute_hash<D: Digest + Default>(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = D::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

pub fn md5_hash(path: &str) -> io::Result<String> {
    compute_hash::<Md5>(path)
}

pub fn sha1_hash(path: &str) -> io::Result<String> {
    compute_hash::<Sha1>(path)
}

pub fn blake2b512_hash(path: &str) -> io::Result<String> {
    compute_hash::<Blake2b512>(path)
}

pub fn sha224_hash(path: &str) -> io::Result<String> {
    compute_hash::<Sha224>(path)
}

pub fn sha256_hash(path: &str) -> io::Result<String> {
    compute_hash::<Sha256>(path)
}

pub fn sha384_hash(path: &str) -> io::Result<String> {
    compute_hash::<Sha384>(path)
}

pub fn sha512_hash(path: &str) -> io::Result<String> {
    compute_hash::<Sha512>(path)
}

pub fn verify_checksum(metadata: &Metadata, file_path: &str) -> io::Result<()> {
    let (expected_hashes, actual, algo_name) = if !metadata.sha512sums.is_empty() {
        (&metadata.sha512sums, sha512_hash(file_path)?, "SHA512")
    } else if !metadata.sha256sums.is_empty() {
        (&metadata.sha256sums, sha256_hash(file_path)?, "SHA256")
    } else if !metadata.sha1sums.is_empty() {
        (&metadata.sha1sums, sha1_hash(file_path)?, "SHA1")
    } else if !metadata.md5sums.is_empty() {
        (&metadata.md5sums, md5_hash(file_path)?, "MD5")
    } else {
        println!("No checksums found in metadata.");
        return Ok(());
    };

    println!("Verifying {} checksum...", algo_name);

    if expected_hashes
        .iter()
        .any(|expected| expected.eq_ignore_ascii_case("SKIP"))
    {
        println!("Actual:            {}", actual);
        return Ok(());
    }

    if expected_hashes
        .iter()
        .any(|expected| actual.eq_ignore_ascii_case(expected))
    {
        println!("Hash matches");
        println!("Expected (one of): {:?}", expected_hashes);
        println!("Actual:            {}", actual);
    } else {
        println!("Hash mismatch");
        println!("Expected (one of): {:?}", expected_hashes);
        println!("Actual:            {}", actual);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Hash mismatch"));
    }

    Ok(())
}
