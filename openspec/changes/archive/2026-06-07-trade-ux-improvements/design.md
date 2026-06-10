## Context

Users need to interact with trade commands using human-readable strings instead of numeric codes. The system should handle the conversion internally.

## Decisions

### 1. Trade Environment

**Decision**: Default to Simulate, use `--real` flag for Real.

**Mapping**:
- "sim" / "simulate" / default → TrdEnv=0
- "real" → TrdEnv=1 (requires config enabled)

### 2. Account Type

**Decision**: Default to Cash, use `--margin` flag for Margin.

**Mapping**:
- "cash" / default → TrdAccType=1
- "margin" → TrdAccType=2

### 3. Order Status Display

**Decision**: Show human-readable descriptions.

**Mapping**:
- -1 → "Unknown"
- 1 → "Waiting Submit"
- 2 → "Submitting"
- 5 → "Submitted"
- 10 → "Partially Filled"
- 11 → "Filled"
- 14 → "Partially Cancelled"
- 15 → "Cancelled"
- 21 → "Failed"
- 22 → "Disabled"
- 23 → "Deleted"
