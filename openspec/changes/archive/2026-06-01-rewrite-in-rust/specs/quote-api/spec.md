## ADDED Requirements

### Requirement: Market snapshot query
The system SHALL provide a get_market_snapshot API that returns current market data for up to 400 securities.

#### Scenario: Single stock snapshot
- **WHEN** the client requests snapshot for HK.00700
- **THEN** the API returns price, volume, market_cap, PE, PB, and other fundamental data

#### Scenario: Batch snapshot
- **WHEN** the client requests snapshot for 100 stocks
- **THEN** the API returns snapshot data for all 100 stocks in a single response

#### Scenario: Exceed limit
- **WHEN** the client requests snapshot for more than 400 stocks
- **THEN** the API returns an error indicating the limit is exceeded

### Requirement: Real-time quote query
The system SHALL provide a get_stock_quote API that returns real-time quote data for subscribed securities.

#### Scenario: Quote for subscribed stock
- **WHEN** the client requests quote for a subscribed stock
- **THEN** the API returns bid/ask prices, last price, volume, and change data

#### Scenario: Quote for unsubscribed stock
- **WHEN** the client requests quote for a stock that is not subscribed
- **THEN** the API returns an error indicating the stock is not subscribed

### Requirement: K-line data query
The system SHALL provide APIs for both real-time and historical K-line data.

#### Scenario: Real-time K-line
- **WHEN** the client calls get_cur_kline with ktype=K_DAY and num=10
- **THEN** the API returns the last 10 daily K-line bars for the subscribed stock

#### Scenario: Historical K-line
- **WHEN** the client calls request_history_kline with start and end dates
- **THEN** the API returns K-line data within the specified date range (max 1000 per request)

#### Scenario: K-line with pagination
- **WHEN** the client requests more than 1000 bars
- **THEN** the API returns the first 1000 bars and a page_req_key for fetching more

### Requirement: Order book query
The system SHALL provide a get_order_book API that returns the order book (bid/ask levels).

#### Scenario: Order book depth
- **WHEN** the client requests order book with num=10
- **THEN** the API returns 10 levels of bid and ask prices with quantities

#### Scenario: Order book for crypto
- **WHEN** the client requests order book for a crypto pair
- **THEN** the API supports up to 40 levels (1/5/10/20/40)

### Requirement: Ticker (trades) query
The system SHALL provide a get_rt_ticker API that returns recent trades.

#### Scenario: Recent trades
- **WHEN** the client requests ticker with num=500
- **THEN** the API returns the last 500 trades with price, volume, and timestamp

### Requirement: Market state query
The system SHALL provide a get_market_state API that returns the current trading state of a security.

#### Scenario: Market state
- **WHEN** the client requests market state for HK.00700
- **THEN** the API returns the state (e.g., MORNING, AFTERNOON, CLOSED, etc.)

### Requirement: Capital flow query
The system SHALL provide a get_capital_flow API that returns capital flow data.

#### Scenario: Intraday capital flow
- **WHEN** the client requests capital flow with period_type=INTRADAY
- **THEN** the API returns intraday capital inflow/outflow data

### Requirement: Plate (sector) queries
The system SHALL provide APIs for plate list, plate stocks, and owner plate queries.

#### Scenario: Get plate list
- **WHEN** the client requests plate list for market HK
- **THEN** the API returns all plates (sectors/industries) for the market

#### Scenario: Get plate stocks
- **WHEN** the client requests stocks in plate HK.BK1107
- **THEN** the API returns all stocks belonging to that plate
