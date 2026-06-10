## ADDED Requirements

### Requirement: Get positions
The system SHALL provide a command to list current positions.

#### Scenario: List positions
- **WHEN** user runs `nine-futu trade positions`
- **THEN** system displays current positions with quantity, cost, and P&L

### Requirement: Get margin data
The system SHALL provide a command to query margin trading data.

#### Scenario: Query margin
- **WHEN** user runs `nine-futu trade margin`
- **THEN** system displays margin ratio and related data
