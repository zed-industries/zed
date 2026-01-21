# Git Graph CommitLine Verification Test Plan

## Background

PR `da06bb5e0e5193bd8e401634120ec1329782767f` ("speed up add_commits function") broke the git graph canvas drawing. The change introduced a `parent_to_lane` HashMap optimization but appears to have broken some line rendering logic.

We need randomized tests that verify `CommitLine`s start and end at the correct row/column positions.

## Architecture Overview

### Key Data Structures

- **`InitialGraphCommitData`**: Input data with `sha`, `parents`, and `ref_names`
- **`GraphData`**: Contains the computed `commits` (with lane assignments) and `lines` (CommitLines)
- **`CommitLine`**: Describes a line from child commit to parent commit
  - `child` / `parent`: Oid of commits being connected
  - `child_column`: Starting lane
  - `full_interval`: Row range (`starting_row..ending_row`)
  - `segments`: List of `CommitLineSegment` (Straight or Curve)
- **`CommitLineSegment`**:
  - `Straight { to_row }`: Vertical line staying in same column
  - `Curve { to_column, on_row, curve_kind }`: Curved line moving to different column

### Test Flow

1. Generate random commit DAG → `Vec<Arc<InitialGraphCommitData>>`
2. Set up `FakeGitRepository` with this graph data
3. Create `GitGraph` entity that loads from the fake repo
4. Verify invariants on the resulting `GraphData.lines`

## Invariants to Verify

### 1. Line Endpoint Row Correctness

- `full_interval.start` == row where child commit is located
- `full_interval.end` == row where parent commit is located
- Final segment lands on parent's row:
  - `Straight { to_row }` → `to_row == full_interval.end`
  - `Curve { on_row, .. }` → `on_row == full_interval.end`

### 2. Column/Lane Assignment Correctness

- `child_column` == `commits[child_row].lane`
- Ending column (traced through segments) == `commits[parent_row].lane`
- For each segment:
  - `Straight` stays in same column
  - `Curve { to_column, .. }` moves to `to_column`

### 3. Segment Continuity

- Segments are in row order (monotonically increasing)
- No gaps between segments
- No overlapping row ranges (exception: curves can visually cross straight lines in different columns)
- Starting point is at `(child_column, full_interval.start)`

### 4. Coverage Invariants

- Every parent-child edge in input commits has a corresponding `CommitLine`
- No orphan lines (every line references valid commits)
- No duplicate lines for same `(child, parent)` pair

### 5. Lane State Consistency

- `max_lanes` accurately reflects maximum lane index used
- Note: Curves can visually cross straight lines in different columns - this is valid

## Implementation Details

### Random DAG Generator

Parameters:

- `rng: &mut StdRng`
- `num_commits: usize` (e.g., 10-200)
- `max_parents_per_commit: usize` (typically 1-4, octopus merges rare)
- `branch_probability: f64` (~20% for adversarial cases)

Constraints for valid git history:

- Parents must come AFTER child in the list (git log outputs newest first)
- At least one commit must have no parents (root commit, will be last in list)
- Merge commits can have multiple parents
- Parents must reference existing commits (valid Oids)

Distribution:

- ~80% of iterations: realistic histories (mostly linear, occasional branches/merges)
- ~20% of iterations: adversarial (many branches, octopus merges, complex topology)

### FakeGitRepository Changes

**`FakeGitRepositoryState`** - add field:

```rust
pub graph_commits: Vec<Arc<InitialGraphCommitData>>,
```

**`FakeGitRepository::initial_graph_data`** - implement to:

1. Read `graph_commits` from state
2. Send commits through channel in chunks (respecting `GRAPH_CHUNK_SIZE`)
3. Return `Ok(())`

**`FakeFs`** - add helper:

```rust
pub fn set_graph_commits(&self, repo_path: &Path, commits: Vec<Arc<InitialGraphCommitData>>)
```

### Verification Functions

Location: `git_graph/src/git_graph.rs` in `#[cfg(test)] mod tests`

```rust
fn verify_line_endpoints(graph: &GraphData) -> Result<(), String>
fn verify_column_correctness(graph: &GraphData) -> Result<(), String>
fn verify_segment_continuity(graph: &GraphData) -> Result<(), String>
fn verify_coverage(graph: &GraphData, commits: &[Arc<InitialGraphCommitData>]) -> Result<(), String>
fn verify_all_invariants(graph: &GraphData, commits: &[Arc<InitialGraphCommitData>]) -> Result<(), String>
```

