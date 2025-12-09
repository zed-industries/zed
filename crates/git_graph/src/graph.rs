use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use git::{
    Oid,
    libgit::{Repository, Sort},
};
use gpui::{AsyncApp, Entity};
use project::Project;
use util::command::new_smol_command;

/// %H - Full commit hash
/// %aN - Author name
/// %aE - Author email
/// %at - Author timestamp
/// %ct - Commit timestamp
/// %P - Parent hashes
/// %D - Ref names
const COMMIT_FORMAT: &str = "--format=%H%n%aN%n%aE%n%at%n%ct%n%P%n%D%n";

/// Commit data needed for the graph
pub struct GraphCommit {
    pub sha: Oid,
    pub parents: Vec<Oid>,
    pub author_name: String,
    pub author_email: String,
    pub commit_timestamp: i64,
    pub summary: String,
}

/// The type of node in the graph
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Commit,
    Stash,
}

/// A node's position in the graph: (row, column, type)
#[derive(Debug, Clone, Copy)]
pub struct GraphNode {
    pub row: usize,
    pub column: usize,
    pub node_type: NodeType,
}

/// The type of edge connecting commits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// Edge to first parent (main line)
    Normal,
    /// Edge to non-first parent (merge)
    Merge,
}

/// An edge in the graph connecting two positions
#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub from_row: usize,
    pub from_column: usize,
    pub to_row: usize,
    pub to_column: usize,
    pub edge_type: EdgeType,
}

/// The computed graph layout for visualization
pub struct CommitGraph {
    /// Map from commit SHA to its visual position
    pub positions: HashMap<Oid, GraphNode>,
    /// The width of the graph (number of columns/lanes)
    pub width: usize,
    /// All edges in the graph, can be queried by row range
    pub edges: Vec<GraphEdge>,
}

/// Repository data needed for graph computation
pub struct RepoGraph {
    /// Commits in topologically sorted order (children before parents)
    pub commits: Vec<GraphCommit>,
    /// Map from SHA to index in commits vec
    pub sha_to_index: HashMap<Oid, usize>,
    /// Map from commit SHA to parent SHAs
    pub parents: HashMap<Oid, Vec<Oid>>,
    /// Map from commit SHA to child SHAs
    pub children: HashMap<Oid, Vec<Oid>>,
    /// Set of stash commit SHAs
    pub stashes: HashSet<Oid>,
    /// The HEAD commit SHA, if any
    pub head_sha: Option<Oid>,
}

impl RepoGraph {
    pub fn new(commits: Vec<GraphCommit>, head_sha: Option<Oid>, stashes: HashSet<Oid>) -> Self {
        let sha_to_index: HashMap<Oid, usize> = commits
            .iter()
            .enumerate()
            .map(|(i, c)| (c.sha, i))
            .collect();

        let mut parents: HashMap<Oid, Vec<Oid>> = HashMap::new();
        let mut children: HashMap<Oid, Vec<Oid>> = HashMap::new();

        for commit in &commits {
            children.insert(commit.sha, Vec::new());
        }

        for commit in &commits {
            let parent_shas = commit.parents.clone();
            parents.insert(commit.sha, parent_shas.clone());

            for parent_sha in parent_shas {
                if let Some(parent_children) = children.get_mut(&parent_sha) {
                    parent_children.push(commit.sha);
                }
            }
        }

        Self {
            commits,
            sha_to_index,
            parents,
            children,
            stashes,
            head_sha,
        }
    }
}

