# Terminal Jobs UI Enhancements

## Current Implementation (Phase 1 - MVP)

### Features Completed
- ✅ Collapsed/expanded view toggle
- ✅ Job cards showing command, status, duration
- ✅ Real-time updates (1-second polling)
- ✅ View/Hide output inline
- ✅ Cancel running jobs
- ✅ Dismiss completed jobs
- ✅ Status icons and colors
- ✅ Auto-expand when jobs start
- ✅ Integrated between header and thread view

### Current UI Structure
```
AgentsPanel
  └─ AgentThreadPane
       ├─ Header (tab controls)
       ├─ TerminalJobsPanel ← NEW
       │    ├─ Collapsed: "⚡ N jobs running [Expand]"
       │    └─ Expanded: List of job cards
       │         └─ Job Card
       │              ├─ Header: icon, job_id, [View] [Cancel/Dismiss]
       │              ├─ Command line
       │              ├─ Status line: duration, dir, status
       │              └─ Output (when visible)
       └─ Thread View (messages & input)
```

## Phase 2: Interactive Features & Polish

### 1. Output Enhancements

#### Live Output Streaming
**Current:** Output only updates on 1-second poll  
**Enhancement:** Stream output in real-time as it arrives

```rust
// In TerminalJobManager
pub fn subscribe_to_output(&self, job_id: &str) -> impl Stream<Item = String> {
    // Return stream of output chunks
}

// In TerminalJobsPanel
fn stream_job_output(&mut self, job_id: &str, cx: &mut ViewContext<Self>) {
    cx.spawn(|this, mut cx| async move {
        let mut stream = job_manager.subscribe_to_output(job_id);
        while let Some(chunk) = stream.next().await {
            this.update(&mut cx, |this, cx| {
                this.append_output(job_id, chunk);
                cx.notify();
            }).ok();
        }
    }).detach();
}
```

#### Auto-scroll to Bottom
When output is visible, keep scrolled to bottom for "follow mode"
```rust
.child(
    div()
        .id(("job-output", job.job_id.clone()))
        .overflow_y_scroll()
        .scroll_to_bottom() // Add scroll behavior
        .child(output_text)
)
```

#### Syntax Highlighting
Parse and highlight output based on content
```rust
fn render_output_with_highlighting(output: &str) -> impl IntoElement {
    if output.contains("error:") || output.contains("Error:") {
        // Highlight error lines in red
    } else if output.contains("warning:") {
        // Highlight warnings in yellow
    } else if output.contains("Compiling") || output.contains("Building") {
        // Highlight build progress in blue
    }
    // etc.
}
```

### 2. Job Actions

#### Copy Actions
```rust
// Copy command
Button::new("copy-command", "Copy Command")
    .icon(IconName::Copy)
    .on_click(|_, cx| {
        cx.write_to_clipboard(job.command.clone());
        // Show toast: "Command copied"
    })

// Copy output
Button::new("copy-output", "Copy Output")
    .icon(IconName::Copy)
    .on_click(|_, cx| {
        cx.write_to_clipboard(job.output.clone());
        // Show toast: "Output copied"
    })
```

#### Re-run Command
```rust
Button::new("rerun", "Re-run")
    .icon(IconName::Refresh)
    .tooltip("Run this command again")
    .on_click(cx.listener(move |this, _, cx| {
        // Create new terminal job with same command
        this.rerun_job(&job_id, cx);
    }))
```

#### Open in Terminal
```rust
Button::new("open-terminal", "Open in Terminal")
    .icon(IconName::Terminal)
    .tooltip("Open new terminal with this command")
    .on_click(|_, cx| {
        // Open terminal panel with command pre-filled
    })
```

### 3. Notifications

#### Toast on Completion
```rust
impl TerminalJobsPanel {
    fn on_job_completed(&mut self, job_id: &str, cx: &mut ViewContext<Self>) {
        let job = self.find_job(job_id);
        
        workspace.update(cx, |workspace, cx| {
            workspace.show_toast(
                Toast::new(
                    format!("Job {} completed", job_id),
                    if job.exit_code == Some(0) {
                        ToastType::Success
                    } else {
                        ToastType::Error
                    }
                ),
                cx
            );
        });
    }
}
```

