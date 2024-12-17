use serde::{Serialize, Deserialize};
use web3::types::{H160, U256};

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub amount: U256,
    pub token: H160,
}