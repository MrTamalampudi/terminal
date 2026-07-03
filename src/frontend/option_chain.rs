pub struct OptionRow {
    pub c_gamma: f32,
    pub c_theta: f32,
    pub c_delta: f32,
    pub c_iv: f32,
    pub c_ltp: f32,
    pub strike: i32,
    pub p_ltp: f32,
    pub p_iv: f32,
    pub p_delta: f32,
    pub p_theta: f32,
    pub p_gamma: f32,
    pub is_atm: bool,
}
use gio::ListStore;
use glib::BoxedAnyObject;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Grid, Label, ListView, NoSelection, Orientation, PolicyType, ScrolledWindow,
    SignalListItemFactory, SizeGroup, SizeGroupMode, Widget,
};

use crate::frontend::mkt_summary::create_ticker_row;

pub fn build_option_chain_column() -> Widget {
    let container = Box::builder().orientation(Orientation::Vertical).build();

    container.append(&create_ticker_row());

    let col_size_group = SizeGroup::new(SizeGroupMode::Horizontal);
    let strike_size_group = SizeGroup::new(SizeGroupMode::Horizontal);

    // --- 1. OPTION CHAIN HEADERS (Unchanged) ---
    let header_grid = Grid::builder().build();
    header_grid.add_css_class("table-header");

    let headers = vec!["Gamma", "Theta", "Delta", "IV", "LTP"];

    for (col_idx, h_text) in headers.iter().enumerate() {
        let h_label = Label::builder()
            .label(*h_text)
            .use_markup(true)
            .halign(Align::Fill)
            .xalign(1.0)
            .hexpand(true)
            .css_classes(vec!["cell", "chain-header"])
            .build();

        col_size_group.add_widget(&h_label);
        header_grid.attach(&h_label, col_idx as i32, 1, 1, 1);
    }

    let strike_label = Label::builder()
        .label("Strike")
        .use_markup(true)
        .halign(Align::Center)
        .hexpand(true)
        .width_request(120)
        .css_classes(vec!["cell", "chain-header"])
        .build();

    strike_size_group.add_widget(&strike_label);
    header_grid.attach(&strike_label, headers.len() as i32, 1, 1, 1);

    for (col_idx, h_text) in headers.iter().rev().enumerate() {
        let h_label = Label::builder()
            .label(*h_text)
            .use_markup(true)
            .halign(Align::Fill)
            .xalign(0.0)
            .hexpand(true)
            .css_classes(vec!["cell", "chain-header"])
            .build();

        col_size_group.add_widget(&h_label);
        header_grid.attach(&h_label, (col_idx + 6) as i32, 1, 1, 1);
    }

    container.append(&header_grid);

    // --- 2. FACTORY SETUP ---
    let factory = SignalListItemFactory::new();

    let col_group_clone = col_size_group.clone();
    let strike_group_clone = strike_size_group.clone();

    // Create the 11 labels dynamically to save boilerplate
    factory.connect_setup(move |_, list_item| {
        let row_bg = Box::builder().orientation(Orientation::Horizontal).build();

        for col_idx in 0..11 {
            let current_xalign = if col_idx < 5 {
                1.0
            } else if col_idx == 5 {
                0.5
            } else {
                0.0
            };

            let label = Label::builder()
                .halign(Align::Fill)
                .xalign(current_xalign)
                .css_classes(vec!["cell"])
                .build();

            if col_idx == 5 {
                label.add_css_class("strike-cell");
                label.set_halign(Align::Center);
                label.set_width_request(120);
                strike_group_clone.add_widget(&label);
            } else {
                label.set_hexpand(true);
                col_group_clone.add_widget(&label);
            }

            row_bg.append(&label);
        }
        list_item.set_child(Some(&row_bg));
    });

    // --- 3. FACTORY BIND ---
    factory.connect_bind(move |_, list_item| {
        let row_bg = list_item.child().unwrap().downcast::<Box>().unwrap();
        let item = list_item
            .item()
            .unwrap()
            .downcast::<BoxedAnyObject>()
            .unwrap();
        let row = item.borrow::<OptionRow>();

        // Format the raw data into strings
        let formatted_data = vec![
            format!("{:.4}", row.c_gamma),
            format!("{:.2}", row.c_theta),
            format!("{:.2}", row.c_delta),
            format!("{:.2}", row.c_iv),
            format!("{:.2}", row.c_ltp),
            format!("{}", row.strike),
            format!("{:.2}", row.p_ltp),
            format!("{:.2}", row.p_iv),
            format!("{:.2}", row.p_delta),
            format!("{:.2}", row.p_theta),
            format!("{:.4}", row.p_gamma),
        ];

        // Iterate through the Box children and update the text
        let mut current_child = row_bg.first_child();
        for val in formatted_data.iter() {
            if let Some(child) = current_child {
                let label = child.downcast::<Label>().unwrap();
                label.set_label(val);

                current_child = label.next_sibling();
            }
        }
    });

    // --- 4. DATA MODEL INITIALIZATION ---
    let store = ListStore::new::<BoxedAnyObject>();
    let base_strike = 23700;

    for row_idx in 0..10 {
        let strike = base_strike + (row_idx * 50);

        let row_data = OptionRow {
            c_gamma: 0.0006 + (row_idx as f32 * 0.0001),
            c_theta: -8.00 - (row_idx as f32 * 1.1),
            c_delta: 0.90 - (row_idx as f32 * 0.05),
            c_iv: 12.62 - (row_idx as f32 * 0.2),
            c_ltp: 409.75 - (row_idx as f32 * 28.0).max(10.0),
            strike,
            p_ltp: 12.95 + (row_idx as f32 * 25.0),
            p_iv: 12.62 + (row_idx as f32 * 0.1),
            p_delta: -0.10 - (row_idx as f32 * 0.05),
            p_theta: -8.00 - (row_idx as f32 * 0.8),
            p_gamma: 0.0006 + (row_idx as f32 * 0.0001),
            is_atm: strike == 24000,
        };

        store.append(&BoxedAnyObject::new(row_data));
    }

    // --- 5. FINAL ASSEMBLY ---
    let selection_model = NoSelection::new(Some(store));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    container.append(&list_view);
    container.upcast()
}
