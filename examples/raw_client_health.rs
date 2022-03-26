use anyhow::Result;
use gamesense::client::GameSenseClient;
use serde_json::json;

// Reference: https://github.com/SteelSeries/gamesense-sdk/blob/master/doc/api/json-handlers-color.md

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = GameSenseClient::new("RAW_CLIENT_HEALTH", "Example Health Meter", "baysmith", None).await?;

    client.raw_client.send_data("bind_game_event", &json!({
        "game": "RAW_CLIENT_HEALTH",
        "event": "HEALTH",
        "handlers": [{
            "device-type": "keyboard",
            "zone": "function-keys",
            "color": {
                "gradient": {
                    "zero": {
                        "red": 0,
                        "green": 0,
                        "blue": 0,
                    },
                    "hundred": {
                        "red": 0,
                        "green": 255,
                        "blue": 0,
                    },
                }
            },
            "mode": "percent",
        }],
    })).await?;

    client.start_heartbeat();
    for health in (0..101).step_by(10).rev() {
        client.raw_client.send_data("game_event", &json!({
            "game": "RAW_CLIENT_HEALTH",
            "event": "HEALTH",
            "data": {
                "value": health,
            }
        })).await?;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        dbg!(health);
    }
    client.stop_heartbeat()?;

    client.raw_client.send_data("remove_game_event", &json!({
        "game": "RAW_CLIENT_HEALTH",
        "event": "HEALTH",
    })).await?;

    client.raw_client.send_data("remove_game", &json!({
        "game": "RAW_CLIENT_HEALTH",
    })).await?;

    Ok(())
}
