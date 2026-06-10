## ADDED Requirements

### Requirement: Place limit order
The system SHALL provide a command to place limit orders with optional SL/TP.

#### Scenario: Buy limit order
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0`
- **THEN** system shows order summary and asks for confirmation (unless -y)

#### Scenario: Buy limit with SL/TP
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0 -sl 400.0 -tp 460.0`
- **THEN** system creates buy order plus separate SL and TP orders

#### Scenario: Sell limit order
- **WHEN** user runs `nine-futu trade sell limit -c 700 -q 100 -p 450.0`
- **THEN** system sells only held positions (no short selling)

### Requirement: Place market order
The system SHALL provide a command to place market orders.

#### Scenario: Buy market order
- **WHEN** user runs `nine-futu trade buy market -c 700 -q 100`
- **THEN** system places market buy order

#### Scenario: Buy market with SL/TP
- **WHEN** user runs `nine-futu trade buy market -c 700 -q 100 -sl 400.0 -tp 460.0`
- **THEN** system places market buy order plus separate SL and TP orders

### Requirement: Modify order
The system SHALL provide a command to modify existing orders.

#### Scenario: Modify order price
- **WHEN** user runs `nine-futu trade modify -oi 12345 -p 435.0`
- **THEN** system modifies the order price

### Requirement: Cancel order
The system SHALL provide a command to cancel orders.

#### Scenario: Cancel order
- **WHEN** user runs `nine-futu trade cancel -oi 12345`
- **THEN** system cancels the order

### Requirement: List orders
The system SHALL provide a command to list current orders.

#### Scenario: List today's orders
- **WHEN** user runs `nine-futu trade orders`
- **THEN** system displays today's orders with status

### Requirement: Trade environment flag
The system SHALL accept --sim or --real flags to override config default.

#### Scenario: Use simulated trading
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0 --sim`
- **THEN** system uses simulated trading

#### Scenario: Use real trading
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0 --real`
- **THEN** system uses real trading (if enabled in config)

#### Scenario: Real trading disabled
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0 --real` and config has real_trade_enabled = false
- **THEN** system rejects and shows warning "Real trading is disabled in config"

### Requirement: Order confirmation
The system SHALL ask for confirmation before placing orders unless -y flag is provided.

#### Scenario: Confirmation prompt
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0`
- **THEN** system displays order summary and asks "Confirm? [y/N]"

#### Scenario: Skip confirmation with -y
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430.0 -y`
- **THEN** system places order without confirmation
