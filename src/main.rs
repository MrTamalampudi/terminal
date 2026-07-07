use std::sync::Arc;

use serde::Serialize;

use gtk4::{Application, ApplicationWindow, Box, Orientation, prelude::*};
use serde_json::Value;

use crate::{
    backend::hsm::data_socket::FyersDataSocket,
    frontend::{orderbook::model::OrderBook, trades::trades},
    ui::load_custom_css,
};

mod backend;
mod frontend;
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
                update_store_with_feed(&store_fut, feed);
            } else if feed.ticker.ends_with("CE") {
                update_store_with_feed(&store_ce, feed);
            } else if feed.ticker.ends_with("PE") {
                update_store_with_feed(&store_pe, feed);
            }
        }
    });
}

fn update_store_with_feed(store: &OrderBook, feed: backend::fyers_tbt::MarketFeed) {
    if let Some(depth) = feed.depth {
        let mut bids_data = store.bids.borrow_mut();
        let mut asks_data = store.asks.borrow_mut();

        // ==========================================
        // 1. UPDATE DATA MODELS FROM FEED
        // ==========================================

        // Update Bids Data
        for bid in &depth.bids {
            let idx = bid.num.unwrap_or(0) as usize;
            if idx < 50 {
                if let Some(bid_) = bids_data.get_mut(idx) {
                    if let Some(value) = bid.qty {
                        bid_.bid_qty = value;
                    }
                    if let Some(value) = bid.nord {
                        bid_.bid_orders = value;
                    }
                    if let Some(value) = bid.price {
                        bid_.bid_price = value;
                    }
                }
            }
        }

        // Update Asks Data
        for ask in &depth.asks {
            let idx = ask.num.unwrap_or(0) as usize;
            if idx < 50 {
                if let Some(ask_) = asks_data.get_mut(idx) {
                    if let Some(value) = ask.qty {
                        ask_.ask_qty = value;
                    }
                    if let Some(value) = ask.nord {
                        ask_.ask_orders = value;
                    }
                    if let Some(value) = ask.price {
                        ask_.ask_price = value;
                    }
                }
            }
        }

        // ==========================================
        // 2. FIND THE MAXIMUM QUANTITIES
        // ==========================================
        let max_bid_qty = bids_data.iter().map(|b| b.bid_qty).max().unwrap_or(0);
        let max_ask_qty = asks_data.iter().map(|a| a.ask_qty).max().unwrap_or(0);
        // Update Bids
        for bid in depth.bids {
            let idx = bid.num.unwrap_or(0);
            if idx < 50 {
                // Fetch existing row, modify it, and splice it back to trigger UI update
                if let Some(row) = store.book.get(idx as usize) {
                    if let Some(value) = bid.qty {
                        row.bid_qty.set_label(&value.to_string());
                    }
                    if let Some(value) = bid.nord {
                        row.bid_orders.set_label(&format!("({})", value));
                    }
                    if let Some(value) = bid.price {
                        row.bid_price
                            .set_label(&format!("{:.2}", value as f32 / 100.0));
                    }
                }
            }
        }

        // Update Asks
        for ask in depth.asks {
            let idx = ask.num.unwrap_or(0);
            if idx < 50 {
                if let Some(row) = store.book.get(idx as usize) {
                    if let Some(value) = ask.qty {
                        row.ask_qty.set_label(&value.to_string());
                    }
                    if let Some(value) = ask.nord {
                        row.ask_orders.set_label(&format!("({})", value));
                    }
                    if let Some(value) = ask.price {
                        row.ask_price
                            .set_label(&format!("{:.2}", value as f32 / 100.0));
                    }
                }
            }
        }

        for idx in 0..50 {
            if let Some(row) = store.book.get(idx) {
                // Check Bid Row
                if let Some(bid_) = bids_data.get(idx) {
                    if max_bid_qty > 0 && bid_.bid_qty == max_bid_qty {
                        row.bid_qty.add_css_class("bold");
                        row.bid_orders.add_css_class("bold");
                        row.bid_price.add_css_class("bold");

                        store.high.bid_qty.set_label(&row.bid_qty.label());
                        store.high.bid_orders.set_label(&row.bid_orders.label());
                        store.high.bid_price.set_label(&row.bid_price.label());
                    } else {
                        row.bid_qty.remove_css_class("bold");
                        row.bid_orders.remove_css_class("bold");
                        row.bid_price.remove_css_class("bold");
                    }
                }

                // Check Ask Row
                if let Some(ask_) = asks_data.get(idx) {
                    if max_ask_qty > 0 && ask_.ask_qty == max_ask_qty {
                        row.ask_qty.add_css_class("bold");
                        row.ask_orders.add_css_class("bold");
                        row.ask_price.add_css_class("bold");

                        store.high.ask_qty.set_label(&row.ask_qty.label());
                        store.high.ask_orders.set_label(&row.ask_orders.label());
                        store.high.ask_price.set_label(&row.ask_price.label());
                    } else {
                        row.ask_qty.remove_css_class("bold");
                        row.ask_orders.remove_css_class("bold");
                        row.ask_price.remove_css_class("bold");
                    }
                }
            }
        }

        //update footer
        if let (Some(buy), Some(sell)) = (depth.tbq, depth.tsq) {
            let total = buy + sell;
            let buy_percentage = if total > 0 {
                (buy as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            let sell_percentage = 100.0 - buy_percentage;
            store
                .footer
                .buy
                .set_label(&format!("{} ({:.2}%)", buy, buy_percentage));
            store
                .footer
                .sell
                .set_label(&format!("({:.2}%) {}", sell_percentage, sell));
        }
    }
}