### Test Structure (Option A: Full Integration)

We use the full `GitGraph` entity creation path to ensure we're testing the real integration:

1. Create `FakeFs` with a git repository
2. Set graph commits on the fake repo
3. Create a `Project` with the fake fs
4. Get the `Repository` entity from the project
5. Create a `GitGraph` entity that subscribes to the repository
6. Wait for graph data to load
7. Verify invariants on the loaded `GraphData`

This approach is future-proof because if `add_commits` logic moves to a background thread in `Repository`, the tests will still work.

### Test Helper Function

Create a helper to reduce boilerplate:

```rust
struct GitGraphTestContext {
    fs: Arc<FakeFs>,
    project: Entity<Project>,
    repository: Entity<Repository>,
    git_graph: Entity<GitGraph>,
}

impl GitGraphTestContext {
    async fn new(
        commits: Vec<Arc<InitialGraphCommitData>>,
        cx: &mut TestAppContext,
    ) -> Self {
        // Set up FakeFs with git repo
        // Set graph commits
        // Create project
        // Get repository
        // Create GitGraph
        // Wait for data to load
        // Return context
    }

    fn graph_data(&self, cx: &TestAppContext) -> &GraphData {
        // Access the graph data for verification
    }
}
```

### Test Code

```rust
#[gpui::test(iterations = 50)]
async fn test_git_graph_random_commits(mut rng: StdRng, cx: &mut TestAppContext) {
    // Determine if this iteration is adversarial (~20% chance)
    let adversarial = rng.gen_bool(0.2);

    // Generate commits
    let commits = generate_random_commit_dag(&mut rng, adversarial);

    // Set up full integration test context
    let test_ctx = GitGraphTestContext::new(commits.clone(), cx).await;

    // Access graph data
    let graph_data = test_ctx.graph_data(cx);

    // Verify invariants
    verify_all_invariants(graph_data, &commits).expect("Graph invariants violated");
}
```

## Files to Modify/Create

1. `crates/git_graph/Cargo.toml` - Add test dependencies
2. `crates/fs/src/fake_git_repo.rs` - Add graph_commits field and implement initial_graph_data
3. `crates/fs/src/fs.rs` - Add set_graph_commits helper to FakeFs
4. `crates/git_graph/src/git_graph.rs` - Add test module with:
   - `GitGraphTestContext` helper struct
   - Random DAG generator
   - Verification functions
   - Integration test

## Cargo.toml Changes

Add to `crates/git_graph/Cargo.toml`:

```toml
[features]
test-support = [
    "project/test-support",
    "gpui/test-support",
]

[dev-dependencies]
fs = { workspace = true, features = ["test-support"] }
gpui = { workspace = true, features = ["test-support"] }
project = { workspace = true, features = ["test-support"] }
settings = { workspace = true, features = ["test-support"] }
pretty_assertions.workspace = true
rand.workspace = true
serde_json.workspace = true
```

## Test Setup Pattern

Based on existing project tests, the setup pattern is:

```rust
#[gpui::test(iterations = 50)]
async fn test_git_graph_random_commits(mut rng: StdRng, cx: &mut TestAppContext) {
    init_test(cx);  // Initialize settings, themes, etc.

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            ".git": {},
            "file.txt": "content",
        }),
    ).await;

    // Set graph commits on the fake repo
    fs.set_graph_commits(path!("/project/.git"), commits.clone());

    // Create project and wait for git scan
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    project.update(cx, |project, cx| project.git_scans_complete(cx)).await;
    cx.run_until_parked();

    // Get repository and create GitGraph
    let repository = project.read_with(cx, |p, cx| p.active_repository(cx).unwrap());
    let git_graph = cx.new(|cx| GitGraph::new(project.clone(), &mut cx.window, cx));

    // Wait for graph data to load
    cx.run_until_parked();

    // Verify invariants
    // ...
}
```

## init_test Function

We'll need an init_test function similar to other crates:

```rust
fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        // Any other initialization needed
    });
}
```

## Todo List

- [x] **Step 1: Update Cargo.toml**
  - [x] Add `[features]` section with `test-support`
  - [x] Add `[dev-dependencies]` for fs, gpui, project, settings, rand, serde_json, pretty_assertions

