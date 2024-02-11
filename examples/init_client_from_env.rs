use anyhow::Result;
use rust_moysklad::MoySkladApiClient;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = MoySkladApiClient::from_env()?;
    Ok(())
}
