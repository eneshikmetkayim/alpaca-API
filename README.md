# Alpaca API Client

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://python.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://typescriptlang.org)
[![Go](https://img.shields.io/badge/Go-1.21+-00ADD8.svg)](https://golang.org)
[![.NET](https://img.shields.io/badge/.NET-8.0+-512BD4.svg)](https://dotnet.microsoft.com)
[![Java](https://img.shields.io/badge/Java-11+-ED8B00.svg)](https://openjdk.org)
[![Rust](https://img.shields.io/badge/Rust-2021+-DEA584.svg)](https://www.rust-lang.org/)

Alpaca Markets Trading ve Market Data API'leri için Python, TypeScript, Go, C#, Java ve Rust desteğine sahip çoklu dil client kütüphanesi.

## 🎯 Ne İşe Yarar?

Bu kütüphane ile Alpaca üzerinden programatik olarak trading yapabilirsiniz:

- **Trading**: Hisse senedi, crypto ve opsiyon alım/satım
- **Market Data**: Gerçek zamanlı ve geçmiş fiyat verileri
- **Streaming**: WebSocket ile canlı veri akışı
- **Paper Trading**: Gerçek para riski olmadan test

## 📊 Desteklenen Özellikler

| Özellik | Açıklama |
|---------|----------|
| **Trading API** | Account, Orders, Positions, Watchlists |
| **Market Data** | Stocks, Crypto, Options (Bars, Trades, Quotes) |
| **Streaming** | Real-time WebSocket (IEX, SIP, Crypto) |
| **Rust Client** | Async Rust 2021 client (`reqwest`, `serde`, `tokio`) |
| **Paper Trading** | Risk-free testing environment |

## 🚀 Hızlı Başlangıç

### Python

```bash
cd python
pip install -r requirements.txt
```

```python
import os
from dotenv import load_dotenv
from alpaca_client import AlpacaClient, TradingAPI, MarketDataAPI

load_dotenv()

client = AlpacaClient(
    api_key=os.environ["ALPACA_API_KEY"],
    api_secret=os.environ["ALPACA_API_SECRET"],
    paper=True
)

trading = TradingAPI(client)
market_data = MarketDataAPI(client)

# Hesap bilgisi
account = trading.get_account()
print(f"Buying Power: ${account.buying_power:,.2f}")

# Fiyat sorgula
price = market_data.get_current_price("AAPL")
print(f"AAPL: ${price}")

# Order ver
order = trading.buy("AAPL", qty=10)
```

### TypeScript

```bash
cd typescript
npm install
```

```typescript
import { AlpacaClient, TradingAPI, MarketDataAPI } from 'alpaca-api-client';

const client = new AlpacaClient({
  apiKey: process.env.ALPACA_API_KEY!,
  apiSecret: process.env.ALPACA_API_SECRET!,
  paper: true
});

const trading = new TradingAPI(client);
const marketData = new MarketDataAPI(client);

// Hesap bilgisi
const account = await trading.getAccount();
console.log(`Buying Power: $${account.buyingPower.toLocaleString()}`);

// Fiyat sorgula
const price = await marketData.getCurrentPrice('AAPL');
console.log(`AAPL: $${price}`);

// Order ver
const order = await trading.buy('AAPL', 10);
```

### Go

```bash
cd go
go mod download
```

```go
package main

import (
    "fmt"
    "os"
    "github.com/eneshenderson/alpaca-API/go/alpaca"
)

func main() {
    client := alpaca.NewClient(alpaca.ClientOptions{
        APIKey:    os.Getenv("ALPACA_API_KEY"),
        APISecret: os.Getenv("ALPACA_API_SECRET"),
        Paper:     true,
    })

    trading := alpaca.NewTradingAPI(client)
    marketData := alpaca.NewMarketDataAPI(client)

    // Hesap bilgisi
    account, _ := trading.GetAccount()
    fmt.Printf("Buying Power: $%s\n", account.BuyingPower)

    // Fiyat sorgula
    price, _ := marketData.GetCurrentPrice("AAPL")
    fmt.Printf("AAPL: $%.2f\n", price)

    // Order ver
    order, _ := trading.Buy("AAPL", 10)
}
```

### C# (.NET)

```bash
cd csharp
dotnet build
```

```csharp
using Alpaca.Api;

var client = new AlpacaClient(new AlpacaClientOptions
{
    ApiKey = Environment.GetEnvironmentVariable("ALPACA_API_KEY")!,
    ApiSecret = Environment.GetEnvironmentVariable("ALPACA_API_SECRET")!,
    Paper = true
});

var trading = new TradingApi(client);
var marketData = new MarketDataApi(client);

// Hesap bilgisi
var account = await trading.GetAccountAsync();
Console.WriteLine($"Buying Power: ${account.BuyingPower:N2}");

// Fiyat sorgula
var price = await marketData.GetCurrentPriceAsync("AAPL");
Console.WriteLine($"AAPL: ${price}");

// Order ver
var order = await trading.BuyAsync("AAPL", 10);
```

### Java

```bash
cd java
gradle run
```

```java
import com.alpaca.client.*;
import com.alpaca.client.models.*;

// Initialize client
AlpacaClient client = new AlpacaClient(apiKey, apiSecret, true);
TradingApi trading = new TradingApi(client);
MarketDataApi marketData = new MarketDataApi(client);

// Hesap bilgisi
Account account = trading.getAccount();
System.out.println("Buying Power: $" + account.getBuyingPower());

// Fiyat sorgula
double price = marketData.getCurrentPrice("AAPL");
System.out.printf("AAPL: $%.2f%n", price);

// Order ver
Order order = trading.submitOrder(OrderRequest.market("AAPL", "10", "buy"));
```

### Rust

```bash
cd rust
cargo check
cargo test
```

Rust istemcisi şu anda stock snapshot/current price ve crypto symbol metadata endpoint'lerini async olarak destekler:

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


## 📚 API Metodları

Tüm dillerde aynı metodlar mevcuttur:

### Trading API

| Metod | Açıklama |
|-------|----------|
| `getAccount()` | Hesap bilgilerini getirir |
| `buy(symbol, qty)` | Market buy order |
| `sell(symbol, qty)` | Market sell order |
| `buyLimit(symbol, qty, price)` | Limit buy order |
| `getOrders()` | Tüm order'ları listeler |
| `cancelOrder(id)` | Order iptal eder |
| `getPositions()` | Açık pozisyonları listeler |
| `closePosition(symbol)` | Pozisyon kapatır |
| `getClock()` | Market durumunu getirir |
| `isMarketOpen()` | Market açık mı kontrol eder |

### Market Data API

| Metod | Açıklama |
|-------|----------|
| `getStockBars(symbols, timeframe)` | Geçmiş OHLCV verileri |
| `getStockSnapshot(symbol)` | Anlık fiyat verisi |
| `getCurrentPrice(symbol)` | Güncel fiyat |
| `getCryptoBars(symbols, timeframe)` | Crypto OHLCV verileri |
| `getOptionsContracts(...)` | Opsiyon kontratları |

### Streaming API (Python & TypeScript)

> WebSocket streaming şu anda Python ve TypeScript istemcilerinde kullanılabilir. Rust streaming desteği sonraki kapsam için ayrılmıştır.

| Metod | Açıklama |
|-------|----------|
| `subscribe(trades, quotes, bars)` | Veri akışına abone ol |
| `unsubscribe(...)` | Aboneliği iptal et |
| `onTrade` | Trade event handler |
| `onQuote` | Quote event handler |
| `onBar` | Bar event handler |

## 🏗️ Proje Yapısı

```
alpaca-API/
├── python/              # Python 3.8+
│   ├── alpaca_client/   # Ana modül
│   ├── tests/           # Test dosyaları
│   └── requirements.txt
├── typescript/          # TypeScript 5.0+
│   ├── src/             # Kaynak kodlar
│   └── package.json
├── go/                  # Go 1.21+
│   ├── alpaca/          # Ana paket
│   └── go.mod
├── csharp/              # .NET 8.0+
│   └── *.cs
├── java/                # Java 11+
│   ├── src/             # Kaynak kodlar
│   └── build.gradle
├── rust/                # Rust 2021+
│   ├── src/             # Async client ve modeller
│   └── Cargo.toml
├── .env.example         # Credential template
├── LICENSE              # MIT Lisansı
└── README.md
```

## ⚙️ Yapılandırma

### Credential Ayarları

1. `.env.example` dosyasını `.env` olarak kopyalayın
2. [Alpaca Dashboard](https://app.alpaca.markets)'dan API key'lerinizi alın
3. `.env` dosyasına key'lerinizi girin

```env
ALPACA_API_KEY=your_api_key_here
ALPACA_API_SECRET=your_api_secret_here
ALPACA_PAPER=true
```

### Client Parametreleri

| Parametre | Varsayılan | Açıklama |
|-----------|------------|----------|
| `paper` | `true` | Paper trading modu |
| `timeout` | `30s` | Request timeout |

## 🔗 Linkler

- [Alpaca Documentation](https://docs.alpaca.markets)
- [Alpaca Dashboard](https://app.alpaca.markets)
- [API Status](https://status.alpaca.markets)

## 🤝 Katkıda Bulunma

1. Fork edin
2. Feature branch oluşturun (`git checkout -b feature/amazing-feature`)
3. Commit edin (`git commit -m 'Add amazing feature'`)
4. Push edin (`git push origin feature/amazing-feature`)
5. Pull Request açın

## 📄 Lisans

Bu proje MIT lisansı altında lisanslanmıştır. Detaylar için [LICENSE](LICENSE) dosyasına bakın.

## 👤 Geliştirici

**Enes Hikmet Kayım**
- GitHub: [@eneshenderson](https://github.com/eneshenderson)

## ⚠️ Sorumluluk Reddi

Bu kütüphane resmi Alpaca API'si değildir. Yatırım kararlarınızda bu verileri kullanmadan önce kendi araştırmanızı yapın. Paper trading ile test etmeniz önerilir.
