use gtk4::{Label, prelude::WidgetExt};
use std::collections::BTreeMap;

use crate::{
    backend,
    frontend::orderbook::model::{Ask, Bid, OrderBook},
    model::enums::DerivativeType,
};

// ==========================================
// MAIN UPDATE ENTRY
// ==========================================

pub fn update_orderbook(
    store: &OrderBook,
    feed: backend::fyers_tbt::MarketFeed,
    derivative_type: DerivativeType,
) {
    let depth = match feed.depth {
        Some(d) => d,
        None => return,
    };

    let mut bids_data = store.bids.borrow_mut();
    let mut asks_data = store.asks.borrow_mut();

    // 1. Update internal state
    update_raw_data(&mut bids_data, &mut asks_data, &depth);

    // 2. Group the data (Passing slices avoids costly .to_vec() cloning!)
    let (grouped_bids, grouped_asks) = group_bids(&bids_data, &asks_data, &derivative_type);

    // 3. Render UI rows
    render_book_ui(store, &grouped_bids, &grouped_asks);

    // 4. Update Footers
    let total_bids: u32 = bids_data.iter().map(|b| b.bid_qty).sum();
    let total_asks: u32 = asks_data.iter().map(|a| a.ask_qty).sum();
    update_footers(store, &depth, total_bids, total_asks);
}

// ==========================================
// DATA PROCESSING HELPERS
// ==========================================

fn update_raw_data(
    bids_data: &mut [Bid],
    asks_data: &mut [Ask],
    depth: &backend::fyers_tbt::Depth,
) {
    for bid in &depth.bids {
        if let Some(idx) = bid.num.map(|n| n as usize).filter(|&i| i < 50) {
            if let Some(data_row) = bids_data.get_mut(idx) {
                if let Some(qty) = bid.qty {
                    data_row.bid_qty = qty;
                }
                if let Some(nord) = bid.nord {
                    data_row.bid_orders = nord;
                }
                if let Some(price) = bid.price {
                    data_row.bid_price = price;
                }
            }
        }
    }

    for ask in &depth.asks {
        if let Some(idx) = ask.num.map(|n| n as usize).filter(|&i| i < 50) {
            if let Some(data_row) = asks_data.get_mut(idx) {
                if let Some(qty) = ask.qty {
                    data_row.ask_qty = qty;
                }
                if let Some(nord) = ask.nord {
                    data_row.ask_orders = nord;
                }
                if let Some(price) = ask.price {
                    data_row.ask_price = price;
                }
            }
        }
    }
}

pub fn group_bids(
    bids: &[Bid],
    asks: &[Ask],
    derivative_type: &DerivativeType,
) -> (Vec<Bid>, Vec<Ask>) {
    let mut grouped_bids: BTreeMap<i64, Bid> = BTreeMap::new();
    let mut grouped_asks: BTreeMap<i64, Ask> = BTreeMap::new();

    let step_size = if derivative_type == &DerivativeType::FUTURES {
        100
    } else {
        25
    };

    // Safely iterate only valid bids (ignores empty zeros)
    for bid in bids.iter().filter(|b| b.bid_price > 0) {
        let rounded_price = ((bid.bid_price + (step_size / 2)) / step_size) * step_size;
        grouped_bids
            .entry(rounded_price)
            .and_modify(|existing| {
                existing.bid_qty += bid.bid_qty;
                existing.bid_orders += bid.bid_orders;
            })
            .or_insert(Bid {
                bid_qty: bid.bid_qty,
                bid_orders: bid.bid_orders,
                bid_price: rounded_price,
            });
    }

    for ask in asks.iter().filter(|a| a.ask_price > 0) {
        let rounded_price = ((ask.ask_price + (step_size / 2)) / step_size) * step_size;
        grouped_asks
            .entry(rounded_price)
            .and_modify(|existing| {
                existing.ask_qty += ask.ask_qty;
                existing.ask_orders += ask.ask_orders;
            })
            .or_insert(Ask {
                ask_qty: ask.ask_qty,
                ask_orders: ask.ask_orders,
                ask_price: rounded_price,
            });
    }

    (
        grouped_bids.into_values().rev().collect(),
        grouped_asks.into_values().collect(),
    )
}

// ==========================================
// UI RENDERING HELPERS
// ==========================================

