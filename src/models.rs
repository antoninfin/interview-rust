use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub asset_type: AssetType,
    pub quantity: f64,
    pub current_price: f64,
    pub symbol: String,
    pub purchase_price: f64,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum AssetType {
    Stock,
    Crypto,
    Cash,
    RealEstate,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Portfolio {
    pub id: String,
    pub name: String,
    pub assets: HashMap<String, Asset>,
    pub cash_balance: f64,
}
