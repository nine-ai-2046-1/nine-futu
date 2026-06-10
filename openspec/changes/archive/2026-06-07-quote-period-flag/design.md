## Context

Traders often need historical data for a specific period (e.g., last 30 days). Currently, users must manually calculate the start date and pass it via `-s`. This is error-prone and tedious.

Additionally, for backtesting and simulation, users need to process data at a controlled pace. The `--delay` flag adds artificial delay between each output line, enabling real-time simulation of historical data.

## Goals / Non-Goals

**Goals:**
- Add `-p` flag to kline command for automatic period calculation
- Add `--delay` flag for delayed output between each data line
- Support common periods: 7, 14, 30, 60, 90, 180, 365 days

**Non-Goals:**
- Support other time units (weeks, months) - keep it simple with days

## Decisions

### 1. Period Flag Behavior

**Decision**: `-p` sets start date to N days before today (or before `--end` if provided).

**Rationale**: Simple and intuitive. `-p 30` means "last 30 days".

### 2. Delay Flag Behavior

**Decision**: `--delay N` adds N seconds delay between each NDJSON line output.

**Rationale**: Enables real-time simulation of historical data. Works with NDJSON output only.

**Example**:
```bash
# Output 10 daily bars, 60 seconds apart
nine-futu quote kline -c 700 -k 1d -p 10 --delay 60
```

### 3. Precedence

**Decision**: If both `-p` and `-s` are provided, `-p` takes precedence.

**Rationale**: `-p` is more explicit about intent.

## Risks / Trade-offs

### Risk: User Confusion
**Impact**: User might not understand that `-p` overrides `-s`
**Mitigation**: Document behavior clearly in help text

### Risk: Long Running Process
**Impact**: `--delay 3600` with 30 days of data = 30 hours runtime
**Mitigation**: User responsibility, document expected runtime
