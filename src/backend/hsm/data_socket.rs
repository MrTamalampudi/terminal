use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use futures_util::{SinkExt, StreamExt};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::backend::hsm::model::{exch_seg_dict, index_dict};
use crate::backend::hsm::symbol_conversion::{DataType, SymbolConversion};

// Protocol code identifiers matching your defines.txt mapping layers
pub mod defines {
    pub const SUCCESS_CODE: i32 = 200;
    pub const INVALID_CODE: i32 = -300;
    pub const TOKEN_EXPIRED: i32 = -99;
    pub const LIMIT_EXCEED_CODE: i32 = -99;

    pub const AUTH_ERROR_CODE: i32 = 11001;
    pub const SUBS_ERROR_CODE: i32 = 11011;
    pub const UNSUBS_ERROR_CODE: i32 = 11012;
    pub const RESUME_ERROR_CODE: i32 = 11031;
    pub const MODE_ERROR_CODE: i32 = 12001;
    pub const PAUSE_ERROR_CODE: i32 = 11032;

    pub const LIMIT_EXCEED_MSG_5000: &str = "Please provide less than 5000 symbols";
    pub const TOKEN_EXPIRED_MSG: &str = "Token is expired";
    pub const INVALID_TOKEN: &str = "Please provide valid token";
    pub const INVALID_SYMBOLS: &str = "Please provide a valid symbol";
    pub const CONNECTION_CLOSED: &str = "Connection Closed";
    pub const INDEX_DEPTH_ERROR_MESSAGE: &str = "Index does not have market depth";
    pub const AUTH_SUCCESS: &str = "Authentication done";
    pub const AUTH_FAIL: &str = "Authentication failed";
    pub const SUBSCRIBE_SUCCESS: &str = "Subscribed";
    pub const SUBSCRIBE_FAIL: &str = "subscription failed";
    pub const UNSUBSCRIBE_SUCCESS: &str = "Unsubscribed";
    pub const UNSUBSCRIBE_FAIL: &str = "unsubscription failed";
    pub const LITE_MODE: &str = "Lite Mode On";
    pub const FULL_MODE: &str = "Full Mode On";
    pub const MODE_CHANGE_ERROR: &str = "Mode change failed";
    pub const CHANNEL_PAUSED: &str = "Channel Paused";
    pub const CHANNEL_RESUMED: &str = "Channel Resumed";
    pub const CHANNEL_CHANGE_FAIL: &str = "Mode change failed";

    pub const SUCCESS: &str = "ok";
    pub const ERROR: &str = "error";

    pub const AUTH_TYPE: &str = "cn";
    pub const SUBS_TYPE: &str = "sub";
    pub const UNSUBS_TYPE: &str = "unsub";
    pub const LITE_MODE_TYPE: &str = "lite_mode";
    pub const FULL_MODE_TYPE: &str = "full_mode";
    pub const CH_PAUSE_TYPE: &str = "ch_pause";
    pub const CH_RESUME_TYPE: &str = "ch_resume";
}

pub struct FyersDataSocketInner {
    pub url: String,
    pub access_token: String,
    pub hsm_token: String,
    pub log_path: String,
    pub lite: bool,
    pub max_retry: usize,
    pub source: String,
    pub channel_num: u8,
    pub channels: Vec<u8>,
    pub running_channels: HashSet<u8>,
    pub data_type: String,
    pub update_tick: bool,
    pub ack_count: u32,
    pub write_to_file: bool,
    pub background_flag: bool,
    pub update_count: u32,
    pub channel_symbol: Vec<String>,
    pub symbol_dict: HashMap<String, String>,
    pub scrips_count: HashMap<u8, Vec<String>>,
    pub scrips_per_channel: HashMap<u8, Vec<String>>,
    pub restart_flag: bool,
    pub unsub_symbol: Vec<String>,
    pub active_channel: Option<u8>,
    pub resp: HashMap<String, Value>,
    pub valid_token: bool,
    pub dp_sym: HashMap<u16, String>,
    pub index_sym: HashMap<u16, String>,
    pub scrips_sym: HashMap<u16, String>,
    pub symbol_token: HashMap<String, String>,
    pub symbol_limit: usize,
    pub reconnect_attempts: usize,
    pub max_reconnect_attempts: usize,
    pub mode: String,
    pub reconnect_delay: u64,
    pub ack_bool: bool,
    pub data_val: Vec<String>,
    pub index_val: Vec<String>,
    pub lite_val: Vec<String>,
    pub depthvalue: Vec<String>,
    pub ws_sender: Option<UnboundedSender<Message>>,
    pub on_message: Option<Arc<dyn Fn(Value) + Send + Sync>>,
    pub on_error: Option<Arc<dyn Fn(Value) + Send + Sync>>,
}

#[derive(Clone)]
pub struct FyersDataSocket {
    pub inner: Arc<Mutex<FyersDataSocketInner>>,
}

