use crate::models::{Asset, AssetType, Portfolio};
use std::collections::HashMap;
use uuid::Uuid;

impl Portfolio {
    pub fn new(name: String, initial_balance: f64) -> Self {
        Portfolio {
            id: Uuid::new_v4().to_string(),
            name,
            assets: HashMap::new(),
            cash_balance: initial_balance,
        }
    }

    pub fn buy_asset(&mut self, symbol: String, quantity: f64, price: f64) -> Result<(), String> {
        todo!()
    }

    pub fn sell_asset(&mut self, symbol: &str, quantity: f64, price: f64) -> Result<(), String> {
        todo!()
    }
}
