## Why

When no date is specified for minute K-line queries, the system defaults to today which returns 0 bars on weekends/holidays. Users expect to see the last trading day's data automatically.

## What Changes

- Auto-find last trading day when no date is specified for minute K-line
- Query previous days until data is found (max 7 days back)
- Keep current behavior for daily/weekly/monthly K-lines

## Capabilities

### New Capabilities

(None - modification of existing behavior)

### Modified Capabilities

- `quote-api`: Auto-find last trading day for minute K-line

## Impact

- **Code**: Update kline command logic
- **CLI**: No flag changes needed
