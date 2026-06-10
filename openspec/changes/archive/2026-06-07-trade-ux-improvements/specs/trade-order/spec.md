## MODIFIED Requirements

### Requirement: Trade environment handling
The system SHALL default to Simulate environment and only allow Real with --real flag and config enabled.

#### Scenario: Default to simulate
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430`
- **THEN** system uses Simulate environment (TrdEnv=0)

#### Scenario: Use real with flag
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --real`
- **THEN** system uses Real environment (TrdEnv=1) if config enabled

#### Scenario: Real disabled in config
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --real` and config has real_trade_enabled = false
- **THEN** system rejects and shows warning

### Requirement: String inputs for environment
The system SHALL accept string inputs for trade environment.

#### Scenario: Accept "sim"
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --env sim`
- **THEN** system uses Simulate environment (TrdEnv=0)

#### Scenario: Accept "real"
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --env real`
- **THEN** system uses Real environment (TrdEnv=1) if config enabled

### Requirement: String inputs for account type
The system SHALL accept string inputs for account type.

#### Scenario: Accept "cash"
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --type cash`
- **THEN** system uses Cash account (TrdAccType=1)

#### Scenario: Accept "margin"
- **WHEN** user runs `nine-futu trade buy limit -c 700 -q 100 -p 430 --margin`
- **THEN** system uses Margin account (TrdAccType=2)
