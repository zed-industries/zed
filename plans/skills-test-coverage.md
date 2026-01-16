# Detailed Test Implementation Plan for Skills Feature

## Overview

This plan addresses gaps in test coverage for the Skills feature. All tests should be **fully deterministic** - no `sleep()` calls or real timers. The codebase uses GPUI's testing infrastructure which provides:

1. **`FakeFs`** - A simulated filesystem that automatically emits file change events when files are created/modified/deleted
2. **`cx.run_until_parked()`** - Processes all pending async work deterministically 
3. **`cx.executor().advance_clock(duration)`** - Advances simulated time for testing timeouts/delays

---

## Test 1: Integration Test for `merge_skills()` Name Conflict Detection

### File Location
`zed3/crates/agent/src/agent.rs` (add to existing `mod internal_tests`)

### Purpose
Verify that when a global skill and a project-local skill have the same name, an error is generated (no silent overrides).

### Implementation Details

```rust
#[gpui::test]
async fn test_merge_skills_name_conflict(cx: &mut TestAppContext) {
    // Create two skills with the same name but different sources
    let global_skill = Skill {
        name: "my-skill".to_string(),
        description: "Global version".to_string(),
        source: SkillSource::Global,
        directory_path: PathBuf::from("/global/skills/my-skill"),
        skill_file_path: PathBuf::from("/global/skills/my-skill/SKILL.md"),
        content: "Global content".to_string(),
    };
    
    let project_skill = Skill {
        name: "my-skill".to_string(), // Same name!
        description: "Project version".to_string(),
        source: SkillSource::ProjectLocal { worktree_id: WorktreeId::from_usize(1) },
        directory_path: PathBuf::from("/project/.zed/skills/my-skill"),
        skill_file_path: PathBuf::from("/project/.zed/skills/my-skill/SKILL.md"),
        content: "Project content".to_string(),
    };
    
    let (skills, errors) = merge_skills(
        vec![Ok(global_skill)],
        vec![Ok(project_skill)].into_iter(),
    );
    
    // First skill wins, second produces error
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].description, "Global version");
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("conflicts with"));
}
```

### Key Points for Implementer
- The `merge_skills()` function is private in `agent.rs` - you may need to make it `pub(crate)` or add the test inside the existing `mod internal_tests`
- Import `SkillSource`, `Skill` from `agent_skills` crate
- This is a synchronous test (no async needed) since `merge_skills` is not async

---

## Test 2: Integration Test for Skill Loading in `build_project_context()`

### File Location
`zed3/crates/agent/src/agent.rs` (add to existing `mod internal_tests`)

### Purpose
Verify that skills are loaded from both global and project-local directories when building project context.

### Implementation Details

```rust
#[gpui::test]
async fn test_build_project_context_loads_skills(cx: &mut TestAppContext) {
    init_test(cx);  // Use existing init_test helper
    
    let fs = FakeFs::new(cx.executor());
    
    // Create global skills directory at the path returned by paths::skills_dir()
    let global_skills_dir = paths::skills_dir();
    fs.insert_tree(
        global_skills_dir,
        json!({
            "global-skill": {
                "SKILL.md": "---\nname: global-skill\ndescription: A global skill\n---\n\nGlobal instructions"
            }
        }),
    ).await;
    
    // Create project with project-local skills
    fs.insert_tree(
        "/project",
        json!({
            ".zed": {
                "skills": {
                    "project-skill": {
                        "SKILL.md": "---\nname: project-skill\ndescription: A project skill\n---\n\nProject instructions"
                    }
                }
            },
            "src": {
                "main.rs": "fn main() {}"
            }
        }),
    ).await;
    
    let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
    
    // Build project context (this is what we're testing)
    let (project_context, skills, errors) = cx
        .update(|cx| {
            NativeAgent::build_project_context(&project, None, fs.clone(), cx)
        })
        .await;
    
    // Verify both skills were loaded
    assert!(errors.is_empty(), "Should have no errors: {:?}", errors);
    assert_eq!(skills.len(), 2);
    
    let skill_names: Vec<&str> = skills.iter().map(|s| s.name.as_str()).collect();
    assert!(skill_names.contains(&"global-skill"));
    assert!(skill_names.contains(&"project-skill"));
    
    // Verify skills appear in project context
    assert!(project_context.has_skills);
    assert_eq!(project_context.skills.len(), 2);
}
```

