use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::utils::serialization::HexOrDecimalU256;
use crate::utils::hex::Hex;
use web3::types::{H160, U256, H256};
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionData {
    pub auction_id: u32,
    pub transaction_hashes: Vec<H256>,
    pub auction_start_block: u32,
    pub competition_simulation_block: u32,
    pub auction: CompetitionAuctionData,
    pub solutions: Vec<CompetitionSolution>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct CompetitionAuctionData {
    #[serde_as(as = "Vec<Hex>")]
    pub orders: Vec<[u8; 56]>,
    #[serde_as(as = "HashMap<_, HexOrDecimalU256>")]
    pub prices: HashMap<H160, U256>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionSolution {
    pub solver: String,
    pub solver_address: H160,
    #[serde_as(as = "HexOrDecimalU256")]
    pub score: U256,
    pub ranking: u32,
    #[serde_as(as = "HashMap<_, HexOrDecimalU256>")]
    pub clearing_prices: HashMap<H160, U256>,
    pub orders: Vec<CompetitionOrderData>,
    pub is_winner: bool,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionOrderData {
    #[serde_as(as = "Hex")] 
    pub id: [u8; 56],
    #[serde_as(as = "HexOrDecimalU256")]
    pub sell_amount: U256,
    #[serde_as(as = "HexOrDecimalU256")]
    pub buy_amount: U256,
}
