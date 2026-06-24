## Why

Traders need to execute trades directly from the command line without switching to other applications. Currently, nine-futu only supports market data queries. Adding trade capabilities enables:

- Automated trading strategies
- Quick order execution
- Position and order management
- Real-time trade monitoring

## What Changes

- **New `trade` command group** with full trading capabilities
- **New `config.toml`** for account configuration
- **All trade APIs**: Account, Assets, Orders, Trades
- **Order commands**:
  - `trade buy limit` - Limit order with SL/TP
  - `trade buy market` - Market order with SL/TP
  - `trade sell limit` - Limit order (sell only, no short)
  - `trade sell market` - Market order (sell only)
  - `trade modify` - Modify order price
  - `trade cancel` - Cancel order
  - `trade orders` - List orders
- **Safety features**: Confirmation prompts, SIM/REAL display, -y auto-approve
- **All markets**: HK, US, CN, SG, JP, Futures

## Capabilities

### New Capabilities

- `trade-config`: Configuration file for account settings
- `trade-account`: Account management (list, funds, unlock)
- `trade-order`: Order management (place, modify, cancel, list, history)
- `trade-position`: Position and assets management
- `trade-history`: Trade history and cash flow

### Modified Capabilities

(None)

## Impact

- **Code**: New modules in nine-futu-lib (trade, config)
- **CLI**: New `trade` command group
- **Storage**: New `config.toml` file
- **Dependencies**: None (uses existing FutuOpenD)
