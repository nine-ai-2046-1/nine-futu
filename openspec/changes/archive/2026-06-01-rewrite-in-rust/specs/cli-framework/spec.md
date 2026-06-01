## ADDED Requirements

### Requirement: Subcommand structure
The CLI SHALL organize commands into subcommands: `quote`, `subscribe`, `monitor`.

#### Scenario: Quote subcommand
- **WHEN** the user runs `nine-futu quote snapshot US.AAPL`
- **THEN** the CLI fetches and displays the market snapshot for US.AAPL

#### Scenario: Subscribe subcommand
- **WHEN** the user runs `nine-futu subscribe HK.00700 --types QUOTE ORDER_BOOK`
- **THEN** the CLI subscribes to the specified data types for HK.00700

#### Scenario: Monitor subcommand
- **WHEN** the user runs `nine-futu monitor HK.00700 --duration 60`
- **THEN** the CLI displays real-time updates for 60 seconds

### Requirement: JSON output mode
The CLI SHALL support a `--json` flag on all commands that outputs machine-readable JSON.

#### Scenario: JSON output
- **WHEN** the user runs `nine-futu quote snapshot US.AAPL --json`
- **THEN** the output is valid JSON containing the snapshot data

#### Scenario: Default output
- **WHEN** the user runs `nine-futu quote snapshot US.AAPL` without --json
- **THEN** the output is human-readable formatted text

### Requirement: Connection configuration
The CLI SHALL support configuring the FutuOpenD connection via flags or environment variables.

#### Scenario: Host and port flags
- **WHEN** the user runs `nine-futu --host 192.168.1.100 --port 11111 quote snapshot HK.00700`
- **THEN** the CLI connects to 192.168.1.100:11111

#### Scenario: Environment variables
- **WHEN** FUTU_HOST and FUTU_PORT environment variables are set
- **THEN** the CLI uses those values as defaults

### Requirement: Quote subcommands
The CLI SHALL provide the following quote subcommands:
- `snapshot`: Market snapshot (one or more stocks)
- `kline`: K-line data with --ktype and --num options
- `orderbook`: Order book depth
- `ticker`: Recent trades
- `quote`: Real-time quote (requires subscription)

#### Scenario: K-line with options
- **WHEN** the user runs `nine-futu quote kline HK.00700 --ktype 1d --num 20`
- **THEN** the CLI returns 20 daily K-line bars

#### Scenario: Order book depth
- **WHEN** the user runs `nine-futu quote orderbook HK.00700 --num 5`
- **THEN** the CLI returns 5 levels of bid/ask data

### Requirement: Subscribe subcommands
The CLI SHALL provide subscribe/unsubscribe/query operations.

#### Scenario: Subscribe
- **WHEN** the user runs `nine-futu subscribe HK.00700 --types QUOTE`
- **THEN** the CLI subscribes to quote data for HK.00700

#### Scenario: Unsubscribe all
- **WHEN** the user runs `nine-futu subscribe --unsubscribe-all`
- **THEN** all active subscriptions are cancelled

#### Scenario: Query subscription
- **WHEN** the user runs `nine-futu subscribe --query`
- **THEN** the CLI displays current subscription status

### Requirement: Agent-friendly output format
The CLI SHALL format output to be easily parseable by AI coding agents.

#### Scenario: Structured output
- **WHEN** the CLI outputs quote data
- **THEN** the output includes clearly labeled fields (stock code, price, volume, change, etc.)

#### Scenario: Error messages
- **WHEN** an error occurs
- **THEN** the CLI outputs a clear error message with error code and description in a parseable format
