## Why

Users need to integrate nine-futu with external tools for processing each data point in real-time. For example, a trading bot ("nine-stock") may need to process each K-line bar as it arrives for backtesting or live trading decisions. Currently, there's no way to pipe data to external tools automatically.

## What Changes

- Add `--cli` flag to `quote kline` command (accepts session ID string)
- Add `--cli` flag to `sub` command (accepts session ID string)
- When `--cli "session-id"` is provided, after outputting each K-line bar, spawn a subprocess calling `nine-stock --session "{session_id}" --code "{stock_code}" --ktype "{timeframe}" --data "{json}"`
- If `nine-stock` crashes or errors, output error to stderr but continue processing
- Only K-line data triggers the CLI callback (not quote, orderbook, etc.)

## Capabilities

### New Capabilities

(None - this is a modification of existing behavior)

### Modified Capabilities

- `quote-api`: Add CLI callback flag for kline output
- `subscription`: Add CLI callback flag for kline push data

## Impact

- **Code**: Update kline command and push data handler
- **CLI**: New `--cli` flag on kline and sub commands
- **Dependencies**: Requires external `nine-stock` binary in PATH
