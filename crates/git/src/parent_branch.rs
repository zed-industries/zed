use anyhow::{Context as _, Result, ensure};
use collections::{HashMap, HashSet};
use smallvec::SmallVec;

struct ReflogEntry<'a> {
    reference: &'a str,
    timestamp: i64,
    commit: &'a str,
    message: &'a str,
}

impl<'a> ReflogEntry<'a> {
    fn parse(line: &'a str) -> Result<Self> {
        let mut fields = line.splitn(3, '\0');
        let (reference, timestamp) = fields
            .next()
            .and_then(|selector| selector.strip_suffix('}'))
            .and_then(|selector| selector.rsplit_once("@{"))
            .context("invalid reflog selector")?;
        Ok(Self {
            reference,
            timestamp: timestamp.parse().context("invalid reflog timestamp")?,
            commit: fields.next().context("missing reflog commit")?,
            message: fields.next().context("missing reflog message")?,
        })
    }
}

pub(crate) fn child_branches_from_reflog<'a>(
    output: &'a str,
    current_branch: &str,
) -> Result<HashSet<&'a str>> {
    let entries = output
        .lines()
        .map(ReflogEntry::parse)
        .collect::<Result<Vec<_>>>()?;
    let mut oldest_entries = HashMap::default();
    for entry in &entries {
        if let Some(branch) = entry.reference.strip_prefix("refs/heads/") {
            oldest_entries.insert(branch, entry);
        }
    }

    let mut first_checkouts = HashMap::default();
    for entry in &entries {
        if entry.reference != "HEAD" {
            continue;
        }
        let Some((source, destination)) = entry
            .message
            .strip_prefix("checkout: moving from ")
            .and_then(|message| message.split_once(" to "))
        else {
            continue;
        };
        if oldest_entries
            .get(destination)
            .is_some_and(|creation| entry.timestamp >= creation.timestamp)
        {
            first_checkouts.insert(destination, (source, entry));
        }
    }

    let mut children: HashMap<&str, Vec<&str>> = HashMap::default();
    for (branch, creation) in &oldest_entries {
        let Some(source) = creation.message.strip_prefix("branch: Created from ") else {
            continue;
        };
        let source = if source == "HEAD" {
            let Some((source, checkout)) = first_checkouts.get(branch) else {
                continue;
            };
            // A later visit to an existing branch is not evidence of its origin.
            if checkout.timestamp != creation.timestamp || checkout.commit != creation.commit {
                continue;
            }
            *source
        } else {
            source.strip_prefix("refs/heads/").unwrap_or(source)
        };
        if oldest_entries.get(source).is_some_and(|parent| {
            parent.message.starts_with("branch: Created from ")
                && parent.timestamp > creation.timestamp
        }) {
            // The source name may have been deleted and recreated since this branch was born.
            continue;
        }
        children.entry(source).or_default().push(branch);
    }

    let mut child_branches = HashSet::default();
    let mut pending = vec![current_branch];
    while let Some(parent) = pending.pop() {
        if let Some(children) = children.get(parent) {
            for &child in children {
                if child != current_branch && child_branches.insert(child) {
                    pending.push(child);
                }
            }
        }
    }
    Ok(child_branches)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParentBranchDistances {
    NotParent,
    Resolved {
        head_distance: u64,
        branch_distance: u64,
    },
    MultipleMergeBases,
}

pub(crate) struct ParentBranchGraph<'a> {
    commit_indices: HashMap<&'a str, usize>,
    parents: Vec<SmallVec<[usize; 1]>>,
    head_index: usize,
    head_ancestors: Vec<bool>,
    ancestor_counts: Vec<Option<u64>>,
    visited: Vec<u32>,
    generation: u32,
}

impl<'a> ParentBranchGraph<'a> {
    pub(crate) fn new(output: &'a str, head: &str) -> Result<Self> {
        let mut commit_indices = HashMap::default();
        for (index, line) in output.lines().enumerate() {
            let commit = line
                .split_ascii_whitespace()
                .next()
                .context("empty commit graph line")?;
            ensure!(
                commit_indices.insert(commit, index).is_none(),
                "duplicate commit {commit}"
            );
        }
        let head_index = *commit_indices
            .get(head)
            .context("HEAD missing from commit graph")?;
        let mut parents = Vec::<SmallVec<[usize; 1]>>::with_capacity(commit_indices.len());
        for (index, line) in output.lines().enumerate() {
            let mut commit_parents = SmallVec::new();
            for parent in line.split_ascii_whitespace().skip(1) {
                let parent_index = *commit_indices
                    .get(parent)
                    .with_context(|| format!("parent {parent} missing from commit graph"))?;
                ensure!(
                    parent_index > index,
                    "commit graph is not topologically ordered"
                );
                commit_parents.push(parent_index);
            }
            parents.push(commit_parents);
        }

        let mut ancestor_counts: Vec<Option<u64>> = vec![None; parents.len()];
        for (index, commit_parents) in parents.iter().enumerate().rev() {
            ancestor_counts[index] = match commit_parents.as_slice() {
                [] => Some(1),
                [parent] => ancestor_counts[*parent].map(|count| count + 1),
                _ => None,
            };
        }
        let mut graph = Self {
            head_ancestors: vec![false; parents.len()],
            visited: vec![0; parents.len()],
            generation: 0,
            ancestor_counts,
            parents,
            commit_indices,
            head_index,
        };
        let mut pending = vec![head_index];
        let mut head_count = 0;
        while let Some(index) = pending.pop() {
            if graph.head_ancestors[index] {
                continue;
            }
            graph.head_ancestors[index] = true;
            head_count += 1;
            pending.extend(graph.parents[index].iter().copied());
        }
        graph.ancestor_counts[head_index] = Some(head_count);
        Ok(graph)
    }

