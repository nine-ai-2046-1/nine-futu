## Context

The `sub` command currently subscribes to all data types. Users typically only need K-line data.

## Decisions

### 1. Default Subscription

**Decision**: Default to K-line only (based on `-t` timeframe).

**Rationale**: Most users use subscription for K-line data collection.

### 2. --all Flag

**Decision**: Add `--all` flag to subscribe to all data types.

**Rationale**: Some users need all data types for comprehensive monitoring.
