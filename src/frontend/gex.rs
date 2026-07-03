use gio::ListStore;
use glib::BoxedAnyObject;
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Grid, Label, ListView, NoSelection, Orientation, PolicyType, ScrolledWindow,
    SignalListItemFactory, SizeGroup, SizeGroupMode, Widget,
};

pub struct GexRow {
    pub strike: i32,
    pub c_vol: i32,
    pub c_oi: i32,
    pub p_vol: i32,
    pub p_oi: i32,
    pub pcr_oi: f32,
    pub pcr_vol: f32,
    pub gex_oi: f32,
    pub is_atm: bool,
}

pub fn build_gex_panel() -> Widget {
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .build();

    let col_group = SizeGroup::new(SizeGroupMode::Horizontal);

    // --- 1. HEADER GRID ---
    let header_grid = Grid::builder().build();
    header_grid.add_css_class("table-header");

    let headers = vec![
        "Strike", "C-Vol", "C-OI", "P-Vol", "P-OI", "PCR(OI)", "PCR(V)", "GEX",
    ];

    for (i, title) in headers.iter().enumerate() {
        let alignment = if i == 0 { 0.0 } else { 1.0 };

        let lbl = Label::builder()
            .label(*title)
            .css_classes(vec!["cell", "chain-header"])
            .halign(Align::Fill)
            .xalign(alignment)
            .hexpand(true)
            .build();

        col_group.add_widget(&lbl);
        header_grid.attach(&lbl, i as i32, 1, 1, 1);
    }
    container.append(&header_grid);

    // --- 2. FACTORY SETUP ---
    let factory = SignalListItemFactory::new();
    let col_group_clone = col_group.clone();

    factory.connect_setup(move |_, list_item| {
        let row_bg = Box::builder().orientation(Orientation::Horizontal).build();

        // 8 columns in the GEX panel
        for col_idx in 0..8 {
            let alignment = if col_idx == 0 { 0.0 } else { 1.0 };

            let label = Label::builder()
                .halign(Align::Fill)
                .xalign(alignment)
                .hexpand(true)
                .css_classes(vec!["cell"])
                .build();

            col_group_clone.add_widget(&label);
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
        let row = item.borrow::<GexRow>();

        let formatted_data = vec![
            format!("{}", row.strike),
            format!("{}", row.c_vol),
            format!("{}", row.c_oi),
            format!("{}", row.p_vol),
            format!("{}", row.p_oi),
            format!("{:.2}", row.pcr_oi),
            format!("{:.2}", row.pcr_vol),
            format!("{:.0}", row.gex_oi),
        ];

        let mut current_child = row_bg.first_child();
        for (col_idx, val) in formatted_data.iter().enumerate() {
            if let Some(child) = current_child {
                let label = child.downcast::<Label>().unwrap();
                label.set_label(val);

                // Handle standard ATM styling

                // Handle custom GEX text coloring (Column 7)
                if col_idx == 7 {
                    label.remove_css_class("text-call");
                    label.remove_css_class("text-put");
                    if row.gex_oi >= 0.0 {
                        label.add_css_class("text-call");
                    } else {
                        label.add_css_class("text-put");
                    }
                }

                current_child = label.next_sibling();
            }
        }
    });

    // --- 4. DATA MODEL INITIALIZATION ---
    let store = ListStore::new::<BoxedAnyObject>();
    let base_strike = 23700;

    for i in 0..10 {
        let strike = base_strike + (i * 50);
        let c_vol = 50000 + (i * 1200);
        let c_oi = 15000 + (i * 800);
        let p_vol = 45000 + (i * 1500);
        let p_oi = 20000 + (i * 600);

        let row_data = GexRow {
            strike,
            c_vol,
            c_oi,
            p_vol,
            p_oi,
            pcr_oi: p_oi as f32 / c_oi as f32,
            pcr_vol: p_vol as f32 / c_vol as f32,
            gex_oi: (i as f32 * 1200.0) - 5000.0,
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
