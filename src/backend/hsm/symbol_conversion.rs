use std::{collections::HashMap, fmt};

use http::{
    HeaderMap, HeaderValue,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use serde::Deserialize;
use serde_json::json;

use crate::backend::hsm::model::{exch_seg_dict, index_dict};

#[derive(Debug)]
pub struct SymbolConversion {
    pub data_type: DataType,
    pub access_token: String,
    pub symbols_token_api: String,
}

impl SymbolConversion {
    pub fn new(access_token: &str, data_type: DataType) -> Self {
        let mut at = String::new();
        if let Some((_, token)) = access_token.split_once(':') {
            at = token.to_string();
        }
        SymbolConversion {
            data_type,
            access_token: at,
            symbols_token_api: String::from("https://api-t1.fyers.in/data/symbol-token"),
        }
    }

    /// Converts symbols to HSM tokens.
    ///
    /// # Arguments
    ///
    /// * `symbols` - A slice of symbols to be converted.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// * `HashMap<String, String>` - A mapping of symbols to HSM tokens.
    /// * `Vec<String>` - A list of symbols that could not be converted.
    pub async fn symbol_to_hsmtoken(
        &self,
        symbols: Vec<String>,
    ) -> Result<(HashMap<String, String>, Vec<String>), Box<dyn std::error::Error>> {
        let data = json!({
            "symbols": symbols
        });

        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.access_token)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = client
            .post(&self.symbols_token_api)
            .headers(headers)
            .json(&data) // Automatically sets Content-Type to application/json
            .send()
            .await?;

        //sample response
        //{'s': 'ok', 'code': 1, 'validSymbol': {'NSE:NIFTY26JULFUT': '101126072861093'}, 'invalidSymbol': []}
        let response_data = response.json::<SymbolResponse>().await?;

        let index_dict = index_dict();
        let exch_seg_dict = exch_seg_dict();

        let mut data_dict = HashMap::new();
        let mut wrong_symbols = Vec::new();

        if "ok" == response_data.s.as_str() {
            for (symbol, fytoken) in response_data.valid_symbol.iter() {
                let segment = exch_seg_dict.get(&fytoken[..4]);
                if segment.is_none() {
                    continue;
                }
                let segment = segment.unwrap();
                let symbol_split: Vec<&str> = symbol.split('-').collect();
                let mut update_dict = true;
                let mut hsm_symbol = String::new();

                if symbol_split.len() > 1
                    && symbol_split.last() == Some(&"INDEX")
                    && self.data_type != DataType::DepthUpdate
                {
                    let exch_token = index_dict.get(symbol);
                    println!("exch_token {:#?}", exch_token);
                    if let Some(et) = exch_token {
                        hsm_symbol = format!("if|{}|{}", segment, et);
                    }
                } else if self.data_type == DataType::DepthUpdate
                    && symbol_split.last() != Some(&"INDEX")
                {
                    let exch_token = &fytoken[10..];
                    hsm_symbol = format!("dp|{}|{}", segment, exch_token);
                } else if self.data_type == DataType::SymbolUpdate {
                    let exch_token = &fytoken[10..];
                    hsm_symbol = format!("sf|{}|{}", segment, exch_token);
                } else if self.data_type == DataType::DepthUpdate
                    && symbol_split.last() == Some(&"INDEX")
                {
                    update_dict = false;
                }

                if update_dict && !hsm_symbol.is_empty() {
                    data_dict.insert(hsm_symbol, symbol.clone());
                }
            }

            if !response_data.invalid_symbol.is_empty() {
                wrong_symbols = response_data
                    .invalid_symbol
                    .iter()
                    .cloned()
                    .map(|v| v)
                    .collect();
            }
            println!("data dict {:#?}", data_dict);
            Ok((data_dict, wrong_symbols))
        } else {
            Err(format!(
                "API request failed with status: {} (code: {})",
                response_data.s, response_data.code
            )
            .into())
        }
    }
}

#[derive(Deserialize, Debug)]
struct SymbolResponse {
    s: String,
    code: i8,
    #[serde(rename = "invalidSymbol")]
    invalid_symbol: Vec<String>,
    #[serde(rename = "validSymbol")]
    valid_symbol: HashMap<String, String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum DataType {
    SymbolUpdate,
    DepthUpdate,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::SymbolUpdate => write!(f, "Symbolupdate"),
            DataType::DepthUpdate => write!(f, "DepthUpdate"),
        }
    }
}