#### Sound on Completion
```rust
use audio::Audio;

fn play_completion_sound(success: bool, cx: &App) {
    if AgentSettings::get_global(cx).play_sound_when_job_done {
        let audio = Audio::global(cx);
        if success {
            audio.play_sound("job_success.wav");
        } else {
            audio.play_sound("job_failed.wav");
        }
    }
}
```

### 4. Visual Polish

#### Animations
```rust
// Slide in when job starts
.with_animation(
    "slide-in",
    Animation::new(Duration::from_millis(200))
        .with_easing(Easing::EaseOut)
)

// Fade out when dismissed
.with_animation(
    "fade-out", 
    Animation::new(Duration::from_millis(150))
)

// Pulse on status change
.when(job.status_changed_recently, |this| {
    this.with_animation("pulse", pulse_animation())
})
```

#### Progress Indicator
For running jobs, show a subtle animated progress indicator
```rust
.when(job.status == "running", |this| {
    this.child(
        div()
            .h_px()
            .w_full()
            .child(
                div()
                    .h_full()
                    .bg(ui::Color::Accent)
                    .with_animation("progress", indeterminate_progress_animation())
            )
    )
})
```

#### Compact Mode
For many jobs, show compact cards
```rust
fn render_job_card_compact(&self, job: &JobCardState) -> impl IntoElement {
    h_flex()
        .gap_2()
        .items_center()
        .child(status_icon)
        .child(Label::new(truncate(&job.command, 40)))
        .child(Label::new(duration).color(Muted))
        .child(action_buttons)
}
```

## Phase 3: Advanced Features

### 1. Job History Modal

```rust
struct JobHistoryModal {
    all_jobs: Vec<TerminalJobRecord>,
    filter_text: String,
    status_filter: Option<TerminalJobStatus>,
    date_range: (Option<SystemTime>, Option<SystemTime>),
}

impl JobHistoryModal {
    fn render_filters(&self) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(TextInput::new("Search commands..."))
            .child(StatusFilterDropdown::new())
            .child(DateRangePicker::new())
    }
    
    fn render_job_list(&self) -> impl IntoElement {
        list(self.filtered_jobs())
            .child_for_each(|job| {
                JobHistoryRow {
                    job,
                    actions: [View, Rerun, Copy, Delete]
                }
            })
    }
}
```

**Features:**
- Search by command text
- Filter by status, date range, working directory
- Sort by start time, duration, exit code
- Export to CSV/JSON
- Delete old jobs
- Re-run any historical command

### 2. Job Grouping

```rust
struct JobGroup {
    name: String,
    job_ids: Vec<String>,
    status: GroupStatus, // All completed, some running, etc.
}

enum GroupStatus {
    AllRunning,
    SomeRunning,
    AllCompleted,
    SomeFailed,
}

// UI: Collapsible group card
┌─────────────────────────────────────────────────┐
│ 📦 Build Pipeline (3 jobs)          [View] [▼]  │
│   └─ 2 completed, 1 running                     │
├─────────────────────────────────────────────────┤
│   ✅ cargo test                   3m 21s         │
│   ✅ cargo clippy                 1m 45s         │
│   🔵 cargo build --release        2m 12s ...    │
└─────────────────────────────────────────────────┘
```

**Features:**
- Group related jobs (e.g., CI pipeline steps)
- Show aggregate status
- Cancel entire group
- Automatic grouping by time proximity
- Manual group creation

### 3. Output Analysis & Smart Features

#### Parse Error Messages
```rust
fn parse_errors_from_output(output: &str) -> Vec<ParsedError> {
    // Detect patterns like:
    // error: ...
    // error[E0599]: ...
    // ERROR: ...
    // /path/file.rs:123:45: error: ...
}

struct ParsedError {
    file: Option<PathBuf>,
    line: Option<usize>,
    column: Option<usize>,
    message: String,
    severity: ErrorSeverity,
}
```

**UI Features:**
- Clickable file:line references → jump to location
- Error count badge on job card
- Filter to show only errors
- Quick actions: "Fix with AI", "Search solution"

