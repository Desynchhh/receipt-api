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


impl Receipt {
    pub fn from_receipt_entry(id:u64, receipt:ReceiptEntry) -> Receipt {
        Receipt {
            id,
            store: receipt.store,
            date: receipt.date,
            paid_by: receipt.paid_by,
            items: receipt.items,
            subtotal: receipt.subtotal,
            contributor_to_pay: receipt.contributor_to_pay,
        }
    }

    pub fn replace_values(&mut self, other:ReceiptEntry) {
        self.store = other.store;
        self.date = other.date;
        self.paid_by = other.paid_by;
        self.items = other.items;
        self.subtotal = other.subtotal;
        self.contributor_to_pay = other.contributor_to_pay;
    }
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