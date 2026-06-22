## Context

When no date is specified for minute K-line queries, the system defaults to today which returns 0 bars on weekends/holidays.

## Decisions

### 1. Auto-find Logic

**Decision**: Query previous days until data is found (max 7 days back).

**Rationale**: Most users expect to see recent data, not nothing.

### 2. K-line Type Restriction

**Decision**: Only applies to minute K-lines (1m, 3m, 5m, 15m, 30m, 60m).

**Rationale**: Daily/weekly/monthly already have fallback behavior.
