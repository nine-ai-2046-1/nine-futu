## MODIFIED Requirements

### Requirement: Auto-select account ID
The system SHALL auto-select account ID based on environment and account type.

#### Scenario: Auto-select for simulate
- **WHEN** user runs trade command with Simulate environment
- **THEN** system selects first account with matching environment and account type

#### Scenario: Auto-select for real
- **WHEN** user runs trade command with Real environment
- **THEN** system selects first account with matching environment and account type

### Requirement: Default account type
The system SHALL default to Cash account type (TrdAccType=1).

#### Scenario: Default to cash
- **WHEN** user runs trade command without --type or --margin flag
- **THEN** system uses Cash account type (TrdAccType=1)

#### Scenario: Use margin
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --margin`
- **THEN** system uses Margin account type (TrdAccType=2)

### Requirement: Order status display
The system SHALL show order status descriptions instead of numeric values.

#### Scenario: Show submitted status
- **WHEN** order is submitted
- **THEN** system displays "Submitted" instead of "5"

#### Scenario: Show filled status
- **WHEN** order is fully filled
- **THEN** system displays "Filled" instead of "11"

#### Scenario: Show cancelled status
- **WHEN** order is cancelled
- **THEN** system displays "Cancelled" instead of "15"
