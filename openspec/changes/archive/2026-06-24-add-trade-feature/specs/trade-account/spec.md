## ADDED Requirements

### Requirement: Get trading accounts
The system SHALL provide a command to list trading accounts.

#### Scenario: List accounts
- **WHEN** user runs `nine-futu trade accounts`
- **THEN** system displays list of trading accounts with IDs and types

### Requirement: Get account funds
The system SHALL provide a command to query account balance.

#### Scenario: Query funds
- **WHEN** user runs `nine-futu trade funds`
- **THEN** system displays account balance, available funds, and market value

### Requirement: Display trading environment
The system SHALL display SIM or REAL in all trade command output.

#### Scenario: Show trading env
- **WHEN** user runs any trade command
- **THEN** output includes `[SIM]` or `[REAL]` prefix
