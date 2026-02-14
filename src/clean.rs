use std::fs;
use std::io;
use std::path::Path;

pub fn remove_source<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let p = path.as_ref();
    if p.exists() {
        println!("Cleaning up {}...", p.display());
        if p.is_dir() {
            fs::remove_dir_all(p)?;
        } else {
            fs::remove_file(p)?;
        }
    }
    Ok(())
}
