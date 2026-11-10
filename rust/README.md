# Alpaca API Client for Rust

Async Rust support for Alpaca Trading and Market Data APIs.

```rust
use alpaca_api_client::AlpacaClient;

#[tokio::main]
async fn main() -> Result<(), alpaca_api_client::AlpacaError> {
    let client = AlpacaClient::new(
        std::env::var("ALPACA_API_KEY").unwrap(),
        std::env::var("ALPACA_API_SECRET").unwrap(),
        true,
    );

    let price = client.market_data().get_current_price("AAPL").await?;
    println!("AAPL: ${price:.2}");
    Ok(())
}
```

Run the local checks with:

```bash
cargo test
cargo check
```