impl FyersDataSocket {
    pub fn new(
        access_token: &str,
        write_to_file: bool,
        log_path: Option<&str>,
        litemode: bool,
        reconnect: bool,
        reconnect_retry: usize,
        on_message: Option<Arc<dyn Fn(Value) + Send + Sync>>,
        on_error: Option<Arc<dyn Fn(Value) + Send + Sync>>,
    ) -> Self {
        let mut scrips_per_channel = HashMap::new();
        for i in 1..=30 {
            scrips_per_channel.insert(i, Vec::new());
        }

        let data_val = vec![
            "ltp",
            "vol_traded_today",
            "last_traded_time",
            "exch_feed_time",
            "bid_size",
            "ask_size",
            "bid_price",
            "ask_price",
            "last_traded_qty",
            "tot_buy_qty",
            "tot_sell_qty",
            "avg_trade_price",
            "OI",
            "low_price",
            "high_price",
            "Yhigh",
            "Ylow",
            "lower_ckt",
            "upper_ckt",
            "open_price",
            "prev_close_price",
            "type",
            "symbol",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let index_val = vec![
            "ltp",
            "prev_close_price",
            "exch_feed_time",
            "high_price",
            "low_price",
            "open_price",
            "type",
            "symbol",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let lite_val = vec!["ltp", "symbol", "type"]
            .into_iter()
            .map(String::from)
            .collect();

        let depthvalue = vec![
            "bid_price1",
            "bid_price2",
            "bid_price3",
            "bid_price4",
            "bid_price5",
            "ask_price1",
            "ask_price2",
            "ask_price3",
            "ask_price4",
            "ask_price5",
            "bid_size1",
            "bid_size2",
            "bid_size3",
            "bid_size4",
            "bid_size5",
            "ask_size1",
            "ask_size2",
            "ask_size3",
            "ask_size4",
            "ask_size5",
            "bid_order1",
            "bid_order2",
            "bid_order3",
            "bid_order4",
            "bid_order5",
            "ask_order1",
            "ask_order2",
            "ask_order3",
            "ask_order4",
            "ask_order5",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let mut max_reconnect_attempts = 50;
        if reconnect_retry < max_reconnect_attempts {
            max_reconnect_attempts = reconnect_retry;
        }

        let inner = FyersDataSocketInner {
            url: "wss://socket.fyers.in/hsm/v1-5/prod".to_string(),
            access_token: access_token.to_string(),
            hsm_token: String::new(),
            log_path: log_path.map_or(String::new(), |p| format!("{}/", p)),
            lite: litemode,
            max_retry: reconnect_retry,
            source: "PythonSDK-3.0.9".to_string(),
            channel_num: 11,
            channels: Vec::new(),
            running_channels: HashSet::new(),
            data_type: String::new(),
            update_tick: false,
            ack_count: 0,
            write_to_file,
            background_flag: false,
            update_count: 0,
            channel_symbol: Vec::new(),
            symbol_dict: HashMap::new(),
            scrips_count: HashMap::new(),
            scrips_per_channel,
            restart_flag: reconnect,
            unsub_symbol: Vec::new(),
            active_channel: None,
            resp: HashMap::new(),
            valid_token: false,
            dp_sym: HashMap::new(),
            index_sym: HashMap::new(),
            scrips_sym: HashMap::new(),
            symbol_token: HashMap::new(),
            symbol_limit: 5000,
            reconnect_attempts: 0,
            max_reconnect_attempts,
            mode: "P".to_string(),
            reconnect_delay: 0,
            ack_bool: false,
            data_val,
            index_val,
            lite_val,
            depthvalue,
            ws_sender: None,
            on_message,
            on_error,
        };

        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn access_token_to_hsmtoken(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        let parts: Vec<&str> = inner.access_token.split('.').collect();
        if parts.len() != 3 {
            inner.trigger_error(json!({
                "type": defines::AUTH_TYPE,
                "code": defines::INVALID_CODE,
                "message": defines::INVALID_TOKEN,
                "s": defines::ERROR,
            }));
            return false;
        }

        // Properly parse Base64 URL_SAFE without attempting broken manual padding math
        let payload_b64 = parts[1].trim_end_matches('=');
        match URL_SAFE_NO_PAD.decode(payload_b64) {
            Ok(decoded) => match serde_json::from_slice::<Value>(&decoded) {
                Ok(token_json) => {
                    let exp = token_json["exp"].as_i64().unwrap_or(0);
                    let today = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;

                    if exp - today < 0 {
                        inner.trigger_error(json!({
                            "type": defines::AUTH_TYPE,
                            "code": defines::TOKEN_EXPIRED,
                            "message": defines::TOKEN_EXPIRED_MSG,
                            "s": defines::ERROR,
                        }));
                        return false;
                    }

                    inner.valid_token = true;
                    inner.hsm_token = token_json["hsm_key"].as_str().unwrap_or("").to_string();
                    println!("hsm token{}", inner.hsm_token);
                    true
                }
                Err(_) => {
                    inner.trigger_invalid_token_error();
                    false
                }
            },
            Err(_) => {
                inner.trigger_invalid_token_error();
                false
            }
        }
    }

    pub async fn subscribe(&self, symbols: Vec<String>, data_type: &str, channel: u8) {
        {
            let mut inner = self.inner.lock().unwrap();
            if !inner.valid_token {
                return;
            }

            inner.data_type = data_type.to_string();
            inner.channel_num = channel;
            inner.channel_resume_pause();

            if let Some(conv_dict) = inner.symbol_conversion(&symbols).await {
                println!("conve {:#?}", conv_dict);
                inner.channel_symbol = conv_dict.values().cloned().collect();

                if inner.symbol_token.len() + conv_dict.len() > 5000 {
                    inner.trigger_error(json!({
                        "code": defines::LIMIT_EXCEED_CODE,
                        "message": defines::LIMIT_EXCEED_MSG_5000,
                        "s": defines::ERROR,
                    }));
                    return;
                }

                inner.symbol_token.extend(conv_dict.clone());

                let current_channel = inner.channel_num;
                inner
                    .scrips_count
                    .insert(current_channel, conv_dict.keys().cloned().collect());

                if let Some(scrips) = inner.scrips_count.get(&inner.channel_num).cloned() {
                    for chunk in scrips.chunks(1500) {
                        if let Ok(msg) = inner.build_subscription_msg(chunk.to_vec()) {
                            inner.add_message(msg);
                        }
                    }
                }
            }
        }
    }

    pub async fn unsubscribe(&self, symbols: Vec<String>, data_type: &str, channel: u8) {
        let mut inner = self.inner.lock().unwrap();
        if !inner.valid_token {
            return;
        }

        inner.data_type = data_type.to_string();
        inner.channel_num = channel;

        inner.channel_resume_pause();
        if let Some(conv_dict) = inner.symbol_conversion(&symbols).await {
            inner.unsub_symbol = conv_dict.keys().cloned().collect();

            let current_channel = inner.channel_num;
            if let Some(scrips) = inner.scrips_count.get(&current_channel).cloned() {
                inner.unsub_symbol.retain(|symb| scrips.contains(symb));
            }

            if !inner.unsub_symbol.is_empty() {
                let targets = inner.unsub_symbol.clone();
                for chunk in targets.chunks(1500) {
                    if let Ok(msg) = inner.build_unsubscription_msg(chunk.to_vec()) {
                        inner.add_message(msg);
                    }
                }
            } else {
                inner.trigger_error(json!({
                    "code": defines::INVALID_CODE,
                    "message": defines::INVALID_SYMBOLS,
                    "s": defines::ERROR,
                }));
            }
        }
    }

    pub fn channel_resume(&self, channel: u8) {
        let mut inner = self.inner.lock().unwrap();
        inner.channel_num = channel;
        inner.channel_resume_pause();
    }

    pub fn connect(&self) {
        let shadow = self.clone();
        tokio::spawn(async move {
            if shadow.access_token_to_hsmtoken() {
                shadow.init_connection().await;
            }
        });
    }

    async fn init_connection(&self) {
        let (url, restart_flag) = {
            let mut inner = self.inner.lock().unwrap();
            if inner.write_to_file {
                inner.background_flag = true;
            }
            (inner.url.clone(), inner.restart_flag)
        };

        loop {
            println!("checkk");
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    let (mut write_half, mut read_half) = ws_stream.split();
                    let (tx, mut rx) = unbounded_channel::<Message>();

                    {
                        let mut inner = self.inner.lock().unwrap();
                        inner.ws_sender = Some(tx);
                        inner.on_open();
                    }

                    // Task 1: Background Write Pipeline
                    let write_task = tokio::spawn(async move {
                        while let Some(message) = rx.recv().await {
                            if let Err(e) = write_half.send(message).await {
                                eprintln!("Outbound write loop pipe disconnected: {}", e);
                                break;
                            }
                        }
                    });

                    // Task 2: Background Keepalive Worker
                    let shadow_ping = self.clone();
                    let ping_task = tokio::spawn(async move {
                        loop {
                            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                            let inner = shadow_ping.inner.lock().unwrap();
                            if inner.ws_sender.is_none() {
                                break;
                            }
                            inner.send_raw_message(vec![0, 1, 11]);
                        }
                    });

                    // Task 3: Inbound Message Parser
                    let shadow_read = self.clone();
                    while let Some(Ok(msg)) = read_half.next().await {
                        println!("checkkss");
                        println!("📥 RAW BINARY PACKET ARRIVED! Length: {} bytes", msg.len());
                        if msg.is_binary() {
                            let bytes = msg.into_data();
                            let mut inner = shadow_read.inner.lock().unwrap();
                            if let Err(e) = inner.route_response_msg(&bytes) {
                                eprintln!("Parser payload failure: {}", e);
                            }
                        }
                    }

                    ping_task.abort();
                    write_task.abort();
                }
                Err(e) => {
                    let inner = self.inner.lock().unwrap();
                    inner.trigger_error(json!(format!("Websocket connection error: {}", e)));
                }
            }

            if !restart_flag {
                break;
            }

            // Exponential backoff logic
            let (delay, attempt, max) = {
                let mut inner = self.inner.lock().unwrap();
                if inner.reconnect_attempts >= inner.max_reconnect_attempts {
                    inner.trigger_error(json!(
                        "Max reconnect attempts reached. Connection abandoned."
                    ));
                    break;
                }
                if inner.reconnect_attempts % 5 == 0 {
                    inner.reconnect_delay += 5;
                }
                inner.reconnect_attempts += 1;
                inner.ws_sender = None;
                let current_channel = inner.channel_num;
                inner.scrips_per_channel.insert(current_channel, Vec::new());
                inner.symbol_token.clear();
                (
                    inner.reconnect_delay,
                    inner.reconnect_attempts,
                    inner.max_reconnect_attempts,
                )
            };

            println!(
                "Attempting reconnect {} of {} after sleeping {}s...",
                attempt, max, delay
            );
            tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
        }
    }
}

impl FyersDataSocketInner {
    fn trigger_invalid_token_error(&self) {
        self.trigger_error(json!({
            "type": defines::AUTH_TYPE,
            "code": defines::INVALID_CODE,
            "message": defines::INVALID_TOKEN,
            "s": defines::ERROR,
        }));
    }

    fn trigger_error(&self, err: Value) {
        if let Some(ref cb) = self.on_error {
            cb(err);
        } else if self.write_to_file {
            println!("LOG DEBUG: ERROR Response:{}", err);
        } else {
            println!("Error: {}", err);
        }
    }

    fn trigger_message(&self, msg: Value) {
        if let Some(ref cb) = self.on_message {
            cb(msg);
        } else if self.write_to_file {
            println!("LOG DEBUG: Response:{}", msg);
        } else {
            println!("Response: {}", msg);
        }
    }

    fn add_message(&self, message: Vec<u8>) {
        self.send_raw_message(message);
    }

    fn send_raw_message(&self, bytes: Vec<u8>) {
        if let Some(ref sender) = self.ws_sender {
            let _ = sender.send(Message::Binary(bytes));
        }
    }

    fn on_open(&mut self) {
        self.trigger_message(json!({"message": "Connection opened successfully"}));

        if let Ok(auth_msg) = self.build_access_token_msg() {
            self.add_message(auth_msg);
        }

        self.reconnect_attempts = 0;
        self.reconnect_delay = 0;

        let mode_msg = if self.lite {
            self.build_lite_mode_msg()
        } else {
            self.build_full_mode_msg()
        };
        if let Ok(msg) = mode_msg {
            self.add_message(msg);
        }
    }

    fn channel_resume_pause(&mut self) {
        if self.ws_sender.is_some() {
            if let Some(active_ch) = self.active_channel {
                if active_ch != self.channel_num {
                    if let Ok(msg) = self.build_channel_control_msg(active_ch, 7) {
                        self.add_message(msg);
                    }
                    if self.running_channels.contains(&self.channel_num) {
                        if let Ok(msg) = self.build_channel_control_msg(self.channel_num, 8) {
                            self.add_message(msg);
                        }
                    }
                }
            }
        }
        self.running_channels.insert(self.channel_num);
        self.active_channel = Some(self.channel_num);
    }

    async fn symbol_conversion(&self, symbols: &[String]) -> Option<HashMap<String, String>> {
        let current_ch_count = self
            .scrips_per_channel
            .get(&self.channel_num)
            .map_or(0, |v| v.len());
        let total_symbols = symbols.len();

        if current_ch_count > self.symbol_limit
            || total_symbols > self.symbol_limit
            || (current_ch_count + total_symbols) > self.symbol_limit
        {
            self.trigger_error(json!({
                "code": defines::LIMIT_EXCEED_CODE,
                "message": defines::LIMIT_EXCEED_MSG_5000,
                "s": defines::ERROR,
            }));
            return None;
        }

        let mut symbol_dict = HashMap::new();
        let mut wrong_symbols = Vec::new();
        let mut symb_flag = false;

        let conv = SymbolConversion::new(&self.access_token, DataType::SymbolUpdate);
        println!("symbols {:#?}", conv);
        for chunk in symbols.chunks(500) {
            let res = conv.symbol_to_hsmtoken(chunk.to_vec()).await;
            if let Ok(res) = res {
                symbol_dict.extend(res.0);
                if !res.1.is_empty() {
                    wrong_symbols.extend(res.1);
                    symb_flag = true;
                }
            }
        }

        if symb_flag {
            self.trigger_error(json!({
                "code": defines::INVALID_CODE,
                "message": defines::INVALID_SYMBOLS,
                "s": defines::ERROR,
                "type": defines::SUBS_TYPE,
                "invalid_symbols": wrong_symbols,
            }));
        }
        Some(symbol_dict)
    }

    // --- PROTOCOL MESSAGE SERIALIZATION LAYER (TLV GENERATION) ---

    fn build_access_token_msg(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        let token_bytes = self.hsm_token.as_bytes();
        let source_bytes = self.source.as_bytes();
        let buffer_size = 18 + token_bytes.len() + source_bytes.len();

        buf.write_u16::<BigEndian>((buffer_size - 2) as u16)?;
        buf.write_u8(1)?; // ReqType
        buf.write_u8(4)?; // FieldCount

        buf.write_u8(1)?; // Field-1 ID
        buf.write_u16::<BigEndian>(token_bytes.len() as u16)?;
        buf.write_all(token_bytes)?;

        buf.write_u8(2)?; // Field-2 ID
        buf.write_u16::<BigEndian>(1)?;
        buf.write_all(self.mode.as_bytes())?;

        buf.write_u8(3)?; // Field-3 ID
        buf.write_u16::<BigEndian>(1)?;
        buf.write_u8(1)?;

        buf.write_u8(4)?; // Field-4 ID
        buf.write_u16::<BigEndian>(source_bytes.len() as u16)?;
        buf.write_all(source_bytes)?;

        Ok(buf)
    }

    fn build_lite_mode_msg(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        buf.write_u16::<BigEndian>(0)?;
        buf.write_u8(12)?;
        buf.write_u8(2)?;

        let mut channel_bits: u64 = 0;
        if self.channel_num < 64 && self.channel_num > 0 {
            channel_bits |= 1 << self.channel_num;
        }

        buf.write_u8(1)?;
        buf.write_u16::<BigEndian>(8)?;
        buf.write_u64::<BigEndian>(channel_bits)?;

        buf.write_u8(2)?;
        buf.write_u16::<BigEndian>(1)?;
        buf.write_u8(76)?;

        Ok(buf)
    }

    fn build_full_mode_msg(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        buf.write_u16::<BigEndian>(0)?;
        buf.write_u8(12)?;
        buf.write_u8(2)?;

        let mut channel_bits: u64 = 0;
        if self.channel_num < 64 && self.channel_num > 0 {
            channel_bits |= 1 << self.channel_num;
        }

        buf.write_u8(1)?;
        buf.write_u16::<BigEndian>(8)?;
        buf.write_u64::<BigEndian>(channel_bits)?;

        buf.write_u8(2)?;
        buf.write_u16::<BigEndian>(1)?;
        buf.write_u8(70)?;

        Ok(buf)
    }

    fn build_subscription_msg(&mut self, symbols: Vec<String>) -> Result<Vec<u8>, std::io::Error> {
        self.scrips_per_channel
            .entry(self.channel_num)
            .or_insert_with(Vec::new)
            .extend(symbols.clone());

        let mut scrips_data = Vec::new();
        scrips_data.write_u16::<BigEndian>(symbols.len() as u16)?;
        for scrip in symbols {
            let scrip_bytes = scrip.as_bytes();
            scrips_data.write_u8(scrip_bytes.len() as u8)?;
            scrips_data.write_all(scrip_bytes)?;
        }

        let mut buf = Vec::new();
        // FIX APPLIED HERE: Ensure data_len strictly matches exactly the amount of packet bytes!
        // The original python script falsely included len(access_token) + len(source)
        let data_len = 9 + scrips_data.len();
        buf.write_u16::<BigEndian>(data_len as u16)?;
        buf.write_u8(4)?;
        buf.write_u8(2)?;

        buf.write_u8(1)?;
        buf.write_u16::<BigEndian>(scrips_data.len() as u16)?;
        buf.write_all(&scrips_data)?;

        buf.write_u8(2)?;
        buf.write_u16::<BigEndian>(1)?;
        buf.write_u8(self.channel_num)?;

        Ok(buf)
    }

    fn build_unsubscription_msg(&self, symbols: Vec<String>) -> Result<Vec<u8>, std::io::Error> {
        let mut scrips_data = Vec::new();
        scrips_data.write_u16::<BigEndian>(symbols.len() as u16)?;
        for scrip in symbols {
            let scrip_bytes = scrip.as_bytes();
            scrips_data.write_u8(scrip_bytes.len() as u8)?;
            scrips_data.write_all(scrip_bytes)?;
        }

        let mut buf = Vec::new();
        // FIX APPLIED HERE:
        let data_len = 9 + scrips_data.len();
        buf.write_u16::<BigEndian>(data_len as u16)?;
        buf.write_u8(5)?;
        buf.write_u8(2)?;

        buf.write_u8(1)?;
        buf.write_u16::<BigEndian>(scrips_data.len() as u16)?;
        buf.write_all(&scrips_data)?;

        buf.write_u8(2)?;
        buf.write_u16::<BigEndian>(1)?;
        buf.write_u8(self.channel_num)?;

        Ok(buf)
    }

    fn build_channel_control_msg(
        &self,
        channel: u8,
        msg_type: u8,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        buf.write_u16::<BigEndian>(0)?;
        buf.write_u8(msg_type)?;
        buf.write_u8(1)?;

        let mut channel_bits: u64 = 0;
        if channel < 64 && channel > 0 {
            channel_bits |= 1 << channel;
        }

        buf.write_u8(1)?;
        buf.write_u16::<BigEndian>(8)?;
        buf.write_u64::<BigEndian>(channel_bits)?;

        Ok(buf)
    }

    fn acknowledgement_msg(&self, message_number: u32) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        buf.write_u16::<BigEndian>(9)?;
        buf.write_u8(3)?;
        buf.write_u8(1)?;
        buf.write_u8(1)?;
        buf.write_u16::<BigEndian>(4)?;
        buf.write_u32::<BigEndian>(message_number)?;
        Ok(buf)
    }

    // --- STREAM RESPONSE ROUTER & HANDLERS ---

    fn route_response_msg(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        if data.len() < 3 {
            return Ok(());
        }
        let mut cursor = Cursor::new(data);
        let _frame_len = cursor.read_u16::<BigEndian>()?;
        let resp_type = cursor.read_u8()?;

        match resp_type {
            1 => self.parse_auth_resp(data)?,
            4 => self.parse_subscribe_resp(data)?,
            5 => self.parse_unsubscribe_resp(data)?,
            6 => self.parse_datafeed_resp(data)?,
            7 | 8 => self.parse_resume_pause_resp(data, resp_type)?,
            12 => self.parse_lite_full_mode_resp(data)?,
            _ => {}
        }
        Ok(())
    }

    fn parse_auth_resp(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(SeekFrom::Start(5))?;
        let field_length = cursor.read_u16::<BigEndian>()?;
        let mut string_buf = vec![0u8; field_length as usize];
        cursor.read_exact(&mut string_buf)?;
        let string_val = String::from_utf8_lossy(&string_buf);

        if string_val == "K" {
            self.trigger_message(json!({
                "type": defines::AUTH_TYPE, "code": defines::SUCCESS_CODE,
                "message": defines::AUTH_SUCCESS, "s": defines::SUCCESS,
            }));
        } else {
            self.trigger_error(json!({
                "type": defines::AUTH_TYPE, "code": defines::AUTH_ERROR_CODE,
                "message": defines::AUTH_FAIL, "s": defines::ERROR,
            }));
        }

        cursor.seek(SeekFrom::Current(1))?;
        let _ack_field_len = cursor.read_u16::<BigEndian>()?;
        self.ack_count = cursor.read_u32::<BigEndian>()?;
        Ok(())
    }

    fn parse_subscribe_resp(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(SeekFrom::Start(5))?;
        let field_length = cursor.read_u16::<BigEndian>()?;
        let mut buf = vec![0u8; field_length as usize];
        cursor.read_exact(&mut buf)?;
        let string_val = String::from_utf8_lossy(&buf);

        if string_val.starts_with('K') {
            self.trigger_message(json!({
                "type": defines::SUBS_TYPE, "code": defines::SUCCESS_CODE,
                "message": defines::SUBSCRIBE_SUCCESS, "s": defines::SUCCESS,
            }));
        } else {
            self.trigger_error(json!({
                "type": defines::SUBS_TYPE, "code": defines::SUBS_ERROR_CODE,
                "message": defines::SUBSCRIBE_FAIL, "s": defines::ERROR,
            }));
        }
        Ok(())
    }

    fn parse_unsubscribe_resp(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(SeekFrom::Start(5))?;
        let field_length = cursor.read_u16::<BigEndian>()?;
        let mut buf = vec![0u8; field_length as usize];
        cursor.read_exact(&mut buf)?;
        let string_val = String::from_utf8_lossy(&buf);

        if string_val.starts_with('K') {
            self.trigger_message(json!({
                "type": defines::UNSUBS_TYPE, "code": defines::SUCCESS_CODE,
                "message": defines::UNSUBSCRIBE_SUCCESS, "s": defines::SUCCESS,
            }));

            let targets = self.unsub_symbol.clone();
            for symbol in &targets {
                if let Some(scrips) = self
                    .scrips_per_channel
                    .get_mut(&self.active_channel.unwrap_or(0))
                {
                    scrips.retain(|s| s != symbol);
                }
                let ch_match_count = self
                    .running_channels
                    .iter()
                    .filter(|&&ch| {
                        self.scrips_per_channel
                            .get(&ch)
                            .map_or(false, |v| v.contains(symbol))
                    })
                    .count();
                if ch_match_count <= 1 {
                    self.symbol_token.remove(symbol);
                }
            }
        } else {
            self.trigger_error(json!({
                "type": defines::UNSUBS_TYPE, "code": defines::UNSUBS_ERROR_CODE,
                "message": defines::UNSUBSCRIBE_FAIL, "s": defines::ERROR,
            }));
        }
        Ok(())
    }

    fn parse_lite_full_mode_resp(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(SeekFrom::Start(3))?;
        let field_count = cursor.read_u8()?;
        if field_count >= 1 {
            cursor.seek(SeekFrom::Current(1))?;
            let field_length = cursor.read_u16::<BigEndian>()?;
            let mut buf = vec![0u8; field_length as usize];
            cursor.read_exact(&mut buf)?;
            let string_val = String::from_utf8_lossy(&buf);

            if string_val == "K" {
                let (m_type, m_text) = if self.lite {
                    (defines::LITE_MODE_TYPE, defines::LITE_MODE)
                } else {
                    (defines::FULL_MODE_TYPE, defines::FULL_MODE)
                };
                self.trigger_message(json!({
                    "type": m_type, "code": defines::SUCCESS_CODE,
                    "message": m_text, "s": defines::SUCCESS,
                }));
            } else {
                self.trigger_error(json!({
                    "code": defines::MODE_ERROR_CODE,
                    "message": defines::MODE_CHANGE_ERROR, "s": defines::ERROR,
                }));
            }
        }
        Ok(())
    }

    fn parse_resume_pause_resp(
        &mut self,
        data: &[u8],
        channeltype: u8,
    ) -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(data);
        cursor.seek(SeekFrom::Start(5))?;
        let field_length = cursor.read_u16::<BigEndian>()?;
        let mut buf = vec![0u8; field_length as usize];
        cursor.read_exact(&mut buf)?;
        let string_val = String::from_utf8_lossy(&buf);

        let is_pause = channeltype == 7;
        let m_type = if is_pause {
            defines::CH_PAUSE_TYPE
        } else {
            defines::CH_RESUME_TYPE
        };
        let m_text = if is_pause {
            defines::CHANNEL_PAUSED
        } else {
            defines::CHANNEL_RESUMED
        };
        let e_code = if is_pause {
            defines::PAUSE_ERROR_CODE
        } else {
            defines::RESUME_ERROR_CODE
        };

        if string_val == "K" {
            self.trigger_message(json!({
                "type": m_type, "code": defines::SUCCESS_CODE,
                "message": m_text, "s": defines::SUCCESS,
            }));
        } else {
            self.trigger_error(json!({
                "type": m_type, "code": e_code,
                "message": defines::CHANNEL_CHANGE_FAIL, "s": defines::ERROR,
            }));
        }
        Ok(())
    }

