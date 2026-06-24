## 1. CLI Flag Updates

- [x] 1.1 Add individual subscription flags to `Sub` command: `--quote`, `--orderbook`, `--ticker`, `--rtdata`, `--broker`
- [x] 1.2 Add `--output [path]` flag to `Sub` command with optional path argument
- [x] 1.3 Update CLI help text for all new flags

## 2. Output Mode Implementation

- [x] 2.1 Create `OutputMode` enum in types.rs: `Stdout`, `File(PathBuf)`, `FileDefault`
- [x] 2.2 Update `PushDataHandler` to accept and store `OutputMode`
- [x] 2.3 Implement stdout output mode (NDJSON to println)
- [x] 2.4 Implement file output mode (existing storage logic)
- [x] 2.5 Route output based on mode and data type

## 3. Kline Bar Completion Buffer

- [x] 3.1 Add `current_bar: Option<KlineBar>` and `current_time_key: Option<String>` to `PushDataHandler`
- [x] 3.2 Implement bar completion logic: buffer updates with same time_key, output when time_key changes
- [x] 3.3 Apply buffer only to minute klines (1m, 3m, 5m, 15m, 30m, 60m)
- [x] 3.4 Pass through non-kline data immediately (no buffering)

## 4. Subscription Type Routing

- [x] 4.1 Build subscription type list from individual flags in `run_subscription`
- [x] 4.2 Ensure kline type is always included regardless of other flags
- [x] 4.3 Handle `--all` flag to include all data types

## 5. Integration and Testing

- [x] 5.1 Wire output mode from CLI args to `PushDataHandler`
- [x] 5.2 Test stdout mode with pipe to jq
- [x] 5.3 Test file output with --output path
- [x] 5.4 Test kline bar completion buffer with 5m kline
- [x] 5.5 Verify non-kline data outputs immediately
