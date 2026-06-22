## Why

Currently, the `sub` command subscribes to all data types (Quote, OrderBook, Ticker, RtData, Broker, K-line). This is wasteful when users only need K-line data for backtesting or analysis. Defaulting to K-line only reduces network traffic and storage usage.

## What Changes

- Default subscription: K-line only (based on `-t` timeframe)
- Add `--all` flag to subscribe to all data types
- Keep existing `--cli` functionality

## Capabilities

### New Capabilities

(None - modification of existing behavior)

### Modified Capabilities

- `subscription`: Default to K-line only, add --all flag

## Impact

- **Code**: Update subscription command in main.rs
- **CLI**: Add --all flag, change default behavior
