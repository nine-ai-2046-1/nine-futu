## Context

Traders need command-line access to execute trades without leaving the terminal. The Futu OpenD gateway provides comprehensive trade APIs that need to be exposed through nine-futu CLI.

## Goals / Non-Goals

**Goals:**
- Implement all trade APIs (Account, Assets, Orders, Trades)
- Support all order types (Normal, Market, Stop, Stop-Limit)
- Support all markets (HK, US, CN, SG, JP, Futures)
- Add config.toml for account settings
- Safety: Confirmation prompts, SIM/REAL display, -y auto-approve

**Non-Goals:**
- Advanced risk management (position limits, daily loss limits)
- Multi-account management in single command
- Trading strategies (leave to users)

## Decisions

### 1. Configuration File

**Decision**: Use `config.toml` in `~/.opens/nine-futu/` directory.

```toml
[account]
account_id = ""
password = ""
real_trade_enabled = false
default_trade_env = "SIMULATE"

[connection]
host = "127.0.0.1"
port = 11111
```

### 2. Safety Features

**Decision**: 
- Default to simulated trading (SIM)
- Show SIM/REAL in all trade command output
- Ask for confirmation before placing orders (unless -y flag)
- No position size or daily loss limits

### 3. Order Commands

**Decision**: Use subcommands for order types:

```bash
# Limit order with SL/TP
nine-futu trade buy limit -c 700 -q 100 -p 300 -sl 280 -tp 330

# Market order with SL/TP
nine-futu trade buy market -c 700 -q 100 -sl 280 -tp 330

# Sell (only sell what you have, no short selling)
nine-futu trade sell limit -c 700 -q 100 -p 450.0
nine-futu trade sell market -c 700 -q 100

# Modify order
nine-futu trade modify -oi <order-id> -p 435.0

# Cancel order
nine-futu trade cancel -oi <order-id>
```

**Notes:**
- `-sl` = Stop Loss (creates separate order)
- `-tp` = Take Profit (creates separate order)
- Sell commands only sell positions held (no short selling)

### 4. Trade Environment

**Decision**: All trade commands accept `--sim` or `--real` flags.

- `--sim`: Use simulated trading (default if not specified)
- `--real`: Use real trading (requires config enabled)

**Safety**: If config has `real_trade_enabled = false`, reject `--real` and show warning.

### 4. Markets

**Decision**: Support all Futu markets:
- HK (Hong Kong)
- US (United States)
- CN (China A-shares)
- SG (Singapore)
- JP (Japan)
- Futures (via OpenFutureTradeContext)

### 5. Trade Context

**Decision**: Use OpenSecTradeContext for stocks, OpenFutureTradeContext for futures.

**Rationale**: FutuOpenD separates stock and futures trading into different contexts.

## Risks / Trade-offs

### Risk: Real Money Trading
**Impact**: Accidental trades with real money
**Mitigation**: Default to SIM, confirmation prompts, -y flag for automation

### Risk: Order Errors
**Impact**: Wrong price/quantity submitted
**Mitigation**: Show order summary before confirmation, support modify/cancel

### Risk: Market Hours
**Impact**: Orders placed outside market hours may fail
**Mitigation**: Check market state before placing orders (optional)
