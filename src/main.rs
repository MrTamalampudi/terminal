use std::mem::transmute;

use futures_util::{SinkExt, StreamExt};
use http::Request;
use prost::Message as ProstMessage;
use serde::Serialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use url::Url;

use gio::ListStore;
use glib::BoxedAnyObject;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Grid, Label, ListView, NoSelection, Orientation,
    ScrolledWindow, SignalListItemFactory, SizeGroup, SizeGroupMode, Widget, prelude::*,
};

use crate::{frontend::orderbook::model::OrderBook, ui::load_custom_css};

// --- 1. PROTOBUF GENERATED TYPES ---
// (Normally this is included via build.rs, but kept inline here as per your snippet)

mod backend;
mod frontend;
mod ui;

// JSON Serialization structs for the subscription request
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

// --- 2. UI DATA MODEL ---
// Added Clone so we can fetch, modify, and replace rows in the GTK ListStore
#[derive(Clone, Default, Debug)]
pub struct OrderRow {
    pub bid_qty: i32,
    pub bid_orders: i32,
    pub bid_price: f32,
    pub ask_price: f32,
    pub ask_qty: i32,
    pub ask_orders: i32,
    pub index: u32, // Changed to u32 to match Fyers protobuf `num`
}

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

    main_layout.append(&order_book_widget_ce);
    main_layout.append(&order_book_widget_fut);
    main_layout.append(&order_book_widget_pe);

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
        rt.block_on(async {
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

// --- 5. DATA BINDING LOGIC ---
// fn update_store_with_feed(store: &ListStore, feed: backend::fyers_tbt::MarketFeed) {
//     if let Some(depth) = feed.depth {
//         // Update Bids
//         for bid in depth.bids {
//             let idx = bid.num.unwrap_or(0);
//             if idx < 50 {
//                 // Fetch existing row, modify it, and splice it back to trigger UI update
//                 if let Some(item) = store.item(idx) {
//                     let boxed = item.downcast_ref::<BoxedAnyObject>().unwrap();
//                     let mut row = boxed.borrow::<frontend::orderbook::OrderRow>().clone();

//                     if let Some(value) = bid.qty {
//                         row.bid_qty = value as i32;
//                     }
//                     if let Some(value) = bid.nord {
//                         row.bid_orders = value as i32;
//                     }
//                     if let Some(value) = bid.price {
//                         row.bid_price = value as f32 / 100.0;
//                     }

//                     //row.bid_qty = bid.qty.unwrap_or(0) as i32;
//                     //row.bid_orders = bid.nord.unwrap_or(0) as i32;
//                     // Note: You may need to divide by 100 based on Fyers ticksize multiplier
//                     //row.bid_price = bid.price.unwrap_or(0) as f32 / 100.0;

//                     store.splice(idx, 1, &[BoxedAnyObject::new(row)]);
//                 }
//             }
//         }

//         // Update Asks
//         for ask in depth.asks {
//             let idx = ask.num.unwrap_or(0);
//             if idx < 50 {
//                 if let Some(item) = store.item(idx) {
//                     let boxed = item.downcast_ref::<BoxedAnyObject>().unwrap();
//                     let mut row = boxed.borrow::<frontend::orderbook::OrderRow>().clone();

//                     if let Some(value) = ask.qty {
//                         row.ask_qty = value as i32;
//                     }
//                     if let Some(value) = ask.nord {
//                         row.ask_orders = value as i32;
//                     }
//                     if let Some(value) = ask.price {
//                         row.ask_price = value as f32 / 100.0;
//                     }

//                     //row.ask_qty = ask.qty.unwrap_or(0) as i32;
//                     // row.ask_orders = ask.nord.unwrap_or(0) as i32;
//                     // row.ask_price = ask.price.unwrap_or(0) as f32 / 100.0;

//                     store.splice(idx, 1, &[BoxedAnyObject::new(row)]);
//                 }
//             }
//         }
//     }
// }