impl CommitGraph {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            width: 0,
            edges: Vec::new(),
        }
    }

    /// Compute visual positions for all commits using the gitamine algorithm.
    ///
    /// The algorithm assigns each commit a (row, column) position where:
    /// - row: the commit's index in topological order (0 = newest)
    /// - column: the visual lane for the commit
    ///
    /// The algorithm tries to:
    /// - Keep commits on the same branch in the same lane
    /// - Minimize lane crossings
    /// - Place merge commits near their first parent's lane
    pub fn compute_positions(&mut self, repo: &RepoGraph) {
        self.positions.clear();

        let head_sha = repo.head_sha;

        // Active branches/lanes - None means the lane is available
        // Lane 0 is reserved for the index/HEAD line
        let mut branches: Vec<Option<Oid>> = vec![None];

        // Track which lanes are "blocked" by edges passing through
        // Maps commit SHA -> set of lane indices that are blocked while traversing to this commit
        let mut active_nodes: HashMap<Oid, HashSet<usize>> = HashMap::new();

        // Priority queue to track when active nodes can be removed
        // (row_to_remove, commit_sha)
        let mut active_nodes_removal: Vec<(usize, Oid)> = Vec::new();

        // Initialize with a placeholder for HEAD/index
        active_nodes.insert(Oid::default(), HashSet::new());

        if let Some(head) = head_sha {
            if let Some(&head_idx) = repo.sha_to_index.get(&head) {
                active_nodes_removal.push((head_idx, Oid::default()));
            }
        }

        for (row, commit) in repo.commits.iter().enumerate() {
            let commit_sha = commit.sha;
            let children = repo
                .children
                .get(&commit_sha)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            let parent_shas = repo
                .parents
                .get(&commit_sha)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            // Separate children into "branch children" (this commit is their first parent)
            // and "merge children" (this commit is a non-first parent)
            let branch_children: Vec<Oid> = children
                .iter()
                .filter(|&&child_sha| {
                    repo.parents
                        .get(&child_sha)
                        .and_then(|p| p.first())
                        .map(|&first| first == commit_sha)
                        .unwrap_or(false)
                })
                .copied()
                .collect();

            let merge_children: Vec<Oid> = children
                .iter()
                .filter(|&&child_sha| {
                    repo.parents
                        .get(&child_sha)
                        .and_then(|p| p.first())
                        .map(|&first| first != commit_sha)
                        .unwrap_or(false)
                })
                .copied()
                .collect();

            // Compute forbidden indices - lanes blocked by merge edges passing through
            let mut forbidden_indices: HashSet<usize> = HashSet::new();
            if let Some(highest_merge_child) = merge_children
                .iter()
                .filter_map(|&c| self.positions.get(&c).map(|p| (p.row, c)))
                .min_by_key(|(r, _)| *r)
                .map(|(_, c)| c)
            {
                if let Some(blocked) = active_nodes.get(&highest_merge_child) {
                    forbidden_indices = blocked.clone();
                }
            }

            // Find a commit to replace in the branches array
            // Prefer replacing a branch child to maintain lane continuity
            let mut commit_to_replace: Option<Oid> = None;
            let mut column_to_replace: Option<usize> = None;

            // Special case: if this is HEAD, take lane 0
            if Some(commit_sha) == head_sha {
                commit_to_replace = Some(Oid::default());
                column_to_replace = Some(0);
            } else {
                // Find the leftmost branch child whose lane isn't forbidden
                for &child_sha in &branch_children {
                    if let Some(child_pos) = self.positions.get(&child_sha) {
                        let child_col = child_pos.column;
                        if !forbidden_indices.contains(&child_col) {
                            if column_to_replace.is_none() || child_col < column_to_replace.unwrap()
                            {
                                commit_to_replace = Some(child_sha);
                                column_to_replace = Some(child_col);
                            }
                        }
                    }
                }
            }

            // Determine the final column for this commit
            let column = if let Some(col) = column_to_replace {
                branches[col] = Some(commit_sha);
                col
            } else {
                // Need to find or create a new lane
                let preferred_col = if !children.is_empty() {
                    // Try to insert near a child
                    self.positions
                        .get(&children[0])
                        .map(|p| p.column)
                        .unwrap_or(0)
                } else {
                    0
                };

                insert_commit_near(&mut branches, commit_sha, preferred_col, &forbidden_indices)
            };

            // Remove stale entries from active_nodes
            while let Some(&(remove_row, _)) = active_nodes_removal.first() {
                if remove_row < row {
                    let (_, sha) = active_nodes_removal.remove(0);
                    active_nodes.remove(&sha);
                } else {
                    break;
                }
            }

            // Update active nodes with new blocked lanes
            let lanes_to_add: Vec<usize> = std::iter::once(column)
                .chain(
                    branch_children
                        .iter()
                        .filter_map(|&c| self.positions.get(&c).map(|p| p.column)),
                )
                .collect();

            for blocked_set in active_nodes.values_mut() {
                for &lane in &lanes_to_add {
                    blocked_set.insert(lane);
                }
            }

            // Add this commit to active nodes
            active_nodes.insert(commit_sha, HashSet::new());

            // Calculate when this commit's active node can be removed
            // (when we've passed all its parents)
            let max_parent_row = parent_shas
                .iter()
                .filter_map(|p| repo.sha_to_index.get(p))
                .max()
                .copied()
                .unwrap_or(row);

            // Insert in sorted order
            let insert_pos = active_nodes_removal
                .iter()
                .position(|(r, _)| *r > max_parent_row)
                .unwrap_or(active_nodes_removal.len());
            active_nodes_removal.insert(insert_pos, (max_parent_row, commit_sha));

            // Free up lanes from branch children that we didn't replace
            for &child_sha in &branch_children {
                if Some(child_sha) != commit_to_replace {
                    if let Some(child_pos) = self.positions.get(&child_sha) {
                        let child_col = child_pos.column;
                        if child_col < branches.len() {
                            branches[child_col] = None;
                        }
                    }
                }
            }

            // If this commit has no parents, free its lane
            if parent_shas.is_empty() && column < branches.len() {
                branches[column] = None;
            }

            // Set the position for this commit
            let node_type = if repo.stashes.contains(&commit_sha) {
                NodeType::Stash
            } else {
                NodeType::Commit
            };

            self.positions.insert(
                commit_sha,
                GraphNode {
                    row,
                    column,
                    node_type,
                },
            );
        }

        self.width = branches.len();
        self.compute_edges(repo);
    }

    /// Build the edge list from the computed positions
    fn compute_edges(&mut self, repo: &RepoGraph) {
        self.edges.clear();

        for (commit_sha, node) in &self.positions {
            let parent_shas = match repo.parents.get(commit_sha) {
                Some(p) => p,
                None => continue,
            };

            for (i, parent_sha) in parent_shas.iter().enumerate() {
                if let Some(parent_node) = self.positions.get(parent_sha) {
                    let edge_type = if i == 0 {
                        EdgeType::Normal
                    } else {
                        EdgeType::Merge
                    };

                    self.edges.push(GraphEdge {
                        from_row: node.row,
                        from_column: node.column,
                        to_row: parent_node.row,
                        to_column: parent_node.column,
                        edge_type,
                    });
                }
            }
        }
    }

    /// Get all edges that pass through a given row range (inclusive)
    pub fn edges_in_range(&self, start_row: usize, end_row: usize) -> Vec<&GraphEdge> {
        self.edges
            .iter()
            .filter(|edge| {
                let edge_start = edge.from_row.min(edge.to_row);
                let edge_end = edge.from_row.max(edge.to_row);
                edge_start <= end_row && edge_end >= start_row
            })
            .collect()
    }
}

