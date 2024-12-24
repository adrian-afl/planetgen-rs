use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct InputData {
    name: String,
    age: u8,
    phones: Vec<String>,
}