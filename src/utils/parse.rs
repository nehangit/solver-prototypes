use serde::de::DeserializeOwned;
use std::fs;
use serde_json;

// usage: let obj: T = json_to_struct(file_name);
pub fn json_to_struct<T>(file: &str) -> T
where T: DeserializeOwned,
{
    let json_data = fs::read_to_string(file).expect("Unable to read file");
    let parsed_struct: T = serde_json::from_str(&json_data).expect("JSON was not well-formatted");
    parsed_struct
}