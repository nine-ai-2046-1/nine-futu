## Context

The Futu OpenD gateway exposes a TCP-based API using a custom 48-byte binary header followed by Protobuf or JSON payloads. The existing Python SDK connects to this gateway, handles encryption (AES/RSA), and provides synchronous/async APIs for quote data and trading. The SDK requires FutuOpenD running locally (default 127.0.0.1:11111).

Key technical constraints:
- Wire protocol: 48-byte header (2-byte flag "FT" + 4-byte proto_id + 1-byte fmt + 1-byte ver + 4-byte serial + 4-byte body_len + 20-byte SHA1 + 8-byte reserved)
- 77 Protobuf message definitions covering all API endpoints
- Encryption: Optional AES-ECB/CBC with RSA key exchange
- Gateway is language-agnostic; only the SDK needs rewriting

## Goals / Non-Goals

**Goals:**
- Complete Rust rewrite of the Futu API SDK as a library crate (`nine-futu`)
- CLI tool (`nine-futu-cli`) for interactive and scriptable use
- MVP scope: Quote APIs, subscription management, push data handling
- Zero-copy Protobuf parsing where possible
- Async/await architecture using Tokio
- JSON output mode for AI agent consumption
- Wire-level compatibility with existing FutuOpenD gateway

**Non-Goals:**
- Trading APIs (deferred to post-MVP)
- FFI bindings for Python/other languages
- FutuOpenD gateway modification
- Support for Python SDK feature parity in MVP (focus on quote first)

## Decisions

### 1. Workspace Structure: Two Crates

**Decision**: Use Cargo workspace with `nine-futu` (lib) and `nine-futu-cli` (bin).

**Rationale**: Separation of concerns allows the library to be used independently (e.g., for custom trading bots) while the CLI provides a user-friendly interface. The workspace enables shared dependencies and unified testing.

**Alternatives considered**:
- Single crate: Rejected because it would prevent library reuse
- Three crates (lib + cli + proto): Rejected because prost-build handles proto generation within the lib crate

### 2. Async Runtime: Tokio

**Decision**: Use Tokio as the async runtime with `tokio::net::TcpStream`.

**Rationale**: Tokio is the de facto standard for async Rust networking. It provides excellent TCP support, timers for keep-alive, and channels for push data handling. The existing Python code uses selectors + threads, which maps naturally to Tokio's poll-based model.

**Alternatives considered**:
- async-std: Less mature ecosystem, fewer middleware options
- smol: Too minimal for this use case

### 3. Protobuf: Prost

**Decision**: Use `prost` for Protobuf encoding/decoding with `prost-build` for code generation.

**Rationale**: Prost generates clean, idiomatic Rust types from .proto files. It supports proto2 syntax (used by Futu) and integrates well with Tokio via the `prost` crate. The 77 proto files can be compiled at build time.

**Alternatives considered**:
- protobuf (rust-protobuf): More complex API, less idiomatic
- protobuf-codegen: Older, less maintained

### 4. Encryption: Pure Rust Crates

**Decision**: Use `aes`, `rsa`, and `sha1` crates for encryption.

**Rationale**: The encryption module is relatively simple (AES-ECB/CBC for packet encryption, RSA for key exchange). Pure Rust implementations avoid C dependency issues and provide good performance.

**Alternatives considered**:
- OpenSSL bindings (openssl-sys): Heavier dependency, C bindings
- ring: More complex API, not needed for this use case

### 5. CLI Framework: Clap

**Decision**: Use `clap` v4 with derive macros for CLI argument parsing.

**Rationale**: Clap is the standard for Rust CLI tools. Derive macros reduce boilerplate. It supports subcommands natively, which maps well to the `quote`, `trade`, `subscribe` command structure.

**Alternatives considered**:
- structopt: Deprecated in favor of clap v4
- argh: Less feature-rich

### 6. Error Handling: Thiserror + Anyhow

**Decision**: Use `thiserror` for library error types and `anyhow` for CLI error handling.

**Rationale**: `thiserror` provides clean error type definitions with derive macros. `anyhow` simplifies error propagation in the CLI without requiring custom error types everywhere.

**Alternatives considered**:
- eyre: Similar to anyhow, less widely adopted
- Manual error types: Too verbose for this scope

### 7. JSON Output: Serde

**Decision**: Use `serde` + `serde_json` for JSON serialization of API responses.

**Rationale**: The CLI needs `--json` output for agent consumption. Serde derives make it trivial to add `Serialize` to all response types. This is a hard requirement for the "good-for-coding-agent" CLI style.

**Alternatives considered**:
- Manual JSON construction: Too error-prone
- simd-json: Overkill for this use case

### 8. Connection Architecture: Channel-Based Push

**Decision**: Use `tokio::sync::mpsc` channels for push data delivery from the network layer to application handlers.

**Rationale**: The existing Python code uses a callback executor pattern. In Rust, channels are more idiomatic and provide natural backpressure. The network task reads data, parses headers, and sends push messages through channels to registered handlers.

**Alternatives considered**:
- Callback traits: More complex lifetime management
- Broadcast channels: Unnecessary for single-consumer use case

## Risks / Trade-offs

### Risk: Protobuf Compatibility
- **Risk**: Prost might handle some proto2 features differently than Python protobuf
- **Mitigation**: Test with actual FutuOpenD responses; implement proto compatibility tests

### Risk: Encryption Edge Cases
- **Risk**: AES/RSA implementation differences could cause connection failures
- **Mitigation**: Use well-tested crates (aes, rsa); add encryption unit tests with known vectors

### Risk: Missing API Coverage
- **Risk**: MVP only covers quote APIs; trading APIs deferred
- **Mitigation**: Design the library architecture to be extensible; trading APIs follow the same pattern

### Risk: FutuOpenD Version Compatibility
- **Risk**: Gateway protocol might change in future versions
- **Mitigation**: Pin to specific protocol version; add version negotiation in InitConnect

### Trade-off: Performance vs. Safety
- **Choice**: Prioritize safety (Rust's type system) over micro-optimizations
- **Rationale**: The bottleneck is network I/O, not CPU parsing. Zero-copy Protobuf gives most of the performance benefit without unsafe code.

### Trade-off: Completeness vs. Ship Date
- **Choice**: MVP with 35 quote APIs first, trading later
- **Rationale**: Quote APIs are the most used and provide immediate value. Trading can be added incrementally.
