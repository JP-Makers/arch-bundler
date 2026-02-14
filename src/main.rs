#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    linux_bundler::build_package().await?;
    Ok(())
}
