## Context

US stocks have extended trading hours (pre-market, after-hours). The Futu API supports this via `extended_time` parameter.

## Decisions

### 1. Default Behavior

**Decision**: Default to regular trading hours only.

**Rationale**: Most users only need regular hours data.

### 2. --extended Flag

**Decision**: Add `--extended` flag to enable extended hours.

**Rationale**: Explicit opt-in for extended data.

### 3. K-line Type Restriction

**Decision**: Extended hours only for 60 minutes and below.

**Rationale**: Futu API limitation.
