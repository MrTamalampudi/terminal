use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation};

pub fn create_ticker_row() -> Box {
    // 1. The main horizontal container
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(10)
        .build();

    // 2. Left side: Instrument name and expiry
    let instrument_label = Label::builder()
        .label("Nifty 23950 CE | 30 Jun")
        .halign(Align::Start)
        .hexpand(true) // This is the secret: it pushes the right_box to the edge
        .build();

    // Use standard GTK style class to make the text dim/grey
    instrument_label.add_css_class("dim-label");

    // 3. Right side: Container for price and percentage
    let right_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6) // Small gap between price and percentage
        .halign(Align::End)
        .build();

    // The current price
    let price_label = Label::builder().label("24,053.60 |").build();

    // The profit/loss change
    let change_label = Label::builder().label("+201.10 (+0.84%)").build();

    // Add a custom class so we can color it green in CSS
    change_label.add_css_class("positive-text");

    // 4. Assemble the widget tree
    right_box.append(&price_label);
    right_box.append(&change_label);

    row_box.append(&instrument_label);
    row_box.append(&right_box);

    row_box
}
