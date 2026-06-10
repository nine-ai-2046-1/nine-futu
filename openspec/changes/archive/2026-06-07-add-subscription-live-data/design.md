## Context

The nine-futu CLI currently only supports one-shot queries (snapshot, kline history). Traders need continuous real-time data monitoring and storage for:

- Live market monitoring during trading hours
- Data collection for backtesting strategies
- Alert-based trading via notification integration

The FutuOpenD gateway supports push-based data delivery via subscription, which is more efficient than polling.

## Goals / Non-Goals

**Goals:**
- Subscribe to real-time data streams (Quote, OrderBook, K-line, Ticker, RTData, Broker)
- Store live data as NDJSON files organized by code/date/timeframe
- Run subscriptions as background daemons with PID management
- Auto-reconnect on connection loss (max 3 retries)
- Notify user of errors via opencb CLI
- Archive old data via clean command

**Non-Goals:**
- Trading functionality (future feature)
- Multi-code subscription in single command
- Real-time data analysis/processing
- Web UI or API server

## Decisions

### 1. Standalone Daemons

**Decision**: Each subscription runs as an independent daemon process, not managed by a central service.

**Rationale**: User requirement - each daemon is standalone, no dependency on centralized process. Simple PID file management.

**Alternatives considered**:
- Central daemon manager: More complex, single point of failure
- Systemd service: Overkill for this use case

### 2. PID File Format

**Decision**: Store PID, timeframe, and start time in PID file.

```
{pid}
{timeframe}
{start_time}
```

**Rationale**: Enables `process list` to display useful information. Simple text format.

### 3. Daemon Mode Default

**Decision**: `sub` command defaults to daemon mode, `-fe` flag for foreground.

**Rationale**: Most use cases require background data collection. Foreground for debugging.

### 4. Error Handling Strategy

**Decision**: Catch all errors, notify via opencb, exit with code 1.

**Rationale**: User requirement - daemon should not silently fail. Opencb notification enables monitoring.

**Flow**:
```
TRY {
    connect (max 3 retries)
    subscribe
    run loop
} CATCH {
    opencb send "WARNING-NINE_FUT Sub-Daemon-Error"
    cleanup PID file
    exit(1)
}
```

### 5. Reconnection Strategy

**Decision**: Auto-reconnect up to 3 times with 5-second delay between attempts.

**Rationale**: Transient network issues are common. 3 retries prevents infinite loops.

**Flow**:
```
Attempt 1 → failed → wait 5s
Attempt 2 → failed → wait 5s
Attempt 3 → failed → opencb send + exit
```

### 6. Storage Path Structure

**Decision**: `~/.opens/nine-futu/data/live/{code}/{date}/{timeframe}/kline.txt`

```
~/.opens/nine-futu/data/live/
├── HK.00700/
│   └── 2026-06-01/
│       ├── quote.txt        # NDJSON
│       ├── orderbook.txt
│       ├── ticker.txt
│       ├── broker.txt
│       ├── rt_data.txt
│       └── 5m/              # timeframe subfolder (kline only)
│           └── kline.txt
└── US.AAPL/
    └── 2026-06-01/
        ├── quote.txt
        └── 1d/
            └── kline.txt
```

**Rationale**: User requirement. Code at top level for easy browsing. Date for daily rotation. Timeframe only for K-line data.

### 7. Data Format

**Decision**: NDJSON (one JSON object per line).

**Rationale**: 
- Easy to append new data
- Line-by-line parsing
- Compatible with standard tools (jq, etc.)
- Consistent with existing CLI output

### 8. Clean Command Destination

**Decision**: Create `{dest}/{code}/` subfolder, then move date folders inside.

**Rationale**: User requirement. Maintains code organization in archive.

```
Source: ~/.opens/.../HK.00700/2026-05-30/
Dest:   /backup/live/HK.00700/2026-05-30/
```

## Risks / Trade-offs

### Risk: PID File Stale After Crash
**Impact**: `process status` may return incorrect PID
**Mitigation**: Check if process exists before returning PID. Simple check: `kill -0 {pid}`

### Risk: Disk Space Growth
**Impact**: Continuous data collection fills disk
**Mitigation**: Clean command for manual archival. Future: auto-rotation policy

### Risk: FutuOpenD Connection Instability
**Impact**: Data gaps during disconnection
**Mitigation**: Auto-reconnect with 3 retries. Notify user on failure.

### Risk: Multiple Daemons for Same Code
**Impact**: Duplicate data storage
**Mitigation**: Check PID file before starting new subscription

### Risk: Daemon Process Orphaned
**Impact**: Zombie process consuming resources
**Mitigation**: PID file cleanup on exit. `process stop` for manual cleanup.

## Open Questions

1. **opencb integration**: Should we capture opencb output or just ignore it?
2. **Log files for daemons**: Should daemons write logs to `~/.opens/nine-futu/log/`?
3. **Data retention policy**: Auto-delete data after N days? (Currently manual via clean)
4. **Subscription quota**: How to handle FutuOpenD subscription limits?
