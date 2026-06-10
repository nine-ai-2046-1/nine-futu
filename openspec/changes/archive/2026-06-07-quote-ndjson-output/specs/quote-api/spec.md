## MODIFIED Requirements

### Requirement: Quote command output format
The system SHALL output quote data in NDJSON format by default (one JSON object per line).

#### Scenario: Default output
- **WHEN** user runs `nine-futu quote snapshot -c 700`
- **THEN** system outputs one JSON object per line (NDJSON format)

#### Scenario: JSON array output
- **WHEN** user runs `nine-futu quote snapshot -c 700 --json`
- **THEN** system outputs a JSON array containing all results

#### Scenario: K-line NDJSON output
- **WHEN** user runs `nine-futu quote kline -c 700 -k 5m`
- **THEN** system outputs one JSON object per line for each K-line bar

#### Scenario: K-line JSON array output
- **WHEN** user runs `nine-futu quote kline -c 700 -k 5m --json`
- **THEN** system outputs a JSON array containing all K-line bars
