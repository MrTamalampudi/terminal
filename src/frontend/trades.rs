use glib::object::Cast;
use gtk4::{
    Align, Box, Label, Orientation, ScrolledWindow, SizeGroup, SizeGroupMode, Widget,
    prelude::{BoxExt, WidgetExt},
};

#[derive(Clone)]
pub struct TradeRowUI {
    pub side: Label,
    pub size: Label,
    pub price: Label,
    pub time: Label,
}

pub struct TradeData {
    pub side: &'static str,
    pub size: String,
    pub price: String,
    pub time: String,
    pub is_buy: bool,
}

pub fn trades() -> Widget {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .build();

    let equal_cols_group = SizeGroup::new(SizeGroupMode::Horizontal);

    container.append(&build_header(&equal_cols_group));
    let body_container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .build();

    let mut rows_ui = Vec::with_capacity(50);
    // for i in 0..100 {
    //     let (row_widget, row_ui) = build_row(i, &equal_cols_group);
    //     body_container.append(&row_widget);
    //     rows_ui.push(row_ui);
    // }
    //
    for (i, trade) in get_mock_trades().iter().enumerate() {
        let (row_widget, row_ui) = build_roww(i, trade, &equal_cols_group);
        body_container.append(&row_widget);
        rows_ui.push(row_ui);
    }

    let scroll = ScrolledWindow::builder()
        .child(&body_container)
        .vexpand(true)
        .build();

    container.append(&scroll);

    container.upcast()
}

fn build_header(size_group: &SizeGroup) -> Box {
    use Orientation::Horizontal as H;

    let header_container = Box::builder()
        .orientation(H)
        .css_classes(["table-header"])
        .hexpand(true)
        .halign(Align::Fill)
        .build();

    let side = Label::builder()
        .label("Side")
        .halign(Align::Center)
        .css_classes(["cell"])
        .width_chars(4)
        .build();

    let size = Label::builder()
        .label("Size")
        .halign(Align::End)
        .xalign(1.0)
        .width_chars(7)
        .css_classes(["cell"])
        .build();

    let price = Label::builder()
        .label("Price")
        .hexpand(true)
        .xalign(1.0)
        .halign(Align::End)
        .css_classes(["cell"])
        .build();

    let time = Label::builder()
        .label("Time")
        .halign(Align::End)
        .xalign(1.0)
        .hexpand(true)
        .css_classes(["cell"])
        .build();

    // Align headers with their respective data columns
    header_container.append(&side);
    header_container.append(&size);
    for widget in [&price, &time] {
        size_group.add_widget(widget);
        header_container.append(widget);
    }

    header_container
}

fn build_row(index: usize, size_group: &SizeGroup) -> (Box, TradeRowUI) {
    use Orientation::Horizontal as H;

    let row = Box::builder().orientation(H).hexpand(true).build();

    // --- LEFT SIDE ---
    let side = Label::builder()
        .label("🠩/🠫")
        .halign(Align::Start)
        .hexpand(true)
        .css_classes(["cell"])
        .build();

    let size = Label::builder()
        .label("Size")
        .halign(Align::End)
        .hexpand(true)
        .xalign(1.0)
        .css_classes(["cell"])
        .build();

    let price = Label::builder()
        .label("Price")
        .hexpand(true)
        .xalign(1.0)
        .halign(Align::End)
        .css_classes(["cell"])
        .build();

    let time = Label::builder()
        .label("Time")
        .halign(Align::End)
        .xalign(1.0)
        .hexpand(true)
        .css_classes(["cell"])
        .build();

    row.append(&side);
    for widget in [&size, &price, &time] {
        size_group.add_widget(widget);
        row.append(widget);
    }

    let row_ui = TradeRowUI {
        side,
        size,
        price,
        time,
    };

    if index % 2 == 0 {
        row.add_css_class("row-even");
    } else {
        row.add_css_class("row-odd");
    }

    (row, row_ui)
}

