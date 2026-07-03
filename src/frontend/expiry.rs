use gtk4::prelude::*;
use gtk4::{Align, Box, Orientation, ToggleButton, Widget};

pub fn build_expiry_selector() -> Widget {
    let container = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .css_classes(vec![""])
        .hexpand(true)
        .halign(Align::Fill)
        .homogeneous(true)
        .build();

    let expiries = vec!["3d", "10d", "17d"];
    let mut buttons = Vec::new();

    for (i, label_text) in expiries.iter().enumerate() {
        let btn = ToggleButton::builder()
            .label(*label_text)
            .css_classes(vec!["expiry-btn"])
            .halign(Align::Fill)
            .hexpand(true)
            .build();

        if i == 0 {
            btn.set_active(true);
        }

        let btn_clone = btn.clone();
        btn.connect_toggled(move |b| {
            if b.is_active() {
                println!("Selected expiry: {}", b.label().unwrap());
            }
        });

        container.append(&btn);
        buttons.push(btn);
    }

    container.upcast()
}
