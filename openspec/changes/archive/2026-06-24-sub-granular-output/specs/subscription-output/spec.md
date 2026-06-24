## ADDED Requirements

### Requirement: Output mode control
The system SHALL support configurable output destinations for subscription data.

#### Scenario: Default stdout mode
- **WHEN** user runs `nine-futu sub -c 700 -t 5m` without --output flag
- **THEN** all subscription data is output to stdout as NDJSON (one JSON object per line)

#### Scenario: File output with path
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --output /data/stocks`
- **THEN** data is saved to `/data/stocks/HK.00700/2026-06-24/5m/kline.txt`

#### Scenario: File output at default path
- **WHEN** user runs `nine-futu sub -c 700 -t 5m --output ""`
- **THEN** data is saved to default path `~/.opens/nine-futu/data/live/HK.00700/2026-06-24/5m/kline.txt`

#### Scenario: File output structure
- **WHEN** using --output flag
- **THEN** file structure follows: `<output_path>/<code>/<date>/<timeframe>/kline.txt` for kline, `<output_path>/<code>/<date>/<data_type>.txt` for other types

### Requirement: Stdout output format
The system SHALL output subscription data in NDJSON format for pipe compatibility.

#### Scenario: Kline stdout format
- **WHEN** kline data is output to stdout
- **THEN** each line is a valid JSON object with fields: code, ktype, date, time, open, high, low, close, volume, turnover

#### Scenario: Quote stdout format
- **WHEN** quote data is output to stdout
- **THEN** each line is a valid JSON object with fields: code, last_done, prev_close_price, open_price, high_price, low_price, volume, turnover, change_val, change_rate, amplitude, pe_ratio, yield_rate

#### Scenario: Pipe to jq
- **WHEN** user pipes output to `jq .`
- **THEN** each line is processed as a separate JSON object

### Requirement: Non-kline data output
The system SHALL output non-kline subscription data immediately (no buffering).

#### Scenario: Quote immediate output
- **WHEN** subscribing to --quote and receiving quote updates
- **THEN** each update is output immediately to the configured destination

#### Scenario: OrderBook immediate output
- **WHEN** subscribing to --orderbook and receiving orderbook updates
- **THEN** each update is output immediately to the configured destination
