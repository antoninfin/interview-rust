use crate::models::Portfolio;
use poem::handler;
use poem::http::StatusCode;
use poem::web;
use poem::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub portfolio: Arc<Mutex<Portfolio>>,
}

#[derive(Deserialize)]
pub struct TradeRequest {
    symbol: String,
    quantity: f64,
    price: f64,
}

#[derive(Serialize)]
pub struct PortfolioResponse {
    name: String,
    cash_balance: f64,
    assets: Vec<AssetResponse>,
}

#[derive(Serialize)]
pub struct AssetResponse {
    symbol: String,
    quantity: f64,
    purchase_price: f64,
}

pub async fn get_portfolio() {}

pub async fn buy_asset() {}

pub async fn sell_asset() {}

pub(crate) fn build_app(app_state: AppState) -> impl poem::Endpoint {
    use poem::EndpointExt as _;
    use poem::{get, post, Route};

    Route::new()
        .at("/portfolio", get(get_portfolio))
        .at("/buy", post(buy_asset))
        .at("/sell", post(sell_asset))
        .data(app_state)
}
