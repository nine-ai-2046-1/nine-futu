## ADDED Requirements

### Requirement: TCP connection to FutuOpenD gateway
The system SHALL establish a TCP connection to a FutuOpenD gateway instance at a configurable host:port (default 127.0.0.1:11111).

#### Scenario: Successful connection
- **WHEN** the client initiates a connection to a running FutuOpenD gateway
- **THEN** a TCP connection is established and the client can send/receive data

#### Scenario: Connection refused
- **WHEN** the client attempts to connect to a non-running FutuOpenD gateway
- **THEN** the client returns a connection error with descriptive message

### Requirement: 48-byte header protocol
The system SHALL use the Futu 48-byte binary header format for all messages:
- Bytes 0-1: Header flag "FT" (0x46, 0x54)
- Bytes 2-5: Proto ID (uint32, little-endian)
- Byte 6: Proto format type (0=Protobuf, 1=JSON)
- Byte 7: Protocol version
- Bytes 8-11: Serial number (uint32, little-endian)
- Bytes 12-15: Body length (uint32, little-endian)
- Bytes 16-35: Body SHA1 hash (20 bytes)
- Bytes 36-43: Reserved (8 bytes)

#### Scenario: Send request with header
- **WHEN** the client sends a request message
- **THEN** the message is prepended with a 48-byte header containing the correct proto_id, serial_no, and body_len

#### Scenario: Receive response with header
- **WHEN** the client receives data from the gateway
- **THEN** the client parses the 48-byte header to extract proto_id, serial_no, and body_len before processing the body

### Requirement: Connection lifecycle management
The system SHALL manage the connection lifecycle including connect, connected, ready, closing, and closed states.

#### Scenario: State transitions
- **WHEN** the client connects
- **THEN** the state transitions through START → CONNECTING → CONNECTED → READY

#### Scenario: Graceful close
- **WHEN** the client calls close()
- **THEN** the connection is closed gracefully and the state becomes CLOSED

### Requirement: Keep-alive mechanism
The system SHALL send keep-alive messages at intervals based on the gateway's keep_alive_interval (using 4/5 of the server-specified interval).

#### Scenario: Regular keep-alive
- **WHEN** the keep-alive interval elapses without data exchange
- **THEN** the client sends a KeepAlive message (proto_id=1004)

#### Scenario: Connection timeout
- **WHEN** no data is received for 33 seconds (configurable)
- **THEN** the client closes the connection with KeepAliveFail reason

### Requirement: Auto-reconnect
The system SHALL automatically attempt to reconnect when the connection is lost unexpectedly.

#### Scenario: Reconnect on disconnect
- **WHEN** the connection drops due to network error
- **THEN** the client waits 6 seconds (configurable) and attempts to re-establish the connection

#### Scenario: Cancel reconnect
- **WHEN** the client explicitly calls close()
- **THEN** auto-reconnect is disabled and no reconnect attempt is made
