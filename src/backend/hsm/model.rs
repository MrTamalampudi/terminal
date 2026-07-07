use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DataValue {
    pub symbol: String,
    pub ltp: f64,
    pub prev_close_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub vol_traded_today: i64,
    pub last_traded_time: i64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub ask_price: f64,
    pub bid_price: f64,
    pub last_traded_qty: i64,
    pub tot_buy_qty: i64,
    pub tot_sell_qty: i64,
    pub avg_trade_price: f64,
    pub r#type: String, // 'type' is a reserved keyword in Rust, so we use r#type

    // Additional fields from your list not explicitly typed in the image
    pub oi: f64,    // Mapping "OI" to lowercase
    pub yhigh: f64, // Mapping "Yhigh" to lowercase
    pub ylow: f64,  // Mapping "Ylow" to lowercase
    pub lower_ckt: f64,
    pub upper_ckt: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IndexValue {
    pub ltp: f64,
    pub prev_close_price: f64,
    pub exch_feed_time: i64, // Based on context from previous requests
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LiteValue {
    pub ltp: f64,
    pub symbol: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

pub fn index_dict() -> HashMap<String, String> {
    HashMap::from([
        (
            "NSE:NIFTYINDIAMFG-INDEX".to_string(),
            "NIFTY INDIA MFG".to_string(),
        ),
        (
            "NSE:NIFTY100ESG-INDEX".to_string(),
            "NIFTY100 ESG".to_string(),
        ),
        (
            "NSE:NIFTYINDDIGITAL-INDEX".to_string(),
            "NIFTY IND DIGITAL".to_string(),
        ),
        (
            "NSE:NIFTYMICROCAP250-INDEX".to_string(),
            "NIFTY MICROCAP250".to_string(),
        ),
        (
            "NSE:NIFTYCONSRDURBL-INDEX".to_string(),
            "NIFTY CONSR DURBL".to_string(),
        ),
        (
            "NSE:NIFTYHEALTHCARE-INDEX".to_string(),
            "NIFTY HEALTHCARE".to_string(),
        ),
        (
            "NSE:NIFTYOILANDGAS-INDEX".to_string(),
            "NIFTY OIL AND GAS".to_string(),
        ),
        (
            "NSE:NIFTY100ESGSECLDR-INDEX".to_string(),
            "Nifty100ESGSecLdr".to_string(),
        ),
        (
            "NSE:NIFTY200MOMENTM30-INDEX".to_string(),
            "Nifty200Momentm30".to_string(),
        ),
        (
            "NSE:NIFTYALPHALOWVOL-INDEX".to_string(),
            "NIFTY AlphaLowVol".to_string(),
        ),
        (
            "NSE:NIFTY200QUALTY30-INDEX".to_string(),
            "NIFTY200 QUALTY30".to_string(),
        ),
        (
            "NSE:NIFTYSMLCAP50-INDEX".to_string(),
            "NIFTY SMLCAP 50".to_string(),
        ),
        (
            "NSE:MIDCPNIFTY-INDEX".to_string(),
            "NIFTY MID SELECT".to_string(),
        ),
        (
            "NSE:NIFTYMIDSELECT-INDEX".to_string(),
            "NIFTY MID SELECT".to_string(),
        ),
        (
            "NSE:NIFTYMIDCAP150-INDEX".to_string(),
            "NIFTY MIDCAP 150".to_string(),
        ),
        (
            "NSE:NIFTY100 EQL WGT-INDEX".to_string(),
            "NIFTY100 EQL Wgt".to_string(),
        ),
        (
            "NSE:NIFTY50 EQL WGT-INDEX".to_string(),
            "NIFTY50 EQL Wgt".to_string(),
        ),
        (
            "NSE:NIFTYGSCOMPSITE-INDEX".to_string(),
            "Nifty GS Compsite".to_string(),
        ),
        (
            "NSE:NIFTYGS1115YR-INDEX".to_string(),
            "Nifty GS 11 15Yr".to_string(),
        ),
        (
            "NSE:NIFTYGS48YR-INDEX".to_string(),
            "Nifty GS 4 8Yr".to_string(),
        ),
        (
            "NSE:NIFTYGS10YRCLN-INDEX".to_string(),
            "Nifty GS 10Yr Cln".to_string(),
        ),
        (
            "NSE:NIFTYGS813YR-INDEX".to_string(),
            "Nifty GS 8 13Yr".to_string(),
        ),
        (
            "NSE:NIFTYSMLCAP100-INDEX".to_string(),
            "NIFTY SMLCAP 100".to_string(),
        ),
        (
            "NSE:NIFTYQUALITY30-INDEX".to_string(),
            "NIFTY100 Qualty30".to_string(),
        ),
        (
            "NSE:NIFTYPVTBANK-INDEX".to_string(),
            "Nifty Pvt Bank".to_string(),
        ),
        (
            "NSE:NIFTYPHARMA-INDEX".to_string(),
            "Nifty Pharma".to_string(),
        ),
        (
            "NSE:NIFTYLARGEMID250-INDEX".to_string(),
            "NIFTY LARGEMID250".to_string(),
        ),
        (
            "NSE:NIFTYGS15YRPLUS-INDEX".to_string(),
            "Nifty GS 15YrPlus".to_string(),
        ),
        (
            "NSE:NIFTYPSUBANK-INDEX".to_string(),
            "Nifty PSU Bank".to_string(),
        ),
        (
            "NSE:NIFTYSMLCAP250-INDEX".to_string(),
            "NIFTY SMLCAP 250".to_string(),
        ),
        (
            "NSE:NIFTYENERGY-INDEX".to_string(),
            "Nifty Energy".to_string(),
        ),
        (
            "NSE:NIFTYALPHA50-INDEX".to_string(),
            "NIFTY Alpha 50".to_string(),
        ),
        ("NSE:NIFTYPSE-INDEX".to_string(), "Nifty PSE".to_string()),
        (
            "NSE:NIFTYFINSRV2550-INDEX".to_string(),
            "Nifty FinSrv25 50".to_string(),
        ),
        (
            "NSE:FINNIFTY-INDEX".to_string(),
            "Nifty Fin Service".to_string(),
        ),
        (
            "NSE:NIFTYREALTY-INDEX".to_string(),
            "Nifty Realty".to_string(),
        ),
        ("NSE:NIFTY500-INDEX".to_string(), "Nifty 500".to_string()),
        (
            "NSE:NIFTY500MULTICAP-INDEX".to_string(),
            "NIFTY500 MULTICAP".to_string(),
        ),
        (
            "NSE:NIFTYMIDCAP50-INDEX".to_string(),
            "Nifty Midcap 50".to_string(),
        ),
        (
            "NSE:NIFTYTOTALMKT-INDEX".to_string(),
            "NIFTY TOTAL MKT".to_string(),
        ),
        (
            "NSE:NIFTY50PR2XLEV-INDEX".to_string(),
            "Nifty50 PR 2x Lev".to_string(),
        ),
        ("NSE:INDIAVIX-INDEX".to_string(), "India VIX".to_string()),
        (
            "NSE:NIFTYDIVOPPS50-INDEX".to_string(),
            "Nifty Div Opps 50".to_string(),
        ),
        ("NSE:NIFTYMNC-INDEX".to_string(), "Nifty MNC".to_string()),
        (
            "NSE:NIFTY50VALUE20-INDEX".to_string(),
            "Nifty50 Value 20".to_string(),
        ),
        ("NSE:NIFTY50-INDEX".to_string(), "Nifty 50".to_string()),
        (
            "NSE:HANGSENG BEES-NAV-INDEX".to_string(),
            "HangSeng BeES-NAV".to_string(),
        ),
        (
            "NSE:NIFTY100LIQ15-INDEX".to_string(),
            "Nifty100 Liq 15".to_string(),
        ),
        (
            "NSE:NIFTY50TR2XLEV-INDEX".to_string(),
            "Nifty50 TR 2x Lev".to_string(),
        ),
        ("NSE:NIFTY100-INDEX".to_string(), "Nifty 100".to_string()),
        (
            "NSE:NIFTY100 LOWVOL30-INDEX".to_string(),
            "NIFTY100 LowVol30".to_string(),
        ),
        ("NSE:NIFTYBANK-INDEX".to_string(), "Nifty Bank".to_string()),
        ("NSE:NIFTYFMCG-INDEX".to_string(), "Nifty FMCG".to_string()),
        ("NSE:NIFTYIT-INDEX".to_string(), "Nifty IT".to_string()),
        (
            "NSE:NIFTYGS10YR-INDEX".to_string(),
            "Nifty GS 10Yr".to_string(),
        ),
        (
            "NSE:NIFTYMIDCAP100-INDEX".to_string(),
            "NIFTY MIDCAP 100".to_string(),
        ),
        (
            "NSE:NIFTYNEXT50-INDEX".to_string(),
            "Nifty Next 50".to_string(),
        ),
        (
            "NSE:NIFTYNXT50-INDEX".to_string(),
            "Nifty Next 50".to_string(),
        ),
        (
            "NSE:NIFTYM150QLTY50-INDEX".to_string(),
            "NIFTY M150 QLTY50".to_string(),
        ),
        (
            "NSE:NIFTYSERVSECTOR-INDEX".to_string(),
            "Nifty Serv Sector".to_string(),
        ),
        (
            "NSE:NIFTYMIDSML400-INDEX".to_string(),
            "NIFTY MIDSML 400".to_string(),
        ),
        ("NSE:NIFTYAUTO-INDEX".to_string(), "Nifty Auto".to_string()),
        (
            "NSE:NIFTYMETAL-INDEX".to_string(),
            "Nifty Metal".to_string(),
        ),
        (
            "NSE:NIFTYINFRA-INDEX".to_string(),
            "Nifty Infra".to_string(),
        ),
        (
            "NSE:NIFTYMEDIA-INDEX".to_string(),
            "Nifty Media".to_string(),
        ),
        (
            "NSE:NIFTY50PR1XINV-INDEX".to_string(),
            "Nifty50 PR 1x Inv".to_string(),
        ),
        ("NSE:NIFTY200-INDEX".to_string(), "Nifty 200".to_string()),
        (
            "NSE:NIFTY50TR1XINV-INDEX".to_string(),
            "Nifty50 TR 1x Inv".to_string(),
        ),
        ("NSE:NIFTYCPSE-INDEX".to_string(), "Nifty CPSE".to_string()),
        (
            "NSE:NIFTYMIDLIQ15-INDEX".to_string(),
            "Nifty Mid Liq 15".to_string(),
        ),
        (
            "NSE:NIFTYCOMMODITIES-INDEX".to_string(),
            "Nifty Commodities".to_string(),
        ),
        (
            "NSE:NIFTYCONSUMPTION-INDEX".to_string(),
            "Nifty Consumption".to_string(),
        ),
        (
            "NSE:NIFTY50DIVPOINT-INDEX".to_string(),
            "Nifty50 Div Point".to_string(),
        ),
        (
            "NSE:NIFTYGROWSECT15-INDEX".to_string(),
            "Nifty GrowSect 15".to_string(),
        ),
        ("BSE:100LARGECAPTMC-INDEX".to_string(), "LCTMCI".to_string()),
        ("BSE:DFRG-INDEX".to_string(), "DFRGRI".to_string()),
        ("BSE:QUALITY-INDEX".to_string(), "BSEQUI".to_string()),
        (
            "BSE:DIVIDENDSTABILITY-INDEX".to_string(),
            "BSEDSI".to_string(),
        ),
        ("BSE:250SMALLCAP-INDEX".to_string(), "SML250".to_string()),
        ("BSE:150MIDCAP-INDEX".to_string(), "MID150".to_string()),
        ("BSE:ESG100-INDEX".to_string(), "ESG100".to_string()),
        ("BSE:SNXT50-INDEX".to_string(), "SNXT50".to_string()),
        ("BSE:SNSX50-INDEX".to_string(), "SNSX50".to_string()),
        ("BSE:UTILS-INDEX".to_string(), "UTILS".to_string()),
        ("BSE:GREENEX-INDEX".to_string(), "GREENX".to_string()),
        ("BSE:SENSEX-INDEX".to_string(), "SENSEX".to_string()),
        ("BSE:REALTY-INDEX".to_string(), "REALTY".to_string()),
        ("BSE:PRIVATEBANKS-INDEX".to_string(), "BSEPBI".to_string()),
        ("BSE:CDGS-INDEX".to_string(), "CDGS".to_string()),
        ("BSE:OILGAS-INDEX".to_string(), "OILGAS".to_string()),
        ("BSE:ENERGY-INDEX".to_string(), "ENERGY".to_string()),
        ("BSE:POWER-INDEX".to_string(), "POWER".to_string()),
        ("BSE:500-INDEX".to_string(), "BSE500".to_string()),
        ("BSE:100-INDEX".to_string(), "BSE100".to_string()),
        ("BSE:PSU-INDEX".to_string(), "BSEPSU".to_string()),
        ("BSE:HC-INDEX".to_string(), "BSE HC".to_string()),
        ("BSE:400MIDSMALLCAP-INDEX".to_string(), "MSL400".to_string()),
        ("BSE:BHRT22-INDEX".to_string(), "BHRT22".to_string()),
        ("BSE:BANKEX-INDEX".to_string(), "BANKEX".to_string()),
        ("BSE:ALLCAP-INDEX".to_string(), "ALLCAP".to_string()),
        ("BSE:INFRA-INDEX".to_string(), "INFRA".to_string()),
        ("BSE:CD-INDEX".to_string(), "BSE CD".to_string()),
        ("BSE:MIDCAP-INDEX".to_string(), "MIDCAP".to_string()),
        ("BSE:AUTO-INDEX".to_string(), "AUTO".to_string()),
        ("BSE:BASMTR-INDEX".to_string(), "BASMTR".to_string()),
        ("BSE:200-INDEX".to_string(), "BSE200".to_string()),
        ("BSE:FIN-INDEX".to_string(), "FIN".to_string()),
        ("BSE:CG-INDEX".to_string(), "BSE CG".to_string()),
        ("BSE:ENHANCEDVALUE-INDEX".to_string(), "BSEEVI".to_string()),
        ("BSE:TECK-INDEX".to_string(), "TECK".to_string()),
        ("BSE:METAL-INDEX".to_string(), "METAL".to_string()),
        ("BSE:CARBONEX-INDEX".to_string(), "CARBON".to_string()),
        ("BSE:MIDSEL-INDEX".to_string(), "MIDSEL".to_string()),
        ("BSE:SME IPO-INDEX".to_string(), "SMEIPO".to_string()),
        ("BSE:MOMENTUM-INDEX".to_string(), "BSEMOI".to_string()),
        ("BSE:TELCOM-INDEX".to_string(), "TELCOM".to_string()),
        ("BSE:CPSE-INDEX".to_string(), "CPSE".to_string()),
        ("BSE:250LARGEMIDCAP-INDEX".to_string(), "LMI250".to_string()),
        ("BSE:SMLCAP-INDEX".to_string(), "SMLCAP".to_string()),
        ("BSE:IT-INDEX".to_string(), "BSE IT".to_string()),
        (
            "BSE:INDIAMANUFACTURING-INDEX".to_string(),
            "MFG".to_string(),
        ),
        ("BSE:INDSTR-INDEX".to_string(), "INDSTR".to_string()),
        ("BSE:LOWVOLATILITY-INDEX".to_string(), "BSELVI".to_string()),
        ("BSE:LRGCAP-INDEX".to_string(), "LRGCAP".to_string()),
        ("BSE:IPO-INDEX".to_string(), "BSEIPO".to_string()),
        ("BSE:FMC-INDEX".to_string(), "BSEFMC".to_string()),
        ("BSE:SMLSEL-INDEX".to_string(), "SMLSEL".to_string()),
    ])
}

pub fn exch_seg_dict() -> HashMap<String, String> {
    HashMap::from([
        ("1010".to_string(), "nse_cm".to_string()),
        ("1011".to_string(), "nse_fo".to_string()),
        ("1120".to_string(), "mcx_fo".to_string()),
        ("1210".to_string(), "bse_cm".to_string()),
        ("1012".to_string(), "cde_fo".to_string()),
        ("1211".to_string(), "bse_fo".to_string()),
        ("1212".to_string(), "bcs_fo".to_string()),
        ("1020".to_string(), "nse_com".to_string()),
    ])
}
