## 0.3.1

### Improvements

The `--fail-fast`, `--shuffle`, and `--shuffle-seed` command-line arguments are
understood.

## 0.3.0

### Breaking Changes

- The minimum required Rust version is now 1.32.0.

### Improvements

- `rusty_fork_test!` can now be `use`d in Rust 2018 code.

- The following flags to the test process are now understood: `--ensure-time`,
  `--exclude-should-panic`, `--force-run-in-process`, `--include-ignored`,
  `--report-time`, `--show-output`.

## 0.2.2

### Minor changes

- `wait_timeout` has been bumped to `0.2.0`.

## 0.2.1

### Bug Fixes

- Dependency on `wait_timeout` crate now requires `0.1.4` rather than `0.1`
  since the build doesn't work with older versions.

## 0.2.0

### Breaking changes

- APIs which used to provide a `std::process::Child` now instead provide a
  `rusty_fork::ChildWrapper`.

### Bug fixes

- Fix that using the "timeout" feature, or otherwise using `wait_timeout` on
  the child process, could cause an unrelated process to get killed if the
  child exits within the timeout.

## 0.1.1

### Minor changes

- `tempfile` updated to 3.0.