    pub(crate) fn distances(&mut self, candidate: &str) -> Result<ParentBranchDistances> {
        let candidate_index = self.index(candidate)?;
        if candidate_index == self.head_index {
            return Ok(ParentBranchDistances::Resolved {
                head_distance: 0,
                branch_distance: 0,
            });
        }
        let generation = self.next_generation();
        let mut pending = vec![candidate_index];
        let mut common = SmallVec::<[usize; 2]>::new();
        let mut branch_distance = 0;
        while let Some(index) = pending.pop() {
            if self.visited[index] == generation {
                continue;
            }
            self.visited[index] = generation;
            if self.head_ancestors[index] {
                common.push(index);
            } else {
                branch_distance += 1;
                pending.extend(self.parents[index].iter().copied());
            }
        }
        if common.is_empty() || common.contains(&self.head_index) {
            return Ok(ParentBranchDistances::NotParent);
        }
        if common.len() > 1 {
            let last_common = *common.iter().max().context("missing common ancestor")?;
            let generation = self.next_generation();
            let mut pending = common
                .iter()
                .flat_map(|index| self.parents[*index].iter().copied())
                .collect::<Vec<_>>();
            while let Some(index) = pending.pop() {
                // Parents have larger indices; older commits cannot dominate this frontier.
                if index > last_common || self.visited[index] == generation {
                    continue;
                }
                self.visited[index] = generation;
                pending.extend(self.parents[index].iter().copied());
            }
            common.retain(|index| self.visited[*index] != generation);
        }
        if common.len() > 1 {
            return Ok(ParentBranchDistances::MultipleMergeBases);
        }
        let base = *common.first().context("missing merge base")?;
        let head_count = self.ancestor_count(self.head_index);
        let base_count = self.ancestor_count(base);
        Ok(ParentBranchDistances::Resolved {
            head_distance: head_count
                .checked_sub(base_count)
                .context("merge base has more ancestors than HEAD")?,
            branch_distance,
        })
    }

    pub(crate) fn distances_from_base(
        &mut self,
        candidate: &str,
        base: &str,
    ) -> Result<(u64, u64)> {
        let candidate_index = self.index(candidate)?;
        let base_index = self.index(base)?;
        let base_count = self.ancestor_count(base_index);
        let head_count = self.ancestor_count(self.head_index);
        let candidate_count = self.ancestor_count(candidate_index);
        Ok((
            head_count
                .checked_sub(base_count)
                .context("merge base has more ancestors than HEAD")?,
            candidate_count
                .checked_sub(base_count)
                .context("merge base has more ancestors than candidate")?,
        ))
    }

    fn index(&self, commit: &str) -> Result<usize> {
        self.commit_indices
            .get(commit)
            .copied()
            .with_context(|| format!("commit {commit} missing from graph"))
    }

    fn next_generation(&mut self) -> u32 {
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 {
            self.visited.fill(0);
            self.generation = 1;
        }
        self.generation
    }

