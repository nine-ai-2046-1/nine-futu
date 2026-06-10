## Context

Users need to integrate nine-futu with external tools for processing each data point. A trading bot ("nine-stock") needs to process each K-line bar as it arrives for backtesting or live trading decisions.

## Goals / Non-Goals

**Goals:**
- Add `--cli` flag to `quote kline` and `sub` commands
- Call `nine-stock --ktype "{timeframe}" --data "{json}"` for each K-line bar
- Handle subprocess errors gracefully (output error, continue processing)
- Only trigger CLI callback for K-line data

**Non-Goals:**
- Implement the `nine-stock` binary (external tool)
- Support CLI callback for quote, orderbook, etc. (K-line only)
- Pass additional context to the subprocess

## Decisions

### 1. Subprocess Execution

**Decision**: Use `std::process::Command` with `.arg()` for argument passing.

**Rationale**: Rust's Command handles argument escaping automatically, avoiding shell injection issues. The JSON string is passed as a single argument.

**Implementation**:
```rust
std::process::Command::new("nine-stock")
    .arg("--session").arg(&session_id)
    .arg("--code").arg(&stock_code)
    .arg("--ktype").arg(&ktype)
    .arg("--data").arg(&json_string)
    .output()
```

### 2. Error Handling

**Decision**: Capture subprocess output, print errors to stderr, continue processing.

**Rationale**: External tool failures should not crash nine-futu. Errors are logged but processing continues.

### 3. Output Capturing

**Decision**: Capture stdout and stderr from subprocess, but don't display them (only errors).

**Rationale**: The external tool's output is not needed for nine-futu's operation. Only errors matter.

### 4. Binary Path

**Decision**: Assume `nine-stock` is in PATH.

**Rationale**: Standard Unix convention. Users can install the binary anywhere in PATH.

## Risks / Trade-offs

### Risk: Subprocess Overhead
**Impact**: Spawning a process for each K-line bar adds latency
**Mitigation**: Acceptable for the use case; subprocess is lightweight

### Risk: Shell Injection
**Impact**: Malicious JSON could cause issues
**Mitigation**: Use `std::process::Command` with `.arg()` (no shell involved)