#### Smart Suggestions
```rust
fn suggest_actions(job: &JobRecord) -> Vec<Suggestion> {
    match (job.command.as_str(), job.exit_code) {
        ("cargo build", Some(101)) => vec![
            Suggestion::FixCompilationErrors,
            Suggestion::RunCargoCheck,
        ],
        ("npm test", Some(1)) => vec![
            Suggestion::ViewFailedTests,
            Suggestion::RunSingleTest,
        ],
        (cmd, Some(127)) if cmd.starts_with("python") => vec![
            Suggestion::InstallDependencies,
        ],
        _ => vec![],
    }
}
```

### 4. Performance Monitoring

```rust
struct JobMetrics {
    cpu_percent: f32,
    memory_mb: f64,
    peak_memory_mb: f64,
    io_read_bytes: u64,
    io_write_bytes: u64,
}

// UI: Expandable metrics section
┌─────────────────────────────────────────────────┐
│ 🔵 terminal-job-7               [Metrics ▼]     │
│   $ cargo build --release                       │
│   ⏱️  2m 34s • 📊 Running                        │
├─────────────────────────────────────────────────┤
│ 📊 Performance Metrics                          │
│   CPU: ▓▓▓▓▓▓▓░░░ 65%                           │
│   Memory: 2.3 GB (peak: 3.1 GB)                 │
│   I/O: ↓ 450 MB  ↑ 1.2 GB                       │
└─────────────────────────────────────────────────┘
```

### 5. Job Templates & Quick Actions

```rust
struct JobTemplate {
    name: String,
    command: String,
    working_dir: String,
    description: String,
}

// Predefined templates
const TEMPLATES: &[JobTemplate] = &[
    JobTemplate {
        name: "Build (Release)",
        command: "cargo build --release",
        working_dir: ".",
        description: "Build the project in release mode",
    },
    JobTemplate {
        name: "Run Tests",
        command: "cargo test",
        working_dir: ".",
        description: "Run all tests",
    },
];

// UI: Quick action buttons above job list
┌─────────────────────────────────────────────────┐
│ ⚡ RUNNING JOBS (0)                              │
├─────────────────────────────────────────────────┤
│ Quick Actions:                                   │
│ [🔨 Build] [🧪 Test] [📝 Lint] [🚀 Deploy]      │
└─────────────────────────────────────────────────┘
```

## Phase 4: Settings & Customization

### Settings Schema

```json
{
  "agent": {
    "terminal_jobs_panel": {
      // Basic settings
      "enabled": true,
      "position": "above_input", // "above_input" | "below_input" | "sidebar"
      "default_expanded": false,
      "max_visible_jobs": 10,
      
      // Auto-behavior
      "auto_expand_on_start": true,
      "auto_collapse_on_complete": false,
      "auto_dismiss_after_seconds": 30, // null = never
      
      // Output display
      "show_output_inline": true,
      "max_output_lines": 200,
      "auto_scroll_output": true,
      "syntax_highlight_output": true,
      
      // Update frequency
      "poll_interval_ms": 1000,
      "enable_streaming_output": false, // Future: WebSocket streaming
      
      // Notifications
      "notifications": {
        "on_complete": true,
        "on_error": true,
        "on_long_running": {
          "enabled": true,
          "threshold_seconds": 300 // Notify if job runs > 5 minutes
        },
        "sound": {
          "enabled": false,
          "success_sound": "complete.wav",
          "error_sound": "error.wav"
        }
      },
      
      // Visual
      "compact_mode": false,
      "show_metrics": false,
      "animations_enabled": true,
      
      // Advanced
      "job_history": {
        "enabled": true,
        "max_history_entries": 100,
        "persist_across_sessions": false
      },
      "job_templates": [
        {
          "name": "Build Release",
          "command": "cargo build --release",
          "working_dir": ".",
          "icon": "🔨"
        }
      ]
    }
  }
}
```

### Keyboard Shortcuts

```json
{
  "bindings": {
    "cmd-shift-j": "terminal_jobs_panel::toggle",
    "cmd-k cmd-j": "terminal_jobs_panel::focus",
    "escape": "terminal_jobs_panel::collapse", // When focused
    "cmd-enter": "terminal_jobs_panel::cancel_selected", // Cancel focused job
    "cmd-o": "terminal_jobs_panel::toggle_output", // Show/hide output
    "cmd-c": "terminal_jobs_panel::copy_output",
    "delete": "terminal_jobs_panel::dismiss_selected"
  }
}
```

