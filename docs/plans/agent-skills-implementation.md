# Agent Skills Implementation Plan for Zed

## Implementation Status

**Last Updated:** 2025-01-14

### Completed ✅
- Phase 1: Core Parsing - DONE
- Phase 2: Loading Infrastructure - DONE
- Phase 3: ProjectContext Integration - DONE
- Phase 4: System Prompt - DONE
- Phase 5: Skill Tool - DONE

### Not Yet Implemented ❌
- **Phase 6: Read File / List Directory Permissions** - NOT DONE
  - Need to add permission checks to allow reading files from skill directories only when skill tool is enabled
  - Need path canonicalization for security
  - Need permission tests
  
- **Phase 7: File Watching** - NOT DONE
  - Skills currently only reload on project context refresh
  - Need explicit file watchers for global skills directory ({config_dir}/skills/)
  - Need explicit file watchers for each worktree's .zed/skills/ directory
  
- **Phase 8: Error UI** - NOT DONE
  - SkillLoadError type exists but errors are not displayed to users
  - Need to implement error banner in AcpThreadView
  - Need "Open File" button to open problematic SKILL.md files
  - Need UI tests
  
- **Phase 9: Polish** - NOT DONE
  - End-to-end testing
  - Documentation
  - Code review and cleanup

**IMPORTANT: All phases are required. Nothing is optional or skippable.**

---

## Overview

