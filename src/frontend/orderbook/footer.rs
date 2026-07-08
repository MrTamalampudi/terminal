use glib::object::Cast;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Widget};

use crate::frontend::orderbook::model::FooterUI;

pub fn footer() -> (Widget, FooterUI) {
    let container = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .hexpand(true)
        .margin_top(4)
        .margin_bottom(4)
        .build();

    let (box50, buy50, sell50) = build_box();
    let (boxx, buy, sell) = build_box();

    container.append(&box50);
    container.append(&boxx);

    let footer_ui = FooterUI {
        buy,
        sell,
        buy50,
        sell50,
    };

    (container.upcast(), footer_ui)
}

fn build_box() -> (Box, Label, Label) {
    let container = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .hexpand(true)
        .build();

    let total_buy_qty = Label::builder()
        .css_classes(["cell", "buy"])
        .halign(Align::Start)
        .xalign(0.0)
        .hexpand(true)
        .label("-")
        .build();
    let total_sell_qty = Label::builder()
        .css_classes(["cell", "sell"])
        .halign(Align::End)
        .xalign(1.0)
        .hexpand(true)
        .label("-")
        .build();

    container.append(&total_buy_qty);
    container.append(&total_sell_qty);

    (container, total_buy_qty, total_sell_qty)
}