## Phase 5: Integration & Workflow Features

### 1. Integration with Chat Messages

When agent starts a job, show it inline in the chat:

```
Agent: I'll run the tests in the background.
┌─────────────────────────────────────────────────┐
│ 🔵 Job Started: terminal-job-8                  │
│   $ cargo test                                   │
│   [View Status] [Cancel]                        │
└─────────────────────────────────────────────────┘

[30 seconds later, job completes]

Agent: The tests completed successfully! ✅
┌─────────────────────────────────────────────────┐
│ ✅ Job Completed: terminal-job-8                │
│   Duration: 28.5s • Exit: 0                     │
│   [View Output]                                 │
└─────────────────────────────────────────────────┘
```

### 2. Job Dependencies

Run jobs in sequence or parallel with dependencies:

```rust
struct JobDependency {
    job_id: String,
    depends_on: Vec<String>, // Other job IDs
    run_on: DependencyTrigger,
}

enum DependencyTrigger {
    OnSuccess,    // Run if dependency succeeds
    OnFailure,    // Run if dependency fails
    OnComplete,   // Run when dependency completes (any status)
}

// UI: Show dependency graph
┌─────────────────────────────────────────────────┐
│ 📦 Pipeline                                      │
│   ✅ cargo test ──────┐                          │
│   ✅ cargo clippy ────┼─→ 🔵 cargo build         │
│   ✅ cargo fmt ───────┘                          │
└─────────────────────────────────────────────────┘
```

### 3. Context-Aware Actions

Based on job status, show relevant actions:

```rust
fn contextual_actions(job: &JobRecord) -> Vec<Action> {
    match (&job.command, &job.status, job.exit_code) {
        // Failed tests
        (cmd, Failed, Some(101)) if cmd.contains("test") => vec![
            Action::ViewFailures,
            Action::DebugFirstFailure,
            Action::RerunFailedOnly,
        ],
        
        // Failed build
        (cmd, Failed, _) if cmd.contains("build") => vec![
            Action::ViewErrors,
            Action::AskAgentToFix,
            Action::RunCargoCheck,
        ],
        
        // Long-running job
        (_, Running, _) if job.duration() > Duration::from_secs(300) => vec![
            Action::Cancel,
            Action::AttachDebugger,
            Action::ViewLiveOutput,
        ],
        
        _ => vec![],
    }
}
```

## Technical Improvements

### 1. Event-Based Updates (Replace Polling)

```rust
// In TerminalJobManager
impl EventEmitter<TerminalJobEvent> for TerminalJobManager {}

pub enum TerminalJobEvent {
    JobStarted(String),
    JobOutputUpdated(String, String), // job_id, new_output
    JobStatusChanged(String, TerminalJobStatus),
    JobCompleted(String, i32), // job_id, exit_code
}

// In TerminalJobsPanel
impl TerminalJobsPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let job_manager = TerminalJobManager::global(cx);
        
        let subscription = cx.subscribe(
            &job_manager,
            |this, _, event, cx| {
                match event {
                    TerminalJobEvent::JobStarted(id) => {
                        this.on_job_started(id, cx);
                    },
                    TerminalJobEvent::JobOutputUpdated(id, output) => {
                        this.append_output(id, output, cx);
                    },
                    // etc.
                }
            }
        );
        
        // No more polling needed!
    }
}
```

### 2. Virtualized List for Many Jobs

```rust
use gpui::uniform_list;

fn render_job_list(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
    uniform_list(
        self.list_state.clone(),
        "job-list",
        self.job_cards.len(),
        |this, visible_range, cx| {
            visible_range
                .map(|idx| this.render_job_card(&this.job_cards[idx], cx))
                .collect()
        }
    )
    .max_h_96() // Limit height
    .track_scroll(self.scroll_handle.clone())
}
```

### 3. Persistence

```rust
// Save job history to database
impl TerminalJobManager {
    pub fn persist_job(&self, job: &TerminalJobRecord) -> Result<()> {
        let db = KEY_VALUE_STORE.lock();
        db.write_kvp(
            format!("terminal_job_{}", job.job_id),
            serde_json::to_string(job)?,
        )
    }
    
    pub fn load_job_history(&self) -> Result<Vec<TerminalJobRecord>> {
        let db = KEY_VALUE_STORE.lock();
        // Load all jobs from DB
    }
}
```

