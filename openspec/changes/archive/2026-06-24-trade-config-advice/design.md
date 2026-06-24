## Context

Users may run trade commands without proper config setup. The system should advise users to update their config.

Also, the trade client should connect to OpenD the same way as quote commands - using host/port from CLI args or environment variables, not requiring config.

## Decisions

### 1. Connection Method

**Decision**: Trade client uses same connection as quote client.

**Rationale**: Consistent behavior across all commands. Config is optional for connection settings.

### 2. Config Advice Message

**Decision**: Show config advice when running any trade command.

**Message format**:
```
[CONFIG] Account: not set | Real trade: disabled
[CONFIG] Update config.toml at ~/.opens/nine-futu/config.toml
```

### 3. When to Show

**Decision**: Show advice if account_id is empty or real_trade_enabled is false.

**Rationale**: These are the most common misconfigurations.
