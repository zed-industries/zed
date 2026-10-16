use anyhow::{Context as _, Result, ensure};
use collections::HashMap;
use smallvec::SmallVec;

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
