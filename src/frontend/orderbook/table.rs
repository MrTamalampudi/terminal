use gtk4::Align;
use gtk4::Box;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::ScrolledWindow;
use gtk4::SizeGroup;
use gtk4::SizeGroupMode;
use gtk4::Widget;
use gtk4::prelude::*;

use crate::frontend::orderbook::model::RowUI;

pub fn table() -> (Widget, Vec<RowUI>, RowUI) {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .build();

    // 1. Create separate SizeGroups for flawless vertical column alignment
    let equal_cols_group = SizeGroup::new(SizeGroupMode::Horizontal);

    // 2. Add Header
    container.append(&build_header(&equal_cols_group));

    // 3. Add Body
    let body_container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .build();

    let mut rows_ui = Vec::with_capacity(50);
    for i in 0..50 {
        let (row_widget, row_ui) = build_row(i, &equal_cols_group);
        body_container.append(&row_widget);
        rows_ui.push(row_ui);
    }

    let scroll = ScrolledWindow::builder()
        .child(&body_container)
        .vexpand(true)
        .build();

    let (row_widget, row_ui) = build_row(0, &equal_cols_group);
    row_ui.bid_qty.add_css_class("bold");
    row_ui.bid_orders.add_css_class("bold");
    row_ui.bid_price.add_css_class("bold");
    row_ui.ask_qty.add_css_class("bold");
    row_ui.ask_orders.add_css_class("bold");
    row_ui.ask_price.add_css_class("bold");

    row_widget.add_css_class("border-top");

    container.append(&scroll);
    container.append(&row_widget);

    (container.upcast(), rows_ui, row_ui)
}

fn build_header(size_group: &SizeGroup) -> Box {
    use Orientation::Horizontal as H;

    let header_container = Box::builder()
        .orientation(H)
        .css_classes(["table-header"])
        .hexpand(true)
        .halign(Align::Fill)
        .build();

    let h_qty = Label::builder()
        .label("<b>Qty(Orders)</b>")
        .use_markup(true)
        .halign(Align::Start)
        .hexpand(true)
        .css_classes(["cell", "chain-header"])
        .build();

    let h_bid = Label::builder()
        .label("<b>Bid</b>")
        .use_markup(true)
        .halign(Align::End)
        .hexpand(true)
        .xalign(1.0)
        .css_classes(["cell", "chain-header"])
        .build();

    let h_sno = Label::builder()
        .label("")
        .halign(Align::Center)
        .width_request(25)
        .hexpand(false) // SNO remains fixed width
        .build();

    let h_ask = Label::builder()
        .label("<b>Ask</b>")
        .use_markup(true)
        .hexpand(true)
        .xalign(0.0)
        .halign(Align::Start)
        .css_classes(["cell", "chain-header"])
        .build();

    let h_o_qty = Label::builder()
        .label("<b>(Orders)Qty</b>")
        .use_markup(true)
        .halign(Align::End)
        .hexpand(true)
        .css_classes(["cell", "chain-header"])
        .build();

    // Align headers with their respective data columns
    for widget in [&h_qty, &h_bid, &h_ask, &h_o_qty] {
        size_group.add_widget(widget);
    }

    // Left Header Box
    let h_left = Box::builder().orientation(H).hexpand(true).build();
    h_left.append(&h_qty);
    h_left.append(&h_bid);

    // Right Header Box
    let h_right = Box::builder().orientation(H).hexpand(true).build();
    h_right.append(&h_ask);
    h_right.append(&h_o_qty);

    header_container.append(&h_left);
    header_container.append(&h_sno);
    header_container.append(&h_right);

    header_container
}

fn build_row(index: usize, size_group: &SizeGroup) -> (Box, RowUI) {
    use Orientation::Horizontal as H;

    // Flattened row box
    let row = Box::builder().orientation(H).hexpand(true).build();

    // --- LEFT SIDE ---
    let leftbox = Box::builder()
        .orientation(H)
        .hexpand(true)
        .spacing(0)
        .build();

    let lbl_bid_size = Label::builder()
        .css_classes(["cell", "text-bid"])
        .halign(Align::Start)
        .xalign(0.0)
        .build();

    let lbl_bid_orders = Label::builder()
        .css_classes(["orders", "text-bid"])
        .halign(Align::Start)
        .xalign(0.0)
        .build();

    leftbox.append(&lbl_bid_size);
    leftbox.append(&lbl_bid_orders);
    size_group.add_widget(&leftbox);
    row.append(&leftbox);

    let bid_wrapper = Box::builder().orientation(H).hexpand(true).build();
    let lbl_bid = Label::builder()
        .css_classes(["cell"])
        .halign(Align::End)
        .hexpand(true)
        .xalign(1.0) // Keeps price from touching the serial number
        .build();

    bid_wrapper.append(&lbl_bid);
    size_group.add_widget(&bid_wrapper);
    row.append(&bid_wrapper);

    // --- CENTER ---
    let sno_wrapper = Box::builder().orientation(H).hexpand(false).build();
    let lbl_sno = Label::builder()
        .label(format!(
            "<span size='small' foreground='gray'>{}</span>",
            index + 1
        ))
        .use_markup(true)
        .halign(Align::Center)
        .width_request(25)
        .hexpand(true)
        .build();

    sno_wrapper.append(&lbl_sno);
    row.append(&sno_wrapper);

    // --- RIGHT SIDE ---
    let ask_wrapper = Box::builder().orientation(H).hexpand(true).build();
    let lbl_ask = Label::builder()
        .css_classes(["cell"])
        .halign(Align::Start)
        .xalign(0.0)
        .build();

    ask_wrapper.append(&lbl_ask);
    size_group.add_widget(&ask_wrapper);
    row.append(&ask_wrapper);

    let right_box = Box::builder()
        .orientation(H)
        .hexpand(true)
        .halign(Align::Fill)
        .build();

    let lbl_ask_orders = Label::builder()
        .css_classes(["orders", "text-ask"])
        .halign(Align::End)
        .hexpand(true)
        .xalign(1.0)
        .build();

    let lbl_ask_size = Label::builder()
        .css_classes(["cell", "text-ask"])
        .halign(Align::End)
        .xalign(1.0)
        .build();

    right_box.append(&lbl_ask_orders);
    right_box.append(&lbl_ask_size);
    size_group.add_widget(&right_box);
    row.append(&right_box);

    // --- STORE & FINISH ROW ---
    let row_ui = RowUI {
        bid_qty: lbl_bid_size,
        bid_orders: lbl_bid_orders,
        bid_price: lbl_bid,
        ask_qty: lbl_ask_size,
        ask_orders: lbl_ask_orders,
        ask_price: lbl_ask,
        index: lbl_sno,
    };

    if index % 2 == 0 {
        row.add_css_class("row-even");
    } else {
        row.add_css_class("row-odd");
    }

    (row, row_ui)
}
