# nine-futu

A Rust CLI tool for Futu OpenD API, designed for stock traders to quickly access market data and execute trades.

## Features

- **Real-time Market Data**: Get live quotes, order book, and ticker data
- **Historical K-line**: Access historical candlestick data with flexible date ranges
- **Subscription Management**: Subscribe/unsubscribe to real-time data streams
- **JSON & NDJSON Output**: Machine-readable output for automation and scripting
- **Multi-market Support**: Hong Kong, US, and more markets
- **Period Calculation**: Auto-set start date with `-p` flag (e.g., `-p 30` for last 30 days)
- **Delayed Output**: Add delay between NDJSON outputs with `--delay` flag
- **CLI Integration**: Call external tools (nine-stock) for each K-line bar with `--cli` flag

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
nine-futu quote snapshot -c 700

# Get daily K-line (last 10 bars)
nine-futu quote kline -c 700 -k 1d

# Get 5-minute K-line for today
nine-futu quote kline -c 700 -k 5m

# Get K-line for specific date
nine-futu quote kline -c 700 -k 5m -s "2026-05-28" -e "2026-05-28"

# JSON output
nine-futu quote snapshot -c 700 --json

# NDJSON output (one JSON per line)
nine-futu quote kline -c 700 -k 5m
```

## Use Cases

### 1. Quick Price Check

```bash
nine-futu quote snapshot -c 700
# Output: {"code":"HK.00700","name":"TENCENT","last_done":436.2,...}
```

### 2. Daily Trend Analysis

```bash
# Last 30 days of daily K-line
nine-futu quote kline -c 700 -k 1d -p 30

# Specific date range
nine-futu quote kline -c 700 -k 1d -s "2026-04-01" -e "2026-04-30"
```

### 3. Intraday Analysis

```bash
# Today's 5-minute K-line
nine-futu quote kline -c 700 -k 5m

# Specific time range
nine-futu quote kline -c 700 -k 1m -s "2026-05-28 09:30" -e "2026-05-28 16:00"
```

### 4. Data Export for Backtesting

```bash
# Export daily data to file
nine-futu quote kline -c 700 -k 1d -p 365 --ndjson > data.jsonl
```

### 5. Delayed Output (Real-time Simulation)

```bash
# Output 5m bars with 60-second delay between each
nine-futu quote kline -c 700 -k 5m -p 7 --delay 60
```

### 6. CLI Integration (Call External Tool)

```bash
# Call nine-stock for each K-line bar
nine-futu quote kline -c 700 -k 5m -p 30 --cli "session-123"

# Subscription with CLI callback
nine-futu sub -c 700 -t 5m --cli "my-session"
```

### 7. Background Subscription

```bash
# Start subscription daemon
nine-futu sub -c 700 -t 5m

# Check running processes
nine-futu process list

# Stop a daemon
nine-futu process stop <PID>
```

## Commands

### Quote Commands

| Command | Description | Example |
|---------|-------------|---------|
| `snapshot` | Get market snapshot | `quote snapshot -c 700` |
| `kline` | Get K-line data | `quote kline -c 700 -k 1d` |

### K-line Flags

| Flag | Description | Example |
|------|-------------|---------|
| `-p <days>` | Period: auto-set start date to N days before today | `-p 30` |
| `--delay <sec>` | Delay between NDJSON outputs | `--delay 60` |
| `--cli <session>` | Call external CLI for each bar | `--cli "session-123"` |
| `--json` | Output as JSON array (default: NDJSON) | `--json` |

### Subscription Commands

| Command | Description | Example |
|---------|-------------|---------|
| `sub` | Start subscription (daemon by default) | `sub -c 700 -t 5m` |
| `sub -f` | Start subscription (foreground) | `sub -c 700 -t 5m -f` |
| `process list` | List running daemons | `process list` |
| `process status <code>` | Check subscription status | `process status 700` |
| `process stop <pid>` | Stop a daemon | `process stop 12345` |

### Subscription Flags

| Flag | Description | Example |
|------|-------------|---------|
| `-t <timeframe>` | K-line timeframe (default: 5m) | `-t 15m` |
| `-f` | Run in foreground (default: daemon) | `-f` |
| `--cli <session>` | Call external CLI for each K-line bar | `--cli "session-123"` |

## Trade Commands

### Buy/Sell Orders

```bash
# Buy limit order with stop loss and take profit
nine-futu trade buy limit -c 700 -q 100 -p 430 -sl 400 -tp 460

