## MODIFIED Requirements

### Requirement: K-line CLI callback flag
The system SHALL support a `--cli` flag on the kline command to call an external binary for each K-line bar.

#### Scenario: CLI callback with session ID
- **WHEN** user runs `nine-futu quote kline -c 700 -k 5m -p 2 --cli "my-session"`
- **THEN** system outputs each K-line bar to stdout and calls `nine-stock --session "my-session" --code "HK.00700" --ktype "5m" --data "{json}"` for each bar

#### Scenario: CLI callback without delay
- **WHEN** user runs `nine-futu quote kline -c 700 -k 5m -p 2 --cli "session-123"`
- **THEN** system outputs each K-line bar and calls subprocess immediately

#### Scenario: Subprocess error
- **WHEN** `nine-stock` crashes or returns non-zero exit code
- **THEN** system outputs error to stderr and continues to next bar

#### Scenario: No CLI flag
- **WHEN** user runs `nine-futu quote kline -c 700 -k 5m -p 2`
- **THEN** system outputs K-line bars without calling subprocess
