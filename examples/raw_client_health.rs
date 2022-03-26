use anyhow::Result;
use gamesense::client::GameSenseClient;
use serde_json::json;

// Reference: https://github.com/SteelSeries/gamesense-sdk/blob/master/doc/api/json-handlers-color.md

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = GameSenseClient::new("RAW_CLIENT_HEALTH", "Example Health Meter", "baysmith", None).await?;

    // Handler for percentage meter
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

    // Handlers for a digital F1-F10 meter
    let on_handler = |key| {
        json!({
            "device-type": "rgb-per-key-zones",
            "zone": key,
            "color": {
                "red": 255,
                "green": 0,
                "blue": 0,
            },
            "mode": "color",
        })
    };
    let off_handler = |key| {
        json!({
            "device-type": "rgb-per-key-zones",
            "zone": key,
            "color": {
                "red": 0,
                "green": 0,
                "blue": 0,
            },
            "mode": "color",
        })
    };

    let meter_event = |value| {
        let mut handlers = Vec::new();
        for f in 1..11 {
            if value >= f {
                handlers.push(on_handler(format!("f{}", f)));
            } else {
                handlers.push(off_handler(format!("f{}", f)));
            }
        }
        json!({
            "game": "RAW_CLIENT_HEALTH",
            "event": format!("HEALTH{}", value),
            "handlers": handlers,
        })
    };

    for health in 0..11 {
        client.raw_client.send_data("bind_game_event", &meter_event(health)).await?;
    }


    client.start_heartbeat();

    // Cycle health meter
    for health in (0..101).step_by(10).rev() {
        dbg!(health);
        client.raw_client.send_data("game_event", &json!({
            "game": "RAW_CLIENT_HEALTH",
            "event": "HEALTH",
            "data": {
                "value": health,
            }
        })).await?;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    // Cycle digital meter
    for value in (0..11).rev() {
        dbg!(value);
        client.raw_client.send_data("game_event", &json!({
            "game": "RAW_CLIENT_HEALTH",
            "event": format!("HEALTH{}", value),
            "data": {
                "value": 100,
            }
        })).await?;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    client.stop_heartbeat()?;

    client.raw_client.send_data("remove_game_event", &json!({
        "game": "RAW_CLIENT_HEALTH",
        "event": "HEALTH",
    })).await?;

    client.raw_client.send_data("stop_game", &json!({
        "game": "RAW_CLIENT_HEALTH",
    })).await?;

    client.raw_client.send_data("remove_game", &json!({
        "game": "RAW_CLIENT_HEALTH",
    })).await?;

    Ok(())
}