This document outlines the implementation plan for adding Agent Skills support to Zed, inspired by the [Claude Agent Skills documentation](https://code.claude.com/docs/en/slash-commands).

Agent Skills are modular, filesystem-based capabilities that extend the agent's functionality. Each Skill packages instructions, metadata, and optional resources that the agent uses automatically when relevant.

## Goals

1. Allow users to define custom Skills in the Zed configuration directory (`{zed_config_dir}/skills/`)
2. Support project-local Skills (`.zed/skills/`)
3. Integrate Skills into the system prompt when available
4. Load and watch Skills on Zed startup, updating when files change
5. Show clear error messages when Skills fail to parse

**Note:** There are no built-in skills. All skills are user-defined. Users do not invoke skills directly - only agents do.

## Directory Structure

### Global Skills Location
```
{paths::config_dir()}/skills/
├── my-skill/
│   ├── SKILL.md
│   └── additional-resources.md
├── another-skill/
│   └── SKILL.md
└── nested/
    └── deep-skill/
        └── SKILL.md
```

### Project-Local Skills Location
```
{project_root}/.zed/skills/
├── project-specific-skill/
│   └── SKILL.md
└── another-skill/
    └── SKILL.md
```

Note: Use `paths::config_dir()` from `crates/paths/src/paths.rs` for the global location, and `paths::local_settings_folder_name()` (which returns `.zed`) for the project-local location.

The skills directory should be created lazily when first needed, not eagerly on startup.

## Skill File Format

Each Skill is a directory containing a `SKILL.md` file with YAML frontmatter:

```markdown
---
name: my-skill-name
description: Brief description of what this Skill does and when to use it
---

# My Skill Name

## Instructions
[Clear, step-by-step guidance for the agent to follow]

## Examples
[Concrete examples of using this Skill]

## Additional Resources
This skill includes helper files:
- `checklist.md` - A detailed checklist for this task
- `templates/` - Template files to use
```

### Field Requirements

- **name**: Required, max 64 characters, lowercase letters, numbers, hyphens only.
- **description**: Required, max 1024 characters, non-empty.

### File Encoding

SKILL.md files must be UTF-8 encoded.

### Size Limits

- Individual SKILL.md files: Maximum 100KB
- Total skill descriptions in system prompt: Maximum 50KB budget

## Architecture Decision: Where Skills Live

**Skills are loaded on Zed startup (when settings load) and watched for changes.**

Rationale:
1. Skills should be available immediately when the first agent thread starts
2. By parsing skills early, there's no delay when submitting the first prompt
3. File watching ensures skills stay up-to-date as users edit them
4. Skills are conceptually part of the "context" that informs the model (like rules files)

Skills are integrated into the NativeAgent's ProjectContext system, similar to how rules files work.

## Implementation Components

### 1. New Crate: `agent_skills`

Create a new crate at `crates/agent_skills/` with the following structure:

#### `crates/agent_skills/Cargo.toml`
```toml
[package]
name = "agent_skills"
version = "0.1.0"
edition = "2024"
publish = false
license = "GPL-3.0-or-later"

[lints]
workspace = true

[lib]
path = "agent_skills.rs"

[dependencies]
anyhow.workspace = true
fs.workspace = true
futures.workspace = true
gpui.workspace = true
serde.workspace = true
serde_yaml.workspace = true
util.workspace = true
```

#### `crates/agent_skills/src/agent_skills.rs` (main lib)
```rust
mod skill;

pub use skill::*;
```

#### `crates/agent_skills/src/skill.rs`
- `Skill` struct containing parsed metadata and content
- `SkillMetadata` struct for name, description  
- `SkillSource` enum (Global, ProjectLocal { worktree_id })
- Parsing functions with validation

### 2. Skill Types

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use worktree::WorktreeId;

#[derive(Debug, Clone, Serialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub source: SkillSource,
    /// Absolute path to the skill directory
    pub directory_path: PathBuf,
    /// Absolute path to the SKILL.md file
    pub skill_file_path: PathBuf,
    /// The full content of SKILL.md (excluding frontmatter)
    pub content: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum SkillSource {
    /// From {config_dir}/skills/
    Global,
    /// From {project}/.zed/skills/
    ProjectLocal { worktree_id: WorktreeId },
}

/// Just the frontmatter, used for system prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}
```

### 3. Skill Parsing

```rust
/// Parse a SKILL.md file into a Skill struct.
/// 
/// The file must have YAML frontmatter between `---` delimiters containing
/// `name` and `description` fields. The content after frontmatter becomes
/// the skill's instructions.
pub fn parse_skill(
    skill_file_path: &Path,
    content: &str,
    source: SkillSource,
) -> Result<Skill> {
    // 1. Check file size (max 100KB)
    // 2. Extract YAML frontmatter between --- delimiters
    // 3. Parse name and description from frontmatter
    // 4. Validate name: max 64 chars, lowercase + numbers + hyphens
    // 5. Validate description: non-empty, max 1024 chars
    // 6. Extract remaining content as skill instructions
    // 7. Derive directory_path from skill_file_path
}

fn validate_name(name: &str) -> Result<()> {
    if name.len() > 64 {
        anyhow::bail!("Skill name must be at most 64 characters");
    }
    if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        anyhow::bail!("Skill name must contain only lowercase letters, numbers, and hyphens");
    }
    Ok(())
}

fn validate_description(description: &str) -> Result<()> {
    if description.is_empty() {
        anyhow::bail!("Skill description cannot be empty");
    }
    if description.len() > 1024 {
        anyhow::bail!("Skill description must be at most 1024 characters");
    }
    Ok(())
}
```

### 4. Skill Loading Functions

```rust
use fs::Fs;
use std::sync::Arc;

/// Maximum size for a single SKILL.md file (100KB)
pub const MAX_SKILL_FILE_SIZE: usize = 100 * 1024;

/// Maximum total size for skill descriptions in system prompt (50KB)
pub const MAX_SKILL_DESCRIPTIONS_SIZE: usize = 50 * 1024;

/// Load all skills from a directory, recursively searching for SKILL.md files.
pub async fn load_skills_from_directory(
    fs: &Arc<dyn Fs>,
    dir: &Path,
    source: SkillSource,
) -> Vec<Result<Skill, SkillLoadError>> {
    // 1. Recursively find all SKILL.md files
    // 2. Parse each one
    // 3. Return results (both successes and failures)
}

