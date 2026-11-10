from unittest.mock import Mock

import pytest

from alpaca_client import AlpacaClient, MarketDataAPI
from alpaca_client.exceptions import APIError


def make_market_data(response):
    client = Mock(spec=AlpacaClient)
    market_data = MarketDataAPI(client)
    market_data._crypto_request = Mock(return_value=response)
    return market_data


def test_crypto_snapshots_accept_direct_symbol_keyed_response():
    market_data = make_market_data(
        {
            "BTC/USD": {
                "latestTrade": {
                    "t": "2026-01-01T00:00:00Z",
                    "p": "100.25",
                    "s": "0.1",
                }
            }
        }
    )

    snapshots = market_data.get_crypto_snapshots("BTC/USD")

    assert snapshots["BTC/USD"].latest_trade.price == 100.25


def test_crypto_snapshots_accept_legacy_wrapped_response():
    market_data = make_market_data(
        {
            "snapshots": {
                "ETH/USD": {
                    "latestTrade": {
                        "t": "2026-01-01T00:00:00Z",
                        "p": "200.50",
                        "s": "0.2",
                    }
                }
            }
        }
    )

    snapshots = market_data.get_crypto_snapshots("ETH/USD")

    assert snapshots["ETH/USD"].latest_trade.price == 200.50


def test_crypto_snapshot_normalizes_symbol_case():
    market_data = make_market_data(
        {
            "BTC/USD": {
                "latestTrade": {
                    "t": "2026-01-01T00:00:00Z",
                    "p": "100.25",
                    "s": "0.1",
                }
            }
        }
    )

    snapshot = market_data.get_crypto_snapshot("btc/usd")

    assert snapshot.symbol == "BTC/USD"
    market_data._crypto_request.assert_called_once_with(
        "GET", "/snapshots", params={"symbols": "BTC/USD"}
    )


@pytest.mark.parametrize(
    "response",
    [
        {"symbols": ["BTC/USD", "ETH/USD"]},
        {"symbols": [{"symbol": "BTC/USD"}, {"symbol": "ETH/USD"}]},
        ["BTC/USD", "ETH/USD"],
    ],
)
def test_crypto_symbols_accepts_supported_response_shapes(response):
    market_data = make_market_data(response)

    assert market_data.get_crypto_symbols() == ["BTC/USD", "ETH/USD"]
    market_data._crypto_request.assert_called_once_with(
        "GET", "/meta/symbols"
    )


def test_crypto_symbols_rejects_unexpected_response_shape():
    market_data = make_market_data({"unexpected": "value"})

    with pytest.raises(APIError, match="Unexpected crypto symbols response"):
        market_data.get_crypto_symbols()
