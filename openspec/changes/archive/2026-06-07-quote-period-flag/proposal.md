## Why

Traders frequently need to query data for the last N days (e.g., last 30 days of K-line data). Currently, users must manually calculate and provide the start date. A `-p` flag simplifies this by automatically setting the start date to N days before today.

Additionally, for backtesting and simulation, users need to process data at a controlled pace rather than all at once. A `--delay` flag adds artificial delay between each data output, simulating real-time data streaming.

## What Changes

- Add `-p` flag to `kline` subcommand for automatic date range calculation
- Add `--delay` flag to `kline` subcommand for delayed output
- When `--delay 60` is provided, output each K-line bar with 60 seconds between them
- Works with NDJSON output (one line per delay period)

## Capabilities

### New Capabilities

(None)

### Modified Capabilities

- `quote-api`: Add period flag and delay flag for kline command

## Impact

- **Code**: Update kline command in main.rs
- **CLI**: New `-p` and `--delay` flags on kline subcommand
