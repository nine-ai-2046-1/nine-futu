## Context

The `sub` command in nine-futu subscribes to real-time data from FutuOpenD. Currently:
- Only `--all` flag exists for non-kline data types
- All data is saved to files automatically
- Kline push data outputs every update within a timeframe period

The codebase uses:
- `main.rs` for CLI parsing (clap)
- `push_handler.rs` for processing push data
- `storage.rs` for file output
- `types.rs` for SubType definitions

## Goals / Non-Goals

**Goals:**
- Allow granular subscription to specific data types via individual flags
- Default to stdout output for pipe-friendly behavior
- Add `--output` flag to opt-in to file storage
- Buffer kline data to only output completed timeframe bars (minute klines only)

**Non-Goals:**
- Daily/weekly/monthly kline bar completion (not used in practice)
- Handling incomplete bars on process exit (killed by system/error/user)
- Changing the `--all` flag behavior (still subscribes to all types)

## Decisions

### 1. Individual Subscription Flags

**Decision**: Add `--quote`, `--orderbook`, `--ticker`, `--rtdata`, `--broker` flags.

**Rationale**: More intuitive than comma-separated list. Users can see all options in `--help`. Consistent with existing flag patterns in the CLI.

**Alternative considered**: Single `--types quote,orderbook` flag — rejected because it's harder to discover options and requires parsing.

### 2. Output Mode Default

**Decision**: Default to stdout (NDJSON). Add `--output [path]` flag for file storage.

**Rationale**: Most CLI tools default to stdout for composability. Users can pipe to other tools: `nine-futu sub -c 700 | jq .`

**Alternative considered**: Keep file output as default — rejected because it's not pipe-friendly.

### 3. Kline Bar Completion Buffer

**Decision**: Buffer kline data and only output when timeframe boundary is crossed (time_key changes).

**Rationale**: FutuOpenD sends updates within a bar period with the same time_key (bar start time). When time_key changes, the previous bar is complete.

**Buffer logic:**
```
incoming time_key == buffered time_key?
  YES → update buffer with latest data
  NO  → output buffer, start new buffer
```

**Scope**: Only minute klines (1m, 3m, 5m, 15m, 30m, 60m). Daily+ klines are not used with sub command.

### 4. Output Routing Architecture

**Decision**: Add output mode enum to PushDataHandler. Route based on mode and data type.

```
enum OutputMode {
    Stdout,              // --output not used
    File(PathBuf),       // --output <path>
    FileDefault,         // --output ""
}
```

- Kline data: goes through bar completion buffer, then to output
- Other data: immediate output (no buffering needed)

## Risks / Trade-offs

**[Risk]** Stdout mode may produce interleaved output with multiple data types
**→ Mitigation**: Each line is valid NDJSON. Consumers should handle line-delimited input.

**[Risk]** Bar completion buffer adds latency (outputs bar after next bar starts)
**→ Mitigation**: This is intentional — ensures only complete bars are output. Users who need raw data can use `quote kline` command.

**[Risk]** Breaking change for users relying on file output by default
**→ Mitigation**: Document migration. Use `--output ""` to restore old behavior.

## Migration Plan

1. Update CLI help text to document new flags
2. Users who relied on file output must add `--output ""` to restore old behavior
3. No data format changes — existing NDJSON format preserved
