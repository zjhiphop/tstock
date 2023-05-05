use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuoteResponse {
    #[serde(rename = "rc")]
    pub rc: i64,
    #[serde(rename = "rt")]
    pub rt: i64,
    #[serde(rename = "svr")]
    pub svr: i64,
    #[serde(rename = "lt")]
    pub lt: i64,
    #[serde(rename = "full")]
    pub full: i64,
    #[serde(rename = "dlmkts")]
    pub dlmkts: String,
    #[serde(rename = "data")]
    pub data: QuoteData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct QuoteData {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "market")]
    pub market: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "decimal")]
    pub decimal: i64,
    #[serde(rename = "dktotal")]
    pub dktotal: i64,
    #[serde(rename = "preKPrice")]
    pub pre_kprice: f64,
    #[serde(rename = "prePrice")]
    pub pre_price: f64,
    #[serde(rename = "qtMiscType")]
    pub qt_misc_type: i64,
    #[serde(rename = "klines")]
    pub klines: Vec<String>,
}
