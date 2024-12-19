use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use bigdecimal::BigDecimal;

#[derive(Deserialize, Debug)]
pub struct GasOracleResponse {
    pub status: String,
    pub message: String,
    pub result: GasPrices,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct GasPrices {
    #[serde_as(as = "DisplayFromStr")]
    pub SafeGasPrice: BigDecimal,   // Low gas price (in gwei)
    #[serde_as(as = "DisplayFromStr")]
    pub ProposeGasPrice: BigDecimal, // Average gas price (in gwei)
    #[serde_as(as = "DisplayFromStr")]
    pub FastGasPrice: BigDecimal,    // High gas price (in gwei)
}