# Buy market order
nine-futu trade buy market -c 700 -q 100

# Sell limit order
nine-futu trade sell limit -c 700 -q 100 -p 450

# Auto-confirm (skip confirmation prompt)
nine-futu trade buy limit -c 700 -q 100 -p 430 -y

# Use real trading (requires config enabled)
nine-futu trade buy limit -c 700 -q 100 -p 430 --real

# Use margin account
nine-futu trade buy limit -c 700 -q 100 -p 430 --margin
```

### Modify/Cancel Orders

```bash
# Modify order price
nine-futu trade modify -oi 12345 -p 435

# Cancel order
nine-futu trade cancel -oi 12345
```

### Account & Position

```bash
# List trading accounts
nine-futu trade accounts

# Get account funds
nine-futu trade funds

# List positions
nine-futu trade positions

# List orders
nine-futu trade orders

# List trades
nine-futu trade trades
```

### Trade Environment

| Input | Meaning |
|-------|---------|
| `--sim` | Simulated trading (default) |
| `--real` | Real trading (requires config) |
| `--margin` | Margin account |
| `-y` | Auto-confirm order |

### Order Status Values

| Status | Description |
|--------|-------------|
| Submitted | Order submitted |
| Filled | Order fully filled |
| Partially Filled | Order partially filled |
| Cancelled | Order cancelled |
| Failed | Order failed |

## CLI Integration

When using `--cli`, nine-futu calls an external CLI tool for each K-line bar:

```bash
nine-futu quote kline -c 700 -k 5m -p 30 --cli "session-123"
```

This spawns a subprocess for each bar:
```bash
nine-stock --session "session-123" --code "HK.00700" --ktype "5m" --data '{"code":"HK.00700",...}'
```

### Dependencies for --cli

The `--cli` feature requires additional tools to be installed:

| Tool | Repository | Description |
|------|------------|-------------|
| [nine-stock](https://github.com/nine-ai-2026-1/nine-stock) | github.com/nine-ai-2026-1/nine-stock | Analyzes K-line data and sends reports |
| [nine-poe](https://github.com/nine-ai-2026-1/nine-poe) | github.com/nine-ai-2026-1/nine-poe | AI-powered analysis engine (required by nine-stock) |
| opencb | (messaging tool) | Sends reports to users (required by nine-stock) |

**Note**: These tools are only required when using the `--cli` flag. The core nine-futu functionality works without them.

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

### NDJSON (default)
```bash
$ nine-futu quote kline -c 700 -k 5m
{"code":"HK.00700","ktype":"5m","date":"2026-06-07","time":"09:35","open":431.0,...}
{"code":"HK.00700","ktype":"5m","date":"2026-06-07","time":"09:40","open":429.2,...}
```

### JSON Array
```bash
$ nine-futu quote snapshot -c 700 --json
[
  {
    "code": "HK.00700",
    "name": "TENCENT",
    "last_done": 436.2,
    ...
  }
]
```

## Debug Mode

Add `--debug` to see connection and subscription details:

```bash
$ nine-futu --debug quote kline -c 700 -k 5m
[DEBUG] Connecting to 127.0.0.1:11111...
[DEBUG] Initializing connection...
[DEBUG] Connected! conn_id=7467121556106515641, server_ver=906
```

## Configuration

### Config File

Location: `~/.opens/nine-futu/config.toml`

```toml
[account]
account_id = ""                    # Your Futu account ID
password = ""                      # Password (optional)
real_trade_enabled = false         # Enable real trading
default_trade_env = "SIMULATE"     # Default: SIMULATE or REAL
default_account_type = "CASH"      # Default: CASH or MARGIN

[connection]
host = "127.0.0.1"                # FutuOpenD host
port = 11111                       # FutuOpenD port
```

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

## Installation

### From Source

```bash
git clone https://github.com/nine-ai-2026-1/nine-futu.git
cd nine-futu
cargo build --release
cp target/release/nine-futu /usr/local/bin/
```

### Using Build Script

```bash
./scripts/build-deploy
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
