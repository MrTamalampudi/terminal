use gtk4::CssProvider;

const APP_ID: &str = "org.market.table";

pub fn load_custom_css() {
    let provider = CssProvider::new();
    provider.load_from_data(
        "
        /* The table header block */
        .table-header {
            background-color: alpha(currentColor, 0.08);
            border-bottom: 1px solid alpha(currentColor, 0.15);
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
        /* Padding for our text cells so they aren't cramped */
        .buy-price-quantity {
            padding: 8px 3px 3px 3px;
        }
        .sell-price-quantity {
            padding: 3px 3px 8px 3px;
        }
        .cell {
            padding: 3px 8px;
        }
        .orders {
            padding: 3px 0px;
        }
        .text-bid { color: lighter(@success_color); }
        .text-ask { color: lighter(@error_color); }

        .positive-text { color: lighter(@success_color); }
        .negative-text { color: lighter(@error_color); }

        .chain-header {
            font-weight: bold;
            color: alpha(currentColor, 0.6);
            border-bottom: 1px solid alpha(currentColor, 0.15);
        }
        .strike-cell {
            font-weight: bold;
        }
        .atm-divider {
            border-bottom: 1px dashed alpha(currentColor, 0.8);
        }
        .text-call { color: #ff6b6b; }
        .text-put { color: #69db7c; }

        .expiry-selector {
            border-radius: 8px;
            padding: 16px;
        }
        .expiry-btn:checked {
        background-color: @dnd;
            color: white;
        }
        .expiry-btn {
            padding: 6px 12px;
            border-radius: 6px;
            border: none;
        }

        .bold {
            font-weight: bold;
        }

        ",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