impl Default for CommitGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Insert a commit into the branches array, trying to place it near `preferred_col`.
/// Returns the column where it was inserted.
fn insert_commit_near(
    branches: &mut Vec<Option<Oid>>,
    commit_sha: Oid,
    preferred_col: usize,
    forbidden: &HashSet<usize>,
) -> usize {
    // Try to find an available slot near preferred_col
    let mut delta = 0usize;
    loop {
        // Try preferred_col + delta
        let col_right = preferred_col.saturating_add(delta);
        if col_right < branches.len()
            && branches[col_right].is_none()
            && !forbidden.contains(&col_right)
        {
            branches[col_right] = Some(commit_sha);
            return col_right;
        }

        // Try preferred_col - delta
        if delta > 0 && preferred_col >= delta {
            let col_left = preferred_col - delta;
            if col_left < branches.len()
                && branches[col_left].is_none()
                && !forbidden.contains(&col_left)
            {
                branches[col_left] = Some(commit_sha);
                return col_left;
            }
        }

        delta += 1;

        // If we've searched all existing slots, append a new one
        if delta > branches.len() {
            branches.push(Some(commit_sha));
            return branches.len() - 1;
        }
    }
}

pub async fn load_commits(project: Entity<Project>, cx: &mut AsyncApp) -> Result<Vec<GraphCommit>> {
    // todo!: Is this the best worktree to use?
    let first_visible_worktree = project
        .read_with(cx, |project, cx| {
            project
                .visible_worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        })?
        .context("Can't show git graph in non projects")?;

    let git_log_output = new_smol_command("git")
        .current_dir(first_visible_worktree)
        .arg("log")
        .arg(COMMIT_FORMAT)
        .arg("--date-order")
        .output()
        .await?;

    let mut commits = Vec::new();

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        let graph_commit = GraphCommit {
            sha: Oid::from_bytes(oid.as_bytes())?,
            parents: commit
                .parent_ids()
                .map(|parent_oid| Oid::from_bytes(parent_oid.as_bytes()))
                .collect::<Result<Vec<_>>>()?,
            author_name: commit.author().name().unwrap_or("").to_string(),
            author_email: commit.author().email().unwrap_or("").to_string(),
            commit_timestamp: commit.time().seconds(),
            summary: commit.summary().unwrap_or("").to_string(),
        };

        commits.push(graph_commit);

        if let Some(limit) = limit {
            if commits.len() >= limit {
                break;
            }
        }
    }

    Ok(commits)
}

/// Load HEAD SHA from the repository
pub fn get_head_sha(repo: &Repository) -> Option<Oid> {
    repo.head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok())
        .and_then(|commit| Oid::from_bytes(commit.id().as_bytes()).ok())
}

/// Build a complete graph from a repository
pub fn build_graph(repo: &Repository, limit: Option<usize>) -> Result<(RepoGraph, CommitGraph)> {
    let commits = load_commits(repo, limit)?;
    let head_sha = get_head_sha(repo);
    let stashes = HashSet::new(); // TODO: Load stashes if needed

    let repo_graph = RepoGraph::new(commits, head_sha, stashes);
    let mut commit_graph = CommitGraph::new();
    commit_graph.compute_positions(&repo_graph);

    Ok((repo_graph, commit_graph))
}
