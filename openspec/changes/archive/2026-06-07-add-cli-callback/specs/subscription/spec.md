## MODIFIED Requirements

### Requirement: Subscription CLI callback flag
The system SHALL support a `--cli` flag on the sub command to call an external binary for each K-line push data.

#### Scenario: CLI callback with session ID
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --cli "my-session"`
- **THEN** system outputs each K-line push data to stdout and calls `nine-stock --session "my-session" --code "HK.00700" --ktype "5m" --data "{json}"` for each bar

#### Scenario: Non-K-line data ignored
- **WHEN** subscription receives quote, orderbook, or ticker data
- **THEN** system does NOT call subprocess (only K-line triggers callback)

#### Scenario: Subprocess error
- **WHEN** `nine-stock` crashes or returns non-zero exit code
- **THEN** system outputs error to stderr and continues processing
