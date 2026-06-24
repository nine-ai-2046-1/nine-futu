## Why

The `sub` command currently has three limitations:
1. **Coarse-grained subscription**: Only `--all` flag exists — no way to subscribe to specific data types (e.g., Quote + Kline only)
2. **Always saves to files**: No stdout-only mode for piping to other CLI tools or apps
3. **Raw kline output**: Kline push data outputs every update within a timeframe period, causing noisy output (e.g., multiple 09:30 bars at 09:30:01, 09:30:10, etc.)

## What Changes

- Add individual subscription flags: `--quote`, `--orderbook`, `--ticker`, `--rtdata`, `--broker`
- Default behavior: kline only (no change)
- Add `--output [path]` flag to control file output (default: stdout only)
- Add kline bar completion buffer: only output completed timeframe bars for minute klines (1m, 3m, 5m, 15m, 30m, 60m)

## Capabilities

### New Capabilities

- `subscription-output`: Control where subscription data is written (stdout vs file)

### Modified Capabilities

- `subscription`: Add granular subscription flags and kline bar completion buffer

## Impact

- **Code**: Update `main.rs` CLI args, update `push_handler.rs` to add bar completion buffer and output routing
- **CLI**: New flags: `--quote`, `--orderbook`, `--ticker`, `--rtdata`, `--broker`, `--output`
- **Behavior**: Default output changes from file to stdout
