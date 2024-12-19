use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::utils::serialization::HexOrDecimalU256;
use web3::types::{H160, U256};
use bigdecimal::BigDecimal;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct Pools {
    pub data: Vec<Pool>,
}

#[derive(Debug, Deserialize)]
pub struct Pool {
    pub id: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Attributes {
    #[serde_as(as = "DisplayFromStr")]
    pub base_token_price_usd: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub base_token_price_native_currency: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_token_price_usd: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_token_price_native_currency: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub base_token_price_quote_token: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_token_price_base_token: BigDecimal,
    pub address: H160,
    pub name: String,
    pub pool_created_at: chrono::DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    pub token_price_usd: BigDecimal,
    #[serde_as(as = "HexOrDecimalU256")]
    pub fdv_usd: U256,
    #[serde_as(as = "DisplayFromStr")]
    pub market_cap_usd: BigDecimal,
    pub price_change_percentage: PriceChangePercentage,
    pub transactions: Transactions,
    pub volume_usd: VolumeUsd,
    #[serde_as(as = "DisplayFromStr")]
    pub reserve_in_usd: BigDecimal,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct PriceChangePercentage {
    #[serde_as(as = "DisplayFromStr")]
    pub m5: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub h1: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub h6: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub h24: BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct Transactions {
    pub m5: TransactionDetail,
    pub m15: TransactionDetail,
    pub m30: TransactionDetail,
    pub h1: TransactionDetail,
    pub h24: TransactionDetail,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct TransactionDetail {
    #[serde_as(as = "DisplayFromStr")]
    pub buys: U256,
    #[serde_as(as = "DisplayFromStr")]
    pub sells: U256,
    #[serde_as(as = "DisplayFromStr")]
    pub buyers: U256,
    #[serde_as(as = "DisplayFromStr")]
    pub sellers: U256,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct VolumeUsd {
    #[serde_as(as = "DisplayFromStr")]
    pub m5: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub h1: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub h6: BigDecimal,
    #[serde_as(as = "DisplayFromStr")]
    pub h24: BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct Relationships {
    pub base_token: RelatedToken,
    pub quote_token: RelatedToken,
    pub dex: Dex,
}

#[derive(Debug, Deserialize)]
pub struct RelatedToken {
    pub data: TokenData,
}

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub id: String,
    #[serde(rename = "type")]
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Dex {
    pub data: DexData,
}

#[derive(Debug, Deserialize)]
pub struct DexData {
    pub id: String,
    #[serde(rename = "type")]
    pub dex_type: String,
}
