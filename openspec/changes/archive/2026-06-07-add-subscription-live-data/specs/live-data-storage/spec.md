## ADDED Requirements

### Requirement: Store live data as NDJSON files
The system SHALL store real-time data as NDJSON (newline-delimited JSON) files.

#### Scenario: Store quote data
- **WHEN** quote data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/quote.txt`

#### Scenario: Store order book data
- **WHEN** order book data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/orderbook.txt`

#### Scenario: Store ticker data
- **WHEN** ticker data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/ticker.txt`

#### Scenario: Store broker data
- **WHEN** broker queue data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/broker.txt`

#### Scenario: Store RT data
- **WHEN** real-time data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/rt_data.txt`

### Requirement: Store K-line data with timeframe
The system SHALL store K-line data in timeframe-specific subfolders.

#### Scenario: Store 5-minute K-line
- **WHEN** 5-minute K-line data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/5m/kline.txt`

#### Scenario: Store 15-minute K-line
- **WHEN** 15-minute K-line data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/15m/kline.txt`

#### Scenario: Store daily K-line
- **WHEN** daily K-line data is received for HK.00700
- **THEN** system appends JSON line to `~/.opens/nine-futu/data/live/HK.00700/{YYYY-MM-DD}/1d/kline.txt`

### Requirement: Create directory structure
The system SHALL create directory structure if it does not exist.

#### Scenario: New code subscription
- **WHEN** subscribing to a new stock code
- **THEN** system creates `~/.opens/nine-futu/data/live/{code}/` directory

#### Scenario: New date
- **WHEN** data arrives for a new date
- **THEN** system creates `~/.opens/nine-futu/data/live/{code}/{YYYY-MM-DD}/` directory

#### Scenario: New timeframe
- **WHEN** K-line data arrives for a new timeframe
- **THEN** system creates `~/.opens/nine-futu/data/live/{code}/{YYYY-MM-DD}/{timeframe}/` directory

### Requirement: Data file naming
The system SHALL use consistent file naming for each data type.

#### Scenario: File names
- **WHEN** data is stored
- **THEN** system uses these file names:
  - Quote: `quote.txt`
  - Order book: `orderbook.txt`
  - Ticker: `ticker.txt`
  - Broker: `broker.txt`
  - RT data: `rt_data.txt`
  - K-line: `kline.txt` (inside timeframe subfolder)
