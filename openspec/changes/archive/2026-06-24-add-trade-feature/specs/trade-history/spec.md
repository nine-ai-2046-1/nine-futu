## ADDED Requirements

### Requirement: Get today's trades
The system SHALL provide a command to list today's executed trades.

#### Scenario: List today's trades
- **WHEN** user runs `nine-futu trade trades`
- **THEN** system displays today's executed trades

### Requirement: Get history trades
The system SHALL provide a command to list historical trades.

#### Scenario: List history trades
- **WHEN** user runs `nine-futu trade trades --history`
- **THEN** system displays historical trades

### Requirement: Get cash flow
The system SHALL provide a command to query cash flow summary.

#### Scenario: Query cash flow
- **WHEN** user runs `nine-futu trade cashflow`
- **THEN** system displays cash flow summary
