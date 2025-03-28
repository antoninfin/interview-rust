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

#[handler]
pub async fn get_portfolio() {}

#[handler]
pub async fn buy_asset() {}

#[handler]
pub async fn sell_asset(
    app_state: web::Data<&AppState>,
    req: web::Json<TradeRequest>,
) -> Result<&'static str> {
    let mut portfolio = app_state.portfolio.lock().unwrap();

    match portfolio.sell_asset(&req.symbol, req.quantity, req.price) {
        Ok(_) => Ok("Sale successful"),
        Err(e) => Err(poem::Error::from_string(e, StatusCode::BAD_REQUEST)),
    }
}

pub(crate) fn build_app(app_state: AppState) -> impl poem::Endpoint {
    use poem::EndpointExt as _;
    use poem::{get, post, Route};

    Route::new()
        .at("/portfolio", get(get_portfolio))
        .at("/buy", post(buy_asset))
        .at("/sell", post(sell_asset))
        .data(app_state)
}
