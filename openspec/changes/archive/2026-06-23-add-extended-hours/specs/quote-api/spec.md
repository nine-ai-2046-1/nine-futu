## MODIFIED Requirements

### Requirement: Extended hours support for kline
The system SHALL support extended trading hours data via --extended flag.

#### Scenario: Default regular hours
- **WHEN** user runs `nine-futu quote kline -c US.AAPL -k 5m -s "2026-06-18" -e "2026-06-18"`
- **THEN** system returns regular trading hours data only (9:30 AM - 4:00 PM ET)

#### Scenario: Extended hours with --extended
- **WHEN** user runs `nine-futu quote kline -c US.AAPL -k 5m -s "2026-06-18" -e "2026-06-18" --extended`
- **THEN** system returns extended hours data (pre-market 4:00 AM - 9:30 AM, after-hours 4:00 PM - 8:00 PM)

#### Scenario: Daily K-line ignores --extended
- **WHEN** user runs `nine-futu quote kline -c US.AAPL -k 1d --extended`
- **THEN** system ignores --extended flag (only 60min and below supported)
