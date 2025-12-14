# Scheduler Integration PR Status

## PR: #44810 - Integrate scheduler crate into GPUI TestDispatcher

### Current State
Branch: `scheduler-integration`
Most CI checks passing, **Windows tests failing**.

### What Was Done This Session

1. **Phase 6: Restored Realtime Priority Support**
   - Added `Priority::Realtime` handling in `gpui::BackgroundExecutor::spawn_with_priority`
   - Spawns dedicated OS thread via `dispatcher.spawn_realtime()`
   - Uses flume channel to send runnables to the dedicated thread
   - Includes profiler timing integration (matches other dispatchers)

2. **Phase 7: Deleted Dead Code**
   - Removed orphaned `crates/gpui/src/platform/platform_scheduler.rs`

3. **Fixed Various Build Issues**
   - Fixed formatting across the branch (`cargo fmt`)
   - Renamed `.probability()` to `.weight()` in `queue.rs` (scheduler crate uses `weight()`)
   - Added explicit type annotation for `PriorityQueueReceiver<RunnableVariant>` in Linux dispatcher
   - Removed unused `RunnableVariant` imports in Linux wayland/x11 clients
   - Fixed `test_parking_panics` expected panic message

### CI Status (Latest Run)
- ✅ check_style
- ✅ check_docs  
- ✅ doctests
- ✅ check_workspace_binaries
- ✅ check_dependencies
- ✅ check_licenses
- ✅ build_nix_mac_aarch64
- ⏳ run_tests_linux (pending)
- ⏳ run_tests_mac (pending)
- ❌ **run_tests_windows** - FAILING

### Windows Test Failures

Multiple tests are panicking with the same error at `crates/gpui/src/arena.rs:201`:
```
"attempted to dereference an ArenaRef after its Arena was cleared"
```

Affected tests include:
- `tests::following_tests::test_basic_following`
- `tests::channel_buffer_tests::test_channel_notes_participant_indices`
- `tests::editor_tests::test_copy_file_location`
- `tests::editor_tests::test_copy_file_name`
- `tests::following_tests::test_auto_unfollowing`
- Many more collab/editor tests

This appears to be a **use-after-free** issue specific to Windows where `ArenaBox` items are being accessed after their Arena was cleared. This could be related to:
1. Task scheduling/execution order differences on Windows
2. The scheduler integration changing when/how tasks are executed
3. Some Windows-specific timing issue with async task completion

### Key Files Changed
- `crates/gpui/src/executor.rs` - Realtime priority support
- `crates/gpui/src/queue.rs` - `.probability()` → `.weight()`
- `crates/gpui/src/platform/linux/dispatcher.rs` - Type annotation fix
- `crates/gpui/src/platform/linux/wayland/client.rs` - Removed unused import
- `crates/gpui/src/platform/linux/x11/client.rs` - Removed unused import
- `crates/scheduler/src/tests.rs` - Fixed test assertion
- `crates/scheduler/full_integration_plan.md` - Updated status

### Reviewers to CC
- **@localcc** - Realtime priority feature author (#44701)
- **@maxbrunsfeld @as-cii** - Re: `spawn_labeled`/`deprioritize` removal
- **@nathansobo** - Scheduler crate author

### Next Steps
1. **Investigate Windows Arena panic** - The scheduler integration may have changed task execution timing in a way that causes use-after-free on Windows
2. Check if this is a pre-existing issue on the branch or newly introduced
3. May need to look at how `TestDispatcher` handles task ordering differently on Windows

### Commands to Reproduce Locally
```bash
# Run all checks
./script/clippy
cargo fmt --check
cargo test -p gpui -p scheduler

# Build specific targets
cargo build -p collab
cargo build -p zed
```

### Plan Document
See `crates/scheduler/full_integration_plan.md` for the complete integration plan and architecture documentation.