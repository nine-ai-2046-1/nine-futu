# nine-futu-cli

A command-line interface for Futu OpenD API, designed for stock traders to quickly access market data and execute trades.

## Features

- **Real-time Market Data**: Get live quotes, order book, and ticker data
- **Historical K-line**: Access historical candlestick data with flexible date ranges
- **Subscription Management**: Subscribe/unsubscribe to real-time data streams
- **JSON & NDJSON Output**: Machine-readable output for automation and scripting
- **Multi-market Support**: Hong Kong, US, and more markets

## Quick Start

### Prerequisites

1. **FutuOpenD** must be running on your machine
   - Download from [FutuOpenD Official Site](https://openapi.futunn.com/futu-api-doc/quick/opend-base.html)
   - Default address: `127.0.0.1:11111`

2. **Install nine-futu-cli**
   ```bash
   cargo install nine-futu-cli
   ```

### Basic Usage

```bash
# Get stock snapshot
nine-futu-cli quote snapshot -c 700

# Get daily K-line (last 10 bars)
nine-futu-cli quote kline -c 700 -k 1d

# Get 5-minute K-line for today
nine-futu-cli quote kline -c 700 -k 5m

# Get K-line for specific date
nine-futu-cli quote kline -c 700 -k 5m -s "2026-05-28" -e "2026-05-28"

# JSON output
nine-futu-cli quote snapshot -c 700 --json

# NDJSON output (one JSON per line)
nine-futu-cli quote kline -c 700 -k 5m --ndjson
```

## Use Cases for Stock Traders

### 1. Check Stock Price Before Trading

```bash
# Quick check current price
nine-futu-cli quote snapshot -c 700

# Output:
# {"code":"HK.00700","name":"TENCENT","last_done":436.2,...}
```

### 2. Analyze Daily Trend

```bash
# Get last 30 days of daily K-line
nine-futu-cli quote kline -c 700 -k 1d --num 30

# Get K-line for specific period
nine-futu-cli quote kline -c 700 -k 1d -s "2026-04-01" -e "2026-04-30"
```

### 3. Intraday Trading Analysis

```bash
# Get 5-minute K-line for today
nine-futu-cli quote kline -c 700 -k 5m

# Get 1-minute K-line for specific time range
nine-futu-cli quote kline -c 700 -k 1m -s "2026-05-28 09:30" -e "2026-05-28 16:00"
```

### 4. Monitor Order Book Depth

```bash
# Subscribe to order book
nine-futu-cli subscribe add -c 700 -t ORDER_BOOK

# Query subscription status
nine-futu-cli subscribe list
```

### 5. Automated Data Collection

```bash
# Collect daily data for backtesting (NDJSON format)
nine-futu-cli quote kline -c 700 -k 1d -s "2026-01-01" -e "2026-12-31" --ndjson > data.jsonl
```

## Commands

### Quote Commands

| Command | Description | Example |
|---------|-------------|---------|
| `snapshot` | Get market snapshot | `quote snapshot -c 700` |
| `kline` | Get K-line data | `quote kline -c 700 -k 1d` |

### Subscription Commands

| Command | Description | Example |
|---------|-------------|---------|
| `subscribe list` | List all subscriptions | `subscribe list` |
| `subscribe add` | Add subscription | `subscribe add -c 700` |
| `subscribe remove` | Remove subscription | `subscribe remove -c 700` |
| `subscribe clear` | Remove all subscriptions | `subscribe clear` |

## Stock Code Format

| Input | Parsed As | Description |
|-------|-----------|-------------|
| `700` | `HK.00700` | Numeric → HK stock, padded to 5 digits |
| `00700` | `HK.00700` | Already 5 digits |
| `AAPL` | `US.AAPL` | Alphabetic → US stock |
| `HK.00700` | `HK.00700` | Full code with prefix |
| `US.AAPL` | `US.AAPL` | Full code with prefix |

## K-line Types

| Flag | Description |
|------|-------------|
| `-k 1m` | 1-minute K-line |
| `-k 5m` | 5-minute K-line |
| `-k 15m` | 15-minute K-line |
| `-k 30m` | 30-minute K-line |
| `-k 60m` | 60-minute K-line |
| `-k 1d` | Daily K-line |
| `-k 1w` | Weekly K-line |
| `-k 1M` | Monthly K-line |

## Subscription Types

| Type | Description |
|------|-------------|
| `QUOTE` | Real-time quotes (default) |
| `ORDER_BOOK` | Order book depth (default) |
| `TICKER` | Ticker (trades) |
| `RT_DATA` | Real-time data |
| `K_1M` | 1-minute K-line |
| `K_5M` | 5-minute K-line |
| `K_15M` | 15-minute K-line |
| `K_30M` | 30-minute K-line |
| `K_60M` | 60-minute K-line |
| `K_DAY` | Daily K-line |
| `K_WEEK` | Weekly K-line |
| `K_MON` | Monthly K-line |
| `BROKER` | Broker queue |

## Output Formats

### JSON (default)
```bash
$ nine-futu-cli quote snapshot -c 700
[
  {
    "code": "HK.00700",
    "name": "TENCENT",
    "last_done": 436.2,
    ...
  }
]
```

### NDJSON (one JSON per line)
```bash
$ nine-futu-cli quote kline -c 700 -k 5m --ndjson
{"code":"HK.00700","ktype":"5m","date":"2026-05-28","time":"09:35","open":431.0,...}
{"code":"HK.00700","ktype":"5m","date":"2026-05-28","time":"09:40","open":429.2,...}
```

## Debug Mode

Add `--debug` to see connection and subscription details:

```bash
$ nine-futu-cli --debug quote kline -c 700 -k 5m
[DEBUG] Connecting to 127.0.0.1:11111...
[DEBUG] Initializing connection...
[DEBUG] Connected! conn_id=7467121556106515641, server_ver=906
[DEBUG] Kline: code=HK.00700, ktype=5m, start=, end=
[DEBUG] Minute mode (all day): start=2026-06-01, end=2026-06-02
[DEBUG] Got 66 kline bars
[
  {"code":"HK.00700",...},
  ...
]
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `FUTU_HOST` | `127.0.0.1` | FutuOpenD host |
| `FUTU_PORT` | `11111` | FutuOpenD port |

### Command Line Options

| Option | Description |
|--------|-------------|
| `--host <HOST>` | FutuOpenD host |
| `--port <PORT>` | FutuOpenD port |
| `--debug` | Enable debug output |

## Installation from Source

```bash
git clone https://github.com/your-repo/nine-futu.git
cd nine-futu
cargo build --release
cp target/release/nine-futu-cli /usr/local/bin/
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with FutuOpenD
cargo test -- --ignored
```

## License

Apache License 2.0
