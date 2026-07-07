use futures_util::{SinkExt, StreamExt};
use http::Request;
use prost::Message;
use serde::Serialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use url::Url;

#[derive(Serialize)]
struct SubscribeData {
    subs: i32,
    symbols: Vec<String>,
    mode: String,
    channel: String,
}

#[derive(Serialize)]
struct SubscribeMsg {
    #[serde(rename = "type")]
    msg_type: i32,
    data: SubscribeData,
}

#[derive(Serialize)]
struct SwitchChannelData {
    #[serde(rename = "resumeChannels")]
    resume_channels: Vec<String>,
    #[serde(rename = "pauseChannels")]
    pause_channels: Vec<String>,
}

#[derive(Serialize)]
struct SwitchChannelMsg {
    #[serde(rename = "type")]
    msg_type: i32,
    data: SwitchChannelData,
}

pub async fn run_fyers_websocket(
    sender: tokio::sync::mpsc::UnboundedSender<super::fyers_tbt::MarketFeed>,
) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let app_id = "xx";
    let access_token = "xx"; // Replace with your Access Token
    let ws_url = "wss://rtsocket-api.fyers.in/versova";

    let url = Url::parse(ws_url).unwrap();
    let auth_header = format!("{}:{}", app_id, access_token);

    let request = Request::builder()
        .uri(url.as_str())
        .header("Authorization", auth_header)
        .header("Host", url.host_str().unwrap())
        .header("Upgrade", "websocket")
        .header("Connection", "Upgrade")
        .header(
            "Sec-WebSocket-Key",
            tokio_tungstenite::tungstenite::handshake::client::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(())?;

    println!("Connecting to Fyers TBT WebSocket...");

    let (ws_stream, _) = connect_async(request)
        .await
        .expect("Failed to connect to Fyers WebSocket");
    println!("WebSocket connected successfully!");

    let (mut write, mut read) = ws_stream.split();

    let sub_req = SubscribeMsg {
        msg_type: 1,
        data: SubscribeData {
            subs: 1,
            symbols: vec![
                "NSE:NIFTY26JULFUT".to_string(),
                "NSE:NIFTY2670724450CE".to_string(),
                "NSE:NIFTY2670724450PE".to_string(),
            ],
            mode: "depth".to_string(),
            channel: "1".to_string(),
        },
    };

    let sub_json = serde_json::to_string(&sub_req)?;
    write.send(WsMessage::Text(sub_json)).await?;
    println!("Subscription sent.");

    let switch_req = SwitchChannelMsg {
        msg_type: 2,
        data: SwitchChannelData {
            resume_channels: vec!["1".to_string()],
            pause_channels: vec![],
        },
    };

    let switch_json = serde_json::to_string(&switch_req)?;
    write.send(WsMessage::Text(switch_json)).await?;
    println!("Channel 1 resumed.");

    println!("Listening for tick data...\n");
    while let Some(message) = read.next().await {
        match message {
            Ok(WsMessage::Binary(bin_data)) => {
                if let Ok(decoded_msg) = super::fyers_tbt::SocketMessage::decode(&bin_data[..]) {
                    if decoded_msg.error {
                        eprintln!("Fyers Error: {}", decoded_msg.msg);
                        continue;
                    }
                    for (_, feed) in decoded_msg.feeds {
                        let _ = sender.send(feed);
                    }
                }
            }
            Ok(WsMessage::Ping(ping)) => {
                println!("ping");
                write.send(WsMessage::Pong(ping)).await?;
            }
            _ => {}
        }
    }

    Ok(())
}