fn build_roww(index: usize, data: &TradeData, size_group: &SizeGroup) -> (Box, TradeRowUI) {
    use Orientation::Horizontal as H;

    let row = Box::builder().orientation(H).hexpand(true).build();

    let color_class = if data.is_buy {
        "price-buy"
    } else {
        "price-sell"
    };

    let side = Label::builder()
        .label(data.side)
        .halign(Align::Center)
        .width_chars(4)
        .css_classes(["cell", color_class])
        .build();

    let size = Label::builder()
        .label(&data.size)
        .halign(Align::End)
        .xalign(1.0)
        .width_chars(7)
        .css_classes(["cell"])
        .build();

    let price = Label::builder()
        .label(&data.price)
        .hexpand(true)
        .xalign(1.0)
        .halign(Align::End)
        .css_classes(["cell", color_class])
        .build();

    let time = Label::builder()
        .label(&data.time)
        .halign(Align::End)
        .xalign(1.0)
        .hexpand(true)
        .css_classes(["cell"])
        .build();

    row.append(&side);
    row.append(&size);
    for widget in [&price, &time] {
        size_group.add_widget(widget);
        row.append(widget);
    }

    let row_ui = TradeRowUI {
        side,
        size,
        price,
        time,
    };

    if index % 2 == 0 {
        row.add_css_class("row-even");
    } else {
        row.add_css_class("row-odd");
    }

    (row, row_ui)
}

