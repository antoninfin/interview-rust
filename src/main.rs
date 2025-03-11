mod api;
mod models;
mod portfolio;

use api::{build_app, AppState};
use models::Portfolio;
use poem::{listener::TcpListener, Server};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let portfolio = Portfolio::new("My Portfolio".to_string(), 10000.0);
    let app_state = AppState {
        portfolio: Arc::new(Mutex::new(portfolio)),
    };

    let app = build_app(app_state);

    println!("Server running at http://localhost:8080");

    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await?;

    Ok(())
}
