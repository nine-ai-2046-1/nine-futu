## ADDED Requirements

### Requirement: Async push data reception
The system SHALL receive and process push data asynchronously without blocking the main application thread.

#### Scenario: Non-blocking push reception
- **WHEN** push data arrives from the gateway
- **THEN** the data is parsed and delivered to handlers without blocking other operations

#### Scenario: High-frequency push
- **WHEN** multiple push messages arrive in rapid succession
- **THEN** all messages are processed in order without loss

### Requirement: Push message parsing
The system SHALL parse incoming push messages by extracting the proto_id and deserializing the body.

#### Scenario: Parse quote push
- **WHEN** a push message with proto_id=3005 (Qot_UpdateBasicQot) arrives
- **THEN** the body is deserialized to the corresponding Rust type and delivered to the quote handler

#### Scenario: Unknown push type
- **WHEN** a push message with an unrecognized proto_id arrives
- **THEN** the message is logged and ignored (not an error)

### Requirement: Channel-based delivery
The system SHALL use Tokio mpsc channels for delivering parsed push data to application handlers.

#### Scenario: Channel delivery
- **WHEN** a push message is parsed
- **THEN** it is sent through an mpsc channel to the registered handler

#### Scenario: Backpressure
- **WHEN** the channel buffer is full
- **THEN** the oldest messages are dropped (with logging) to prevent blocking

### Requirement: Push data types for MVP
The system SHALL support the following push data types in MVP:
- Qot_UpdateBasicQot (3005): Real-time quote updates
- Qot_UpdateKL (3007): K-line updates
- Qot_UpdateOrderBook (3013): Order book updates
- Qot_UpdateTicker (3011): Ticker (trade) updates

#### Scenario: Quote push
- **WHEN** the gateway sends a quote update
- **THEN** the system delivers it as a StockQuoteData struct to the quote handler

#### Scenario: K-line push
- **WHEN** the gateway sends a K-line update
- **THEN** the system delivers it as a KlineUpdate struct to the kline handler
