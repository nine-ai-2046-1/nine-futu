## ADDED Requirements

### Requirement: Trade configuration file
The system SHALL use a config.toml file for trade settings.

#### Scenario: Config file location
- **WHEN** user runs any trade command
- **THEN** system reads config from `~/.opens/nine-futu/config.toml`

#### Scenario: Config file not exists
- **WHEN** config.toml does not exist
- **THEN** system creates default config with simulated trading enabled

#### Scenario: Default config structure
- **WHEN** config is created
- **THEN** system uses:
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
