use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box, Widget};

use crate::frontend::orderbook::footer::footer;
use crate::frontend::orderbook::model::{Ask, Bid, OrderBook};
use crate::frontend::orderbook::table::table;

mod footer;
pub mod model;
mod table;

pub fn order_book() -> (Widget, OrderBook) {
    let container = Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .hexpand(true)
        .build();

    let table = table();
    let footer = footer();

    container.append(&table.0);
    container.append(&footer.0);

    (
        container.upcast(),
        OrderBook {
            bids: Rc::new(RefCell::new(vec![Bid::default(); 50])), // Assuming Bid implements Default and Clone
            asks: Rc::new(RefCell::new(vec![Ask::default(); 50])),
            high: table.2,
            book: table.1,
            footer: footer.1,
        },
    )
}
