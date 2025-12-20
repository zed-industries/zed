//! Core data structures for git commit graph

use collections::{HashMap, HashSet};
use gpui::SharedString;
use log;
use serde::{Deserialize, Serialize};

/// A commit node in the git graph
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitNode {
    /// Full SHA hash
    pub sha: SharedString,
    /// Short SHA for display (7 chars)
    pub short_sha: SharedString,
    /// Commit subject (first line of message)
    pub subject: SharedString,
    /// Author name
    pub author_name: SharedString,
    /// Author email
    pub author_email: SharedString,
    /// Unix timestamp
    pub timestamp: i64,
    /// Parent commit SHAs (1 for normal commits, 2+ for merges)
    pub parent_shas: Vec<SharedString>,
    /// Branch/tag refs pointing at this commit
    pub refs: Vec<GraphRef>,
    /// Assigned lane/column for graph rendering
    pub lane: usize,
}

impl CommitNode {
    pub fn is_merge(&self) -> bool {
        self.parent_shas.len() > 1
    }

    pub fn is_root(&self) -> bool {
        self.parent_shas.is_empty()
    }
}

/// A reference (branch, tag, HEAD) pointing to a commit
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphRef {
    pub name: SharedString,
    pub ref_type: RefType,
    pub is_remote: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefType {
    Head,
    LocalBranch,
    RemoteBranch,
    Tag,
}

impl GraphRef {
    pub fn short_name(&self) -> &str {
        self.name
            .strip_prefix("refs/heads/")
            .or_else(|| self.name.strip_prefix("refs/remotes/"))
            .or_else(|| self.name.strip_prefix("refs/tags/"))
            .unwrap_or(&self.name)
    }
}

/// A branch in the graph with its own color and lane
#[derive(Clone, Debug)]
pub struct GraphBranch {
    pub name: SharedString,
    pub color_index: usize,
    pub primary_lane: usize,
}

/// The complete git graph structure
#[derive(Clone, Debug, Default)]
pub struct GitGraph {
    /// All commits indexed by SHA
    pub commits: HashMap<SharedString, CommitNode>,
    /// Commits in topological order (newest first)
    pub ordered_commits: Vec<SharedString>,
    /// Branch information
    pub branches: Vec<GraphBranch>,
    /// HEAD ref
    pub head: Option<SharedString>,
    /// Number of lanes needed for layout
    pub lane_count: usize,
}

impl GitGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build graph from git log output with parent info
    /// Expected format per commit: SHA\0PARENT_SHAS\0SUBJECT\0TIMESTAMP\0AUTHOR_NAME\0AUTHOR_EMAIL\0REFS
    pub fn from_git_log(output: &str, delimiter: &str) -> anyhow::Result<Self> {
        let mut graph = Self::new();

        for commit_block in output.split(delimiter) {
            let commit_block = commit_block.trim();
            if commit_block.is_empty() {
                continue;
            }

            let fields: Vec<&str> = commit_block.split('\0').collect();
            if fields.len() >= 6 {
                let sha: SharedString = fields[0].trim().to_string().into();
                let parent_str = fields[1].trim();
                let subject: SharedString = fields[2].trim().to_string().into();
                let timestamp: i64 = fields[3].trim().parse().unwrap_or_else(|_| {
                    log::warn!("Failed to parse timestamp for commit {}, using 0", fields[0]);
                    0
                });
                let author_name: SharedString = fields[4].trim().to_string().into();
                let author_email: SharedString = fields[5].trim().to_string().into();
                let refs_str = if fields.len() > 6 { fields[6].trim() } else { "" };

                // Optimize: avoid double trim by splitting on whitespace directly
                let parent_shas: Vec<SharedString> = if parent_str.is_empty() {
                    vec![]
                } else {
                    parent_str.split_whitespace().map(|s| s.into()).collect()
                };

                let refs = Self::parse_refs(refs_str);

                // Optimize: create short_sha directly from &str without full string conversion
                let sha_str = sha.as_str();
                let short_sha: SharedString = sha_str[..7.min(sha_str.len())].into();

                let node = CommitNode {
                    sha: sha.clone(),
                    short_sha,
                    subject,
                    author_name,
                    author_email,
                    timestamp,
                    parent_shas,
                    refs,
                    lane: 0, // Will be assigned by layout
                };

                graph.ordered_commits.push(sha.clone());
                graph.commits.insert(sha, node);
            }
        }

        // Assign lanes (simple algorithm: each branch gets a lane)
        graph.assign_lanes();

        Ok(graph)
    }

    fn parse_refs(refs_str: &str) -> Vec<GraphRef> {
        if refs_str.is_empty() || refs_str == "()" {
            return vec![];
        }

        // Parse refs like "(HEAD -> main, origin/main, tag: v1.0)"
        let refs_str = refs_str.trim_start_matches('(').trim_end_matches(')');
        refs_str
            .split(',')
            .filter_map(|r| {
                let r = r.trim();
                if r.is_empty() {
                    return None;
                }

                let (name, ref_type, is_remote) = if r.starts_with("HEAD -> ") {
                    (r.strip_prefix("HEAD -> ").unwrap_or(r), RefType::Head, false)
                } else if r == "HEAD" {
                    ("HEAD", RefType::Head, false)
                } else if r.starts_with("tag: ") {
                    (r.strip_prefix("tag: ").unwrap_or(r), RefType::Tag, false)
                } else if r.contains('/') {
                    (r, RefType::RemoteBranch, true)
                } else {
                    (r, RefType::LocalBranch, false)
                };

                Some(GraphRef {
                    name: name.to_string().into(),
                    ref_type,
                    is_remote,
                })
            })
            .collect()
    }

    /// Simple lane assignment algorithm
    fn assign_lanes(&mut self) {
        let mut active_lanes: HashSet<usize> = HashSet::default();
        let mut commit_to_lane: HashMap<SharedString, usize> = HashMap::default();
        let mut next_lane = 0usize;

        for sha in &self.ordered_commits {
            if let Some(commit) = self.commits.get_mut(sha) {
                // Find or create a lane for this commit
                let lane = if let Some(&existing_lane) = commit_to_lane.get(sha) {
                    existing_lane
                } else {
                    // Find first available lane
                    let lane = (0..=next_lane).find(|l| !active_lanes.contains(l)).unwrap_or(next_lane);
                    if lane == next_lane {
                        next_lane += 1;
                    }
                    lane
                };

                commit.lane = lane;
                active_lanes.insert(lane);

                // Reserve lane for first parent (keeps main branch straight)
                if let Some(first_parent) = commit.parent_shas.first() {
                    commit_to_lane.insert(first_parent.clone(), lane);
                }

                // Other parents get new lanes
                for parent in commit.parent_shas.iter().skip(1) {
                    if !commit_to_lane.contains_key(parent) {
                        let new_lane = (0..=next_lane).find(|l| !active_lanes.contains(l)).unwrap_or(next_lane);
                        if new_lane == next_lane {
                            next_lane += 1;
                        }
                        commit_to_lane.insert(parent.clone(), new_lane);
                    }
                }

                // If this is the last commit referencing this lane, free it
                // (This would need a more complex algorithm for accurate lane reuse)
            }
        }

        self.lane_count = next_lane.max(1);
    }

    pub fn get_commit(&self, sha: &str) -> Option<&CommitNode> {
        self.commits.get(sha)
    }

    pub fn commit_count(&self) -> usize {
        self.ordered_commits.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_refs() {
        let refs = GitGraph::parse_refs("(HEAD -> main, origin/main)");
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].ref_type, RefType::Head);
        assert_eq!(refs[0].short_name(), "main");
        assert_eq!(refs[1].ref_type, RefType::RemoteBranch);
    }

    #[test]
    fn test_empty_graph() {
        let graph = GitGraph::new();
        assert_eq!(graph.commit_count(), 0);
    }
}
