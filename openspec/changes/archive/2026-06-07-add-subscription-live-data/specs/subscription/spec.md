## ADDED Requirements

### Requirement: Subscribe to real-time data streams
The system SHALL allow users to subscribe to real-time data streams for a stock code using the `sub` command.

#### Scenario: Subscribe with default timeframe
- **WHEN** user runs `nine-futu sub -c "700"`
- **THEN** system subscribes to K_5M and all push types (QUOTE, ORDER_BOOK, TICKER, RT_DATA, BROKER) for HK.00700

#### Scenario: Subscribe with custom timeframe
- **WHEN** user runs `nine-futu sub -c "700" -tf "15m"`
- **THEN** system subscribes to K_15M and all push types for HK.00700

#### Scenario: Subscribe in foreground mode
- **WHEN** user runs `nine-futu sub -c "700" -fe`
- **THEN** system runs subscription in foreground until Ctrl+C

#### Scenario: Subscribe in daemon mode (default)
- **WHEN** user runs `nine-futu sub -c "700"`
- **THEN** system forks to background and writes PID file

### Requirement: Prevent duplicate subscriptions
The system SHALL prevent multiple subscriptions for the same stock code.

#### Scenario: Code already subscribed
- **WHEN** user runs `nine-futu sub -c "700"` and daemon is already running for HK.00700
- **THEN** system displays error "Already running on PID {pid}" and exits with code 1

#### Scenario: Code not subscribed
- **WHEN** user runs `nine-futu sub -c "700"` and no daemon is running
- **THEN** system proceeds with subscription

### Requirement: Auto-reconnect on connection loss
The system SHALL automatically reconnect to FutuOpenD when connection is lost.

#### Scenario: Connection lost, retry succeeds
- **WHEN** connection to FutuOpenD is lost
- **THEN** system attempts to reconnect up to 3 times with 5-second delay

#### Scenario: Connection lost, all retries failed
- **WHEN** all 3 reconnection attempts fail
- **THEN** system calls `opencb send "WARNING-NINE_FUT OpenD-Connection-Error"` and exits with code 1

### Requirement: Error notification via opencb
The system SHALL notify users of errors via the opencb CLI.

#### Scenario: Runtime error
- **WHEN** a runtime error occurs during subscription
- **THEN** system calls `opencb send "WARNING-NINE_FUT Sub-Daemon-Error"` and exits with code 1

#### Scenario: Panic recovery
- **WHEN** a panic occurs
- **THEN** system catches panic, calls `opencb send "WARNING-NINE_FUT Sub-Daemon-Error"`, cleans up PID file, and exits with code 1

#### Scenario: opencb send failure
- **WHEN** opencb send command fails
- **THEN** system silently ignores the error and proceeds with exit
