# interview-senior-backend-rust

# Financial Portfolio Manager

A simple REST API for managing financial assets portfolio, built with Rust and Poem / Tokio.

## Features

- Asset management (stocks, crypto, real estate)
- Cash balance tracking
- Buy and sell transactions
- REST API for portfolio interaction

## Prerequisites

- Rust (2021 edition or higher)
- Cargo

## Installation

```bash
git clone <repository-url>
cd financial-portfolio
cargo build
```

## Getting Started

```bash
cargo run
```

Server starts at `http://localhost:8080`

## API Endpoints

### Get Portfolio
```bash
GET /portfolio
```

### Buy Asset
```bash
POST /buy
Content-Type: application/json

{
    "symbol": "AAPL",
    "quantity": 10.0,
    "price": 150.0
}
```

### Sell Asset
```bash
POST /sell
Content-Type: application/json

{
    "symbol": "AAPL",
    "quantity": 5.0,
    "price": 160.0
}
```

## Project Structure

- `src/models.rs`: Data structures definitions
- `src/portfolio.rs`: Portfolio business logic
- `src/api.rs`: REST API endpoints
- `src/main.rs`: Application entry point

## Tests

```bash
cargo test
```

## Built With

- Rust
- Actix-web (Web framework)
- Serde (Serialization/Deserialization)
