use gtk4::CssProvider;

const APP_ID: &str = "org.market.table";

const CSS: &str = r#"
/* The table header block */

.table-header {
    font-weight: bold;
    background-color: alpha(currentColor, 0.08);
    border-bottom: 1px solid alpha(currentColor, 0.15);
    color: alpha(currentColor, 0.6);
}
.border-top {
    border-top: 1px solid alpha(currentColor, 0.15);
}
/* Alternating row colors (Zebra Striping) */
.row-even {
    background-color: transparent;
}
.row-odd {
    background-color: alpha(currentColor, 0.03);
}
.cell {
    padding: 3px 8px;
}
.orders {
    padding: 3px 0px;
}
.buy {
    color: lighter(@success_color);
}
.sell {
    color: lighter(@error_color);
}

.strike-cell {
    font-weight: bold;
}
.atm-divider {
    border-bottom: 1px dashed alpha(currentColor, 0.8);
}

.expiry-selector {
    border-radius: 8px;
    padding: 16px;
}

.expiry-btn {
    padding: 6px 12px;
    border-radius: 6px;
    border: none;
}

.bold {
    font-weight: bold;
}
"#;

pub fn load_custom_css() {
    let provider = CssProvider::new();
    provider.load_from_data(CSS);

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
