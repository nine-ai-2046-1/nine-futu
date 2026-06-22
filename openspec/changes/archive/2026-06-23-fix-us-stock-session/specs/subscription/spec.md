## MODIFIED Requirements

### Requirement: Session parameter for subscription
The system SHALL set session=Session_ALL (3) for all subscriptions.

#### Scenario: US stock subscription
- **WHEN** user subscribes to US.AAPL
- **THEN** system sets session=3 (Session_ALL) in request

#### Scenario: HK stock subscription
- **WHEN** user subscribes to HK.00700
- **THEN** system sets session=3 (Session_ALL) in request

### Requirement: US stock quote
The system SHALL allow quoting US stocks after subscription.

#### Scenario: Quote US stock
- **WHEN** user runs `nine-futu quote snapshot -c US.AAPL`
- **THEN** system returns stock data (not "Unknown stock")
