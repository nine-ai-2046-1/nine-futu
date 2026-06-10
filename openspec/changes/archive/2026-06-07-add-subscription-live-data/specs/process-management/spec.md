## ADDED Requirements

### Requirement: List running daemon processes
The system SHALL display all running subscription daemon processes.

#### Scenario: List with running daemons
- **WHEN** user runs `nine-futu process list` and daemons are running
- **THEN** system displays table with PID, code, timeframe, and start time

#### Scenario: List with no daemons
- **WHEN** user runs `nine-futu process list` and no daemons are running
- **THEN** system displays "No running processes"

### Requirement: Stop daemon by PID
The system SHALL stop a running daemon process by PID.

#### Scenario: Stop existing daemon
- **WHEN** user runs `nine-futu process stop {pid}` and process exists
- **THEN** system kills the process and removes PID file

#### Scenario: Stop non-existent PID
- **WHEN** user runs `nine-futu process stop {pid}` and PID does not exist
- **THEN** system displays error "Process {pid} not found" and exits with code 1

### Requirement: Check subscription status for code
The system SHALL check if a subscription daemon is running for a specific stock code.

#### Scenario: Code has running daemon
- **WHEN** user runs `nine-futu process status "700"` and daemon is running
- **THEN** system returns the PID of the running daemon

#### Scenario: Code has no running daemon
- **WHEN** user runs `nine-futu process status "700"` and no daemon is running
- **THEN** system returns -1

### Requirement: PID file management
The system SHALL manage PID files for daemon processes.

#### Scenario: Create PID file on daemon start
- **WHEN** daemon starts
- **THEN** system creates PID file at `~/.opens/nine-futu/pid/{code}.pid` with format: `{pid}\n{timeframe}\n{start_time}`

#### Scenario: Remove PID file on daemon exit
- **WHEN** daemon exits (normal or error)
- **THEN** system removes PID file

#### Scenario: Foreground mode PID file
- **WHEN** user runs `nine-futu sub -c "700" -fe`
- **THEN** system creates PID file (for `process list` visibility)
