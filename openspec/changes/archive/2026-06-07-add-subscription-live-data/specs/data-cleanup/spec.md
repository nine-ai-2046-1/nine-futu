## ADDED Requirements

### Requirement: Archive old live data
The system SHALL move live data folders older than 1 day to a specified destination.

#### Scenario: Clean with confirmation
- **WHEN** user runs `nine-futu clean -d "/backup/live"`
- **THEN** system lists old folders, asks for confirmation, and moves them

#### Scenario: Clean without confirmation
- **WHEN** user runs `nine-futu clean -d "/backup/live" -y`
- **THEN** system moves old folders without asking

#### Scenario: No old folders
- **WHEN** user runs `nine-futu clean -d "/backup/live"` and no folders are older than 1 day
- **THEN** system displays "No old folders to move"

### Requirement: Preserve folder structure in destination
The system SHALL maintain the code/date folder structure when moving.

#### Scenario: Move folder structure
- **WHEN** moving `HK.00700/2026-05-30/` to `/backup/live/`
- **THEN** system creates `/backup/live/HK.00700/2026-05-30/` and moves all contents

#### Scenario: Create code subfolder
- **WHEN** destination code folder does not exist
- **THEN** system creates `{dest}/{code}/` before moving

### Requirement: Date-based filtering
The system SHALL only move folders with dates older than 1 day from today.

#### Scenario: Filter old folders
- **WHEN** today is 2026-06-02 and folders exist for 2026-05-30, 2026-05-31, 2026-06-01
- **THEN** system selects 2026-05-30 and 2026-05-31 (older than 1 day)

#### Scenario: Exclude recent folders
- **WHEN** today is 2026-06-02 and folder exists for 2026-06-01
- **THEN** system does NOT move 2026-06-01 folder (today - 1 is not older)

### Requirement: Display move summary
The system SHALL display a summary of moved folders.

#### Scenario: Show moved folders
- **WHEN** folders are moved successfully
- **THEN** system displays list of moved folders and count