#[derive(Debug, Clone)]
pub struct SkillLoadError {
    pub path: PathBuf,
    pub message: String,
}
```

### 5. Integration with NativeAgent

Modify `crates/agent/src/agent.rs` to load skills as part of project context:

```rust
// Add to NativeAgent
impl NativeAgent {
    fn build_project_context(
        project: &Entity<Project>,
        prompt_store: Option<&Entity<PromptStore>>,
        cx: &mut App,
    ) -> Task<ProjectContext> {
        // ... existing worktree and user_rules loading ...
        
        // Load skills
        let global_skills_task = load_global_skills(fs.clone(), cx);
        let project_skills_tasks = worktrees.iter().map(|wt| {
            load_worktree_skills(wt.clone(), fs.clone(), cx)
        }).collect::<Vec<_>>();
        
        cx.spawn(async move |_cx| {
            // ... existing code ...
            
            // Load and merge skills
            let global_skills = global_skills_task.await;
            let project_skills = future::join_all(project_skills_tasks).await;
            let (skills, skill_errors) = merge_skills(global_skills, project_skills);
            
            // Return context with skills
            ProjectContext::new(worktrees, default_user_rules, skills)
        })
    }
}

/// Merge global and project-local skills.
/// Name conflicts produce errors - NO OVERRIDES ALLOWED.
fn merge_skills(
    global: Vec<Result<Skill, SkillLoadError>>,
    project: Vec<Vec<Result<Skill, SkillLoadError>>>,
) -> (Vec<Skill>, Vec<SkillLoadError>) {
    let mut skills = Vec::new();
    let mut errors = Vec::new();
    let mut seen_names: HashMap<String, PathBuf> = HashMap::new();
    
    for result in global.into_iter().chain(project.into_iter().flatten()) {
        match result {
            Ok(skill) => {
                if let Some(existing_path) = seen_names.get(&skill.name) {
                    errors.push(SkillLoadError {
                        path: skill.skill_file_path.clone(),
                        message: format!(
                            "Skill name '{}' conflicts with skill at '{}'",
                            skill.name,
                            existing_path.display()
                        ),
                    });
                } else {
                    seen_names.insert(skill.name.clone(), skill.skill_file_path.clone());
                    skills.push(skill);
                }
            }
            Err(e) => errors.push(e),
        }
    }
    
    (skills, errors)
}
```

### 6. Skill Loading Error Events

Add error event type for UI to display:

```rust
// In crates/agent/src/agent.rs

pub struct SkillLoadingError {
    pub path: PathBuf,
    pub message: SharedString,
}

// NativeAgent emits these events when skills fail to load
impl EventEmitter<SkillLoadingError> for NativeAgent {}
```

### 7. Error UI in Agent Panel

Modify `crates/agent_ui/src/acp/thread_view.rs` to display skill loading errors:

```rust
impl AcpThreadView {
    fn render_skill_error_banner(
        &self,
        error: &SkillLoadingError,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // Render a warning banner with:
        // - Warning icon
        // - Error message
        // - "Open File" button that opens the problematic SKILL.md in an editor buffer
    }
}
```

The UI should:
1. Subscribe to `SkillLoadingError` events from NativeAgent
2. Collect errors and display them as dismissible warning banners
3. Each banner has an "Open File" button that opens the skill file in a Zed buffer

When one skill fails to parse, it should display an error but other skills should still load successfully.

### 8. Skill Tool

Create a new tool `SkillTool` that allows the agent to retrieve skill content:

```rust
// In crates/agent/src/tools/skill_tool.rs

pub struct SkillTool;

/// Input for the Skill tool
#[derive(Debug, Deserialize)]
pub struct SkillToolInput {
    /// The name of the skill to retrieve
    pub name: String,
}

/// Output from the Skill tool
#[derive(Debug, Serialize)]
pub struct SkillToolOutput {
    /// Whether the skill is global or project-local
    pub source: String,
    /// For project-local skills, which worktree it belongs to
    pub worktree: Option<String>,
    /// The full content of SKILL.md
    pub content: String,
    /// List of all files in the skill's directory (capped at 100KB total listing)
    pub files: Vec<String>,
}

