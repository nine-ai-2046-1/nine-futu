## MODIFIED Requirements

### Requirement: Default subscription type
The system SHALL default to K-line only subscription when using `sub` command.

#### Scenario: Default to K-line
- **WHEN** user runs `nine-futu sub -c 700 -t 5m`
- **THEN** system subscribes to K_5M only

#### Scenario: Use --all flag
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --all`
- **THEN** system subscribes to all data types (Quote, OrderBook, Ticker, RtData, Broker, K_5M)
