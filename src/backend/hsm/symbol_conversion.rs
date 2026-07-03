use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::backend::hsm::map::FyersMap;

pub struct SymbolConversion {
    pub access_token: String,
    pub data_type: String,
    pub log_path: String,
    symbols_token_api: String,
}

impl SymbolConversion {
    pub fn new(access_token: &str, data_type: &str, log_path: &str) -> Self {
        let token = if access_token.contains(':') {
            access_token.split(':').nth(1).unwrap_or("").to_string()
        } else {
            access_token.to_string()
        };

        Self {
            access_token: token,
            data_type: data_type.to_string(),
            log_path: log_path.to_string(),
            symbols_token_api: "https://api-t1.fyers.in/data/symbol-token".to_string(),
        }
    }

    pub fn symbol_to_hsmtoken(
        &self,
        symbols: Vec<String>,
    ) -> Result<(HashMap<String, String>, Vec<String>, bool, String), Box<dyn std::error::Error>>
    {
        let client = Client::new();
        let payload = serde_json::json!({ "symbols": symbols });

        let response: Value = client
            .post(&self.symbols_token_api)
            .header("Authorization", &self.access_token)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()?
            .json()?;

        let mut data_dict = HashMap::new();
        let mut wrong_symbols = Vec::new();
        let mut dp_index_flag = false;

        // LOAD VIA OUR STRUCT INSTEAD OF RAW JSON
        let mapper = FyersMap::from_file("map.json")?;
        let index_dict = &mapper.index_dict;
        let exch_seg_dict = &mapper.exch_seg_dict;

        if response["s"] == "ok" {
            if let Some(valid_symbols) = response["validSymbol"].as_object() {
                for (symbol, fytoken_val) in valid_symbols {
                    let fytoken = fytoken_val.as_str().unwrap_or("");
                    if fytoken.len() < 4 {
                        continue;
                    }

                    let ex_sg = &fytoken[0..4];

                    // Now exch_seg_dict returns a standard Rust &String, no unwrapping required
                    if let Some(segment) = exch_seg_dict.get(ex_sg) {
                        let symbol_split: Vec<&str> = symbol.split('-').collect();
                        let mut update_dict = true;
                        let mut hsm_symbol = String::new();

                        if symbol_split.len() > 1
                            && symbol_split.last() == Some(&"INDEX")
                            && self.data_type != "DepthUpdate"
                        {
                            // index_dict also safely returns a &String
                            let exch_token = index_dict.get(symbol).map_or_else(
                                || {
                                    symbol
                                        .split(':')
                                        .nth(1)
                                        .unwrap_or("")
                                        .split('-')
                                        .next()
                                        .unwrap_or("")
                                },
                                |v| v.as_str(),
                            );
                            hsm_symbol = format!("if|{}|{}", segment, exch_token);
                        } else if self.data_type == "DepthUpdate"
                            && symbol_split.last() != Some(&"INDEX")
                        {
                            let exch_token = &fytoken[10..];
                            hsm_symbol = format!("dp|{}|{}", segment, exch_token);
                        } else if self.data_type == "SymbolUpdate" {
                            let exch_token = &fytoken[10..];
                            hsm_symbol = format!("sf|{}|{}", segment, exch_token);
                        } else if self.data_type == "DepthUpdate"
                            && symbol_split.last() == Some(&"INDEX")
                        {
                            update_dict = false;
                            dp_index_flag = true;
                        }

                        if update_dict && !hsm_symbol.is_empty() {
                            data_dict.insert(hsm_symbol, symbol.clone());
                        }
                    }
                }
            }

            if let Some(invalid) = response["invalidSymbol"].as_array() {
                wrong_symbols = invalid
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect();
            }

            Ok((data_dict, wrong_symbols, dp_index_flag, String::new()))
        } else {
            let msg = response["message"].as_str().unwrap_or("").to_string();
            Ok((HashMap::new(), Vec::new(), dp_index_flag, msg))
        }
    }
}
