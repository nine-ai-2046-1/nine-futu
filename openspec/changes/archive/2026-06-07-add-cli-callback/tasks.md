## 1. CLI Callback Module

- [x] 1.1 Create cli_callback module in nine-futu-lib
- [x] 1.2 Implement subprocess spawning with std::process::Command
- [x] 1.3 Implement error handling (capture output, print to stderr)
- [x] 1.4 Implement K-line data formatting for subprocess argument

## 2. Quote Command Updates

- [x] 2.1 Add `--cli` flag to kline subcommand definition
- [x] 2.2 Integrate CLI callback in kline output loop

## 3. Subscription Updates

- [x] 3.1 Add `--cli` flag to sub command definition
- [x] 3.2 Integrate CLI callback in push data handler for K-line data

## 4. Testing

- [x] 4.1 Test CLI callback with quote kline command
- [x] 4.2 Test CLI callback with sub command
- [x] 4.3 Test subprocess error handling
