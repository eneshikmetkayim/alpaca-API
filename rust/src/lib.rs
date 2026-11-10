use reqwest::{Client as HttpClient, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

const TRADING_LIVE: &str = "https://api.alpaca.markets";
const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";
const MARKET_DATA: &str = "https://data.alpaca.markets";

#[derive(Debug, Error)]
pub enum AlpacaError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Alpaca API returned HTTP {status}: {message}")]
    Api { status: StatusCode, message: String },
    #[error("unexpected response: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct AlpacaClient {
    http: HttpClient,
    trading_url: String,
    market_data_url: String,
}

impl AlpacaClient {
    pub fn new(api_key: impl Into<String>, api_secret: impl Into<String>, paper: bool) -> Self {
        let api_key = api_key.into();
        let api_secret = api_secret.into();
        let http = HttpClient::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "APCA-API-KEY-ID",
                    reqwest::header::HeaderValue::from_str(&api_key)
                        .expect("API key must be a valid header value"),
                );
                headers.insert(
                    "APCA-API-SECRET-KEY",
                    reqwest::header::HeaderValue::from_str(&api_secret)
                        .expect("API secret must be a valid header value"),
                );
                headers
            })
            .build()
            .expect("building the HTTP client should not fail");

        Self {
            http,
            trading_url: if paper {
                TRADING_PAPER.to_string()
            } else {
                TRADING_LIVE.to_string()
            },
            market_data_url: MARKET_DATA.to_string(),
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        market_data: bool,
    ) -> Result<T, AlpacaError> {
        let base_url = if market_data {
            &self.market_data_url
        } else {
            &self.trading_url
        };
        let response = self
            .http
            .get(format!("{base_url}{endpoint}"))
            .send()
            .await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            let message = serde_json::from_str::<Value>(&body)
                .ok()
                .and_then(|value| {
                    value
                        .get("message")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })
                .unwrap_or(body);
            return Err(AlpacaError::Api { status, message });
        }

        Ok(serde_json::from_str(&body)?)
    }

    pub fn market_data(&self) -> MarketDataApi {
        MarketDataApi {
            client: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct MarketDataApi {
    client: AlpacaClient,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<Trade>,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: f64,
}

impl MarketDataApi {
    pub async fn get_stock_snapshot(&self, symbol: &str) -> Result<Value, AlpacaError> {
        self.client
            .get(
                &format!("/v2/stocks/{}/snapshot", symbol.to_uppercase()),
                true,
            )
            .await
    }

    pub async fn get_current_price(&self, symbol: &str) -> Result<f64, AlpacaError> {
        let snapshot: Snapshot = self
            .client
            .get(
                &format!("/v2/stocks/{}/snapshot", symbol.to_uppercase()),
                true,
            )
            .await?;
        snapshot
            .latest_trade
            .map(|trade| trade.price)
            .ok_or_else(|| AlpacaError::Api {
                status: StatusCode::NO_CONTENT,
                message: format!("no trade data for {symbol}"),
            })
    }

    pub async fn get_crypto_symbols(&self) -> Result<Vec<String>, AlpacaError> {
        let response: Value = self
            .client
            .get("/v1beta3/crypto/us/meta/symbols", true)
            .await?;
        let symbols = response.get("symbols").unwrap_or(&response);
        let symbols = symbols.as_array().ok_or_else(|| AlpacaError::Api {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: "unexpected crypto symbols response".to_string(),
        })?;

        symbols
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_owned)
                    .or_else(|| {
                        item.get("symbol")
                            .and_then(Value::as_str)
                            .map(str::to_owned)
                    })
                    .ok_or_else(|| AlpacaError::Api {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        message: "crypto symbol entry is missing symbol".to_string(),
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_use_alpaca_api_hosts() {
        assert!(TRADING_PAPER.starts_with("https://paper-api."));
        assert!(MARKET_DATA.starts_with("https://data."));
    }

    #[test]
    fn crypto_symbol_payload_supports_wrapped_shape() {
        let value = serde_json::json!({"symbols": [{"symbol": "BTC/USD"}]});
        let symbols = value.get("symbols").unwrap().as_array().unwrap();
        assert_eq!(symbols[0]["symbol"], "BTC/USD");
    }
}