### Key Points for Implementer
- `build_project_context` is a private method - the test needs to be inside `mod internal_tests` in `agent.rs`
- Uses `FakeFs` which is the standard way to test filesystem operations
- The `paths::skills_dir()` function returns the global skills path - you need to create files there in the FakeFs
- No timing dependencies - just setup files and call the method

---

## Test 3: Global Skills File Watcher Test

### File Location
`zed3/crates/agent/src/agent.rs` (add to existing `mod internal_tests`)

### Purpose
Verify that when a SKILL.md file is created/modified in the global skills directory, the skills are reloaded.

### Implementation Details

The key insight is that `FakeFs` automatically emits file change events when you call `insert_file()`, `insert_tree()`, etc. The test uses `cx.run_until_parked()` to process these events deterministically.

```rust
#[gpui::test]
async fn test_global_skills_file_watcher(cx: &mut TestAppContext) {
    init_test(cx);
    
    let fs = FakeFs::new(cx.executor());
    
    // Create initial global skills directory with one skill
    let global_skills_dir = paths::skills_dir();
    fs.insert_tree(
        global_skills_dir,
        json!({
            "skill-one": {
                "SKILL.md": "---\nname: skill-one\ndescription: First skill\n---\n\nContent one"
            }
        }),
    ).await;
    
    // Create empty project
    fs.insert_tree("/project", json!({ "file.txt": "hello" })).await;
    let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
    
    // Create NativeAgent
    let agent = NativeAgent::new(
        project.clone(),
        None,  // No prompt store
        fs.clone(),
        Arc::new(FakeLanguageModelProvider::new()),
        None,
        &mut cx.to_async(),
    ).await.unwrap();
    
    // Verify initial skill count
    agent.read_with(cx, |agent, _| {
        assert_eq!(agent.skills.len(), 1);
        assert_eq!(agent.skills[0].name, "skill-one");
    });
    
    // Add a new skill file to the global skills directory
    let new_skill_path = global_skills_dir.join("skill-two/SKILL.md");
    fs.create_dir(&global_skills_dir.join("skill-two")).await.unwrap();
    fs.insert_file(
        &new_skill_path,
        "---\nname: skill-two\ndescription: Second skill\n---\n\nContent two".as_bytes().to_vec(),
    ).await;
    
    // Process the file system event - FakeFs automatically emits events
    cx.run_until_parked();
    
    // Verify skills were reloaded
    agent.read_with(cx, |agent, _| {
        assert_eq!(agent.skills.len(), 2);
        let names: Vec<&str> = agent.skills.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"skill-one"));
        assert!(names.contains(&"skill-two"));
    });
}
```

### Key Points for Implementer
- **No real timers!** The `FakeFs` automatically emits events when files change
- `cx.run_until_parked()` processes all pending async work including file watcher callbacks
- The watcher in `watch_global_skills_directory()` uses `fs.watch()` which works with `FakeFs`
- The 500ms debounce duration in the real code is handled by the executor's simulated time

---

## Test 4: Skill Budget Limiting in `ProjectContext::new()`

### File Location
`zed3/crates/prompt_store/src/prompts.rs` (add new test module)

### Purpose
Verify that when skill descriptions exceed the 50KB budget, later skills are truncated from the system prompt.

