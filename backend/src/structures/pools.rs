use serde::{Deserialize, Deserializer};
use web3::types::{H160, U256};
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Pools {
    pub data: Vec<Pool>,
}

#[derive(Debug, Deserialize)]
pub struct Pool {
    // #[serde(deserialize_with = "deserialize_eth_id_to_h160")]
    pub id: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub base_token_price_usd: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub base_token_price_native_currency: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub quote_token_price_usd: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub quote_token_price_native_currency: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub base_token_price_quote_token: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub quote_token_price_base_token: BigDecimal,

    #[serde(deserialize_with = "deserialize_h160")]
    pub address: H160,

    pub name: String,
    pub pool_created_at: String, // keep as String or parse into a datetime type

    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub token_price_usd: BigDecimal,

    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub fdv_usd: BigDecimal,

    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub market_cap_usd: BigDecimal,

    pub price_change_percentage: PriceChangePercentage,
    pub transactions: Transactions,
    pub volume_usd: VolumeUSD,

    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub reserve_in_usd: BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct PriceChangePercentage {
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub m5: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub h1: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub h6: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub h24: BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct Transactions {
    pub m5: TxInterval,
    pub m15: TxInterval,
    pub m30: TxInterval,
    pub h1: TxInterval,
    pub h24: TxInterval,
}

#[derive(Debug, Deserialize)]
pub struct TxInterval {
    #[serde(deserialize_with = "deserialize_u256_from_int")]
    pub buys: U256,
    #[serde(deserialize_with = "deserialize_u256_from_int")]
    pub sells: U256,
    #[serde(deserialize_with = "deserialize_u256_from_int")]
    pub buyers: U256,
    #[serde(deserialize_with = "deserialize_u256_from_int")]
    pub sellers: U256,
}

#[derive(Debug, Deserialize)]
pub struct VolumeUSD {
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub m5: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub h1: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub h6: BigDecimal,
    #[serde(deserialize_with = "deserialize_bigdecimal")]
    pub h24: BigDecimal,
}

#[derive(Debug, Deserialize)]
pub struct Relationships {
    pub base_token: Relationship<TokenRef>,
    pub quote_token: Relationship<TokenRef>,
    pub dex: Relationship<DexRef>,
}

#[derive(Debug, Deserialize)]
pub struct Relationship<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct TokenRef {
    // #[serde(deserialize_with = "deserialize_eth_id_to_h160")]
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DexRef {
    // Dex IDs are not normal eth addresses
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,
}

// ------------------- Custom Deserializers -------------------
fn deserialize_bigdecimal<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::Null => Ok(BigDecimal::from(0)),
        serde_json::Value::String(s) => {
            s.parse::<BigDecimal>().map_err(|e| serde::de::Error::custom(e.to_string()))
        }
        serde_json::Value::Number(n) => {
            let s = n.to_string();
            s.parse::<BigDecimal>()
                .map_err(|e| serde::de::Error::custom(e.to_string()))
        }
        _ => Err(serde::de::Error::custom("expected a numeric value (string or number)")),
    }
}

fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::Null => Ok(U256::from(0)),
        serde_json::Value::String(s) => U256::from_dec_str(&s)
            .map_err(|_| serde::de::Error::custom("Failed to parse U256 from string")),
        serde_json::Value::Number(n) => {
            let s = n.to_string();
            U256::from_dec_str(&s)
                .map_err(|_| serde::de::Error::custom("Failed to parse U256 from number"))
        }
        _ => Err(serde::de::Error::custom(
            "expected a numeric value (string or number)",
        )),
    }
}

fn deserialize_eth_id_to_h160<'de, D>(deserializer: D) -> Result<H160, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match Option::<String>::deserialize(deserializer)? {
        Some(s) => s,
        None => return Ok(H160::zero()),
    };

    if let Some(addr_str) = s.strip_prefix("eth_") {
        parse_h160(addr_str)
            .map_err(|e| serde::de::Error::custom(format!("Failed to parse H160: {}", e)))
    } else {
        Err(serde::de::Error::custom(
            "Expected 'eth_' prefix for H160 field",
        ))
    }
}

fn deserialize_h160<'de, D>(deserializer: D) -> Result<H160, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match Option::<String>::deserialize(deserializer)? {
        Some(s) => s,
        None => return Ok(H160::zero()),
    };

    parse_h160(&s)
        .map_err(|e| serde::de::Error::custom(format!("Failed to parse H160: {}", e)))
}

fn parse_h160(s: &str) -> Result<H160, String> {
    let hex_str = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(hex_str).map_err(|e| format!("Hex decode error: {}", e))?;
    if bytes.len() != 20 {
        return Err(format!(
            "Invalid H160 length: expected 20 bytes, got {}",
            bytes.len()
        ));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(H160(arr))
}

fn deserialize_u256_from_int<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::Null => Ok(U256::from(0)),
        serde_json::Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                Ok(U256::from(u))
            } else {
                // If we encounter a floating number or something non-integer-like,
                // we could fall back to a default:
                Ok(U256::from(0))
            }
        }
        serde_json::Value::String(s) => {
            U256::from_dec_str(&s)
                .map_err(|_| serde::de::Error::custom("Failed to parse U256 from string"))
        }
        _ => Ok(U256::from(0)),
    }
}