fn render_book_ui(store: &OrderBook, grouped_bids: &[Bid], grouped_asks: &[Ask]) {
    let max_bid_qty = grouped_bids.iter().map(|b| b.bid_qty).max().unwrap_or(0);
    let max_ask_qty = grouped_asks.iter().map(|a| a.ask_qty).max().unwrap_or(0);

    for (idx, ui_row) in store.book.iter().enumerate().take(50) {
        // --- Bids ---
        if let Some(bid) = grouped_bids.get(idx) {
            let is_max = max_bid_qty > 0 && bid.bid_qty == max_bid_qty;
            render_row_side(
                &ui_row.bid_qty,
                &ui_row.bid_orders,
                &ui_row.bid_price,
                bid.bid_qty,
                bid.bid_orders,
                bid.bid_price,
                is_max,
                if is_max {
                    Some((
                        &store.high.bid_qty,
                        &store.high.bid_orders,
                        &store.high.bid_price,
                    ))
                } else {
                    None
                },
            );
        } else {
            clear_row_side(&ui_row.bid_qty, &ui_row.bid_orders, &ui_row.bid_price);
        }

        // --- Asks ---
        if let Some(ask) = grouped_asks.get(idx) {
            let is_max = max_ask_qty > 0 && ask.ask_qty == max_ask_qty;
            render_row_side(
                &ui_row.ask_qty,
                &ui_row.ask_orders,
                &ui_row.ask_price,
                ask.ask_qty,
                ask.ask_orders,
                ask.ask_price,
                is_max,
                if is_max {
                    Some((
                        &store.high.ask_qty,
                        &store.high.ask_orders,
                        &store.high.ask_price,
                    ))
                } else {
                    None
                },
            );
        } else {
            clear_row_side(&ui_row.ask_qty, &ui_row.ask_orders, &ui_row.ask_price);
        }
    }
}

fn update_footers(
    store: &OrderBook,
    depth: &backend::fyers_tbt::Depth,
    total_bids: u32,
    total_asks: u32,
) {
    if let (Some(buy), Some(sell)) = (depth.tbq, depth.tsq) {
        let total = buy + sell;
        let buy_pct = if total > 0 {
            (buy as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let sell_pct = 100.0 - buy_pct;

        store
            .footer
            .buy
            .set_label(&format!("{buy} ({buy_pct:.2}%)"));
        store
            .footer
            .sell
            .set_label(&format!("({sell_pct:.2}%) {sell}"));
    }

    let total50 = total_asks + total_bids;
    let buy50_pct = if total50 > 0 {
        (total_bids as f64 / total50 as f64) * 100.0
    } else {
        0.0
    };
    let sell50_pct = 100.0 - buy50_pct;

    store
        .footer
        .buy50
        .set_label(&format!("{total_bids} ({buy50_pct:.2}%)"));
    store
        .footer
        .sell50
        .set_label(&format!("({sell50_pct:.2}%) {total_asks}"));
}

fn render_row_side(
    qty_lbl: &Label,
    orders_lbl: &Label,
    price_lbl: &Label,
    qty: u32,
    orders: u32,
    price: i64,
    is_max: bool,
    high_store: Option<(&Label, &Label, &Label)>,
) {
    qty_lbl.set_label(&qty.to_string());
    orders_lbl.set_label(&format!("({orders})"));
    price_lbl.set_label(&format!("{:.2}", price as f32 / 100.0));

    toggle_css_class(qty_lbl, is_max);
    toggle_css_class(orders_lbl, is_max);
    toggle_css_class(price_lbl, is_max);

    if let Some((h_qty, h_orders, h_price)) = high_store {
        h_qty.set_label(&qty.to_string());
        h_orders.set_label(&format!("({orders})"));
        h_price.set_label(&format!("{:.2}", price as f32 / 100.0));
    }
}

fn clear_row_side(qty_lbl: &Label, orders_lbl: &Label, price_lbl: &Label) {
    qty_lbl.set_label("");
    orders_lbl.set_label("");
    price_lbl.set_label("");
    toggle_css_class(qty_lbl, false);
    toggle_css_class(orders_lbl, false);
    toggle_css_class(price_lbl, false);
}

fn toggle_css_class(widget: &Label, add: bool) {
    if add {
        widget.add_css_class("bold");
    } else {
        widget.remove_css_class("bold");
    }
}
