#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define your callbacks for handling message data and error states
    let on_message_cb = Arc::new(|message: serde_json::Value| {
        println!("Received real-time feed: {}", message);
    });

    let on_error_cb = Arc::new(|error: serde_json::Value| {
        eprintln!("Socket encountered an error: {}", error);
    });

    // 2. Instantiate the FyersDataSocket client shell
    // Arguments match your new constructor:
    // new(access_token, write_to_file, log_path, litemode, reconnect, reconnect_retry, on_msg, on_err)
    let access_token = "YOUR_APP_ID:YOUR_DECODABLE_JWT_ACCESS_TOKEN";
    let client = FyersDataSocket::new(
        access_token,
        false, // write_to_file
        None,  // log_path
        false, // litemode (false = Full Mode, true = Lite Mode)
        true,  // reconnect on failure
        5,     // max reconnect retries
        Some(on_message_cb),
        Some(on_error_cb),
    );

    // 3. Fire up the background connection worker loop tasks
    // This decodes your JWT token and spawns the connection handlers into the tokio runtime
    println!("Connecting to Fyers WebSocket...");
    client.connect();

    // Give the network connection loop a brief moment to finish handshaking and opening fields
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // 4. Trigger your Symbol Subscriptions
    // The data_type argument maps directly to your token mapping parameters
    // standard channel indices range typically from 11 up to 30.
    let symbols_to_watch = vec!["NSE:SBIN-EQ".to_string(), "NSE:RELIANCE-EQ".to_string()];

    println!("Subscribing to stock ticks...");
    client.subscribe(symbols_to_watch, "SymbolUpdate", 11);

    // 5. Keep the asynchronous application runtime scope alive
    // This allows background tasks managed by tokio::spawn to stream updates indefinitely
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
