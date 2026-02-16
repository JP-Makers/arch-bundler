use linux_bundler::alpm_build;
use linux_bundler::appimage_build;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    linux_bundler::build_package().await?;
    alpm_build()?;
    //appimage_build()?;
    Ok(())
}
