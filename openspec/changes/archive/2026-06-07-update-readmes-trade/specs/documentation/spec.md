## ADDED Requirements

### Requirement: Trade commands documentation
The README files SHALL document all trade commands with examples.

#### Scenario: Document buy command
- **WHEN** user reads README
- **THEN** README shows buy command with -p, -sl, -tp flags

#### Scenario: Document sell command
- **WHEN** user reads README
- **THEN** README shows sell command

#### Scenario: Document modify/cancel commands
- **WHEN** user reads README
- **THEN** README shows modify and cancel commands

### Requirement: Config documentation
The README files SHALL document config.toml structure.

#### Scenario: Document config structure
- **WHEN** user reads README
- **THEN** README shows config.toml with all fields

### Requirement: String inputs documentation
The README files SHALL document string inputs for environment and account type.

#### Scenario: Document string inputs
- **WHEN** user reads README
- **THEN** README shows "sim"/"real" and "cash"/"margin" options