fn get_mock_trades() -> Vec<TradeData> {
    vec![
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,000.3".into(),
            time: "11:41:37".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,000.3".into(),
            time: "11:41:29".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "24,000.1".into(),
            time: "11:41:21".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,001.5".into(),
            time: "11:41:14".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "3".into(),
            price: "24,001.5".into(),
            time: "11:41:13".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,002.5".into(),
            time: "11:41:08".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "5".into(),
            price: "24,000.5".into(),
            time: "11:41:06".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "4".into(),
            price: "23,998.2".into(),
            time: "11:41:04".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "24,000.0".into(),
            time: "11:41:03".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,000.0".into(),
            time: "11:41:01".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "5".into(),
            price: "24,002.5".into(),
            time: "11:40:59".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,998.0".into(),
            time: "11:40:54".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,003.8".into(),
            time: "11:40:53".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,003.8".into(),
            time: "11:40:49".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "24,003.0".into(),
            time: "11:40:48".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,003.0".into(),
            time: "11:40:41".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "24,003.8".into(),
            time: "11:40:26".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,995.0".into(),
            time: "11:40:18".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "10".into(),
            price: "24,004.1".into(),
            time: "11:40:16".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "23,997.0".into(),
            time: "11:40:15".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,992.0".into(),
            time: "11:40:01".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "27".into(),
            price: "24,003.7".into(),
            time: "11:40:00".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,988.1".into(),
            time: "11:39:56".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "27".into(),
            price: "24,001.3".into(),
            time: "11:39:53".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "195".into(),
            price: "23,998.7".into(),
            time: "11:39:48".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,992.0".into(),
            time: "11:39:39".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,992.0".into(),
            time: "11:39:37".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "7".into(),
            price: "23,995.1".into(),
            time: "11:39:33".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,995.1".into(),
            time: "11:39:32".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,995.1".into(),
            time: "11:39:29".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,996.0".into(),
            time: "11:39:25".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,995.1".into(),
            time: "11:39:12".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "27".into(),
            price: "23,990.3".into(),
            time: "11:39:07".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,999.5".into(),
            time: "11:39:05".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "27".into(),
            price: "23,995.1".into(),
            time: "11:39:00".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,000.0".into(),
            time: "11:38:45".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,003.7".into(),
            time: "11:38:44".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,000.0".into(),
            time: "11:38:42".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "24,002.0".into(),
            time: "11:38:34".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,002.3".into(),
            time: "11:38:31".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,003.9".into(),
            time: "11:38:27".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "5".into(),
            price: "24,003.9".into(),
            time: "11:38:24".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,003.9".into(),
            time: "11:38:22".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,004.9".into(),
            time: "11:38:16".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "24,004.9".into(),
            time: "11:38:13".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "24,002.5".into(),
            time: "11:38:11".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "3".into(),
            price: "24,002.5".into(),
            time: "11:38:08".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,002.4".into(),
            time: "11:38:01".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,004.9".into(),
            time: "11:37:59".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "3".into(),
            price: "24,004.9".into(),
            time: "11:37:58".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "24,004.9".into(),
            time: "11:37:37".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,006.9".into(),
            time: "11:37:25".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "15".into(),
            price: "24,007.2".into(),
            time: "11:37:12".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "4".into(),
            price: "24,007.2".into(),
            time: "11:37:05".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,005.0".into(),
            time: "11:36:58".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "12".into(),
            price: "24,004.1".into(),
            time: "11:36:50".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,005.5".into(),
            time: "11:36:44".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "8".into(),
            price: "24,005.5".into(),
            time: "11:36:42".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "24,001.8".into(),
            time: "11:36:31".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "24,001.0".into(),
            time: "11:36:25".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "45".into(),
            price: "24,003.0".into(),
            time: "11:36:18".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "3".into(),
            price: "24,002.5".into(),
            time: "11:36:10".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,999.9".into(),
            time: "11:36:01".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "5".into(),
            price: "23,998.5".into(),
            time: "11:35:55".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "24,000.1".into(),
            time: "11:35:42".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "24,000.1".into(),
            time: "11:35:38".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "10".into(),
            price: "23,997.0".into(),
            time: "11:35:30".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,997.0".into(),
            time: "11:35:28".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "6".into(),
            price: "23,999.5".into(),
            time: "11:35:15".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,999.5".into(),
            time: "11:35:12".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "4".into(),
            price: "23,995.2".into(),
            time: "11:35:05".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "23,995.0".into(),
            time: "11:34:58".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,998.8".into(),
            time: "11:34:44".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,998.8".into(),
            time: "11:34:40".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "3".into(),
            price: "23,994.5".into(),
            time: "11:34:32".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "7".into(),
            price: "23,994.0".into(),
            time: "11:34:25".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,996.3".into(),
            time: "11:34:18".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "23,996.3".into(),
            time: "11:34:15".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,991.1".into(),
            time: "11:34:02".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,991.1".into(),
            time: "11:34:00".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "14".into(),
            price: "23,995.0".into(),
            time: "11:33:48".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "5".into(),
            price: "23,995.0".into(),
            time: "11:33:42".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "2".into(),
            price: "23,990.5".into(),
            time: "11:33:35".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,990.0".into(),
            time: "11:33:28".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,992.4".into(),
            time: "11:33:15".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "3".into(),
            price: "23,992.4".into(),
            time: "11:33:11".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "8".into(),
            price: "23,988.8".into(),
            time: "11:33:04".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,988.5".into(),
            time: "11:32:55".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "23,990.0".into(),
            time: "11:32:42".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,990.0".into(),
            time: "11:32:38".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,985.2".into(),
            time: "11:32:25".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "5".into(),
            price: "23,985.0".into(),
            time: "11:32:18".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "22".into(),
            price: "23,989.5".into(),
            time: "11:32:05".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,989.5".into(),
            time: "11:32:01".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "3".into(),
            price: "23,984.1".into(),
            time: "11:31:55".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,984.1".into(),
            time: "11:31:48".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬆",
            size: "2".into(),
            price: "23,987.0".into(),
            time: "11:31:35".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬆",
            size: "1".into(),
            price: "23,987.0".into(),
            time: "11:31:31".into(),
            is_buy: true,
        },
        TradeData {
            side: "⬇",
            size: "4".into(),
            price: "23,982.5".into(),
            time: "11:31:22".into(),
            is_buy: false,
        },
        TradeData {
            side: "⬇",
            size: "1".into(),
            price: "23,982.0".into(),
            time: "11:31:15".into(),
            is_buy: false,
        },
    ]
}