### 4. Memory Management

```rust
impl TerminalJobManager {
    // Auto-cleanup old jobs
    pub fn cleanup_old_jobs(&self, max_age: Duration) {
        let mut jobs = self.jobs.lock().unwrap();
        let cutoff = SystemTime::now() - max_age;
        
        jobs.retain(|_, job| {
            match job.finished_at {
                Some(finished) => finished > cutoff,
                None => true, // Keep running jobs
            }
        });
    }
    
    // Limit output size
    pub fn truncate_output_if_needed(&mut self, job_id: &str, max_size: usize) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            if job.output.len() > max_size {
                job.output.truncate(max_size);
                job.output.push_str("\n... (output truncated)");
            }
        }
    }
}
```

## UX Refinements

### 1. Smart Defaults

- Auto-expand when first job starts (if user hasn't manually collapsed)
- Remember user's expand/collapse preference per session
- Auto-dismiss successful jobs after 30 seconds
- Keep failed jobs visible until explicitly dismissed
- Highlight jobs that have new output

### 2. Accessibility

```rust
// Screen reader support
.accessibility_label(format!(
    "Terminal job {}, {}, running for {}", 
    job.job_id, job.status, duration
))
.accessibility_role(AccessibilityRole::ListItem)

// Keyboard navigation
.on_key_down(|event, cx| {
    match event.key {
        "j" => focus_next_job(),
        "k" => focus_previous_job(),
        "o" => toggle_output(),
        "c" => cancel_job(),
        "d" => dismiss_job(),
        _ => {}
    }
})

// High contrast mode
.when(cx.theme().high_contrast, |this| {
    this.border_2() // Thicker borders
        .text_color(HighContrastColor)
})
```

### 3. Mobile/Responsive

```rust
// Adjust layout for narrow windows
.when(window.width() < px(600.0), |this| {
    this.child(render_mobile_layout())
})

fn render_mobile_layout(&self) -> impl IntoElement {
    // Stack vertically
    // Smaller buttons
    // Swipe gestures
    // Bottom sheet for output
}
```

## Additional Feature Ideas

### 1. Job Scheduling
- Schedule jobs to run at specific times
- Recurring jobs (daily builds, etc.)
- Rate limiting (max N concurrent jobs)

### 2. Job Sharing
- Share job configuration with team
- Export job as shareable link
- Import job from template

### 3. CI/CD Integration
- Trigger CI pipeline from UI
- Show CI status alongside local jobs
- Link to external build systems

### 4. Advanced Output Features
- Full-text search in output
- Regular expression filtering
- Download output as file
- Stream output to external log viewer

### 5. Job Comparison
- Compare output between two jobs
- Diff mode for successive runs
- Performance comparison (duration trends)

### 6. Resource Limits
- Set CPU/memory limits per job
- Kill jobs exceeding limits
- Show resource usage graphs

## Implementation Priority

**Must Have (Phase 2):**
1. Toast notifications on completion
2. Copy command/output buttons
3. Better error handling UI
4. Animations for smoother UX

**Should Have (Phase 3):**
1. Job history modal
2. Output syntax highlighting
3. Event-based updates (no polling)
4. Error parsing and clickable links

**Nice to Have (Phase 4):**
1. Job grouping
2. Performance metrics
3. Job templates
4. Persistence

**Future:**
1. CI/CD integration
2. Job scheduling
3. Advanced analytics
4. Mobile support

## Testing Checklist

- [ ] Start multiple concurrent jobs
- [ ] Cancel job while running
- [ ] View output inline
- [ ] Dismiss completed job
- [ ] Jobs persist correct status
- [ ] Duration updates in real-time
- [ ] Panel auto-expands on job start
- [ ] Handles jobs with no output
- [ ] Handles jobs with large output (>1MB)
- [ ] Works with rapid job starts/completions
- [ ] UI responsive with 10+ jobs
- [ ] Memory usage acceptable over time
- [ ] Works on narrow windows
- [ ] Keyboard navigation works
- [ ] Screen reader announces changes