extern crate gamesense;
extern crate anyhow;
use anyhow::{Result};
use gamesense::client::GameSenseClient;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = GameSenseClient::new("EVENT_LOOP", "Example Event Loop", "ptrstr", None).await?;
    client.register_event("EVENT").await?;
    client.start_heartbeat();
    for i in 0..60 {
        client.trigger_event("EVENT", i).await?;
    }
    client.stop_heartbeat()?;
    Ok(())
}
