use std::sync::Arc;

use serde::Serialize;

use gtk4::{Application, ApplicationWindow, Box, Orientation, prelude::*};
use serde_json::Value;

use crate::{
    backend::hsm::data_socket::FyersDataSocket,
    frontend::{
        orderbook::{model::OrderBook, update::update_orderbook},
        trades::trades,
    },
    model::enums::DerivativeType::{self, OPTIONS},
    ui::load_custom_css,
};

mod backend;
mod frontend;
mod model;
mod ui;

// --- 3. MAIN APP ENTRY POINT ---
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.fyers.tbt.orderbook")
        .build();

    app.connect_startup(|_| {
        load_custom_css();
    });
    app.connect_activate(build_ui);
    app.run()
}

// --- 4. GTK UI INITIALIZATION & THREAD BRIDGING ---
fn build_ui(app: &Application) {
    // Create a GLib channel to communicate from the Tokio background thread to the GTK main thread
    let (sender, mut receiver) =
        tokio::sync::mpsc::unbounded_channel::<backend::fyers_tbt::MarketFeed>();

    //
    let (order_book_widget_fut, store_fut) = frontend::orderbook::order_book();
    let (order_book_widget_ce, store_ce) = frontend::orderbook::order_book();
    let (order_book_widget_pe, store_pe) = frontend::orderbook::order_book();

    let main_layout = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // main_layout.append(&trades());
    main_layout.append(&order_book_widget_ce);
    main_layout.append(&order_book_widget_fut);
    main_layout.append(&order_book_widget_pe);
    //main_layout.append(&trades());

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fyers Tick-by-Tick Order Book")
        .fullscreened(true)
        .child(&main_layout)
        .build();

    window.present();

    // Spawn a standard OS thread that will host the Tokio async runtime
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        // rt.block_on(async {
        //     if let Err(e) = backend::tbt_ws::run_fyers_websocket(sender).await {
        //         eprintln!("WebSocket Client Error: {:#?}", e);
        //     }
        // });
        rt.block_on(async {
            // let app_id = "xx";
            // let access_token = "xxx"; // Replace with your Access Token

            // let on_message = Arc::new(|msg: Value| {
            //     println!("📥 [Fyers Message]: {}", msg);
            // });

            // let on_error = Arc::new(|err: Value| {
            //     eprintln!("❌ [Fyers Error]: {}", err);
            // });

            // // 3. Initialize the WebSocket struct
            // let socket = FyersDataSocket::new(
            //     format!("{}:{}", app_id, access_token).as_str(),
            //     false, // write_to_file
            //     None,  // log_path
            //     false, // litemode (false = Full mode)
            //     true,  // auto-reconnect
            //     5,     // max reconnect retries
            //     Some(on_message as Arc<dyn Fn(Value) + Send + Sync>),
            //     Some(on_error as Arc<dyn Fn(Value) + Send + Sync>),
            // );

            // println!("Starting Fyers WebSocket connection...");

            // // 4. Connect to the WebSocket (this spawns background Tokio tasks and returns immediately)
            // socket.connect();

            // // 5. Wait a brief moment to allow the handshake and HSM token auth to complete
            // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

            // // 6. Subscribe to your symbols
            // let symbols = vec![
            //     "NSE:NIFTY2670724450CE".to_string(),
            //     "NSE:NIFTY50-INDEX".to_string(),
            // ];

            // println!("Subscribing to symbols: {:?}", symbols);

            // // Note: Since we changed `subscribe` to be `async` in the previous fix, we must `.await` it here!
            // socket.subscribe(symbols.clone(), "SymbolUpdate", 11).await;

            // // 7. Keep the main thread alive indefinitely to listen to the incoming stream.
            // // We use tokio::signal::ctrl_c() to cleanly catch a Ctrl+C shutdown.
            // println!("Streaming active. Press Ctrl+C to exit.");
            // match tokio::signal::ctrl_c().await {
            //     Ok(()) => {
            //         println!("\nShutting down Fyers socket...");
            //     }
            //     Err(err) => {
            //         eprintln!("Unable to listen for shutdown signal: {}", err);
            //     }
            // }

            if let Err(e) = backend::tbt_ws::run_fyers_websocket(sender).await {
                eprintln!("WebSocket Client Error: {:#?}", e);
            }
        });
    });

    glib::MainContext::default().spawn_local(async move {
        while let Some(feed) = receiver.recv().await {
            if feed.ticker.ends_with("FUT") {
                update_orderbook(&store_fut, feed, DerivativeType::FUTURES);
            } else if feed.ticker.ends_with("CE") {
                update_orderbook(&store_ce, feed, DerivativeType::OPTIONS);
            } else if feed.ticker.ends_with("PE") {
                update_orderbook(&store_pe, feed, DerivativeType::OPTIONS);
            }
        }
    });
}
