## Context

The quote command currently outputs JSON arrays for all query results. While JSON is machine-readable, it has limitations:
- Requires loading entire response into memory before processing
- Not suitable for streaming large datasets
- Incompatible with standard Unix text processing tools

NDJSON (newline-delimited JSON) solves these issues by outputting one JSON object per line.

## Goals / Non-Goals

**Goals:**
- Change default output format to NDJSON
- Maintain backward compatibility with `--json` flag
- Apply to all quote subcommands (snapshot, kline)

**Non-Goals:**
- Modify subscription data storage (already uses NDJSON)
- Change the data structure of JSON objects

## Decisions

### 1. Default Output Format

**Decision**: NDJSON is the default output format.

**Rationale**: NDJSON is more suitable for:
- Streaming data to files
- Processing with Unix tools (`head`, `tail`, `grep`, `jq`)
- AI agent consumption (line-by-line processing)

### 2. Backward Compatibility

**Decision**: Add `--json` flag to restore JSON array output.

**Rationale**: Some users may depend on JSON array format. The `--json` flag preserves this capability.

### 3. Flag Naming

**Decision**: Use `--json` for JSON array output, no flag for NDJSON.

**Rationale**: NDJSON is the new default, so no flag needed. `--json` explicitly requests the old format.

## Risks / Trade-offs

### Risk: Breaking Change
**Impact**: Scripts relying on JSON array output will break
**Mitigation**: Document in release notes, `--json` flag available

### Risk: Tool Compatibility
**Impact**: Some tools may not support NDJSON
**Mitigation**: NDJSON is widely supported (jq, grep, awk, etc.)