    fn response_output(&self, mut data_resp: Value, data_type: &str) {
        let precision = data_resp["precision"].as_u64().unwrap_or(0) as i32;
        let multiplier = data_resp["multiplier"].as_f64().unwrap_or(1.0);
        let scale = 10.0f64.powi(precision) * multiplier;

        let precision_calcu_value = vec![
            "ltp",
            "bid_price",
            "ask_price",
            "avg_trade_price",
            "low_price",
            "high_price",
            "open_price",
            "prev_close_price",
        ];

        let mut out = json!({});
        if data_resp.get("bidPrice1").is_none() && self.lite {
            for val in &self.lite_val {
                if let Some(v) = data_resp.get(val) {
                    if val == "ltp" {
                        out[val] = json!(v.as_f64().unwrap_or(0.0) / scale);
                    } else {
                        out[val] = v.clone();
                    }
                }
            }
            if out.get("prev_close_price").is_some() && out.get("ltp").is_some() {
                let ltp = out["ltp"].as_f64().unwrap_or(0.0);
                let prev = out["prev_close_price"].as_f64().unwrap_or(0.0);
                let ch = (ltp - prev * 100.0).round() / 100.0;
                out["ch"] = json!(ch);
                if prev != 0.0 {
                    out["chp"] = json!(((ch / prev) * 100.0 * 100.0).round() / 100.0);
                }
            }
        } else {
            match data_type {
                "depth" => {
                    for (i, val) in self.depthvalue.iter().enumerate() {
                        if let Some(v) = data_resp.get(val) {
                            if i < 10 {
                                out[val] = json!(v.as_f64().unwrap_or(0.0) / scale);
                            } else {
                                out[val] = v.clone();
                            }
                        }
                    }
                }
                "scrips" => {
                    for val in &self.data_val {
                        if let Some(v) = data_resp.get(val) {
                            if precision_calcu_value.contains(&val.as_str())
                                && val != "upper_ckt"
                                && val != "lower_ckt"
                            {
                                out[val] = json!(v.as_f64().unwrap_or(0.0) / scale);
                            } else {
                                out[val] = v.clone();
                            }
                        }
                    }
                    out["lower_ckt"] = json!(0);
                    out["upper_ckt"] = json!(0);
                    if out.get("prev_close_price").is_some() && out.get("ltp").is_some() {
                        let ltp = out["ltp"].as_f64().unwrap_or(0.0);
                        let prev = out["prev_close_price"].as_f64().unwrap_or(0.0);
                        if prev != 0.0 {
                            let ch = (ltp - prev * 10000.0).round() / 10000.0;
                            out["ch"] = json!(ch);
                            out["chp"] = json!(((ch / prev) * 100.0 * 10000.0).round() / 10000.0);
                        }
                    }
                    if let Some(obj) = out.as_object_mut() {
                        obj.remove("OI");
                        obj.remove("Yhigh");
                        obj.remove("Ylow");
                    }
                }
                _ => {
                    // index
                    for (i, val) in self.index_val.iter().enumerate() {
                        if let Some(v) = data_resp.get(val) {
                            if [0, 1, 3, 4, 5].contains(&i) {
                                out[val] = json!(v.as_f64().unwrap_or(0.0) / scale);
                            } else {
                                out[val] = v.clone();
                            }
                        }
                    }
                    if out.get("prev_close_price").is_some() && out.get("ltp").is_some() {
                        let ltp = out["ltp"].as_f64().unwrap_or(0.0);
                        let prev = out["prev_close_price"].as_f64().unwrap_or(0.0);
                        let ch = (ltp - prev * 100.0).round() / 100.0;
                        out["ch"] = json!(ch);
                        if prev != 0.0 {
                            out["chp"] = json!(((ch / prev) * 100.0 * 100.0).round() / 100.0);
                        }
                    }
                }
            }
        }
        self.trigger_message(out);
    }

