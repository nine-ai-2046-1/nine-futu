## Why

The current trade commands require users to remember numeric values for account types, trading environments, and order statuses. This is error-prone and unfriendly. Users should be able to use human-readable strings like "cash", "margin", "sim", "real" instead.

## What Changes

- Default to Simulate environment (TrdEnv=0)
- Auto-select account ID based on environment and account type
- Default to Cash account type (TrdAccType=1)
- Allow --margin flag to use Margin account
- Allow string inputs for trade environment and account type
- Show order status descriptions instead of numeric values

## Capabilities

### New Capabilities

(None - modification of existing behavior)

### Modified Capabilities

- `trade-order`: Use string inputs for environment and account type
- `trade-account`: Auto-select account based on environment

## Impact

- **Code**: Update trade client and CLI commands
- **Config**: Add default_account_type setting
