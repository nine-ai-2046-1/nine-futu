## 1. Protocol & Client Updates

- [x] 1.1 Implement actual protobuf encoding for subscription requests (proto_id=3001)
- [x] 1.2 Implement actual protobuf decoding for subscription responses
- [x] 1.3 Implement actual protobuf decoding for push data (BasicQot, OrderBook, KLine, Ticker, etc.)
- [x] 1.4 Update FutuClient to expose subscription context

## 2. Storage Module

- [x] 2.1 Create storage module with path construction logic
- [x] 2.2 Implement directory creation for code/date/timeframe structure
- [x] 2.3 Implement NDJSON file append functionality
- [x] 2.4 Implement file handle management for multiple data types

## 3. Data Parser Module

- [x] 3.1 Create data parser module for proto-to-JSON conversion
- [x] 3.2 Implement BasicQot to JSON conversion (quote)
- [x] 3.3 Implement OrderBook to JSON conversion
- [x] 3.4 Implement KLine to JSON conversion
- [x] 3.5 Implement Ticker to JSON conversion
- [x] 3.6 Implement RTData to JSON conversion
- [x] 3.7 Implement Broker to JSON conversion

## 4. Push Handler Updates

- [x] 4.1 Update PushDataHandler to parse actual proto data
- [x] 4.2 Add storage integration to PushDataHandler
- [x] 4.3 Add all push type handlers (QUOTE, ORDER_BOOK, TICKER, RT_DATA, BROKER)
- [x] 4.4 Add K-line push handler with timeframe support

## 5. Process Management

- [x] 5.1 Implement PID file creation with format: {pid}\n{timeframe}\n{start_time}
- [x] 5.2 Implement PID file removal on exit
- [x] 5.3 Implement process list command (scan pid/*.pid)
- [x] 5.4 Implement process stop command (kill + cleanup)
- [x] 5.5 Implement process status command (check PID or return -1)

## 6. Daemon Mode

- [x] 6.1 Implement daemon fork using daemonize crate or manual fork+setsid
- [x] 6.2 Implement PID file locking to prevent duplicate subscriptions
- [x] 6.3 Implement graceful shutdown on SIGTERM/SIGINT

## 7. Error Handling & Notifications

- [x] 7.1 Implement TRY/CATCH wrapper for daemon process
- [x] 7.2 Implement reconnection logic (max 3 retries, 5s delay)
- [x] 7.3 Implement opencb notification on connection failure
- [x] 7.4 Implement opencb notification on runtime error
- [x] 7.5 Implement PID cleanup on exit (normal or error)

## 8. CLI Commands

- [x] 8.1 Add `sub` command with -c, -tf, -fe flags
- [x] 8.2 Add `process` command group with list, stop, status subcommands
- [x] 8.3 Add `clean` command with -d and -y flags
- [x] 8.4 Implement clean command folder scanning and moving logic
- [x] 8.5 Implement clean command confirmation prompt

## 9. Integration & Testing

- [x] 9.1 Test subscription flow with FutuOpenD
- [x] 9.2 Test daemon mode and PID management
- [x] 9.3 Test data storage and file creation
- [x] 9.4 Test error handling and opencb notifications
- [x] 9.5 Test clean command with old data folders