### Implementation Details

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use agent_skills::{Skill, SkillSource, MAX_SKILL_DESCRIPTIONS_SIZE};
    use std::path::PathBuf;

    #[test]
    fn test_skill_budget_limiting() {
        // Create skills that exceed the 50KB budget
        let mut skills = Vec::new();
        let mut total_size = 0;
        let mut skill_count = 0;
        
        // Each skill has ~1KB description
        let description = "x".repeat(1000);
        
        // Create enough skills to exceed the budget
        while total_size < MAX_SKILL_DESCRIPTIONS_SIZE + 10_000 {
            let name = format!("skill-{:04}", skill_count);
            skills.push(Skill {
                name: name.clone(),
                description: description.clone(),
                source: SkillSource::Global,
                directory_path: PathBuf::from(format!("/skills/{}", name)),
                skill_file_path: PathBuf::from(format!("/skills/{}/SKILL.md", name)),
                content: "Content".to_string(),
            });
            total_size += name.len() + description.len();
            skill_count += 1;
        }
        
        let context = ProjectContext::new(vec![], vec![], skills.clone());
        
        // Verify some skills were excluded due to budget
        assert!(context.skills.len() < skills.len());
        assert!(context.has_skills);
        
        // Verify the included skills' total size is within budget
        let included_size: usize = context.skills
            .iter()
            .map(|s| s.name.len() + s.description.len())
            .sum();
        assert!(included_size <= MAX_SKILL_DESCRIPTIONS_SIZE);
    }

    #[test]
    fn test_empty_skills_sets_has_skills_false() {
        let context = ProjectContext::new(vec![], vec![], vec![]);
        assert!(!context.has_skills);
        assert!(context.skills.is_empty());
    }
}
```

### Key Points for Implementer
- This is a synchronous unit test - no async or GPUI needed
- Import `Skill`, `SkillSource`, `MAX_SKILL_DESCRIPTIONS_SIZE` from `agent_skills`
- The budget is 50KB (`MAX_SKILL_DESCRIPTIONS_SIZE = 50 * 1024`)
- Test both the truncation case and the edge case of empty skills

---

## Test 5: `ListDirectoryTool` Skill Directory Listing

### File Location
`zed3/crates/agent/src/tools/list_directory_tool.rs` (add to existing `mod tests`)

### Purpose
Verify that `ListDirectoryTool` can list skill directory contents when the skill tool is enabled.

### Implementation Details

```rust
#[gpui::test]
async fn test_list_skill_directory_when_enabled(cx: &mut TestAppContext) {
    init_test(cx);
    
    let fs = FakeFs::new(cx.executor());
    
    // Create global skills directory with files
    let skills_dir = paths::skills_dir();
    fs.insert_tree(
        skills_dir,
        json!({
            "my-skill": {
                "SKILL.md": "---\nname: my-skill\ndescription: Test\n---\n\nContent",
                "helper.py": "# helper script",
                "data": {
                    "config.json": "{}"
                }
            }
        }),
    ).await;
    
    // Create project
    fs.insert_tree("/project", json!({ "file.txt": "hello" })).await;
    let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
    
    // Create a skill and a thread with skill tool enabled
    let skill = agent_skills::parse_skill(
        &skills_dir.join("my-skill/SKILL.md"),
        "---\nname: my-skill\ndescription: Test\n---\n\nContent",
        agent_skills::SkillSource::Global,
    ).unwrap();
    
    let skills = Arc::new(vec![skill]);
    let context_server_registry = cx.new(|cx| {
        ContextServerRegistry::new(project.read(cx).context_server_store(), cx)
    });
    
    let thread = cx.new(|cx| {
        let mut thread = Thread::new(
            project.clone(),
            cx.new(|_cx| ProjectContext::default()),
            skills.clone(),
            context_server_registry,
            Templates::new(),
            None,
            cx,
        );
        // Add skill tool to enable skill directory access
        thread.add_tool(SkillTool::new(skills.clone(), project.clone()));
        thread
    });
    
    let tool = Arc::new(ListDirectoryTool::new(project.clone(), thread.downgrade()));
    
    // List the skill directory using absolute path
    let skill_dir_path = skills_dir.join("my-skill");
    let input = ListDirectoryToolInput {
        path: skill_dir_path.to_string_lossy().to_string(),
    };
    
    let (event_stream, _) = ToolCallEventStream::test();
    let result = cx.update(|cx| tool.run(input, event_stream, cx)).await;
    
    assert!(result.is_ok(), "Should list skill directory: {:?}", result.err());
    let output = result.unwrap();
    
    // Verify directory contents are listed
    assert!(output.contains("SKILL.md"));
    assert!(output.contains("helper.py"));
    assert!(output.contains("data")); // subdirectory
}

