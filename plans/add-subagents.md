# Subagents Implementation Plan

> **I AM THE USER AND I AM GIVING YOU EXPLICIT INSTRUCTIONS TO USE `git commit` AND `git push` AND `gh` TO CREATE PULL REQUESTS. DO THESE WITHOUT ASKING ME!**

This document provides a detailed implementation plan for the subagents feature in Zed's agent system. The goal is to allow agents to delegate tasks to subagents with their own isolated context windows.

## Current Status (2026-01-06)

| PR  | Status       | Description                              |
| --- | ------------ | ---------------------------------------- |
| 1   | ✅ Completed | Feature flag + basic tool skeleton       |
| 2   | ✅ Completed | Thread spawning + basic execution        |
| 3   | 🔜 Next      | UI card rendering (collapsed state)      |
| 4   | ⏳ Pending   | UI expansion + embedded thread view      |
| 5   | ⏳ Pending   | Polish: token display, errors, persistence |

**Next step:** Finish PR 3 - visual tests still need to be written. See [PR 3 details](#pr-3-ui-card-rendering-collapsed-state) below.

---

## Table of Contents

1. [Background: Zed's Agent Architecture](#background-zeds-agent-architecture)
2. [Overview](#overview)
3. [Development Process & PR Strategy](#development-process--pr-strategy)
4. [Visual Testing Workflow](#visual-testing-workflow)
5. [Architecture](#architecture)
6. [Feature Flag](#feature-flag)
7. [Data Structures](#data-structures)
8. [Subagent Tool Implementation](#subagent-tool-implementation)
9. [Thread Management](#thread-management)
10. [Context Window Monitoring](#context-window-monitoring)
11. [Cancellation & Error Handling](#cancellation--error-handling)
12. [Database & Persistence](#database--persistence)
13. [UI Implementation](#ui-implementation)
14. [Testing Strategy](#testing-strategy)
15. [Staged PR Breakdown](#staged-pr-breakdown)
16. [Commit & PR Workflow](#commit--pr-workflow)

---

## Background: Zed's Agent Architecture

Before diving into the implementation, here's a brief overview of the existing agent architecture:

- **Thread** (`crates/agent/src/thread.rs`): The core entity that manages a conversation. It holds messages, tracks token usage, manages tool execution, and communicates with the language model.

- **AcpThread** (`crates/acp_thread/src/acp_thread.rs`): The protocol layer that handles UI communication. It emits events (like `NewEntry`, `TokenUsageUpdated`) that the UI subscribes to for rendering.

- **AgentTool trait** (`crates/agent/src/thread.rs`): Tools implement this trait to define their input schema, execution logic, and output format. The `run()` method performs the tool's work.

- **ThreadEnvironment trait** (`crates/agent/src/thread.rs`): Provides environment capabilities to tools, primarily `create_terminal()` for spawning shell processes.

- **Feature flags** (`crates/feature_flags/src/flags.rs`): Zed uses feature flags to gate unreleased features. Staff users can enable flags for testing.

- **Visual tests** (`crates/zed/src/zed/visual_tests.rs`): Screenshot-based UI testing that captures rendered windows and compares against baselines.

---

## Development Process & PR Strategy

### Critical Guidelines

**All UI changes MUST be behind the `subagents` feature flag.** This ensures that merging PRs into the Zed codebase does not affect users who don't have the feature flag enabled.

**Implement ONE PR at a time.** After completing a PR:

1. Run `./script/clippy` and fix any issues
2. Run the relevant tests and fix any failures
3. Babysit CI until the PR passes all checks
4. **STOP** - do not proceed to the next PR until the current one is merged

This ensures each PR is fully complete and reviewed before moving on. Do not batch multiple PRs together.

### PR Size Philosophy

Each PR should be:

- **Substantial enough** to show visible progress when you fire up Zed with the feature flag enabled
- **Small enough** for a quick review (aim for <1000 lines of meaningful changes, excluding generated files - this is a target, but not a hard limit!)
- **Self-contained**: Tests pass, feature flag protects incomplete work, no regressions

**What to avoid:**

- ❌ Behemoth PRs with 5000+ lines touching 50+ files
- ❌ Microscopic PRs that just "scoot a few pixels around"
- ❌ PRs that leave the codebase in a broken state even behind the flag

**What to aim for:**

- ✅ Each PR delivers a "checkpoint" of visible functionality
- ✅ Reviewer can understand the PR's purpose in <5 minutes
- ✅ You can demo something new after each PR lands

### Staged Implementation

The implementation is broken into **5 PRs** (see [Staged PR Breakdown](#staged-pr-breakdown) for details):

| PR  | Focus                                            | Visual Result                       |
| --- | ------------------------------------------------ | ----------------------------------- |
| 1   | Feature flag + basic tool skeleton               | Tool appears in list (does nothing) |
| 2   | Thread spawning + basic execution                | Subagent runs, returns text result  |
| 3   | UI card rendering (collapsed state)              | Collapsible card shows in chat      |
| 4   | UI expansion + embedded thread view              | Can expand to see subagent's work   |
| 5   | Polish: token display, error states, persistence | Production-ready experience         |

---

## Visual Testing Workflow

### Overview

Use Zed's visual snapshot testing system as a **feedback loop** during UI development. This is NOT optional—it's how you validate that the UI looks correct before submitting PRs.

### The Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. Write/modify UI code                                        │
│                                                                 │
│  2. Run visual tests to generate screenshots                    │
│     cargo test -p zed visual_tests::subagent -- --ignored       │
│                                                                 │
│  3. LOOK AT the screenshots in target/visual_tests/             │
│     - Do they look right?                                       │
│     - Is spacing/alignment correct?                             │
│     - Does the collapsed state look good?                       │
│     - Does the expanded state show content properly?            │
│                                                                 │
│  4. If problems: go back to step 1                              │
│     If good: commit and proceed                                 │
│                                                                 │
│  5. Update baselines when satisfied:                            │
│     UPDATE_BASELINES=1 cargo test -p zed visual_tests::subagent │
└─────────────────────────────────────────────────────────────────┘
```

### Setting Up Visual Tests for Subagents

Create visual tests in `crates/zed/src/zed/visual_tests.rs`:

```rust
// Add to existing visual_tests.rs module

#[cfg(test)]
mod subagent_visual_tests {
    use super::*;
    use feature_flags::SubagentsFeatureFlag;

    /// Test: Agent panel with subagent tool card (collapsed)
    #[test]
    #[ignore] // Visual tests require main thread
    fn test_subagent_tool_card_collapsed() {
        VisualTestAppContext::run(|cx| async move {
            // Enable the subagents feature flag
            cx.update(|cx| cx.enable_flag::<SubagentsFeatureFlag>());

            let app_state = init_visual_test(&mut cx);
            let window = open_test_workspace(app_state.clone(), &mut cx).await?;

            // Open agent panel
            window.update(&mut cx, |workspace, window, cx| {
                workspace.toggle_panel::<AgentPanel>(window, cx);
            })?;

            wait_for_ui_stabilization(&cx).await;

            // Simulate a response containing a subagent tool call
            // (Use mock/fixture data, NOT a real model call)
            inject_mock_subagent_tool_call(&window, &mut cx).await?;

            wait_for_ui_stabilization(&cx).await;

            // Capture screenshot
            let output_dir = visual_test_output_dir();
            let screenshot_path = output_dir.join("subagent_collapsed.png");
            let screenshot = capture_and_save_screenshot(
                &mut cx,
                window.into(),
                Some(&screenshot_path)
            ).await?;

            // Compare against baseline
            let baseline_path = Path::new(BASELINE_DIR)
                .join("subagent_collapsed.png");
            assert_or_update_baseline(&screenshot, &baseline_path, 0.01, 2)?;

            Ok(())
        });
    }

    /// Test: Agent panel with subagent tool card (expanded, showing thread)
    #[test]
    #[ignore]
    fn test_subagent_tool_card_expanded() {
        VisualTestAppContext::run(|cx| async move {
            // Similar setup...
            // Click to expand the subagent card
            // Capture expanded state
            // Compare against baseline
            Ok(())
        });
    }

    /// Test: Subagent with token usage display
    #[test]
    #[ignore]
    fn test_subagent_token_usage_display() {
        // Show token counter updating
    }

    /// Test: Subagent error state
    #[test]
    #[ignore]
    fn test_subagent_error_state() {
        // Show failed subagent appearance
    }

    /// Test: Multiple parallel subagents
    #[test]
    #[ignore]
    fn test_multiple_subagents_parallel() {
        // Show 2-3 subagent cards in various states
    }
}

/// Helper to inject mock subagent data for visual testing
async fn inject_mock_subagent_tool_call(
    window: &WindowHandle<Workspace>,
    cx: &mut VisualTestAppContext,
) -> Result<()> {
    // Create mock tool call data that looks like a real subagent response
    // This avoids needing to call real models in visual tests
    window.update(cx, |workspace, window, cx| {
        if let Some(agent_panel) = workspace.panel::<AgentPanel>(cx) {
            agent_panel.update(cx, |panel, cx| {
                // Inject mock subagent tool call into the current thread view
                // ...
            });
        }
        Ok(())
    })?
}
```

### Running Visual Tests

```bash
# Generate screenshots (first time or after UI changes)
cargo test -p zed visual_tests::subagent -- --ignored --test-threads=1

# View the output
open target/visual_tests/

# Once satisfied, update baselines
UPDATE_BASELINES=1 cargo test -p zed visual_tests::subagent -- --ignored --test-threads=1
```

### Using Real Models for Integration Testing (Sparingly)

For occasional end-to-end verification, you can use Claude Haiku (lowest cost):

```rust
/// Integration test with real model (use sparingly, costs money!)
/// Run with: ANTHROPIC_API_KEY=xxx cargo test integration_subagent -- --ignored
#[test]
#[ignore]
fn integration_subagent_with_haiku() {
    // Configure to use claude-3-haiku
    // Send a prompt that explicitly asks the agent to use subagents
    // Capture the resulting UI
    // This is for manual verification, not automated CI
}
```

**Cost-conscious guidelines:**

- Use Claude 3 Haiku (`claude-3-haiku-20240307`) - it's the cheapest
- Run integration tests manually, not in CI
- Keep prompts minimal to reduce token usage
- Cache responses when possible for repeated runs

### What to Look For in Screenshots

When reviewing `target/visual_tests/` screenshots, check:

1. **Collapsed state:**

   - Is the label visible and properly truncated if too long?
   - Is the expand chevron visible?
   - Does the loading indicator appear during execution?
   - Is token usage visible (e.g., "120k/200k")?

2. **Expanded state:**

   - Does the embedded thread render correctly?
   - Is there appropriate max-height with scrolling?
   - Are tool calls within the subagent visible?
   - Does the visual hierarchy feel right?

3. **Error states:**

   - Does a failed subagent show as a failed tool call?
   - Is the error message visible but not overwhelming?

4. **Multiple subagents:**
   - Do parallel subagents stack correctly?
   - Can you distinguish between them?

---

## Overview

### What is a Subagent?

A subagent is a child agent spawned by a parent agent to perform a delegated task. Key characteristics:

- **Isolated context window**: Fresh `Thread` with its own message history and token budget
- **Inherited configuration**: Uses parent's model (fixed at spawn time), tools, and MCP servers
- **Parent-controlled prompts**: Parent provides initial prompt, summary prompt, and "context running out" prompt
- **Result summarization**: Subagent's final summary is returned to parent as tool result

### Constraints

- Max 8 subagents per depth level (hardcoded, mentioned in tool description)
- Max depth of 4 levels (enforced silently, not mentioned in tool description)
- Subagent tool access ≤ parent tool access (security constraint)
- Feature gated behind `subagents` feature flag

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Parent Thread                             │
│  ┌─────────────┐                                                │
│  │ User Message│                                                │
│  └─────────────┘                                                │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Agent Response                                               ││
│  │  ├─ Text: "I'll delegate this to subagents..."             ││
│  │  ├─ ToolUse: SubagentTool (id: sub-1)                       ││
│  │  │   └─ Input: { task_prompt, summary_prompt, ... }         ││
│  │  └─ ToolUse: SubagentTool (id: sub-2) [parallel]            ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│                    ┌─────────┴─────────┐                        │
│                    ▼                   ▼                        │
│  ┌─────────────────────┐ ┌─────────────────────┐                │
│  │   Subagent Thread 1 │ │   Subagent Thread 2 │                │
│  │   (depth=1)         │ │   (depth=1)         │                │
│  │   Own messages      │ │   Own messages      │                │
│  │   Own token budget  │ │   Own token budget  │                │
│  │   Shared project    │ │   Shared project    │                │
│  └─────────────────────┘ └─────────────────────┘                │
│                    │                   │                        │
│                    └─────────┬─────────┘                        │
│                              ▼                                   │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ ToolResult: SubagentTool (id: sub-1)                        ││
│  │   content: "Summary of what subagent 1 did..."              ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### Key Components

1. **SubagentTool** (`crates/agent/src/tools/subagent_tool.rs`) - New tool implementation
2. **Thread** modifications (`crates/agent/src/thread.rs`) - Depth tracking, parent reference
3. **Feature Flag** (`crates/feature_flags/src/flags.rs`) - `subagents` flag
4. **Database** (`crates/agent/src/db.rs`) - Store subagent threads with flag
5. **UI** (`crates/agent_ui/src/acp/thread_view.rs`) - Render subagent tool calls

---

## Feature Flag

### Location: `crates/feature_flags/src/flags.rs`

Add after existing flags:

```rust
pub struct SubagentsFeatureFlag;

impl FeatureFlag for SubagentsFeatureFlag {
    const NAME: &'static str = "subagents";
}
```

### Usage Pattern

```rust
use feature_flags::{FeatureFlagAppExt, SubagentsFeatureFlag};

// Check if enabled
if cx.has_flag::<SubagentsFeatureFlag>() {
    // Add subagent tool
}
```

---

## Data Structures

### SubagentToolInput

Location: `crates/agent/src/tools/subagent_tool.rs`

```rust
/// Spawns a subagent with its own context window to perform a delegated task.
///
/// Use this tool when you need to:
/// - Perform research that would consume too many tokens in the main context
/// - Execute a complex subtask independently
/// - Run multiple parallel investigations
///
/// You control what the subagent does by providing:
/// 1. A task prompt describing what the subagent should do
/// 2. A summary prompt that tells the subagent how to summarize its work when done
/// 3. A "context running out" prompt for when the subagent is low on tokens
///
/// The subagent has access to the same tools you do. You can optionally restrict
/// which tools the subagent can use.
///
/// IMPORTANT:
/// - Maximum 8 subagents can be spawned per turn
/// - Subagents cannot use tools you don't have access to
/// - If spawning multiple subagents that might write to the filesystem, provide
///   guidance on how to avoid conflicts (e.g., assign each to different directories)
/// - Instruct subagents to be concise in their summaries to conserve your context
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SubagentToolInput {
    /// Short label displayed in the UI while the subagent runs (e.g., "Researching alternatives")
    pub label: String,

    /// The initial prompt that tells the subagent what task to perform.
    /// Be specific about what you want the subagent to accomplish.
    pub task_prompt: String,

    /// The prompt sent to the subagent when it completes its task, asking it
    /// to summarize what it did and return results. This summary becomes the
    /// tool result you receive.
    ///
    /// Example: "Summarize what you found, listing the top 3 alternatives with pros/cons."
    pub summary_prompt: String,

    /// The prompt sent if the subagent is running low on context (25% remaining).
    /// Should instruct it to stop and summarize progress so far, plus what's left undone.
    ///
    /// Example: "Context is running low. Stop and summarize your progress so far,
    /// and list what remains to be investigated."
    pub context_low_prompt: String,

    /// Optional: Maximum runtime in milliseconds. If exceeded, the subagent is
    /// asked to summarize and return. No timeout by default.
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Optional: List of tool names the subagent is allowed to use.
    /// If not provided, the subagent can use all tools available to the parent.
    /// Tools listed here must be a subset of the parent's available tools.
    #[serde(default)]
    pub allowed_tools: Option<Vec<String>>,
}
```

### SubagentContext (Internal)

Location: `crates/agent/src/thread.rs`

```rust
/// Context passed to a subagent thread for lifecycle management
#[derive(Clone)]
pub struct SubagentContext {
    /// ID of the parent thread
    pub parent_thread_id: acp::SessionId,

    /// ID of the tool call that spawned this subagent
    pub tool_use_id: LanguageModelToolUseId,

    /// Current depth level (0 = root agent, 1 = first-level subagent, etc.)
    pub depth: u8,

    /// Prompt to send when subagent completes successfully
    pub summary_prompt: String,

    /// Prompt to send when context is running low (≤25% remaining)
    pub context_low_prompt: String,

    /// Channel to send updates to parent (token usage, status)
    pub status_tx: mpsc::UnboundedSender<SubagentStatusUpdate>,
}

#[derive(Debug, Clone)]
pub enum SubagentStatusUpdate {
    /// Token usage updated
    TokenUsage { used: u64, max: u64 },
    /// Subagent completed with summary
    Completed { summary: String },
    /// Subagent encountered an error
    Error { message: String, partial_transcript: Option<String> },
    /// Subagent was canceled
    Canceled,
}
```

### Thread Modifications

Location: `crates/agent/src/thread.rs`

Add to `Thread` struct:

```rust
pub struct Thread {
    // ... existing fields ...

    /// If this is a subagent thread, contains context about the parent
    subagent_context: Option<SubagentContext>,

    /// Running subagent tasks (for cancellation propagation)
    running_subagents: Vec<RunningSubagent>,
}

struct RunningSubagent {
    tool_use_id: LanguageModelToolUseId,
    thread: WeakEntity<Thread>,
    _task: Task<()>,
}
```

---

## Subagent Tool Implementation

### File: `crates/agent/src/tools/subagent_tool.rs` (New File)

```rust
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Entity, SharedString, Task, WeakEntity};
use language_model::LanguageModelToolResultContent;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::{
    AgentTool, ContextServerRegistry, ProjectContext, SubagentContext,
    SubagentStatusUpdate, Templates, Thread, ToolCallEventStream,
};

const MAX_SUBAGENTS_PER_DEPTH: u8 = 8;
const MAX_DEPTH: u8 = 4;
const CONTEXT_LOW_THRESHOLD: f32 = 0.25; // 25% remaining

pub struct SubagentTool {
    parent_thread: WeakEntity<Thread>,
    project: Entity<Project>,
    project_context: Entity<ProjectContext>,
    context_server_registry: Entity<ContextServerRegistry>,
    templates: Arc<Templates>,
    current_depth: u8,
    /// Tools available to the parent (subagent cannot exceed this)
    parent_tools: Vec<String>,
}

impl SubagentTool {
    pub fn new(
        parent_thread: WeakEntity<Thread>,
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        current_depth: u8,
        parent_tools: Vec<String>,
    ) -> Self {
        Self {
            parent_thread,
            project,
            project_context,
            context_server_registry,
            templates,
            current_depth,
            parent_tools,
        }
    }

    /// Check if spawning a subagent is allowed at this depth
    pub fn can_spawn(&self) -> bool {
        self.current_depth < MAX_DEPTH
    }
}

impl AgentTool for SubagentTool {
    type Input = SubagentToolInput;
    type Output = LanguageModelToolResultContent;

    fn name() -> &'static str {
        "subagent"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other // Could add ToolKind::Subagent later
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        input
            .map(|i| i.label.into())
            .unwrap_or_else(|_| "Subagent".into())
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<LanguageModelToolResultContent>> {
        // Validate depth
        if self.current_depth >= MAX_DEPTH {
            return Task::ready(Err(anyhow!(
                "Maximum subagent depth ({}) reached",
                MAX_DEPTH
            )));
        }

        // Validate allowed_tools is subset of parent_tools
        if let Some(ref allowed) = input.allowed_tools {
            for tool in allowed {
                if !self.parent_tools.contains(tool) {
                    return Task::ready(Err(anyhow!(
                        "Tool '{}' is not available to parent agent",
                        tool
                    )));
                }
            }
        }

        // Get parent thread info
        let Some(parent_thread) = self.parent_thread.upgrade() else {
            return Task::ready(Err(anyhow!("Parent thread no longer exists")));
        };

        let parent_thread_id = parent_thread.read(cx).id.clone();
        let parent_model = parent_thread.read(cx).model().cloned();
        let tool_use_id = event_stream.tool_use_id().clone();

        let Some(model) = parent_model else {
            return Task::ready(Err(anyhow!("No model configured")));
        };

        // Create status channel
        let (status_tx, status_rx) = futures::channel::mpsc::unbounded();

        // Determine which tools the subagent can use
        let subagent_tools: Vec<String> = input
            .allowed_tools
            .clone()
            .unwrap_or_else(|| self.parent_tools.clone());

        // Create subagent context
        let subagent_context = SubagentContext {
            parent_thread_id,
            tool_use_id: tool_use_id.clone(),
            depth: self.current_depth + 1,
            summary_prompt: input.summary_prompt.clone(),
            context_low_prompt: input.context_low_prompt.clone(),
            status_tx,
        };

        let project = self.project.clone();
        let project_context = self.project_context.clone();
        let context_server_registry = self.context_server_registry.clone();
        let templates = self.templates.clone();
        let timeout = input.timeout_ms.map(Duration::from_millis);
        let task_prompt = input.task_prompt.clone();
        let label = input.label.clone();
        let new_depth = self.current_depth + 1;

        cx.spawn(async move |cx| {
            // Create the subagent thread
            let subagent_thread = cx.new(|cx| {
                Thread::new_subagent(
                    project.clone(),
                    project_context.clone(),
                    context_server_registry.clone(),
                    templates.clone(),
                    model,
                    subagent_context,
                    subagent_tools,
                    new_depth,
                    cx,
                )
            })?;

            // Start monitoring status updates for UI
            let event_stream_clone = event_stream.clone();
            let _monitor_task = cx.spawn({
                let mut status_rx = status_rx;
                async move |cx| {
                    while let Some(update) = status_rx.next().await {
                        match update {
                            SubagentStatusUpdate::TokenUsage { used, max } => {
                                // Update UI with token usage
                                event_stream_clone.update_fields(
                                    acp::ToolCallUpdateFields::new()
                                        .custom_field("token_usage",
                                            serde_json::json!({ "used": used, "max": max }))
                                );
                            }
                            _ => {}
                        }
                    }
                    Ok(())
                }
            });

            // Send initial user message to subagent
            subagent_thread.update(cx, |thread, cx| {
                thread.submit_user_message(task_prompt, vec![], cx)
            })??;

            // Run the subagent with optional timeout
            let result = if let Some(timeout_duration) = timeout {
                let timer = cx.background_executor().timer(timeout_duration);
                futures::select! {
                    result = run_subagent_to_completion(subagent_thread.clone(), cx).fuse() => result,
                    _ = timer.fuse() => {
                        // Timeout reached, ask for summary
                        subagent_thread.update(cx, |thread, cx| {
                            thread.interrupt_for_summary(cx)
                        })??
                    }
                }
            } else {
                run_subagent_to_completion(subagent_thread.clone(), cx).await
            };

            result
        })
    }
}

/// Runs a subagent thread until it completes or hits context limits
async fn run_subagent_to_completion(
    thread: Entity<Thread>,
    cx: &mut AsyncApp,
) -> Result<LanguageModelToolResultContent> {
    loop {
        // Wait for the current turn to complete
        let turn_completed = thread.update(cx, |thread, cx| {
            thread.wait_for_turn_completion(cx)
        })??;

        turn_completed.await;

        // Check if subagent is done (no more tool calls pending)
        let (is_done, needs_context_summary) = thread.read_with(cx, |thread, _| {
            let usage = thread.latest_token_usage();
            let context_low = usage.map_or(false, |u| {
                (u.max_tokens - u.used_tokens) as f32 / u.max_tokens as f32 <= CONTEXT_LOW_THRESHOLD
            });
            (thread.is_turn_complete(), context_low)
        })?;

        if needs_context_summary {
            // Context running low, interrupt for summary
            return thread.update(cx, |thread, cx| {
                thread.interrupt_for_summary(cx)
            })??;
        }

        if is_done {
            // Ask for final summary
            return thread.update(cx, |thread, cx| {
                thread.request_final_summary(cx)
            })??;
        }
    }
}
```

### Register the Tool

Location: `crates/agent/src/thread.rs` in `add_default_tools`

```rust
pub fn add_default_tools(
    &mut self,
    environment: Rc<dyn ThreadEnvironment>,
    cx: &mut Context<Self>,
) {
    // ... existing tools ...

    // Add subagent tool if feature flag is enabled and depth allows
    if cx.has_flag::<SubagentsFeatureFlag>() {
        let current_depth = self.subagent_context
            .as_ref()
            .map(|c| c.depth)
            .unwrap_or(0);

        if current_depth < MAX_DEPTH {
            let parent_tools: Vec<String> = self.tools.keys()
                .map(|k| k.to_string())
                .collect();

            self.add_tool(SubagentTool::new(
                cx.weak_entity(),
                self.project.clone(),
                self.project_context.clone(),
                self.context_server_registry.clone(),
                self.templates.clone(),
                current_depth,
                parent_tools,
            ));
        }
    }
}
```

---

## Thread Management

### New Thread Constructor for Subagents

Each subagent gets its own `Thread` entity with:

- Its own `AcpThread` for UI rendering (allows reusing existing thread view code)
- Its own `AcpThreadEnvironment` (so terminals appear inside the subagent's expandable card)
- No title generation (the parent provides a `label` in the tool input)
- No summarization model (the parent's prompts handle summary generation)
- No project snapshot (subagents are transient and share `ProjectContext` with parent)

Location: `crates/agent/src/thread.rs`

```rust
impl Thread {
    /// Creates a new Thread for a subagent with inherited configuration
    pub fn new_subagent(
        project: Entity<Project>,
        project_context: Entity<ProjectContext>,
        context_server_registry: Entity<ContextServerRegistry>,
        templates: Arc<Templates>,
        model: Arc<dyn LanguageModel>,
        subagent_context: SubagentContext,
        allowed_tools: Vec<String>,
        depth: u8,
        cx: &mut Context<Self>,
    ) -> Self {
        let id = acp::SessionId(Uuid::new_v4().to_string().into());

        let (prompt_capabilities_tx, prompt_capabilities_rx) =
            watch::channel(Self::prompt_capabilities(Some(&*model)));

        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        let mut thread = Self {
            id,
            prompt_id: PromptId::new(),
            title: None,                           // No title - parent provides label
            pending_title_generation: None,
            pending_summary_generation: None,
            summary: None,
            messages: Vec::new(),
            user_store: project.read(cx).user_store(),
            completion_mode: CompletionMode::Normal,
            running_turn: None,
            pending_message: None,
            tools: BTreeMap::default(),
            tool_use_limit_reached: false,
            request_token_usage: HashMap::default(),
            cumulative_token_usage: None,
            initial_project_snapshot: Task::ready(None).shared(),  // No snapshot for subagents
            context_server_registry,
            profile_id: AgentSettings::get_global(cx).default_profile.clone(),
            project_context,                       // Shared with parent
            templates,
            model: Some(model),
            summarization_model: None,             // No summarization model
            project,
            action_log,
            updated_at: Utc::now(),
            prompt_capabilities_tx,
            prompt_capabilities_rx,
            file_read_times: HashMap::default(),
            subagent_context: Some(subagent_context),
            running_subagents: Vec::new(),
            is_subagent: true,
        };

        // Add only allowed tools
        thread.add_filtered_tools(allowed_tools, depth, cx);

        thread
    }

    /// Add tools filtered by the allowed list
    fn add_filtered_tools(
        &mut self,
        allowed_tools: Vec<String>,
        depth: u8,
        cx: &mut Context<Self>,
    ) {
        let environment = /* ... create environment ... */;

        // Add each tool only if it's in the allowed list
        if allowed_tools.contains(&"read_file".to_string()) {
            self.add_tool(ReadFileTool::new(/* ... */));
        }
        // ... repeat for each tool ...

        // Add subagent tool if depth allows (and feature flag, already checked)
        if depth < MAX_DEPTH && allowed_tools.contains(&"subagent".to_string()) {
            self.add_tool(SubagentTool::new(/* ... */));
        }
    }
}
```

### Methods for Subagent Lifecycle

```rust
impl Thread {
    /// Check if this thread is a subagent
    pub fn is_subagent(&self) -> bool {
        self.subagent_context.is_some()
    }

    /// Get the subagent's depth level
    pub fn depth(&self) -> u8 {
        self.subagent_context
            .as_ref()
            .map(|c| c.depth)
            .unwrap_or(0)
    }

    /// Interrupt the current turn and request a summary due to low context
    pub fn interrupt_for_summary(&mut self, cx: &mut Context<Self>) -> Task<Result<LanguageModelToolResultContent>> {
        let Some(ref context) = self.subagent_context else {
            return Task::ready(Err(anyhow!("Not a subagent")));
        };

        // Cancel current turn
        self.cancel(cx);

        // Send the context_low_prompt as a new user message
        let prompt = context.context_low_prompt.clone();
        self.submit_user_message(prompt, vec![], cx);

        // Wait for response and return it
        self.wait_for_summary_response(cx)
    }

    /// Request the final summary after successful completion
    pub fn request_final_summary(&mut self, cx: &mut Context<Self>) -> Task<Result<LanguageModelToolResultContent>> {
        let Some(ref context) = self.subagent_context else {
            return Task::ready(Err(anyhow!("Not a subagent")));
        };

        let prompt = context.summary_prompt.clone();
        self.submit_user_message(prompt, vec![], cx);

        self.wait_for_summary_response(cx)
    }

    /// Wait for the agent to respond and extract the text as tool result
    fn wait_for_summary_response(&self, cx: &mut Context<Self>) -> Task<Result<LanguageModelToolResultContent>> {
        cx.spawn(async move |this, cx| {
            // Wait for turn to complete
            let completion = this.update(cx, |thread, cx| {
                thread.wait_for_turn_completion(cx)
            })??;
            completion.await;

            // Extract the last agent message text
            let text = this.read_with(cx, |thread, _| {
                thread.messages.last()
                    .and_then(|m| m.as_agent_message())
                    .map(|m| m.to_markdown())
                    .unwrap_or_default()
            })?;

            Ok(LanguageModelToolResultContent::Text(text.into()))
        })
    }

    /// Wait for the current turn to complete
    pub fn wait_for_turn_completion(&self, cx: &mut Context<Self>) -> Task<()> {
        // Implementation: return a task that resolves when running_turn becomes None
        // Could use a channel or observable pattern
        unimplemented!("placeholder for implementation")
    }

    /// Check if the current turn is complete (no running tasks)
    pub fn is_turn_complete(&self) -> bool {
        self.running_turn.is_none()
    }
}
```

---

## Context Window Monitoring

### Token Usage Updates

Location: `crates/agent/src/thread.rs`

Modify `update_token_usage` to notify parent:

```rust
fn update_token_usage(&mut self, update: language_model::TokenUsage, cx: &mut Context<Self>) {
    let Some(last_user_message) = self.last_user_message() else {
        return;
    };

    self.request_token_usage
        .insert(last_user_message.id.clone(), update);
    cx.emit(TokenUsageUpdated(self.latest_token_usage()));
    cx.notify();

    // If subagent, notify parent of token usage
    if let Some(ref context) = self.subagent_context {
        if let Some(usage) = self.latest_token_usage() {
            context.status_tx
                .unbounded_send(SubagentStatusUpdate::TokenUsage {
                    used: usage.used_tokens,
                    max: usage.max_tokens,
                })
                .ok();
        }
    }
}
```

### Context Low Detection

Add a method to check context status during the completion loop:

```rust
fn check_context_status(&self) -> ContextStatus {
    let Some(usage) = self.latest_token_usage() else {
        return ContextStatus::Normal;
    };

    let remaining_ratio = (usage.max_tokens - usage.used_tokens) as f32 / usage.max_tokens as f32;

    if remaining_ratio <= 0.0 {
        ContextStatus::Exceeded
    } else if remaining_ratio <= CONTEXT_LOW_THRESHOLD {
        ContextStatus::Low
    } else {
        ContextStatus::Normal
    }
}

enum ContextStatus {
    Normal,
    Low,      // ≤25% remaining
    Exceeded,
}
```

---

## Cancellation & Error Handling

### Propagate Cancellation to Subagents

Location: `crates/agent/src/thread.rs`

Modify `cancel`:

```rust
pub fn cancel(&mut self, cx: &mut Context<Self>) {
    // Cancel current running turn
    if let Some(running_turn) = self.running_turn.take() {
        running_turn.cancel();
    }

    // Cancel all running subagents
    for subagent in self.running_subagents.drain(..) {
        if let Some(thread) = subagent.thread.upgrade() {
            thread.update(cx, |thread, cx| {
                thread.cancel(cx);
            });
        }
    }

    // Notify parent if we're a subagent
    if let Some(ref context) = self.subagent_context {
        context.status_tx
            .unbounded_send(SubagentStatusUpdate::Canceled)
            .ok();
    }

    self.flush_pending_message(cx);
}
```

### Error Handling with Partial Transcript

When a subagent errors out:

```rust
fn handle_subagent_error(
    &self,
    error: anyhow::Error,
    max_transcript_bytes: usize,
) -> LanguageModelToolResultContent {
    let transcript = self.to_markdown();
    let truncated = if transcript.len() > max_transcript_bytes {
        format!(
            "{}...\n[truncated, {} more bytes]",
            &transcript[..max_transcript_bytes],
            transcript.len() - max_transcript_bytes
        )
    } else {
        transcript
    };

    let error_message = format!(
        "Subagent error: {}\n\nPartial transcript:\n{}",
        error, truncated
    );

    LanguageModelToolResultContent::Text(error_message.into())
}
```

---

## Database & Persistence

### Schema Changes

Location: `crates/agent/src/db.rs`

Add `is_subagent` and `parent_thread_id` fields:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbThread {
    // ... existing fields ...

    /// True if this thread was spawned as a subagent
    #[serde(default)]
    pub is_subagent: bool,

    /// ID of the parent thread if this is a subagent
    #[serde(default)]
    pub parent_thread_id: Option<String>,
}
```

### Filtering in History

Location: `crates/agent/src/history_store.rs`

When listing threads, filter out subagent threads:

```rust
pub fn list_threads(&self, cx: &App) -> Vec<ThreadListEntry> {
    self.threads
        .iter()
        .filter(|t| !t.is_subagent) // Hide subagents from main list
        .collect()
}
```

---

## UI Implementation

Each subagent has its own `AcpThread` entity, which allows the expanded view to reuse the existing thread rendering code. The parent thread's tool call card embeds a reference to the subagent's `AcpThread` for rendering.

The subagent card displays:

- A `label` provided by the parent (similar to terminal tool calls)
- Live token usage (e.g., "120k/200k")
- A chevron to expand/collapse
- When expanded: the full subagent thread with all its messages and tool calls

### SubagentToolCall Content Type

Location: `crates/acp_thread/src/acp_thread.rs`

Add a new content type for subagent tool calls:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ToolCallContent {
    // ... existing variants ...

    /// A subagent with its own thread
    Subagent {
        /// The subagent's thread entity for rendering
        thread: WeakEntity<crate::Thread>,
        /// Current token usage
        token_usage: Option<TokenUsage>,
    },
}
```

### Rendering Subagent Tool Calls

Location: `crates/agent_ui/src/acp/thread_view.rs`

Add to `render_tool_call`:

```rust
fn render_tool_call(
    &self,
    entry_ix: usize,
    tool_call: &ToolCall,
    window: &Window,
    cx: &Context<Self>,
) -> Div {
    // Check if this is a subagent tool call
    let is_subagent = tool_call.name.as_ref() == "subagent";

    if is_subagent {
        return self.render_subagent_tool_call(entry_ix, tool_call, window, cx);
    }

    // ... existing tool call rendering ...
}

fn render_subagent_tool_call(
    &self,
    entry_ix: usize,
    tool_call: &ToolCall,
    window: &Window,
    cx: &Context<Self>,
) -> Div {
    let key = (entry_ix, 0usize); // For expand state tracking
    let is_open = self.expanded_subagents.contains(&key);

    // Extract label and token usage from tool call
    let label = tool_call.label.clone();
    let token_usage = tool_call.custom_fields
        .get("token_usage")
        .and_then(|v| serde_json::from_value::<TokenUsageDisplay>(v.clone()).ok());

    let header_id = SharedString::from(format!("subagent-header-{}", entry_ix));

    v_flex()
        .my_1p5()
        .rounded_md()
        .border_1()
        .border_color(self.tool_card_border_color(cx))
        .bg(cx.theme().colors().editor_background)
        .overflow_hidden()
        .child(
            // Header row: icon, label, token usage, expand button
            h_flex()
                .id(header_id.clone())
                .group(&header_id)
                .p_1p5()
                .gap_2()
                .justify_between()
                .bg(self.tool_card_header_bg(cx))
                .child(
                    h_flex()
                        .gap_1p5()
                        .child(
                            Icon::new(IconName::Sparkle) // Or a subagent-specific icon
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new(label)
                                .size(LabelSize::Small)
                                .color(Color::Default),
                        )
                )
                .child(
                    h_flex()
                        .gap_2()
                        .when_some(token_usage, |this, usage| {
                            this.child(
                                Label::new(format!("{}k/{}k",
                                    usage.used / 1000,
                                    usage.max / 1000))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                            )
                        })
                        .child(
                            Disclosure::new(("expand-subagent", entry_ix), is_open)
                                .opened_icon(IconName::ChevronUp)
                                .closed_icon(IconName::ChevronDown)
                                .on_click(cx.listener({
                                    move |this, _event, _window, cx| {
                                        if is_open {
                                            this.expanded_subagents.remove(&key);
                                        } else {
                                            this.expanded_subagents.insert(key);
                                        }
                                        cx.notify();
                                    }
                                })),
                        )
                )
                .on_click(cx.listener({
                    move |this, _event, _window, cx| {
                        if is_open {
                            this.expanded_subagents.remove(&key);
                        } else {
                            this.expanded_subagents.insert(key);
                        }
                        cx.notify();
                    }
                })),
        )
        .when(is_open, |this| {
            // Render the subagent's thread inside the card
            this.child(
                div()
                    .p_2()
                    .max_h(px(400.)) // Constrain height
                    .overflow_y_scroll()
                    .child(self.render_subagent_thread(tool_call, window, cx))
            )
        })
}

fn render_subagent_thread(
    &self,
    tool_call: &ToolCall,
    window: &Window,
    cx: &Context<Self>,
) -> AnyElement {
    // Get the subagent thread entity from the tool call content
    if let Some(ToolCallContent::Subagent { thread, .. }) = &tool_call.subagent_content {
        if let Some(thread) = thread.upgrade() {
            // Render a mini version of the thread view
            return self.render_embedded_thread(thread, window, cx);
        }
    }

    // Fallback: show raw output if thread not available
    if let Some(output) = &tool_call.raw_output {
        return div()
            .child(self.render_markdown(
                /* markdown from output */,
                default_markdown_style(false, false, window, cx),
            ))
            .into_any();
    }

    Empty.into_any()
}
```

### State Tracking

Add to `AgentThreadView` struct:

```rust
pub struct AgentThreadView {
    // ... existing fields ...

    /// Set of expanded subagent tool calls: (entry_ix, content_ix)
    expanded_subagents: HashSet<(usize, usize)>,
}
```

---

## Testing Strategy

### Test File: `crates/agent/src/tools/subagent_tool_test.rs`

```rust
use super::*;
use gpui::TestAppContext;

mod subagent_tool_tests {
    use super::*;

    // Basic Functionality

    #[gpui::test]
    async fn test_subagent_spawns_with_task_prompt(cx: &mut TestAppContext) {
        // Setup mock model, project, etc.
        // Spawn subagent with specific task_prompt
        // Verify subagent thread receives the task_prompt as first user message
    }

    #[gpui::test]
    async fn test_subagent_returns_summary_on_completion(cx: &mut TestAppContext) {
        // Setup mock model that completes immediately
        // Verify summary_prompt is sent after completion
        // Verify summary response becomes tool result
    }

    #[gpui::test]
    async fn test_subagent_inherits_parent_model(cx: &mut TestAppContext) {
        // Create parent with specific model
        // Spawn subagent
        // Verify subagent uses same model
    }

    #[gpui::test]
    async fn test_subagent_model_fixed_after_spawn(cx: &mut TestAppContext) {
        // Spawn subagent
        // Change parent model
        // Verify subagent still uses original model
    }

    // Tool Access Control

    #[gpui::test]
    async fn test_subagent_inherits_all_parent_tools_by_default(cx: &mut TestAppContext) {
        // Parent has tools A, B, C
        // Spawn subagent without allowed_tools
        // Verify subagent has A, B, C
    }

    #[gpui::test]
    async fn test_subagent_respects_allowed_tools_restriction(cx: &mut TestAppContext) {
        // Parent has tools A, B, C
        // Spawn subagent with allowed_tools = [A, B]
        // Verify subagent only has A, B
    }

    #[gpui::test]
    async fn test_subagent_cannot_exceed_parent_tools(cx: &mut TestAppContext) {
        // Parent has tools A, B
        // Spawn subagent with allowed_tools = [A, B, C]
        // Verify error returned
    }

    #[gpui::test]
    async fn test_subagent_tool_not_available_at_max_depth(cx: &mut TestAppContext) {
        // Create thread at depth MAX_DEPTH - 1
        // Verify subagent tool is not present
    }

    // Depth Limits

    #[gpui::test]
    async fn test_max_depth_enforced(cx: &mut TestAppContext) {
        // Spawn subagent at depth MAX_DEPTH
        // Verify error returned
    }

    #[gpui::test]
    async fn test_subagent_at_depth_3_cannot_spawn_subagent(cx: &mut TestAppContext) {
        // Create subagent chain: 0 -> 1 -> 2 -> 3
        // Verify subagent at depth 3 has no subagent tool
    }

    #[gpui::test]
    async fn test_max_subagents_per_depth_enforced(cx: &mut TestAppContext) {
        // Spawn 8 subagents from same parent
        // Attempt to spawn 9th
        // Verify appropriate handling (error or queuing)
    }

    // Context Window Management

    #[gpui::test]
    async fn test_context_low_prompt_sent_at_25_percent(cx: &mut TestAppContext) {
        // Mock model with 1000 token limit
        // Simulate usage reaching 750 tokens
        // Verify context_low_prompt is sent
    }

    #[gpui::test]
    async fn test_token_usage_updates_sent_to_parent(cx: &mut TestAppContext) {
        // Spawn subagent
        // Simulate token usage updates
        // Verify parent receives status updates
    }

    #[gpui::test]
    async fn test_context_exceeded_returns_error_with_partial_transcript(cx: &mut TestAppContext) {
        // Mock model that exceeds context
        // Verify error returned with first N bytes of transcript
    }

    // Cancellation

    #[gpui::test]
    async fn test_parent_cancel_propagates_to_subagent(cx: &mut TestAppContext) {
        // Spawn subagent
        // Cancel parent
        // Verify subagent is canceled
    }

    #[gpui::test]
    async fn test_nested_subagent_cancel_propagates(cx: &mut TestAppContext) {
        // Spawn subagent that spawns another subagent
        // Cancel root
        // Verify all subagents canceled
    }

    #[gpui::test]
    async fn test_canceled_subagent_reports_status(cx: &mut TestAppContext) {
        // Spawn and cancel subagent
        // Verify Canceled status sent to parent
    }

    // Timeout

    #[gpui::test]
    async fn test_timeout_triggers_summary_request(cx: &mut TestAppContext) {
        // Spawn subagent with 100ms timeout
        // Mock model that takes longer
        // Verify summary is requested after timeout
    }

    #[gpui::test]
    async fn test_no_timeout_by_default(cx: &mut TestAppContext) {
        // Spawn subagent without timeout_ms
        // Verify it runs until natural completion
    }

    // Parallel Subagents

    #[gpui::test]
    async fn test_multiple_subagents_run_in_parallel(cx: &mut TestAppContext) {
        // Spawn 3 subagents simultaneously
        // Verify all run concurrently (not sequentially)
    }

    #[gpui::test]
    async fn test_parallel_subagents_independent_token_tracking(cx: &mut TestAppContext) {
        // Spawn 2 subagents
        // Each uses different token amounts
        // Verify each has independent usage tracking
    }

    // Error Scenarios

    #[gpui::test]
    async fn test_parent_thread_dropped_returns_error(cx: &mut TestAppContext) {
        // Create weak reference scenario
        // Verify error handling
    }

    #[gpui::test]
    async fn test_model_not_configured_returns_error(cx: &mut TestAppContext) {
        // Parent with no model
        // Verify error
    }

    #[gpui::test]
    async fn test_subagent_model_error_returned_as_tool_error(cx: &mut TestAppContext) {
        // Mock model that returns error
        // Verify it appears as failed tool call, not top-level error
    }

    // Persistence

    #[gpui::test]
    async fn test_subagent_thread_saved_with_flag(cx: &mut TestAppContext) {
        // Spawn subagent
        // Save to DB
        // Verify is_subagent = true
    }

    #[gpui::test]
    async fn test_subagent_thread_hidden_from_history(cx: &mut TestAppContext) {
        // Spawn subagent
        // List threads
        // Verify subagent not in list
    }

    #[gpui::test]
    async fn test_subagent_thread_has_parent_reference(cx: &mut TestAppContext) {
        // Spawn subagent
        // Save to DB
        // Verify parent_thread_id set correctly
    }

    // Feature Flag

    #[gpui::test]
    async fn test_subagent_tool_hidden_without_feature_flag(cx: &mut TestAppContext) {
        // Disable feature flag
        // Create thread
        // Verify subagent tool not in tools list
    }

    #[gpui::test]
    async fn test_subagent_tool_available_with_feature_flag(cx: &mut TestAppContext) {
        // Enable feature flag
        // Create thread
        // Verify subagent tool in tools list
    }
}
```

### Integration Test Helpers

```rust
// Mock model for deterministic testing
struct MockLanguageModel {
    responses: Vec<MockResponse>,
    current_response: AtomicUsize,
    token_usage: language_model::TokenUsage,
}

enum MockResponse {
    Text(String),
    ToolUse { name: String, input: serde_json::Value },
    Error(String),
}

impl LanguageModel for MockLanguageModel {
    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
        let response = self.responses.get(
            self.current_response.fetch_add(1, Ordering::SeqCst)
        ).cloned();

        Box::pin(async move {
            match response {
                Some(MockResponse::Text(text)) => {
                    Ok(stream::once(async move {
                        Ok(LanguageModelCompletionEvent::Text(text))
                    }).chain(stream::once(async {
                        Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn))
                    })).boxed())
                }
                Some(MockResponse::ToolUse { name, input }) => {
                    Ok(stream::once(async move {
                        Ok(LanguageModelCompletionEvent::ToolUse(LanguageModelToolUse {
                            id: LanguageModelToolUseId::new(),
                            name: name.into(),
                            input,
                            is_input_complete: true,
                        }))
                    }).boxed())
                }
                Some(MockResponse::Error(msg)) => {
                    Err(anyhow::anyhow!(msg))
                }
                None => {
                    Ok(stream::empty().boxed())
                }
            }
        })
    }

    fn max_token_count(&self) -> u64 {
        200_000
    }

    // ... other trait methods with mock implementations
}
```

---

## Staged PR Breakdown

This section breaks down the implementation into 5 reviewable PRs. Each PR is designed to be:

- **Landable independently** (no broken states)
- **Visually verifiable** (you can see something new in Zed)
- **Quick to review** (<500 meaningful lines)

---

### PR 1: Feature Flag + Tool Skeleton

**Goal:** Establish the foundation. After this PR, the `subagent` tool appears in the tool list (when flag is enabled), but does nothing.

**Estimated size:** ~150-200 lines

**Files to create/modify:**

| File                                      | Changes                                                                                                          |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `crates/feature_flags/src/flags.rs`       | Add `SubagentsFeatureFlag`                                                                                       |
| `crates/agent/src/tools/subagent_tool.rs` | New file: `SubagentToolInput` struct, `SubagentTool` struct with stub `run()` that returns "Not implemented yet" |
| `crates/agent/src/tools.rs`               | Add `mod subagent_tool`, `pub use subagent_tool::*`                                                              |
| `crates/agent/src/thread.rs`              | In `add_default_tools()`, conditionally add `SubagentTool` behind feature flag                                   |

**Visual verification:**

1. Enable `subagents` feature flag
2. Open Agent panel
3. Check tool list and also try asking "What tools do you have?"
4. Verify `subagent` appears in the list

**Tests to include:**

- Unit test: `SubagentTool` is included when flag enabled
- Unit test: `SubagentTool` is NOT included when flag disabled
- Unit test: `SubagentToolInput` JSON schema is valid

**Definition of Done:**

- [x] Feature flag works
- [x] Tool appears in tool list when flag enabled
- [x] Tool schema is correctly generated
- [x] All existing tests pass
- [x] `./script/clippy` passes

**STATUS: ✅ COMPLETED**

---

### PR 2: Thread Spawning + Basic Execution

**Goal:** The subagent actually runs. After this PR, you can see the agent spawn a subagent, the subagent does work, and returns a text summary.

**Estimated size:** ~300-400 lines

**Files to create/modify:**

| File                                      | Changes                                                                                                                                   |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/agent/src/thread.rs`              | Add `SubagentContext`, `SubagentStatusUpdate`, `subagent_context` field, `new_subagent()` constructor, `is_subagent()`, `depth()` methods |
| `crates/agent/src/tools/subagent_tool.rs` | Implement real `run()`: spawn thread, send task prompt, wait for completion, send summary prompt, return result                           |
| `crates/agent/src/thread.rs`              | Add `interrupt_for_summary()`, `request_final_summary()`, `wait_for_turn_completion()`                                                    |

**Visual verification:**

1. Enable feature flag
2. Prompt: "Use a subagent to find all TODO comments in this project and summarize them"
3. Observe: Agent spawns subagent, subagent runs, result appears as tool result text

**Tests to include:**

- [x] `test_subagent_receives_task_prompt`
- [x] `test_subagent_returns_summary_on_completion`
- [x] `test_subagent_inherits_parent_model` (named `test_subagent_thread_inherits_parent_model`)
- [x] `test_max_depth_enforced` (named `test_max_subagent_depth_prevents_tool_registration`)
- [x] `test_allowed_tools_validated` (named `test_allowed_tools_restricts_subagent_capabilities`)
- [x] `test_parent_cancel_stops_subagent`
- [x] `test_subagent_model_error_returned_as_tool_error`
- [x] `test_context_low_check_returns_true_when_usage_high` (tests the context low detection)
- [x] `test_subagent_timeout_triggers_early_summary`
- [x] `test_allowed_tools_rejects_unknown_tool` (validates error when requesting invalid tool)
- [x] `test_subagent_empty_response_handled` (graceful handling of empty model responses)
- [x] `test_nested_subagent_at_depth_2_succeeds` (verifies depth-2 subagents work)
- [x] `test_subagent_uses_tool_and_returns_result` (confirms subagent can invoke tools)
- [x] `test_max_parallel_subagents_enforced` (validates 8-subagent limit)
- [x] `test_subagent_tool_end_to_end` (full integration: spawn → task → summary → return)

**Definition of Done:**

- [x] Subagent spawns with correct model
- [x] Task prompt is sent to subagent
- [x] Subagent can use tools
- [x] Summary prompt triggers final response
- [x] Result returned to parent as tool result
- [x] `timeout_ms` is implemented and triggers early summary
- [x] `allowed_tools` filtering is implemented
- [x] Cancellation propagates from parent to subagent
- [x] `MAX_PARALLEL_SUBAGENTS` (8) limit is enforced
- [x] `./script/clippy` passes
- [x] All tests listed above pass

**STATUS: ✅ COMPLETED**

---

### PR 3: UI Card Rendering (Collapsed State)

**Goal:** Subagent tool calls render as special cards (like terminal tool calls) instead of generic tool output. Initially collapsed.

**Estimated size:** ~250-350 lines

**Files to create/modify:**

| File                                     | Changes                                                                                                 |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `crates/acp_thread/src/acp_thread.rs`    | Add handling for subagent tool calls, token usage custom field                                          |
| `crates/agent_ui/src/acp/thread_view.rs` | Add `expanded_subagents` state, `render_subagent_tool_call()` for collapsed card with label and chevron |
| `crates/zed/src/zed/visual_tests.rs`     | Add `test_subagent_tool_card_collapsed` visual test                                                     |
| `crates/zed/test_fixtures/visual_tests/` | Add baseline image                                                                                      |

**Visual testing workflow:**

```bash
# 1. Implement the collapsed card rendering
# 2. Generate screenshot
cargo test -p zed visual_tests::subagent_collapsed -- --ignored --test-threads=1
# 3. Open target/visual_tests/subagent_collapsed.png
# 4. Check: Is the card visible? Label correct? Chevron present?
# 5. Iterate until it looks right
# 6. Update baseline
UPDATE_BASELINES=1 cargo test -p zed visual_tests::subagent_collapsed -- --ignored --test-threads=1
```

**Visual verification:**

1. Run subagent
2. Card appears with label (e.g., "Researching alternatives")
3. Chevron indicates it can be expanded
4. Card styling matches terminal tool call cards

**Tests to include:**

- Visual test: `test_subagent_tool_card_collapsed`
- Unit test: `expanded_subagents` state toggles correctly

**Definition of Done:**

- [x] Collapsed card renders with label
- [x] Chevron/disclosure icon visible
- [x] Card styling consistent with other tool cards
- [ ] Visual test baseline committed ← **STILL NEEDS TO BE DONE**
- [x] `./script/clippy` passes
- [x] Unit test for is_subagent() detection

**STATUS: 🔜 IN PROGRESS** (visual tests not yet written)

---

### PR 4: UI Expansion + Embedded Thread View

**Goal:** Clicking the card expands to show the subagent's full conversation. You can see what the subagent did.

**Estimated size:** ~300-400 lines

**Files to create/modify:**

| File                                      | Changes                                                                                                                |
| ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `crates/agent_ui/src/acp/thread_view.rs`  | Add `render_subagent_thread()`, implement expand/collapse click handler, add max-height and scroll for embedded thread |
| `crates/acp_thread/src/acp_thread.rs`     | Store weak reference to subagent thread for rendering                                                                  |
| `crates/agent/src/tools/subagent_tool.rs` | Pass thread reference through to UI layer                                                                              |
| `crates/zed/src/zed/visual_tests.rs`      | Add `test_subagent_tool_card_expanded`, `test_multiple_subagents_parallel`                                             |

**Visual testing workflow:**

```bash
# Test expanded state
cargo test -p zed visual_tests::subagent_expanded -- --ignored --test-threads=1
open target/visual_tests/subagent_expanded.png
# Check: Does the thread render inside the card? Is there scrolling?
# Can you see the subagent's tool calls?
```

**Visual verification:**

1. Run subagent
2. Click collapsed card
3. Card expands, shows subagent's messages and tool calls
4. Content is scrollable if too long
5. Click again to collapse

**Tests to include:**

- Visual test: `test_subagent_tool_card_expanded`
- Visual test: `test_multiple_subagents_parallel`
- Unit test: Click toggles expand state
- Unit test: Subagent thread renders all message types

**Definition of Done:**

- [ ] Expand/collapse works
- [ ] Subagent thread renders inside card
- [ ] Scrolling works for long content
- [ ] Multiple subagents display correctly
- [ ] Visual test baselines committed
- [ ] `./script/clippy` passes

---

### PR 5: Polish – Token Display, Errors, Persistence, Cancellation

**Goal:** Production-ready experience. Token usage updates live, errors display correctly, subagent threads persist correctly, cancellation propagates.

**Estimated size:** ~350-450 lines

**Files to create/modify:**

| File                                     | Changes                                                                       |
| ---------------------------------------- | ----------------------------------------------------------------------------- |
| `crates/agent/src/thread.rs`             | Modify `cancel()` to propagate to subagents, add `running_subagents` tracking |
| `crates/agent/src/thread.rs`             | Modify `update_token_usage()` to send updates to parent                       |
| `crates/agent_ui/src/acp/thread_view.rs` | Render live token usage (e.g., "120k/200k"), error states                     |
| `crates/agent/src/db.rs`                 | Add `is_subagent`, `parent_thread_id` fields                                  |
| `crates/agent/src/history_store.rs`      | Filter subagent threads from history list                                     |
| `crates/zed/src/zed/visual_tests.rs`     | Add `test_subagent_token_usage_display`, `test_subagent_error_state`          |

**Visual testing workflow:**

```bash
# Test token display
cargo test -p zed visual_tests::subagent_tokens -- --ignored --test-threads=1
# Test error state
cargo test -p zed visual_tests::subagent_error -- --ignored --test-threads=1
```

**Visual verification:**

1. Run subagent, observe token counter updating
2. Trigger an error (e.g., invalid tool), verify failed state display
3. Cancel parent during subagent execution, verify subagent stops
4. Check history: subagent threads should NOT appear

**Tests to include:**

- Visual test: `test_subagent_token_usage_display`
- Visual test: `test_subagent_error_state`
- Unit test: `test_parent_cancel_propagates_to_subagent`
- Unit test: `test_token_usage_updates_sent_to_parent`
- Unit test: `test_subagent_thread_hidden_from_history`
- Unit test: `test_context_low_prompt_sent_at_25_percent`

**Definition of Done:**

- [ ] Token usage displays and updates
- [ ] Errors display as failed tool calls
- [ ] Cancellation propagates to subagents
- [ ] Subagent threads saved with `is_subagent` flag
- [ ] Subagent threads hidden from history
- [ ] All visual test baselines committed
- [ ] All unit tests pass
- [ ] `./script/clippy` passes

---

## PR Review Checklist

Before submitting each PR, verify:

```markdown
## Pre-Submit Checklist

- [ ] Feature flag guards all new UI code
- [ ] `./script/clippy` passes
- [ ] All existing tests pass
- [ ] New tests cover the changes
- [ ] Visual tests generate correct baselines
- [ ] I looked at the screenshots and the UI looks good
- [ ] PR description explains what's new and how to test it
- [ ] Demo-ready: reviewer can enable flag and see the feature
```

---

## Commit & PR Workflow

### Commit Practices

**Commit frequently as you go.** Don't batch up a day's worth of work into one commit.

Each commit message should:

- Be clear and descriptive
- Include this co-author trailer (exactly as shown):

```
Co-Authored-By: Claude Opus 4.5
```

**Do NOT include:**

- ❌ "Generated by [tool name]"
- ❌ "Created with AI assistance"
- ❌ Any mention of specific AI tools or services
- ❌ Comments like "// AI-generated code"

### Code Style Notes

**Do NOT use banner-style comment separators** like this:

```rust
// ============================================
// Section Name
// ============================================
```

Instead, use a simple single-line comment:

```rust
// Section Name
```

Example commit:

```
Add SubagentTool skeleton with feature flag

Implement the basic structure for the subagent tool:
- Add SubagentsFeatureFlag to feature_flags crate
- Create SubagentToolInput with JSON schema
- Add stub run() method that returns placeholder text
- Conditionally register tool when flag is enabled

Co-Authored-By: Claude Opus 4.5
```

### Before Pushing a Draft PR

When the PR's work is complete, run through this checklist **before pushing**:

```bash
# 1. Fetch and merge latest main
git fetch origin
git merge origin/main
# Resolve any conflicts, then commit

# 2. Run clippy
./script/clippy
# Fix any issues

# 3. Run all tests
cargo test
# Fix any failures

# 4. Clean up visual test baselines
#    - Remove any debugging/temporary screenshots
#    - Keep only the baselines that are part of the feature
#    - Check: crates/zed/test_fixtures/visual_tests/
#    - Delete anything like "debug_*.png" or "temp_*.png"

# 5. Final verification
./script/clippy && cargo test
```

### Pushing the Draft PR

```bash
# Push your branch
git push -u origin feature/subagents-pr-N

# Create a draft PR with a concise title
gh pr create --draft --title "Add subagent tool (PR N/5): [brief description]" --body "
## Summary
[One sentence describing what this PR adds]

## Visual Changes
[Screenshot or description of what's visually different]

## How to Test
1. Enable the \`subagents\` feature flag
2. [Steps to verify the feature works]

## Checklist
- [ ] Feature flag guards all UI changes
- [ ] Visual tests pass and baselines committed
- [ ] \`./script/clippy\` passes
- [ ] \`cargo test\` passes
"
```

### Babysitting CI

After pushing the draft PR, monitor CI until it's green:

```bash
# Watch CI status
gh pr checks --watch

# If a check fails, investigate:
gh run view [run-id] --log-failed

# Fix issues locally, commit, push:
git add .
git commit -m "Fix CI: [description]

Co-Authored-By: Claude Opus 4.5"
git push

# Repeat until all checks pass
gh pr checks --watch
```

**Common CI issues to watch for:**

- Clippy warnings treated as errors
- Tests that pass locally but fail in CI (timing, env differences)
- Visual test baseline mismatches (different rendering on CI machines)
- Missing feature flags in test configurations

### After CI is Green

Once all checks pass:

1. Review the PR diff one more time
2. Ensure the PR description is accurate
3. Mark ready for review (or leave as draft if waiting for feedback)

```bash
# Optional: mark ready for review
gh pr ready
```

---

## Appendix: Constants

```rust
// crates/agent/src/tools/subagent_tool.rs

/// Maximum number of subagents that can be spawned per depth level
pub const MAX_SUBAGENTS_PER_DEPTH: u8 = 8;

/// Maximum depth of subagent nesting (0 = root, 4 = deepest subagent)
pub const MAX_DEPTH: u8 = 4;

/// Context remaining threshold to trigger early summarization (25%)
pub const CONTEXT_LOW_THRESHOLD: f32 = 0.25;

/// Maximum bytes of transcript to include in error messages
pub const MAX_ERROR_TRANSCRIPT_BYTES: usize = 8 * 1024; // 8KB
```
