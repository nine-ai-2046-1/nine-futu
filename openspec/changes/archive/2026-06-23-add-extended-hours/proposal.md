## Why

Traders need access to extended trading hours data (pre-market and after-hours) for US stocks. The Futu API supports this via `extended_time` and `session` parameters, but the CLI doesn't expose these options.

## What Changes

- Add `--extended` flag to `quote kline` command
- When `--extended` is used, set `extended_time=true` in request
- Default: regular trading hours only (extended_time=false)
- Only applies to K-line types 60 minutes and below

## Capabilities

### New Capabilities

(None - modification of existing behavior)

### Modified Capabilities

- `quote-api`: Add extended time support for kline command

## Impact

- **Code**: Update kline command and client methods
- **CLI**: Add --extended flag
