use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FyersMap {
    pub data_val: Vec<String>,
    pub index_val: Vec<String>,
    pub lite_val: Vec<String>,
    pub depthvalue: Vec<String>,
    pub index_dict: HashMap<String, String>,
    pub exch_seg_dict: HashMap<String, String>,
    pub position_mapper: HashMap<String, String>,
    pub order_mapper: HashMap<String, String>,
    pub trade_mapper: HashMap<String, String>,
}

impl FyersMap {
    /// Loads the Fyers mapping configuration from a JSON string.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Loads the Fyers mapping configuration directly from a file path.
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(file_path)?;
        let map: FyersMap = serde_json::from_str(&file_content)?;
        Ok(map)
    }
}
