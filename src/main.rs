use linux_bundler::alpm_build;
use linux_bundler::appimage_build;
use linux_bundler::deb_build;
use linux_bundler::rpm_build;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    linux_bundler::build_package().await?;
    alpm_build()?;
    appimage_build()?;
    deb_build()?;
    //rpm_build()?;
    Ok(())
}