    fn parse_datafeed_resp(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut cursor = Cursor::new(data);

        if self.ack_count > 0 {
            self.update_count += 1;
            cursor.seek(SeekFrom::Start(3))?;
            let message_num = cursor.read_u32::<BigEndian>()?;
            if self.update_count == self.ack_count {
                if let Ok(ack) = self.acknowledgement_msg(message_num) {
                    self.add_message(ack);
                }
                self.update_count = 0;
            }
        }

        cursor.seek(SeekFrom::Start(7))?;
        let scrip_count = cursor.read_u16::<BigEndian>()?;

        for _ in 0..scrip_count {
            let data_type = cursor.read_u8()?;
            match data_type {
                83 => {
                    // --- SNAPSHOT ENGINE ---
                    let topic_id = cursor.read_u16::<BigEndian>()?;
                    let name_len = cursor.read_u8()? as usize;
                    let mut name_buf = vec![0u8; name_len];
                    cursor.read_exact(&mut name_buf)?;
                    let topic_name = String::from_utf8_lossy(&name_buf).to_string();

                    let prefix = if topic_name.len() >= 2 {
                        &topic_name[..2]
                    } else {
                        ""
                    };
                    match prefix {
                        "dp" => {
                            self.dp_sym.insert(topic_id, topic_name.clone());
                            let mut inner_map = json!({});
                            let field_count = cursor.read_u8()?;
                            for idx in 0..field_count as usize {
                                let value = cursor.read_i32::<BigEndian>()?;
                                if value != i32::MIN {
                                    if let Some(k) = self.depthvalue.get(idx) {
                                        inner_map[k] = json!(value);
                                    }
                                }
                            }
                            cursor.seek(SeekFrom::Current(2))?;
                            inner_map["multiplier"] = json!(cursor.read_u16::<BigEndian>()?);
                            inner_map["precision"] = json!(cursor.read_u8()?);

                            let keys = ["exchange", "exchange_token", "symbol"];
                            for k in keys.iter() {
                                let s_len = cursor.read_u8()? as usize;
                                let mut s_buf = vec![0u8; s_len];
                                cursor.read_exact(&mut s_buf)?;
                                inner_map[*k] = json!(String::from_utf8_lossy(&s_buf).to_string());
                            }
                            inner_map["type"] = json!("dp");
                            if let Some(tok) = self.symbol_token.get(&topic_name) {
                                inner_map["symbol"] = json!(tok);
                            }

                            self.resp.insert(topic_name.clone(), inner_map);
                            if let Some(target) = self.resp.get(&topic_name).cloned() {
                                self.response_output(target, "depth");
                            }
                        }
                        "if" => {
                            self.index_sym.insert(topic_id, topic_name.clone());
                            let mut inner_map = json!({});
                            let field_count = cursor.read_u8()?;
                            for idx in 0..field_count as usize {
                                let value = cursor.read_i32::<BigEndian>()?;
                                if value != i32::MIN {
                                    if let Some(k) = self.index_val.get(idx) {
                                        inner_map[k] = json!(value);
                                    }
                                }
                            }
                            cursor.seek(SeekFrom::Current(2))?;
                            inner_map["multiplier"] = json!(cursor.read_u16::<BigEndian>()?);
                            inner_map["precision"] = json!(cursor.read_u8()?);

                            let keys = ["exchange", "exchange_token", "symbol"];
                            for k in keys.iter() {
                                let s_len = cursor.read_u8()? as usize;
                                let mut s_buf = vec![0u8; s_len];
                                cursor.read_exact(&mut s_buf)?;
                                inner_map[*k] = json!(String::from_utf8_lossy(&s_buf).to_string());
                            }
                            inner_map["type"] = json!("if");
                            if let Some(tok) = self.symbol_token.get(&topic_name) {
                                inner_map["symbol"] = json!(tok);
                            }

                            self.resp.insert(topic_name.clone(), inner_map);
                            if let Some(target) = self.resp.get(&topic_name).cloned() {
                                self.response_output(target, "index");
                            }
                        }
                        _ => {
                            self.scrips_sym.insert(topic_id, topic_name.clone());
                            let mut inner_map = json!({});
                            let field_count = cursor.read_u8()?;
                            for idx in 0..field_count as usize {
                                let value = cursor.read_i32::<BigEndian>()?;
                                if value != i32::MIN {
                                    if let Some(k) = self.data_val.get(idx) {
                                        inner_map[k] = json!(value);
                                    }
                                }
                            }
                            cursor.seek(SeekFrom::Current(2))?;
                            inner_map["multiplier"] = json!(cursor.read_u16::<BigEndian>()?);
                            inner_map["precision"] = json!(cursor.read_u8()?);

                            let keys = ["exchange", "exchange_token", "symbol"];
                            for k in keys.iter() {
                                let s_len = cursor.read_u8()? as usize;
                                let mut s_buf = vec![0u8; s_len];
                                cursor.read_exact(&mut s_buf)?;
                                inner_map[*k] = json!(String::from_utf8_lossy(&s_buf).to_string());
                            }
                            inner_map["type"] = json!("sf");
                            if let Some(tok) = self.symbol_token.get(&topic_name) {
                                inner_map["symbol"] = json!(tok);
                            }

                            self.resp.insert(topic_name.clone(), inner_map);
                            if let Some(target) = self.resp.get(&topic_name).cloned() {
                                self.response_output(target, "scrips");
                            }
                        }
                    }
                }
                85 => {
                    // --- FULL MODE STREAM TICK PARSER ---
                    let topic_id = cursor.read_u16::<BigEndian>()?;
                    let field_count = cursor.read_u8()?;

                    let mut sf_flag = false;
                    let mut idx_flag = false;
                    let mut dp_flag = false;
                    self.update_tick = false;

                    for idx in 0..field_count as usize {
                        let value = cursor.read_i32::<BigEndian>()?;
                        if value == i32::MIN {
                            continue;
                        }

                        if let Some(t_name) = self.scrips_sym.get(&topic_id) {
                            if let Some(key) = self.data_val.get(idx) {
                                if let Some(map) = self.resp.get_mut(t_name) {
                                    if map.get(key).map_or(true, |v| {
                                        v.as_i64().unwrap_or(i64::MIN) != value as i64
                                    }) {
                                        map[key] = json!(value);
                                        self.update_tick = true;
                                    }
                                }
                            }
                            sf_flag = true;
                        } else if let Some(t_name) = self.index_sym.get(&topic_id) {
                            if let Some(key) = self.index_val.get(idx) {
                                if let Some(map) = self.resp.get_mut(t_name) {
                                    if map.get(key).map_or(true, |v| {
                                        v.as_i64().unwrap_or(i64::MIN) != value as i64
                                    }) {
                                        map[key] = json!(value);
                                        self.update_tick = true;
                                    }
                                }
                            }
                            idx_flag = true;
                        } else if let Some(t_name) = self.dp_sym.get(&topic_id) {
                            if let Some(key) = self.depthvalue.get(idx) {
                                if let Some(map) = self.resp.get_mut(t_name) {
                                    if map.get(key).map_or(true, |v| {
                                        v.as_i64().unwrap_or(i64::MIN) != value as i64
                                    }) {
                                        map[key] = json!(value);
                                        self.update_tick = true;
                                    }
                                }
                            }
                            dp_flag = true;
                        }
                    }

                    if self.update_tick {
                        if sf_flag {
                            if let Some(t_name) = self.scrips_sym.get(&topic_id) {
                                if let Some(target) = self.resp.get(t_name).cloned() {
                                    self.response_output(target, "scrips");
                                }
                            }
                        } else if idx_flag {
                            if let Some(t_name) = self.index_sym.get(&topic_id) {
                                if let Some(target) = self.resp.get(t_name).cloned() {
                                    self.response_output(target, "index");
                                }
                            }
                        } else if dp_flag {
                            if let Some(t_name) = self.dp_sym.get(&topic_id) {
                                if let Some(target) = self.resp.get(t_name).cloned() {
                                    self.response_output(target, "depth");
                                }
                            }
                        }
                    }
                }
                76 => {
                    // --- LITE MODE STREAM PARSER ---
                    let topic_id = cursor.read_u16::<BigEndian>()?;
                    let value = cursor.read_i32::<BigEndian>()?;
                    if value == i32::MIN {
                        continue;
                    }

                    let mut sf_flag = false;
                    let mut idx_flag = false;
                    if let Some(t_name) = self.scrips_sym.get(&topic_id) {
                        if let Some(key) = self.data_val.get(0) {
                            if let Some(map) = self.resp.get_mut(t_name) {
                                if map.get(key).map_or(true, |v| {
                                    v.as_i64().unwrap_or(i64::MIN) != value as i64
                                }) {
                                    map[key] = json!(value);
                                    map["type"] = json!("sf");
                                    sf_flag = true;
                                }
                            }
                        }
                    } else if let Some(t_name) = self.index_sym.get(&topic_id) {
                        if let Some(key) = self.index_val.get(0) {
                            if let Some(map) = self.resp.get_mut(t_name) {
                                if map.get(key).map_or(true, |v| {
                                    v.as_i64().unwrap_or(i64::MIN) != value as i64
                                }) {
                                    map[key] = json!(value);
                                    map["type"] = json!("if");
                                    idx_flag = true;
                                }
                            }
                        }
                    }

                    if sf_flag {
                        if let Some(t_name) = self.scrips_sym.get(&topic_id) {
                            if let Some(target) = self.resp.get(t_name).cloned() {
                                self.response_output(target, "scrips");
                            }
                        }
                    } else if idx_flag {
                        if let Some(t_name) = self.index_sym.get(&topic_id) {
                            if let Some(target) = self.resp.get(t_name).cloned() {
                                self.response_output(target, "index");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
