use crate::models::Portfolio;
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

    pub fn buy_asset(
        &mut self,
        _symbol: String,
        _quantity: f64,
        _price: f64,
    ) -> Result<(), String> {
        todo!()
    }

    pub fn sell_asset(&mut self, symbol: &str, quantity: f64, price: f64) -> Result<(), String> {
        if let Some(asset) = self.assets.get_mut(symbol) {
            if asset.quantity < quantity {
                return Err("Insufficient quantity".to_string());
            }

            asset.quantity -= quantity;
            self.cash_balance += quantity * price;

            if asset.quantity == 0.0 {
                self.assets.remove(symbol);
            }
            Ok(())
        } else {
            Err("Asset not found".to_string())
        }
    }
}