- [x] **Step 2: Update FakeGitRepositoryState**
  - [x] Add `graph_commits: Vec<Arc<InitialGraphCommitData>>` field
  - [x] Initialize in `FakeGitRepositoryState::new()`

- [x] **Step 3: Implement initial_graph_data on FakeGitRepository**
  - [x] Read graph_commits from state
  - [x] Send commits through channel in GRAPH_CHUNK_SIZE chunks
  - [x] Handle empty graph_commits case

- [x] **Step 4: Add set_graph_commits helper to FakeFs**
  - [x] Find repo state by path
  - [x] Update graph_commits field

- [x] **Step 5: Create test module in git_graph.rs**
  - [x] Add `#[cfg(test)] mod tests`
  - [x] Add necessary test imports
  - [x] Add `init_test` function
  - [x] Add `generate_random_oid` helper

- [x] **Step 6: Implement random DAG generator**
  - [x] `generate_random_oid(rng)` helper (using `Oid::random`)
  - [x] `generate_random_commit_dag(rng, adversarial)` main function
  - [x] Ensure valid parent relationships (parents come after children in list)
  - [x] Support linear, branching, merging, and octopus merge patterns
  - [x] 80% realistic / 20% adversarial distribution

- [x] **Step 7: Implement verification functions**
  - [x] `verify_line_endpoints` - check full_interval matches commit rows
  - [x] `verify_column_correctness` - check columns match commit lanes
  - [x] `verify_segment_continuity` - check segments are ordered and continuous
  - [x] `verify_coverage` - check all parent-child edges have lines
  - [x] `verify_all_invariants` - call all verification functions
  - [x] `find_commit_row` - helper to find commit row by Oid

- [x] **Step 8: Create GitGraphTestContext helper**
  - [x] ~~Implement `GitGraphTestContext::new()` to set up full integration~~ (Simplified to direct unit test due to test-support dependency issues)
  - [x] Add `#[cfg(any(test, feature = "test-support"))]` accessor for `graph_data` on GitGraph

- [x] **Step 9: Write the test**
  - [x] Use `#[test]` with manual seeded RNG loop (50 iterations)
  - [x] Generate random commits with adversarial flag (20% chance)
  - [x] Use `GraphData::add_commits` directly (simpler than full integration)
  - [x] Run all verification functions
  - [x] Include descriptive panic messages with seed info for reproducibility
  - [x] **Test successfully catches the bug!** (seed=1 fails with `full_interval.end (37) != parent_row (39)`)

- [x] **Step 10: Test and debug**
  - [x] Run tests to verify they catch the existing bug ✓
  - [ ] Ensure tests pass on known-good state (before the buggy commit)
  - [x] Add descriptive error messages for debugging failures ✓
  - [x] Verify test is deterministic (same seed = same failure) ✓

## Notes

- The bug is in the `add_commits` function's use of `parent_to_lane` HashMap
- Tests should be deterministic with seeded RNG for reproducibility
- Consider using `OPERATIONS` env var pattern for configurable iteration count (like text crate)
- Curves crossing straight lines in different columns is valid - don't flag as error
- Using full integration (Option A) because `add_commits` logic may move to background thread in Repository in the future
- The `GitGraphTestContext` helper reduces boilerplate and makes it easy to write additional git graph tests later

## Key Implementation Details

### Random Oid Generation

```rust
fn generate_random_oid(rng: &mut StdRng) -> Oid {
    let mut bytes = [0u8; 20];
    rng.fill(&mut bytes);
    Oid::from_bytes(&bytes)
}
```

### Commit DAG Structure

Git log outputs commits newest-first. So in our Vec:

- Index 0 = most recent commit (HEAD)
- Index N = oldest commit (root, has no parents)
- Parents of commit at index I must have index > I

### Realistic vs Adversarial

- **Realistic**: Linear chain with occasional feature branches (1-2 parents typical)
- **Adversarial**: Many parallel branches, octopus merges (3+ parents), complex topology

### Accessing GraphData

The `GitGraph` struct has a private `graph_data` field. For tests, we may need to either:

1. Add a `#[cfg(test)]` accessor method
2. Make `graph_data` pub(crate)
3. Use a different approach to verify the data

### GRAPH_CHUNK_SIZE

Located in `git/src/repository.rs`:

```rust
pub const GRAPH_CHUNK_SIZE: usize = 1000;
```

The fake implementation should respect this for realistic behavior.
