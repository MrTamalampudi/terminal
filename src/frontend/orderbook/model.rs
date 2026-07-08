use std::{cell::RefCell, rc::Rc};

use gtk4::Label;

#[derive(Clone)]
pub struct RowUI {
    pub bid_qty: Label,
    pub bid_orders: Label,
    pub bid_price: Label,
    pub ask_price: Label,
    pub ask_orders: Label,
    pub ask_qty: Label,
    pub index: Label,
}

#[derive(Default, Clone)]
pub struct Bid {
    pub bid_qty: u32,
    pub bid_orders: u32,
    pub bid_price: i64,
}

#[derive(Default, Clone)]
pub struct Ask {
    pub ask_price: i64,
    pub ask_orders: u32,
    pub ask_qty: u32,
}

#[derive(Clone)]
pub struct FooterUI {
    pub buy: Label,
    pub sell: Label,
    pub buy50: Label,
    pub sell50: Label,
}

#[derive(Clone)]
pub struct OrderBook {
    pub book: Vec<RowUI>,
    pub high: RowUI,
    pub bids: Rc<RefCell<Vec<Bid>>>,
    pub asks: Rc<RefCell<Vec<Ask>>>,
    pub footer: FooterUI,
}
