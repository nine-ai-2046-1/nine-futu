## MODIFIED Requirements

### Requirement: Config advice on trade commands
The system SHALL show config advice when running any trade command.

#### Scenario: Empty account_id
- **WHEN** user runs any trade command and account_id is empty
- **THEN** system displays "[CONFIG] Account: not set" and advises to update config.toml

#### Scenario: Real trade disabled
- **WHEN** user runs any trade command and real_trade_enabled is false
- **THEN** system displays "[CONFIG] Real trade: disabled" and advises to update config.toml

#### Scenario: Config path
- **WHEN** config advice is shown
- **THEN** system displays "Update config.toml at ~/.opens/nine-futu/config.toml"
