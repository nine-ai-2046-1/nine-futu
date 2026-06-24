## MODIFIED Requirements

### Requirement: Default subscription type
The system SHALL default to K-line only subscription when using `sub` command.

#### Scenario: Default to K-line
- **WHEN** user runs `nine-futu sub -c 700 -t 5m`
- **THEN** system subscribes to K_5M only

#### Scenario: Use --all flag
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --all`
- **THEN** system subscribes to all data types (Quote, OrderBook, Ticker, RtData, Broker, K_5M)

### Requirement: Granular subscription flags
The system SHALL allow subscribing to specific data types via individual flags.

#### Scenario: Subscribe to quote only
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --quote`
- **THEN** system subscribes to Quote and K_5M

#### Scenario: Subscribe to multiple types
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --quote --orderbook --ticker`
- **THEN** system subscribes to Quote, OrderBook, Ticker, and K_5M

#### Scenario: No subscription flags
- **WHEN** user runs `nine-futu sub -c 700 -t 5m`
- **THEN** system subscribes to K_5M only (kline is always included)

#### Scenario: Combine with --all
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --all --quote`
- **THEN** system subscribes to all data types (--all takes precedence)

### Requirement: Kline bar completion buffer
The system SHALL buffer minute kline data and only output completed timeframe bars.

#### Scenario: Buffer 5m kline updates
- **WHEN** subscribing to 5m kline and receiving updates at 09:30:01, 09:30:15, 09:31:00
- **THEN** all updates with time_key "2026-06-24 09:30:00" are buffered (not output)

#### Scenario: Output completed bar
- **WHEN** receiving a kline update with time_key "2026-06-24 09:35:00" while buffer contains "2026-06-24 09:30:00"
- **THEN** system outputs the buffered 09:30:00 bar and buffers the new 09:35:00 bar

#### Scenario: 1m kline buffer
- **WHEN** subscribing to 1m kline
- **THEN** same buffer logic applies — output when minute boundary is crossed

#### Scenario: Non-minute kline no buffer
- **WHEN** subscribing to daily kline (1d)
- **THEN** data is output immediately without buffering
