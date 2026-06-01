## 1. Project Setup

- [x] 1.1 Create Cargo workspace with nine-futu (lib) and nine-futu-cli (bin) crates
- [x] 1.2 Add core dependencies: tokio, prost, prost-build, aes, rsa, sha1, clap, serde, serde_json, thiserror, anyhow, bytes, chrono
- [x] 1.3 Copy all 77 .proto files from futu/common/pb/ to nine-futu/proto/
- [x] 1.4 Create build.rs for prost-build proto compilation
- [x] 1.5 Create basic module structure: src/lib.rs, src/net.rs, src/proto.rs, src/quote.rs, src/trade.rs, src/types.rs, src/error.rs

## 2. Protocol Layer

- [x] 2.1 Implement 48-byte header parser (FutuHeader struct with parse/serialize methods)
- [x] 2.2 Implement serial number manager with auto-increment and overflow handling
- [x] 2.3 Implement request packer (ProtoRequest with proto_id, serial_no, body bytes)
- [x] 2.4 Implement response unpacker (parse header, extract body, route to correct handler)
- [x] 2.5 Implement push message detection (check proto_id against All_PushId list)

## 3. TCP Connection

- [x] 3.1 Implement TcpConnection struct with Tokio TcpStream
- [x] 3.2 Implement connection state machine (Start, Connecting, Connected, Ready, Closing, Closed)
- [x] 3.3 Implement connect() with timeout support
- [x] 3.4 Implement send() for sending requests
- [x] 3.5 Implement receive loop for reading responses
- [x] 3.6 Implement keep-alive timer (4/5 of server interval)
- [x] 3.7 Implement connection timeout detection (33 seconds no data)
- [x] 3.8 Implement auto-reconnect with configurable interval (6 seconds)

## 4. Encryption

- [x] 4.1 Implement AES-ECB encryption/decryption
- [x] 4.2 Implement AES-CBC encryption/decryption
- [x] 4.3 Implement RSA key exchange (encrypt AES key with server's RSA public key)
- [x] 4.4 Implement PacketEncAlgo selection (FTAES_ECB, AES_ECB, AES_CBC)
- [x] 4.5 Implement SHA1 hash computation for header

## 5. Connection Context

- [x] 5.1 Implement OpenContextBase trait with common connection logic
- [x] 5.2 Implement InitConnect protocol (proto_id=1001) with client_ver, client_id, recv_notify, is_encrypt, push_proto_fmt
- [x] 5.3 Implement connection ID management (opend_conn_id from InitConnect response)
- [x] 5.4 Implement request-response matching (ReqInfo with serial_no, timeout, event)
- [x] 5.5 Implement sync query processor (send request, wait for response with timeout)
- [x] 5.6 Implement async query processor (send request, return immediately)
- [x] 5.7 Implement keep_alive protocol (proto_id=1004)
- [x] 5.8 Implement GetGlobalState protocol (proto_id=1002)

## 6. Quote API

- [x] 6.1 Implement OpenQuoteContext struct
- [x] 6.2 Implement get_market_snapshot (proto_id=3203, max 400 stocks)
- [x] 6.3 Implement get_stock_quote (proto_id=3004, requires subscription)
- [x] 6.4 Implement get_cur_kline (proto_id=3006, requires subscription)
- [x] 6.5 Implement request_history_kline (proto_id=3103, with pagination)
- [x] 6.6 Implement get_order_book (proto_id=3012, requires subscription)
- [x] 6.7 Implement get_rt_ticker (proto_id=3010, requires subscription)
- [x] 6.8 Implement get_market_state (proto_id=3223)
- [x] 6.9 Implement get_capital_flow (proto_id=3211)
- [x] 6.10 Implement get_plate_list (proto_id=3204)
- [x] 6.11 Implement get_plate_stock (proto_id=3205)
- [x] 6.12 Implement get_stock_basicinfo (proto_id=3202)
- [x] 6.13 Implement get_history_kl_quota (proto_id=3104)

## 7. Subscription Management

- [x] 7.1 Implement subscribe (proto_id=3001) with code_list and subtype_list
- [x] 7.2 Implement unsubscribe (proto_id=3001) with unsubscribe flag
- [x] 7.3 Implement unsubscribe_all
- [x] 7.4 Implement query_subscription (proto_id=3003)
- [x] 7.5 Implement subscription quota tracking

## 8. Push Data Handler

- [x] 8.1 Implement push data channel (tokio::sync::mpsc)
- [x] 8.2 Implement StockQuoteHandlerBase trait and delivery for proto_id=3005
- [x] 8.3 Implement CurKlineHandlerBase trait and delivery for proto_id=3007
- [x] 8.4 Implement OrderBookHandlerBase trait and delivery for proto_id=3013
- [x] 8.5 Implement TickerHandlerBase trait and delivery for proto_id=3011
- [x] 8.6 Implement backpressure handling (drop oldest when channel full)

## 9. Error Handling

- [x] 9.1 Define FutuError enum with thiserror (ConnectionLost, Timeout, NotConnected, etc.)
- [x] 9.2 Define error codes matching Python SDK (0-999 general, 1000-1999 quote, 2000-2999 trade)
- [x] 9.3 Implement error conversion from proto ret_type to FutuError
- [x] 9.4 Implement Display trait for user-friendly error messages

## 10. Type Definitions

- [x] 10.1 Define Rust enums for Market, SecurityType, SubType, KLType, TrdEnv, TrdSide, OrderType
- [x] 10.2 Define StockQuote, KlineBar, OrderBook, Ticker structs matching proto definitions
- [x] 10.3 Define SnapshotData, MarketState, CapitalFlow structs
- [x] 10.4 Implement From<proto::xxx> conversions for all types

## 11. CLI Framework

- [x] 11.1 Set up clap v4 with derive macros
- [x] 11.2 Implement main CLI structure with global options (--host, --port, --json)
- [x] 11.3 Implement quote subcommand group (snapshot, kline, orderbook, ticker, quote)
- [x] 11.4 Implement subscribe subcommand group (subscribe, unsubscribe, query)
- [x] 11.5 Implement monitor subcommand for real-time data display
- [x] 11.6 Implement JSON output mode (--json flag on all commands)
- [x] 11.7 Implement human-readable output formatting (tables, colors)

## 12. Testing

- [x] 12.1 Write unit tests for header parsing/serialization
- [x] 12.2 Write unit tests for protobuf packing/unpacking
- [x] 12.3 Write integration tests for connection lifecycle (requires FutuOpenD)
- [x] 12.4 Write integration tests for quote APIs (requires FutuOpenD + account)
- [x] 12.5 Write integration tests for subscription and push data (requires FutuOpenD)

## 13. Documentation

- [x] 13.1 Write README.md with installation, usage examples, and API reference
- [x] 13.2 Write CLI help text for all commands and subcommands
- [x] 13.3 Add doc comments to all public types and functions