    fn ancestor_count(&mut self, commit: usize) -> u64 {
        if let Some(count) = self.ancestor_counts[commit] {
            return count;
        }
        let generation = self.next_generation();
        let mut pending = vec![commit];
        let mut count = 0;
        while let Some(index) = pending.pop() {
            if self.visited[index] == generation {
                continue;
            }
            self.visited[index] = generation;
            count += 1;
            pending.extend(self.parents[index].iter().copied());
        }
        self.ancestor_counts[commit] = Some(count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_branches_from_reflog_preserves_same_tip_parent() {
        // All events deliberately have the same timestamp: a later checkout
        // from the child back to its parent must not reverse the relationship.
        let output = concat!(
            "HEAD@{1}\0tip\0checkout: moving from new-child to feature\n",
            "refs/heads/new-child@{1}\0tip\0branch: Created from HEAD\n",
            "HEAD@{1}\0tip\0checkout: moving from feature to new-child\n",
            "refs/heads/feature@{1}\0tip\0branch: Created from HEAD\n",
            "HEAD@{1}\0tip\0checkout: moving from main to feature\n",
        );
        assert_eq!(
            child_branches_from_reflog(output, "feature").expect("infer child branches"),
            HashSet::from_iter(["new-child"])
        );
        assert!(
            child_branches_from_reflog(output, "new-child")
                .expect("preserve genuine parent")
                .is_empty()
        );
    }

    #[test]
    fn test_child_branches_from_reflog_supports_explicit_sources() {
        let output = concat!(
            "refs/heads/feature/new-child@{3}\0tip\0branch: Created from refs/heads/feature\n",
            "refs/heads/grandchild@{4}\0tip\0branch: Created from feature/new-child\n",
            "refs/heads/feature@{2}\0tip\0branch: Created from main\n",
        );
        assert_eq!(
            child_branches_from_reflog(output, "feature").expect("infer child branches"),
            HashSet::from_iter(["feature/new-child", "grandchild"])
        );
    }

    #[test]
    fn test_child_branches_from_reflog_requires_creation_evidence() {
        for output in [
            "",
            "HEAD@{2}\0tip\0checkout: moving from feature to new-child\n",
            concat!(
                "HEAD@{2}\0tip\0checkout: moving from feature to new-child\n",
                "refs/heads/new-child@{1}\0tip\0branch: Created from HEAD\n",
            ),
            concat!(
                "HEAD@{1}\0different\0checkout: moving from feature to new-child\n",
                "refs/heads/new-child@{1}\0tip\0branch: Created from HEAD\n",
            ),
            concat!(
                "refs/heads/feature@{3}\0tip\0branch: Created from main\n",
                "refs/heads/new-child@{2}\0tip\0branch: Created from feature\n",
            ),
        ] {
            assert!(
                child_branches_from_reflog(output, "feature")
                    .expect("fall back without creation evidence")
                    .is_empty(),
                "reflog: {output:?}"
            );
        }
    }

    #[test]
    fn test_child_branches_from_reflog_handles_cycles_and_invalid_data() {
        let output = concat!(
            "refs/heads/new-child@{1}\0tip\0branch: Created from feature\n",
            "refs/heads/feature@{1}\0tip\0branch: Created from new-child\n",
        );
        assert_eq!(
            child_branches_from_reflog(output, "feature").expect("handle cyclic origins"),
            HashSet::from_iter(["new-child"])
        );
        for output in ["invalid", "HEAD@{invalid}\0tip\0message", "HEAD@{1}\0tip"] {
            assert!(child_branches_from_reflog(output, "feature").is_err());
        }
    }

    #[test]
    fn test_parent_branch_graph_counts_unique_commits() {
        let mut graph = ParentBranchGraph::new(
            "head merge\nchild root\nmerge left right\nleft root\nright root\nroot\n",
            "head",
        )
        .expect("parse graph");
        assert_eq!(
            graph.distances("child").expect("score child"),
            ParentBranchDistances::Resolved {
                head_distance: 4,
                branch_distance: 1
            }
        );
    }

    #[test]
    fn test_parent_branch_graph_prunes_dominated_common_ancestors() {
        let mut graph = ParentBranchGraph::new(
            "head newer\nchild newer older\nnewer older\nolder root\nroot\n",
            "head",
        )
        .expect("parse graph");
        assert_eq!(
            graph.distances("child").expect("score child"),
            ParentBranchDistances::Resolved {
                head_distance: 1,
                branch_distance: 1
            }
        );
    }

    #[test]
    fn test_parent_branch_graph_handles_criss_cross_merges() {
        let mut graph = ParentBranchGraph::new(
            "head left right\nchild right left\nleft older\nright root\nolder root\nroot\n",
            "head",
        )
        .expect("parse graph");
        assert_eq!(
            graph.distances("child").expect("score child"),
            ParentBranchDistances::MultipleMergeBases
        );
        assert_eq!(
            graph
                .distances_from_base("child", "left")
                .expect("score left base"),
            (2, 2)
        );
        assert_eq!(
            graph
                .distances_from_base("child", "right")
                .expect("score right base"),
            (3, 3)
        );
    }

    #[test]
    fn test_parent_branch_graph_handles_descendants_and_unrelated_history() {
        let mut graph =
            ParentBranchGraph::new("descendant head\nhead root\nunrelated\nroot\n", "head")
                .expect("parse graph");
        assert_eq!(
            graph.distances("descendant").expect("score descendant"),
            ParentBranchDistances::NotParent
        );
        assert_eq!(
            graph.distances("unrelated").expect("score unrelated"),
            ParentBranchDistances::NotParent
        );
        assert_eq!(
            graph.distances("head").expect("score HEAD"),
            ParentBranchDistances::Resolved {
                head_distance: 0,
                branch_distance: 0
            }
        );
    }

    #[test]
    fn test_parent_branch_graph_rejects_invalid_graphs() {
        for output in [
            "head missing",
            "root\nhead root",
            "head head",
            "head\nhead",
            "head\n\nroot",
        ] {
            assert!(ParentBranchGraph::new(output, "head").is_err());
        }
    }
}
