## 1. K-line Command Updates

- [x] 1.1 Add `-p` flag to kline subcommand definition
- [x] 1.2 Add `--delay` flag to kline subcommand definition
- [x] 1.3 Implement period calculation logic (today - N days)
- [x] 1.4 Update kline command to use period flag for start date
- [x] 1.5 Implement delay logic between NDJSON output lines

## 2. Testing

- [x] 2.1 Test kline with -p 30 (default end = today)
- [x] 2.2 Test kline with -p 30 and -e (explicit end)
- [x] 2.3 Test kline with -p 7 for minute data
- [x] 2.4 Test kline with --delay 5 (verify timing)
- [x] 2.5 Test kline with -p 30 and -s (period overrides start)
