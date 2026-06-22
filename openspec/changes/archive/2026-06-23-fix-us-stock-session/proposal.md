## Why

US stocks return "Unknown stock" error because the subscription doesn't set the `session` parameter. The Python SDK uses `Session.ALL` (value 3) when subscribing, which is required for US stocks to specify which trading session to use (pre-market, regular, after-hours).

## What Changes

- Add `session` parameter to subscription requests
- Use `Session_ALL = 3` for US stocks
- Optionally allow user to specify session type

## Capabilities

### New Capabilities

(None - bug fix)

### Modified Capabilities

- `subscription`: Add session parameter for US market compatibility

## Impact

- **Code**: Update subscription request in client.rs
- **CLI**: Optionally add --session flag
