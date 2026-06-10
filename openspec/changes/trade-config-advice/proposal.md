## Why

When users run trade commands, they may not have configured their account settings properly. The system should remind users to update their config.toml with account details, trade type (SIM/REAL), and other settings.

Also, the trade client should connect to OpenD the same way as quote commands (using host/port from CLI args or env vars, not requiring config).

## What Changes

- Add config advice message when running any trade command
- Show current config status (account_id, real_trade_enabled, etc.)
- Advise user to update config.toml if settings are empty or default
- Trade client uses same connection method as quote (host/port from CLI or env)

## Capabilities

### New Capabilities

(None - modification of existing behavior)

### Modified Capabilities

- `trade-config`: Add config advice on trade command execution
- `trade-client`: Use same connection method as quote client

## Impact

- **Code**: Update trade command handlers and trade client
- **CLI**: Show config advice message
