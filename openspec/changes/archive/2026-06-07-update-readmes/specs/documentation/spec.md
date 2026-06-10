## ADDED Requirements

### Requirement: README documentation for new CLI flags
The README files SHALL document all CLI flags including `-p`, `--delay`, and `--cli`.

#### Scenario: Document period flag
- **WHEN** user reads README
- **THEN** README explains `-p <days>` flag with example

#### Scenario: Document delay flag
- **WHEN** user reads README
- **THEN** README explains `--delay <seconds>` flag with example

#### Scenario: Document CLI flag
- **WHEN** user reads README
- **THEN** README explains `--cli <session-id>` flag with example

### Requirement: README documentation for dependencies
The README files SHALL document nine-stock and nine-poe as optional dependencies for --cli feature.

#### Scenario: Document dependencies
- **WHEN** user reads README
- **THEN** README lists nine-stock, nine-poe, opencb as optional dependencies with links

### Requirement: README use cases
The README files SHALL include use cases from testing sessions.

#### Scenario: Use case documentation
- **WHEN** user reads README
- **THEN** README shows real-world examples with expected output
