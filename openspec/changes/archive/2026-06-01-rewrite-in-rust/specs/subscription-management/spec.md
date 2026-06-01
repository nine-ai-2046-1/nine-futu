## ADDED Requirements

### Requirement: Subscribe to data streams
The system SHALL allow subscribing to real-time data streams for specific securities and data types.

#### Scenario: Subscribe to quote
- **WHEN** the client subscribes to HK.00700 with SubType QUOTE
- **THEN** the client receives real-time quote updates for HK.00700

#### Scenario: Multiple types
- **WHEN** the client subscribes to HK.00700 with SubType [QUOTE, ORDER_BOOK, TICKER]
- **THEN** the client receives all three types of updates

#### Scenario: Multiple stocks
- **WHEN** the client subscribes to [HK.00700, US.AAPL] with SubType QUOTE
- **THEN** the client receives quote updates for both stocks

### Requirement: Unsubscribe from data streams
The system SHALL allow unsubscribing from specific or all data streams.

#### Scenario: Unsubscribe specific
- **WHEN** the client unsubscribes from HK.00700 with SubType QUOTE
- **THEN** the client no longer receives quote updates for HK.00700

#### Scenario: Unsubscribe all
- **WHEN** the client calls unsubscribe_all()
- **THEN** all active subscriptions are cancelled

#### Scenario: Unsubscribe timing
- **WHEN** the client attempts to unsubscribe within 1 minute of subscribing
- **THEN** the unsubscribe request is rejected with an appropriate error

### Requirement: Query subscription status
The system SHALL provide a query_subscription API that returns current subscription state.

#### Scenario: Query all connections
- **WHEN** the client queries subscription status with is_all_conn=true
- **THEN** the API returns subscriptions across all active connections

#### Scenario: Query current connection
- **WHEN** the client queries subscription status with is_all_conn=false
- **THEN** the API returns only subscriptions for the current connection

### Requirement: Subscription quota management
The system SHALL track and enforce subscription quotas based on user tier.

#### Scenario: Check quota before subscribe
- **WHEN** the client attempts to subscribe to a new data stream
- **THEN** the system checks if the user has available subscription quota

#### Scenario: Quota exceeded
- **WHEN** the user's subscription quota is fully used
- **THEN** new subscription attempts fail with a quota exceeded error

### Requirement: Push callback registration
The system SHALL allow registering callback handlers for different push data types.

#### Scenario: Register quote handler
- **WHEN** the client registers a StockQuoteHandlerBase
- **THEN** all incoming quote push data is delivered to that handler

#### Scenario: Register multiple handlers
- **WHEN** the client registers handlers for quote, orderbook, and ticker
- **THEN** each type of push data is delivered to its respective handler
