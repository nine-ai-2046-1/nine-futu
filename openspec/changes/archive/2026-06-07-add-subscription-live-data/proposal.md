## Why

Traders need to continuously monitor real-time market data (quotes, order book, K-line, ticker, etc.) for specific stocks and store this data for analysis. Currently, nine-futu only supports one-shot queries. A subscription system with persistent data storage enables:

- Real-time monitoring during trading hours
- Historical data collection for backtesting
- Automated alerts via notification integration (opencb)
- Background data collection via daemon mode

## What Changes

- **New `sub` command**: Subscribe to real-time data streams for a stock code
  - Default: daemon mode (background process)
  - `-fe` flag: foreground mode
  - `-tf` flag: specify K-line timeframe (default: 5m)
  - Subscribes to all push types: K-line, Quote, OrderBook, Ticker, RTData, Broker

- **New `process` command group**: Manage daemon processes
  - `process list`: Show all running subscription daemons
  - `process stop {pid}`: Kill a daemon by PID
  - `process status {code}`: Check if subscription is running for a code

- **New `clean` command**: Archive old live data
  - Move data folders older than 1 day to specified destination
  - Support `-y` flag for non-interactive mode

- **New data storage**: Live data stored as NDJSON files
  - Path: `~/.opens/nine-futu/data/live/{code}/{date}/{tf}/kline.txt`
  - Path: `~/.opens/nine-futu/data/live/{code}/{date}/{data_type}.txt`

- **Error handling & notifications**: Auto-reconnect (max 3 retries), crash reporting via opencb

## Capabilities

### New Capabilities

- `subscription`: Subscribe to real-time data streams from FutuOpenD with push callbacks
- `process-management`: Manage daemon processes (list, stop, status) with PID files
- `live-data-storage`: Store real-time data as NDJSON files with date-based organization
- `data-cleanup`: Archive old live data to specified destination folders

### Modified Capabilities

(None - this is entirely new functionality)

## Impact

- **Code**: New modules in nine-futu-lib (subscription, process, storage, data_parser)
- **CLI**: New subcommands in nine-futu binary
- **Storage**: New directory structure under `~/.opens/nine-futu/`
- **Dependencies**: May need `daemonize` crate for daemon mode, `opencb` CLI for notifications
- **Proto**: Need to implement actual protobuf encoding/decoding for subscription requests
