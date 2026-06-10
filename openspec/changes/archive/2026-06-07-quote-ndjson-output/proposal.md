## Why

The quote command currently outputs JSON arrays which require loading all data into memory. For trading applications and AI agents, NDJSON (one JSON object per line) is more efficient because:
- Each line can be processed independently
- Data can be streamed without buffering entire response
- Compatible with tools like `jq`, `grep`, and standard Unix pipelines
- Matches the output format used by the subscription data storage

## What Changes

- **BREAKING**: Change default output format from JSON array to NDJSON
- Add `--ndjson` flag for explicit NDJSON output (for backward compatibility)
- Add `--json` flag for JSON array output (current behavior)
- Update all quote subcommands: `snapshot`, `kline`

## Capabilities

### New Capabilities

(None - this is a modification of existing behavior)

### Modified Capabilities

- `quote-api`: Output format changed from JSON array to NDJSON by default

## Impact

- **Code**: Update main.rs quote command output formatting
- **CLI**: New `--json` flag for JSON array output
- **Breaking**: Default output format changes (NDJSON instead of JSON array)
- **Dependencies**: None