#[gpui::test]
async fn test_list_skill_directory_denied_when_disabled(cx: &mut TestAppContext) {
    init_test(cx);
    
    let fs = FakeFs::new(cx.executor());
    
    // Create global skills directory
    let skills_dir = paths::skills_dir();
    fs.insert_tree(
        skills_dir,
        json!({
            "my-skill": {
                "SKILL.md": "---\nname: my-skill\ndescription: Test\n---\n\nContent"
            }
        }),
    ).await;
    
    // Create project
    fs.insert_tree("/project", json!({ "file.txt": "hello" })).await;
    let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
    
    // Create thread WITHOUT skill tool
    let context_server_registry = cx.new(|cx| {
        ContextServerRegistry::new(project.read(cx).context_server_store(), cx)
    });
    
    let thread = cx.new(|cx| {
        Thread::new(
            project.clone(),
            cx.new(|_cx| ProjectContext::default()),
            Arc::new(Vec::new()), // No skills!
            context_server_registry,
            Templates::new(),
            None,
            cx,
        )
    });
    
    let tool = Arc::new(ListDirectoryTool::new(project.clone(), thread.downgrade()));
    
    // Try to list skill directory
    let skill_dir_path = skills_dir.join("my-skill");
    let input = ListDirectoryToolInput {
        path: skill_dir_path.to_string_lossy().to_string(),
    };
    
    let (event_stream, _) = ToolCallEventStream::test();
    let result = cx.update(|cx| tool.run(input, event_stream, cx)).await;
    
    assert!(result.is_err(), "Should NOT be able to list skill directory when skill tool disabled");
}
```

### Key Points for Implementer
- Follow the existing test patterns in `list_directory_tool.rs`
- Need to import `SkillTool`, `Thread`, `ProjectContext`, `Templates` from parent module
- The key difference from existing tests is using absolute paths to skill directories
- The `try_list_skill_directory` method checks if path starts with `paths::skills_dir()`

---

## Test 6: Improve Validation Test Assertions

### File Location
`zed3/crates/agent_skills/skill.rs` (modify existing tests)

### Purpose
Make `test_parse_missing_name` and `test_parse_missing_description` verify the actual error messages like other validation tests do.

### Implementation Details

```rust
#[test]
fn test_parse_missing_name() {
    let content = r#"---
description: A test skill
---

Content here.
"#;

    let result = parse_skill(
        Path::new("/skills/test/SKILL.md"),
        content,
        SkillSource::Global,
    );
    assert!(result.is_err());
    // Add specific error message check
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("missing field") || err_msg.contains("name"),
        "Error should mention missing name field: {}", err_msg
    );
}

#[test]
fn test_parse_missing_description() {
    let content = r#"---
name: test-skill
---

Content here.
"#;

    let result = parse_skill(
        Path::new("/skills/test/SKILL.md"),
        content,
        SkillSource::Global,
    );
    assert!(result.is_err());
    // Add specific error message check
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("missing field") || err_msg.contains("description"),
        "Error should mention missing description field: {}", err_msg
    );
}
```

### Key Points for Implementer
- The YAML parsing via `serde_yaml` will produce errors like "missing field `name`"
- Checking for either "missing field" or the field name handles different error formats

---

## Test 7: Empty Body After Frontmatter

### File Location
`zed3/crates/agent_skills/skill.rs` (add to existing tests)

### Purpose
Verify that a SKILL.md with valid frontmatter but empty content body is handled correctly.

### Implementation Details

```rust
#[test]
fn test_parse_empty_body_after_frontmatter() {
    let content = r#"---
name: minimal-skill
description: A skill with no body content
---
"#;

    let result = parse_skill(
        Path::new("/skills/minimal/SKILL.md"),
        content,
        SkillSource::Global,
    );
    
    // Should succeed - empty body is valid
    let skill = result.expect("Empty body should be allowed");
    assert_eq!(skill.name, "minimal-skill");
    assert_eq!(skill.description, "A skill with no body content");
    assert!(skill.content.is_empty() || skill.content.trim().is_empty());
}

