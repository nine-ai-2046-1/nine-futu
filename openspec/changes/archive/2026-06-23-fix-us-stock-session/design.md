## Context

US stocks require `session=Session.ALL` in subscription requests to work properly.

## Decisions

### 1. Session Parameter

**Decision**: Use `Session_ALL = 3` for all subscriptions.

**Rationale**: Simplifies the API and ensures compatibility with all markets.
