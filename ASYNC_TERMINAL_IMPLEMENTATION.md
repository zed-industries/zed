# Async Terminal Implementation

## Overview

This implementation adds asynchronous terminal command execution to the Zed agent, allowing long-running commands to be executed in the background with job tracking and management capabilities.

## Features

### 1. Async Terminal Execution
- New `async` parameter on the `terminal` tool
- Commands run in background and return immediately with a job ID
- Non-blocking execution for long-running tasks

### 2. Job Management Tools

#### `terminal_job_status`
Check the status and output of a background job.

**Parameters:**
- `job_id` (string): The job ID returned from async terminal execution
- `incremental` (boolean, default: true): Return only new output since last check

**Returns:**
- Job status (running, completed, failed, canceled)
- Command and working directory
- Duration and exit code
- Output (incremental or full)

**Example:**
```json
{
  "job_id": "terminal-job-1",
  "incremental": true
}
```

#### `terminal_job_list`
List all background terminal jobs with optional filtering.

**Parameters:**
- `status_filter` (array, optional): Filter by status (e.g., ["running", "completed"])
- `limit` (number, default: 50): Maximum number of jobs to return

**Returns:**
- List of jobs with metadata
- Total count and running count

**Example:**
```json
{
  "status_filter": ["running"],
  "limit": 10
}
```

#### `terminal_job_cancel`
Cancel a running background job and kill its terminal process.

**Parameters:**
- `job_id` (string): The job ID to cancel

**Returns:**
- Success status
- Previous job status
- Confirmation message

**Example:**
```json
{
  "job_id": "terminal-job-2"
}
```

## Architecture

### Components

#### TerminalJobManager
- **Type:** Global singleton (`impl Global`)
- **Purpose:** Centralized job tracking and terminal handle management
- **Storage:**
  - `jobs: Arc<Mutex<HashMap<String, TerminalJobRecord>>>` - Job metadata
  - `terminal_handles: Arc<Mutex<HashMap<String, Rc<dyn TerminalHandle>>>>` - Terminal references for process control
  - `job_counter: Arc<Mutex<u64>>` - Unique ID generation

#### TerminalJobRecord
Stores metadata for each background job:
```rust
pub struct TerminalJobRecord {
    pub job_id: String,
    pub command: String,
    pub working_dir: String,
    pub started_at: SystemTime,
    pub finished_at: Option<SystemTime>,
    pub status: TerminalJobStatus,
    pub exit_code: Option<i32>,
    pub terminal_id: acp::TerminalId,
    pub output: String,
    pub last_read_position: usize,
}
```

#### TerminalJobStatus
```rust
pub enum TerminalJobStatus {
    Running,
    Completed,
    Failed,
    Canceled,
}
```

### Key Design Decisions

#### 1. Separate Handle Storage
Terminal handles (`Rc<dyn TerminalHandle>`) are stored separately from job records because:
- Job records need to be `Clone + Serialize`
- Terminal handles are not `Send` and cannot be serialized
- Handles are removed when jobs complete or are canceled

#### 2. Incremental Output Tracking
Each job tracks `last_read_position` to support efficient incremental output retrieval:
- First call returns all accumulated output
- Subsequent calls return only new output since last read
- Reduces bandwidth for polling long-running jobs

#### 3. Foreground Executor for Terminal Operations
Terminal operations use `foreground_executor().spawn()` instead of `background_spawn()` because:
- `Rc<dyn TerminalHandle>` is not `Send`
- `AsyncApp` contains `Rc` types and is not `Send`
- Terminal operations must run on the main thread

#### 4. Helper Async Functions
To avoid Rust lifetime inference issues with `cx.spawn()`, we use helper `async fn` functions:
```rust
async fn run_async_terminal(
    // parameters...
    cx: &mut gpui::AsyncApp,
) -> Result<String> {
    // implementation
}

// Called via:
cx.foreground_executor().spawn(async move {
    run_async_terminal(..., &mut cx_async.clone()).await
})
```

This pattern avoids higher-ranked trait bound (`for<'a>`) inference problems.

### Process Control

#### Job Cancellation Flow
1. Verify job exists and is running
2. Retrieve terminal handle from `terminal_handles` map
3. Call `terminal.kill(cx)` to terminate the process
4. Update job status to `Canceled`
5. Remove terminal handle from storage

#### Job Completion Flow
Monitored in a background task spawned with the async command:
1. Wait for terminal exit (with optional timeout)
2. Retrieve final output
3. Update job record with exit code and status
4. Remove terminal handle from storage

## Integration

### Agent Profiles
Tools are enabled in agent profiles via `assets/settings/default.json`:

**Write Profile:**
- `terminal` (with async support)
- `terminal_job_status`
- `terminal_job_list`
- `terminal_job_cancel`