#[test]
fn test_parse_whitespace_only_body() {
    let content = "---\nname: whitespace-skill\ndescription: Test\n---\n\n   \n\n   \n";

    let result = parse_skill(
        Path::new("/skills/ws/SKILL.md"),
        content,
        SkillSource::Global,
    );
    
    let skill = result.expect("Whitespace-only body should be allowed");
    assert!(skill.content.trim().is_empty());
}
```

### Key Points for Implementer
- The `parse_skill` function already trims the body with `.trim().to_string()`
- This test documents the expected behavior for edge cases

---

## Test 8: Subagent Skill Inheritance

### File Location
`zed3/crates/agent/src/tests/mod.rs` (add to existing subagent tests)

### Purpose
Verify that subagent threads can access the same skills as their parent thread.

### Implementation Details

```rust
#[gpui::test]
async fn test_subagent_inherits_skills(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(|cx| {
        cx.set_flag::<SubagentsFeatureFlag>(true);
    });
    
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/project", json!({ "file.txt": "hello" })).await;
    let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;
    
    // Create a skill
    let skill = agent_skills::parse_skill(
        Path::new("/skills/test-skill/SKILL.md"),
        "---\nname: test-skill\ndescription: Test skill\n---\n\nInstructions",
        agent_skills::SkillSource::Global,
    ).unwrap();
    let skills = Arc::new(vec![skill]);
    
    let context_server_registry = cx.new(|cx| {
        ContextServerRegistry::new(project.read(cx).context_server_store(), cx)
    });
    let project_context = cx.new(|_| ProjectContext::default());
    let model = Arc::new(FakeLanguageModel::default());
    
    // Create parent thread with skills
    let parent = cx.new(|cx| {
        let mut thread = Thread::new(
            project.clone(),
            project_context.clone(),
            skills.clone(),
            context_server_registry.clone(),
            Templates::new(),
            Some(model.clone()),
            cx,
        );
        thread.add_tools(cx);
        thread
    });
    
    // Verify parent has skill tool
    parent.read_with(cx, |thread, _| {
        assert!(thread.has_registered_tool("skill"), "Parent should have skill tool");
    });
    
    // Create subagent
    let subagent_context = SubagentContext {
        task_prompt: "Do something".into(),
        summary_prompt: "Summarize".into(),
        context_low_prompt: "Context low".into(),
        parent_tools: parent.read(cx).tools.clone(),
        allowed_tools: None,
        timeout_ms: None,
    };
    
    let subagent = cx.new(|cx| {
        let mut thread = Thread::new_subagent(
            project.clone(),
            project_context.clone(),
            skills.clone(),
            context_server_registry.clone(),
            Templates::new(),
            model.clone(),
            subagent_context,
            parent.read(cx).id().clone(),
            1,
            cx,
        );
        thread.add_tools(cx);
        thread
    });
    
    // Verify subagent also has skill tool
    subagent.read_with(cx, |thread, _| {
        assert!(thread.has_registered_tool("skill"), "Subagent should inherit skill tool");
    });
    
    // Verify subagent has same skills
    subagent.read_with(cx, |thread, _| {
        assert_eq!(thread.skills().len(), 1);
        assert_eq!(thread.skills()[0].name, "test-skill");
    });
}
```

### Key Points for Implementer
- Enable the subagents feature flag with `cx.set_flag::<SubagentsFeatureFlag>(true)`
- `Thread::new_subagent` takes the skills parameter and should make skill tool available
- Use `has_registered_tool("skill")` to check tool availability
- The `skills()` accessor returns `&Arc<Vec<Skill>>`

---

## Summary Checklist

| # | Test | File | Type | Complexity |
|---|------|------|------|------------|
| 1 | `merge_skills()` name conflict | `agent.rs` | Unit | Low |
| 2 | `build_project_context()` loads skills | `agent.rs` | Integration | Medium |
| 3 | Global skills file watcher | `agent.rs` | Integration | Medium |
| 4 | Skill budget limiting | `prompts.rs` | Unit | Low |
| 5 | `ListDirectoryTool` skill paths | `list_directory_tool.rs` | Integration | Medium |
| 6 | Improve validation assertions | `skill.rs` | Unit | Low |
| 7 | Empty body handling | `skill.rs` | Unit | Low |
| 8 | Subagent skill inheritance | `tests/mod.rs` | Integration | Medium |

## Important Reminders

1. **Never use real timers or sleep** - Use `cx.executor().advance_clock()` for time-based tests
2. **Use `cx.run_until_parked()`** after any operation that triggers async work
3. **`FakeFs` automatically emits events** - No need to manually trigger file system events
4. **Follow existing test patterns** - Look at nearby tests in the same file for setup helpers like `init_test()`
5. **Run tests with `cargo test -p <crate_name>`** - e.g., `cargo test -p agent_skills`
