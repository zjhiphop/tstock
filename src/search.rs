use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
    pub quotation_code_table: QuotationCodeTable,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuotationCodeTable {
    pub data: Vec<Daum>,
    pub status: i64,
    pub message: String,
    pub total_count: i64,
    pub biz_code: String,
    pub biz_msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Daum {
    pub code: String,
    pub name: String,
    pub pin_yin: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "JYS")]
    pub jys: String,
    pub classify: String,
    pub market_type: String,
    pub security_type_name: String,
    pub security_type: String,
    pub mkt_num: String,
    #[serde(rename = "TypeUS")]
    pub type_us: String,
    #[serde(rename = "QuoteID")]
    pub quote_id: String,
    pub unified_code: String,
    pub inner_code: String,
}
