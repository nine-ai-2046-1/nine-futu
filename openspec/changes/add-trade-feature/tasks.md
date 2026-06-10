## 1. Config Module

- [x] 1.1 Create config module with TOML parsing
- [x] 1.2 Implement default config creation
- [x] 1.3 Implement config loading from ~/.opens/nine-futu/config.toml
- [x] 1.4 Implement real_trade_enabled check for --real flag

## 2. Trade Client

- [x] 2.1 Add trade proto modules to proto.rs
- [x] 2.2 Implement trade client with OpenSecTradeContext
- [x] 2.3 Implement futures client with OpenFutureTradeContext

## 3. Account Commands

- [x] 3.1 Implement `trade accounts` command
- [x] 3.2 Implement `trade funds` command
- [x] 3.3 Add SIM/REAL display to all trade output

## 4. Order Commands

- [x] 4.1 Implement `trade buy limit` command with -p, -sl, -tp
- [x] 4.2 Implement `trade buy market` command with -sl, -tp
- [x] 4.3 Implement `trade sell limit` command (no short)
- [x] 4.4 Implement `trade sell market` command (no short)
- [x] 4.5 Implement `trade modify` command with -oi, -p
- [x] 4.6 Implement `trade cancel` command with -oi
- [x] 4.7 Implement `trade orders` command
- [x] 4.8 Implement order confirmation with -y flag
- [x] 4.9 Implement --sim/--real flag with config check

## 5. Position Commands

- [x] 5.1 Implement `trade positions` command
- [x] 5.2 Implement `trade margin` command

## 6. History Commands

- [x] 6.1 Implement `trade trades` command
- [x] 6.2 Implement `trade trades --history` command
- [x] 6.3 Implement `trade cashflow` command

## 7. Testing

- [x] 7.1 Test config loading
- [x] 7.2 Test account commands
- [x] 7.3 Test order commands (simulated)
- [x] 7.4 Test position commands
- [x] 7.5 Test history commands
