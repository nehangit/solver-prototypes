use std::fs;
use solvers_dto::auction::Auction;

pub fn json_to_auction(file: &str) -> Auction {
    let json_data = fs::read_to_string(&file).expect("Unable to read file");
    let auction: Auction = serde_json::from_str(&json_data).expect("JSON was not well-formatted");
    auction
}