**Ask Profile (Read-only):**
- `terminal_job_status`
- `terminal_job_list`
- No `terminal_job_cancel` for safety

### Initialization
`TerminalJobManager::init_global(cx)` is called during Zed startup in `crates/zed/src/main.rs`.

## Usage Examples

### Example 1: Run a Build in Background
```
User: Run the cargo build in the background
Agent: [calls terminal tool with async=true]
Response: Job ID: terminal-job-3

User: Check the build status
Agent: [calls terminal_job_status with job_id="terminal-job-3"]
Response: Status: running, Duration: 45.2s (running)
```

### Example 2: Cancel a Long-Running Test
```
User: Run the tests but cancel if it takes too long
Agent: [calls terminal tool with async=true]
Response: Job ID: terminal-job-4

[After 2 minutes]
User: Cancel it
Agent: [calls terminal_job_cancel with job_id="terminal-job-4"]
Response: Job canceled and terminal process killed successfully.
```

### Example 3: Monitor Multiple Jobs
```
User: Show me all running jobs
Agent: [calls terminal_job_list with status_filter=["running"]]
Response: 
  - terminal-job-5: cargo test (3m 12s)
  - terminal-job-6: npm run dev (45s)
```

## Implementation Notes

### Exit Code Handling
- Exit code stored as `Option<i32>` for compatibility
- `Some(0)` = successful completion
- `Some(non-zero)` = failed
- Job killed by cancel returns exit code from signal

### Output Buffering
- Output is accumulated as the terminal produces it
- No explicit size limit (controlled by terminal output limit)
- Output persists in memory until job record is cleaned up

### Thread Safety
All shared state protected by `Arc<Mutex<>>`:
- Safe concurrent access from multiple threads
- Lock held for minimal duration
- No risk of deadlock (locks are never nested)

### Memory Considerations
- Job records persist until manually cleaned (future: add auto-cleanup)
- Terminal handles released immediately when jobs complete
- Typical job record: ~1KB + output size

## Testing

The implementation has been manually tested with:
- ✅ Async command execution
- ✅ Job status checking (incremental and full output)
- ✅ Job listing with filtering
- ✅ Job cancellation (verified process termination)
- ✅ Multiple concurrent jobs
- ✅ Exit code tracking

## Future Enhancements

Potential improvements:
1. **Auto-cleanup:** Automatically remove old completed jobs after a configurable time
2. **Job persistence:** Save job history across Zed restarts
3. **Output streaming:** Real-time output updates via events
4. **Job grouping:** Organize related jobs into groups
5. **Resource limits:** Limit number of concurrent jobs
6. **Job priority:** Queue management for many concurrent jobs

## Technical Challenges Resolved

### 1. Async Closure Lifetime Issues
**Problem:** Rust compiler couldn't infer higher-ranked trait bounds for `cx.spawn(move |cx| ...)`.

**Solution:** Use helper `async fn` with explicit `&mut gpui::AsyncApp` parameter, called via `foreground_executor().spawn()`.

### 2. Terminal Handle Storage
**Problem:** `TerminalJobRecord` needs to be `Clone + Serialize`, but `Rc<dyn TerminalHandle>` is neither.

**Solution:** Store terminal handles in a separate `HashMap` with same job ID as key.

### 3. Non-Send Types
**Problem:** `Rc<dyn TerminalHandle>` and `AsyncApp` are not `Send`, can't use `background_executor()`.

**Solution:** Use `foreground_executor().spawn()` for all terminal operations.

### 4. Borrow Checker Issues
**Problem:** Moving `cx` into terminal operations prevented reuse.

**Solution:** Use explicit reborrows (`&mut *cx`, `&*cx`) and clone `AsyncApp` when needed for nested spawns.

## Files Modified/Added

### New Files
- `crates/agent/src/tools/terminal_job_manager.rs` - Core job management
- `crates/agent/src/tools/terminal_job_status_tool.rs` - Status checking tool
- `crates/agent/src/tools/terminal_job_list_tool.rs` - Job listing tool
- `crates/agent/src/tools/terminal_job_cancel_tool.rs` - Job cancellation tool

### Modified Files
- `crates/agent/src/tools/terminal_tool.rs` - Added async parameter and job registration
- `crates/agent/src/tools.rs` - Added new tool exports and macro entries
- `crates/agent/src/thread.rs` - Added tools to default tool set
- `assets/settings/default.json` - Added tools to agent profiles
- `crates/zed/src/main.rs` - Initialize TerminalJobManager global
- `crates/zed/Cargo.toml` - Added agent dependency

## Conclusion

This implementation provides a robust foundation for asynchronous terminal command execution in the Zed agent. The design balances simplicity with functionality, providing essential job management capabilities while maintaining clean integration with Zed's architecture.