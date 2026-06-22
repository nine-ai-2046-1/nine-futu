## MODIFIED Requirements

### Requirement: Auto-find last trading day
The system SHALL automatically find the last trading day when no date is specified for minute K-line.

#### Scenario: Weekend query
- **WHEN** user runs `nine-futu quote kline -c US.AAPL -k 5m` on Sunday
- **THEN** system queries previous trading day (Friday) and returns data

#### Scenario: Holiday query
- **WHEN** user runs `nine-futu quote kline -c US.AAPL -k 5m` on a holiday
- **THEN** system queries previous trading day and returns data

#### Scenario: Max lookback
- **WHEN** no trading day found within 7 days
- **THEN** system returns empty result
