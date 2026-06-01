## Why

The existing Python SDK (futu-api) for Futu OpenD gateway has several limitations for high-frequency quantitative trading scenarios: GIL-induced latency spikes, GC pauses affecting real-time data processing, and dependency-heavy deployment (pandas, protobuf, PyCryptodome). Rewriting in Rust eliminates these issues while providing zero-copy parsing, deterministic latency, and single-binary deployment. This also enables building a modern CLI tool optimized for both human operators and AI coding agents.

## What Changes

- **New Rust SDK library crate** (`nine-futu`): Complete rewrite of the Futu API client with async/await support via Tokio, zero-copy Protobuf parsing via Prost, and native AES/RSA encryption
- **New Rust CLI binary crate** (`nine-futu-cli`): Command-line interface for quote data, trading operations, and real-time monitoring, designed for both human and AI agent usage
- **MVP Scope (Quote First)**:
  - TCP connection to FutuOpenD gateway (48-byte header + Protobuf)
  - Connection initialization and keep-alive
  - Market snapshot, real-time quotes, K-line data, order book, ticker
  - Subscription management (subscribe/unsubscribe/query)
  - Real-time push data handling (quote, kline, orderbook, ticker)
  - JSON output mode for agent consumption
- **Future Scope (After MVP)**:
  - Trading APIs (place/modify/cancel order, portfolio, accounts)
  - Crypto trading support
  - All 66 API endpoints
  - Subscription quota management

## Capabilities

### New Capabilities

- `tcp-connection`: TCP client connecting to FutuOpenD gateway with 48-byte header protocol, connection lifecycle management, keep-alive, and auto-reconnect
- `protobuf-protocol`: Protobuf message serialization/deserialization for all Futu API message types (77 proto files), including request packing and response unpacking
- `encryption`: AES-ECB/CBC encryption and RSA key exchange for secure communication with FutuOpenD
- `quote-api`: Quote-related API implementations including snapshot, real-time quote, K-line, order book, ticker, and market state queries
- `subscription-management`: Subscribe/unsubscribe to real-time data streams with quota tracking and management
- `push-data-handler`: Async push data handling for real-time quote, kline, orderbook, and ticker updates
- `cli-framework`: CLI application with subcommands for quote, trade, subscribe, and monitor operations, with JSON output support

### Modified Capabilities

(No existing specs - this is a greenfield project)

## Impact

- **Code**: New Rust codebase replacing Python SDK; no modifications to existing Python code
- **Dependencies**: Python packages (pandas, protobuf, PyCryptodome, simplejson) replaced by Rust crates (tokio, prost, aes, rsa, clap)
- **Deployment**: Single static binary replacing Python + pip packages
- **API Compatibility**: Must maintain wire-level compatibility with FutuOpenD gateway (same TCP protocol, same Protobuf messages)
- **FutuOpenD**: No changes required; the gateway is language-agnostic
- **Performance**: Expected 10-100x reduction in P99 latency for quote parsing; zero GC pauses