impl Tool for SkillTool {
    fn name(&self) -> &'static str {
        "skill"
    }
    
    fn description(&self) -> &'static str {
        "Retrieves the content and resources of a skill by name. Use this when a user's request matches a skill's description."
    }
    
    async fn run(&self, input: SkillToolInput, cx: &mut AsyncApp) -> Result<SkillToolOutput> {
        // 1. Look up skill by name from loaded skills
        // 2. Read SKILL.md content
        // 3. List all files in the skill's directory (cap at 100KB total listing)
        // 4. Return source, content, and file listing
    }
}
```

### 9. Read File and List Directory Tool Permissions

Modify `crates/agent/src/tools/read_file_tool.rs` and `list_directory_tool.rs` to allow reading skill files only when the Skill tool is enabled:

```rust
impl ReadFileTool {
    fn is_path_allowed(
        &self,
        path: &Path,
        project: &Project,
        skill_tool_enabled: bool,
        cx: &App,
    ) -> bool {
        // Existing project file checks...
        
        // Allow reading from skills directories ONLY if skill tool is enabled
        if skill_tool_enabled {
            if let Ok(canonical_path) = std::fs::canonicalize(path) {
                let global_skills_dir = paths::config_dir().join("skills");
                if let Ok(canonical_skills_dir) = std::fs::canonicalize(&global_skills_dir) {
                    if canonical_path.starts_with(&canonical_skills_dir) {
                        return true;
                    }
                }
                
                // Also allow .zed/skills in any worktree
                for worktree in project.worktrees(cx) {
                    let worktree = worktree.read(cx);
                    let worktree_skills_dir = worktree.abs_path().join(".zed/skills");
                    if let Ok(canonical_wt_skills_dir) = std::fs::canonicalize(&worktree_skills_dir) {
                        if canonical_path.starts_with(&canonical_wt_skills_dir) {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
}
```

The same logic applies to `ListDirectoryTool` - it should only allow listing skills directories when the Skill tool is enabled in the current thread.

### 10. ProjectContext Updates

Modify `crates/prompt_store/src/prompts.rs` to include skills:

```rust
#[derive(Default, Debug, Clone, Serialize)]
pub struct ProjectContext {
    pub worktrees: Vec<WorktreeContext>,
    pub has_rules: bool,
    pub user_rules: Vec<UserRulesContext>,
    pub has_user_rules: bool,
    pub os: String,
    pub arch: String,
    pub shell: String,
    pub skills: Vec<SkillSummary>,
    pub has_skills: bool,
}

/// Minimal skill info for system prompt (not full content)
#[derive(Debug, Clone, Serialize)]
pub struct SkillSummary {
    pub name: String,
    pub description: String,
}

impl ProjectContext {
    pub fn new(
        worktrees: Vec<WorktreeContext>,
        default_user_rules: Vec<UserRulesContext>,
        skills: Vec<Skill>,
    ) -> Self {
        let has_rules = worktrees.iter().any(|wt| wt.rules_file.is_some());
        
        // Apply 50KB budget for skill descriptions
        let mut total_size = 0;
        let skill_summaries: Vec<SkillSummary> = skills
            .iter()
            .filter_map(|s| {
                let entry_size = s.name.len() + s.description.len();
                if total_size + entry_size <= MAX_SKILL_DESCRIPTIONS_SIZE {
                    total_size += entry_size;
                    Some(SkillSummary {
                        name: s.name.clone(),
                        description: s.description.clone(),
                    })
                } else {
                    None // Skip skills that exceed budget
                }
            })
            .collect();
        
        let has_skills = !skill_summaries.is_empty();
        
        Self {
            worktrees,
            has_rules,
            has_user_rules: !default_user_rules.is_empty(),
            user_rules: default_user_rules,
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            shell: ShellKind::new(&get_default_system_shell_preferring_bash(), cfg!(windows))
                .to_string(),
            skills: skill_summaries,
            has_skills,
        }
    }
}
```

### 11. System Prompt Template Updates

Modify `crates/agent/src/templates/system_prompt.hbs` to include skills similar to how tools are listed:

```handlebars
{{#if has_skills}}
## Agent Skills

You have access to the following Skills - modular capabilities that provide specialized instructions for specific tasks. When a user's request matches a Skill's description, use the `skill` tool to retrieve the full instructions.

Available Skills:
{{#each skills}}
- **{{name}}**: {{description}}
{{/each}}

To use a Skill:
1. Identify when a user's request matches a Skill's description
2. Use the `skill` tool with the skill's name to get detailed instructions
3. Follow the instructions in the Skill
4. If the Skill references additional files, use `read_file` to access them

{{/if}}
```

### 12. File Watching

Skills should be parsed on Zed startup (when settings load) and watched for changes:

```rust
// Load skills when settings are initialized
fn init_skills(fs: Arc<dyn Fs>, cx: &mut App) {
    // 1. Load global skills from {config_dir}/skills/
    // 2. Set up file watcher for global skills directory
    // 3. Project-local skills are loaded per-project when NativeAgent initializes
}

// In NativeAgent, extend the maintain_project_context task
async fn maintain_project_context(
    this: WeakEntity<Self>,
    mut needs_refresh: watch::Receiver<()>,
    cx: &mut AsyncApp,
) -> Result<()> {
    // Set up watchers for:
    // 1. Global skills directory: {config_dir}/skills/
    // 2. Each worktree's .zed/skills/ directory
    
    // When any watcher fires, trigger needs_refresh
    // The existing refresh logic will reload all context including skills
}
```

### 13. Paths Helper

Add to `crates/paths/src/paths.rs`:

```rust
/// Returns the path to the global skills directory.
/// The directory is created lazily when first accessed.
pub fn skills_dir() -> &'static PathBuf {
    static SKILLS_DIR: OnceLock<PathBuf> = OnceLock::new();
    SKILLS_DIR.get_or_init(|| config_dir().join("skills"))
}
```

## File Changes Summary

### New Files
- `crates/agent_skills/Cargo.toml`
- `crates/agent_skills/agent_skills.rs`
- `crates/agent_skills/skill.rs`
- `crates/agent/src/tools/skill_tool.rs`

### Modified Files
- `Cargo.toml` (workspace members)
- `crates/agent/Cargo.toml` (add agent_skills dependency)
- `crates/agent/src/agent.rs` (skill loading in project context, error events)
- `crates/agent/src/tools.rs` (register skill tool)
- `crates/agent/src/tools/read_file_tool.rs` (allow reading skill files when skill tool enabled)
- `crates/agent/src/tools/list_directory_tool.rs` (allow listing skill dirs when skill tool enabled)
- `crates/agent/src/templates.rs` (update SystemPromptTemplate if needed)
- `crates/agent/src/templates/system_prompt.hbs` (add skills section)
- `crates/agent_ui/Cargo.toml` (add agent_skills dependency if needed)
- `crates/agent_ui/src/acp/thread_view.rs` (skill error UI)
- `crates/prompt_store/Cargo.toml` (add agent_skills dependency)
- `crates/prompt_store/src/prompts.rs` (add skills to ProjectContext)
- `crates/paths/src/paths.rs` (add skills_dir function)

## Testing Requirements

### Unit Tests (`crates/agent_skills/src/skill.rs`)

1. **Parsing Tests**
   - `test_parse_valid_skill` - Parse a well-formed SKILL.md
   - `test_parse_missing_frontmatter` - Error on missing frontmatter
   - `test_parse_missing_name` - Error on missing name field
   - `test_parse_missing_description` - Error on missing description
   - `test_parse_name_too_long` - Error when name > 64 chars
   - `test_parse_name_invalid_chars` - Error on uppercase, special chars
   - `test_parse_description_too_long` - Error when description > 1024 chars
   - `test_parse_empty_description` - Error on empty description
   - `test_parse_content_after_frontmatter` - Correctly extract content
   - `test_parse_file_too_large` - Error when SKILL.md > 100KB

### Integration Tests (`crates/agent_skills/`)

1. **Loading Tests**
   - `test_load_skills_from_empty_directory` - Returns empty vec
   - `test_load_single_skill` - Correctly loads one skill
   - `test_load_nested_skills` - Finds skills in subdirectories
   - `test_load_ignores_non_skill_files` - Only loads SKILL.md files
   - `test_load_returns_errors_for_invalid_skills` - Returns SkillLoadError for malformed files

2. **Merging Tests**
   - `test_merge_unique_skills` - All unique skills from both sources included
   - `test_merge_name_conflict_error` - Conflicting names produce error (no overrides)
   - `test_merge_preserves_load_errors` - Load errors passed through

3. **Budget Tests**
   - `test_skill_descriptions_within_budget` - Skills within 50KB included
   - `test_skill_descriptions_exceed_budget` - Skills exceeding budget are omitted

### Skill Tool Tests (`crates/agent/src/tools/skill_tool.rs`)

1. **Tool Tests**
   - `test_skill_tool_returns_content` - Returns SKILL.md content
   - `test_skill_tool_returns_source` - Indicates global vs project-local
   - `test_skill_tool_lists_files` - Lists directory contents
   - `test_skill_tool_unknown_skill` - Error on unknown skill name
   - `test_skill_tool_file_listing_cap` - Directory listing capped at 100KB

### Read File Tool Tests (`crates/agent/src/tools/read_file_tool.rs`)

1. **Permission Tests**
   - `test_read_skill_file_allowed_when_skill_tool_enabled` - Allowed when skill tool enabled
   - `test_read_skill_file_denied_when_skill_tool_disabled` - Denied when skill tool disabled
   - `test_read_skill_file_blocks_path_traversal` - Blocks `../` attempts to escape skills dir
   - `test_read_skill_file_requires_canonical_path` - Resolves symlinks before checking

### System Prompt Tests (`crates/agent/src/templates.rs`)

1. **Template Rendering Tests**
   - `test_system_prompt_with_skills` - Skills section rendered when present
   - `test_system_prompt_without_skills` - No skills section when empty
   - `test_system_prompt_skills_budget_applied` - Only skills within budget shown

### Agent Integration Tests (`crates/agent/src/agent.rs`)

1. **Context Loading Tests**
   - `test_project_context_includes_skills` - Skills loaded into ProjectContext
   - `test_skill_errors_emitted` - SkillLoadingError events emitted for invalid skills

### UI Tests (`crates/agent_ui/`)

1. **Error Banner Tests**
   - `test_skill_error_banner_displayed` - Error banner shown when skill fails to load
   - `test_skill_error_open_file_button` - Button opens the problematic file

## Implementation Order

1. **Phase 1: Core Parsing**
   - Create `agent_skills` crate
   - Implement skill parsing with validation
   - Write parsing unit tests

2. **Phase 2: Loading Infrastructure**
   - Add `skills_dir()` to paths crate
   - Implement directory scanning and loading
   - Write loading tests

3. **Phase 3: ProjectContext Integration**
   - Add skills to `ProjectContext`
   - Integrate skill loading into `build_project_context`
   - Update `ProjectContext::new()` signature
   - Implement 50KB budget for descriptions

4. **Phase 4: System Prompt**
   - Update `system_prompt.hbs` template
   - Write template rendering tests

5. **Phase 5: Skill Tool**
   - Implement `SkillTool` 
   - Returns content, source, and directory listing
   - Write skill tool tests

6. **Phase 6: Read File / List Directory Permissions**
   - Add skill directory permission check (only when skill tool enabled)
   - Ensure path canonicalization for security
   - Write permission tests

7. **Phase 7: File Watching**
   - Load skills on Zed startup (when settings load)
   - Add watchers for global and project skills directories
   - Integrate with existing context refresh mechanism

8. **Phase 8: Error UI**
   - Add `SkillLoadingError` event type
   - Implement error banner in thread view
   - Add "Open File" button functionality
   - Write UI tests

9. **Phase 9: Polish**
   - End-to-end testing
   - Documentation
   - Code review and cleanup