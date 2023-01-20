use std::collections::HashMap;

use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub price: f32,
    pub discount: Option<f32>,
    pub contributors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub id: u64,
    pub store: String,
    pub date: String,
    pub paid_by: String,
    pub items: Vec<Item>,
    pub subtotal: f32,
    pub contributor_to_pay: HashMap<String, f32>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptIdentifier {
    pub id: u64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptEntry {
    pub store: String,
    pub date: String,
    pub paid_by: String,
    pub items: Vec<Item>,
    pub subtotal: f32,
    pub contributor_to_pay: HashMap<String, f32>
}

#[derive(Deserialize, Serialize)]
pub struct StoreIdentifier {
    pub store: String
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    error: String
}

impl ErrorResponse {
    pub fn new(message:&str) -> ErrorResponse {
        ErrorResponse { error: message.to_string() }
    }
}