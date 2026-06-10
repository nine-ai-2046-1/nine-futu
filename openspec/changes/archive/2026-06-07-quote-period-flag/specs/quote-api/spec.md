## MODIFIED Requirements

### Requirement: K-line period flag
The system SHALL support a `-p` flag on the kline command to automatically set the start date to N days before today.

#### Scenario: Period flag with daily kline
- **WHEN** user runs `nine-futu quote kline -c 700 -k 1d -p 30`
- **THEN** system queries K-line data from 30 days ago to today

#### Scenario: Period flag with end date
- **WHEN** user runs `nine-futu quote kline -c 700 -k 1d -p 30 -e "2026-06-01"`
- **THEN** system queries K-line data from 2026-05-02 to 2026-06-01

#### Scenario: Period flag with minute kline
- **WHEN** user runs `nine-futu quote kline -c 700 -k 5m -p 7`
- **THEN** system queries 5-minute K-line data for the last 7 days

#### Scenario: Period flag overrides start flag
- **WHEN** user runs `nine-futu quote kline -c 700 -k 1d -p 30 -s "2026-01-01"`
- **THEN** system uses `-p 30` (30 days ago) and ignores `-s`

### Requirement: K-line delay flag
The system SHALL support a `--delay` flag on the kline command to add delay between each data output.

#### Scenario: Delay with NDJSON output
- **WHEN** user runs `nine-futu quote kline -c 700 -k 1d -p 10 --delay 60`
- **THEN** system outputs each K-line bar with 60 seconds between them

#### Scenario: Delay with JSON output
- **WHEN** user runs `nine-futu quote kline -c 700 -k 1d -p 10 --json --delay 30`
- **THEN** system outputs JSON array after all delays complete

#### Scenario: No delay (default)
- **WHEN** user runs `nine-futu quote kline -c 700 -k 1d -p 10`
- **THEN** system outputs all data immediately without delay
