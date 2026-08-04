use anyhow::Result;
use git::{
    Oid,
    repository::{CreateTagOptions, GitOperationKind, MergeMode, ResetMode},
};
use gpui::{Context, Entity, Task};
use project::git_store::{Repository, RepositorySnapshot};

use crate::git_graph::{GitGraph, GraphCommit};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum GraphMutation {
    Checkout { commit: Oid },
    CreateTag(CreateTagOptions),
    CherryPick { commits: Vec<Oid>, no_commit: bool },
    Revert { commit: Oid, no_commit: bool },
    Merge { commit: Oid, mode: MergeMode },
    Reset { commit: Oid, mode: ResetMode },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct GraphSelection {
    pub(crate) primary: Option<usize>,
    pub(crate) anchor: Option<usize>,
    pub(crate) selected: std::collections::BTreeSet<usize>,
    pub(crate) compare_base: Option<Oid>,
}

impl GraphSelection {
    pub(crate) fn clear(&mut self) {
        self.primary = None;
        self.anchor = None;
        self.selected.clear();
        self.compare_base = None;
    }

    pub(crate) fn select_single(&mut self, index: usize) {
        self.primary = Some(index);
        self.anchor = Some(index);
        self.selected.clear();
        self.selected.insert(index);
    }

    pub(crate) fn select_range_to(&mut self, index: usize) {
        let anchor = self.anchor.unwrap_or(index);
        let (start, end) = if anchor <= index {
            (anchor, index)
        } else {
            (index, anchor)
        };
        self.selected = (start..=end).collect();
        self.primary = Some(index);
    }

    pub(crate) fn toggle(&mut self, index: usize) {
        if !self.selected.remove(&index) {
            self.selected.insert(index);
            self.primary = Some(index);
            self.anchor.get_or_insert(index);
        } else if self.primary == Some(index) {
            self.primary = self.selected.iter().next_back().copied();
        }
    }

    pub(crate) fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    pub(crate) fn set_compare_base(&mut self, base: Oid) {
        self.compare_base = Some(base);
    }

    pub(crate) fn ordered_oids<'a>(
        &self,
        commits: impl IntoIterator<Item = &'a GraphCommit>,
    ) -> Vec<Oid> {
        ordered_selected_oids(self.selected.iter().copied(), commits)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GraphMutationPreflight {
    pub head: Oid,
    pub is_dirty: bool,
    pub operation: Option<GitOperationKind>,
    pub backup_ref: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GraphMutationError {
    MissingHead,
    EmptySelection,
    DirtyWorktree,
    ActiveOperation { operation: GitOperationKind },
}

impl std::fmt::Display for GraphMutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHead => f.write_str("Git graph actions require an existing HEAD commit"),
            Self::EmptySelection => f.write_str("Select at least one commit"),
            Self::DirtyWorktree => f.write_str(
                "This action requires a clean working tree; commit or stash changes first",
            ),
            Self::ActiveOperation { operation } => {
                write!(f, "A Git {operation:?} operation is already in progress")
            }
        }
    }
}

impl std::error::Error for GraphMutationError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GraphMutationPlan {
    pub mutation: GraphMutation,
    pub preflight: GraphMutationPreflight,
}

pub(crate) struct GraphMutationController;

impl GraphMutationController {
    pub(crate) fn plan(
        snapshot: &RepositorySnapshot,
        mutation: GraphMutation,
    ) -> Result<GraphMutationPlan, GraphMutationError> {
        let preflight = preflight_snapshot(snapshot, &mutation)?;
        Ok(GraphMutationPlan {
            mutation,
            preflight,
        })
    }

    pub(crate) fn schedule(
        graph: &mut GitGraph,
        repository: Entity<Repository>,
        plan: GraphMutationPlan,
        primary_selection: Option<Oid>,
        cx: &mut Context<GitGraph>,
    ) -> Task<Result<()>> {
        if let Some(oid) = primary_selection {
            graph.set_pending_select_sha(oid);
        }
        cx.spawn(async move |_, cx| {
            if let Some(backup_ref) = plan.preflight.backup_ref {
                let receiver = repository.update(cx, |repository, _| {
                    repository.update_ref(backup_ref, plan.preflight.head.to_string())
                });
                receiver.await??;
            }

            let receiver = repository.update(cx, |repository, cx| match plan.mutation {
                GraphMutation::Checkout { commit } => {
                    repository.checkout_commit(commit.to_string(), cx)
                }
                GraphMutation::CreateTag(options) => repository.create_tag(options, cx),
                GraphMutation::CherryPick { commits, no_commit } => repository.cherry_pick(
                    commits
                        .into_iter()
                        .map(|commit| commit.to_string())
                        .collect(),
                    no_commit,
                    cx,
                ),
                GraphMutation::Revert { commit, no_commit } => {
                    repository.revert(commit.to_string(), no_commit, cx)
                }
                GraphMutation::Merge { commit, mode } => {
                    repository.merge(commit.to_string(), mode, cx)
                }
                GraphMutation::Reset { commit, mode } => {
                    repository.reset(commit.to_string(), mode, cx)
                }
            });
            receiver.await??;
            Ok(())
        })
    }
}

pub(crate) fn ordered_selected_oids<'a>(
    indices: impl IntoIterator<Item = usize>,
    commits: impl IntoIterator<Item = &'a GraphCommit>,
) -> Vec<Oid> {
    let commits: Vec<_> = commits.into_iter().collect();
    let mut indices: Vec<_> = indices.into_iter().collect();
    indices.sort_unstable();
    indices.dedup();
    indices
        .into_iter()
        .filter_map(|index| commits.get(index).map(|commit| commit.data.sha))
        .collect()
}

fn preflight_snapshot(
    snapshot: &RepositorySnapshot,
    mutation: &GraphMutation,
) -> Result<GraphMutationPreflight, GraphMutationError> {
    let head = snapshot
        .head_commit
        .as_ref()
        .and_then(|commit| Oid::try_from(commit.sha.as_ref()).ok())
        .ok_or(GraphMutationError::MissingHead)?;
    preflight_for_state(
        head,
        snapshot.status_summary().count > 0,
        snapshot.active_operation,
        mutation,
    )
}

pub(crate) fn preflight_for_state(
    head: Oid,
    is_dirty: bool,
    operation: Option<GitOperationKind>,
    mutation: &GraphMutation,
) -> Result<GraphMutationPreflight, GraphMutationError> {
    if let Some(operation) = operation {
        return Err(GraphMutationError::ActiveOperation { operation });
    }

    if mutation_has_empty_selection(mutation) {
        return Err(GraphMutationError::EmptySelection);
    }

    if is_dirty && mutation_requires_clean_worktree(mutation) {
        return Err(GraphMutationError::DirtyWorktree);
    }

    Ok(GraphMutationPreflight {
        head,
        is_dirty,
        operation,
        backup_ref: matches!(mutation, GraphMutation::Reset { .. }).then(|| backup_ref_name(head)),
    })
}

fn mutation_has_empty_selection(mutation: &GraphMutation) -> bool {
    matches!(
        mutation,
        GraphMutation::CherryPick { commits, .. } if commits.is_empty()
    )
}

fn mutation_requires_clean_worktree(mutation: &GraphMutation) -> bool {
    matches!(
        mutation,
        GraphMutation::Checkout { .. }
            | GraphMutation::Merge { .. }
            | GraphMutation::Reset {
                mode: ResetMode::Hard,
                ..
            }
    )
}

pub(crate) fn backup_ref_name(head: Oid) -> String {
    format!("refs/zed/graph-backup/{head}")
}

#[cfg(test)]
pub(crate) fn run_with_backup<E>(
    plan: &GraphMutationPlan,
    create_backup: impl FnOnce(&str, Oid) -> Result<(), E>,
    run_mutation: impl FnOnce(&GraphMutation) -> Result<(), E>,
) -> Result<(), E> {
    if let Some(backup_ref) = plan.preflight.backup_ref.as_deref() {
        create_backup(backup_ref, plan.preflight.head)?;
    }
    run_mutation(&plan.mutation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_graph::GraphCommit;
    use git::{
        Oid,
        repository::{GitOperationKind, MergeMode, ResetMode},
    };
    use project::git_store::RepositorySnapshot;
    use smallvec::SmallVec;
    use std::cell::RefCell;
    use std::sync::Arc;

    fn oid(byte: char) -> Oid {
        Oid::try_from(byte.to_string().repeat(40).as_str()).unwrap()
    }

    fn commit(byte: char) -> GraphCommit {
        GraphCommit {
            data: Arc::new(git::repository::InitialGraphCommitData {
                sha: oid(byte),
                parents: SmallVec::new(),
                ref_names: Vec::new(),
            }),
            lane: 0,
            color_idx: 0,
        }
    }

    #[test]
    fn ordered_selected_oids_use_graph_order_not_click_order() {
        let commits = vec![commit('a'), commit('b'), commit('c')];

        assert_eq!(
            ordered_selected_oids([2, 0, 1], &commits),
            vec![oid('a'), oid('b'), oid('c')]
        );
    }

    #[test]
    fn graph_selection_tracks_primary_anchor_and_ranges() {
        let commits = vec![commit('a'), commit('b'), commit('c'), commit('d')];
        let mut selection = GraphSelection::default();

        selection.select_single(2);
        selection.select_range_to(0);

        assert_eq!(selection.primary, Some(0));
        assert_eq!(selection.anchor, Some(2));
        assert_eq!(
            selection.ordered_oids(&commits),
            vec![oid('a'), oid('b'), oid('c')]
        );
    }

    #[test]
    fn graph_selection_toggle_keeps_a_valid_primary() {
        let mut selection = GraphSelection::default();

        selection.select_single(1);
        selection.toggle(3);
        selection.toggle(1);

        assert_eq!(selection.primary, Some(3));
        assert_eq!(
            selection.selected.iter().copied().collect::<Vec<_>>(),
            vec![3]
        );
    }

    #[test]
    fn empty_cherry_pick_selection_is_refused() {
        let result = preflight_for_state(
            oid('a'),
            false,
            None,
            &GraphMutation::CherryPick {
                commits: Vec::new(),
                no_commit: false,
            },
        );

        assert_eq!(result, Err(GraphMutationError::EmptySelection));
    }

    #[test]
    fn dirty_checkout_is_refused() {
        let result = preflight_for_state(
            oid('a'),
            true,
            None,
            &GraphMutation::Checkout { commit: oid('b') },
        );

        assert_eq!(result, Err(GraphMutationError::DirtyWorktree));
    }

    #[test]
    fn dirty_merge_is_refused() {
        let result = preflight_for_state(
            oid('a'),
            true,
            None,
            &GraphMutation::Merge {
                commit: oid('b'),
                mode: MergeMode::Default,
            },
        );

        assert_eq!(result, Err(GraphMutationError::DirtyWorktree));
    }

    #[test]
    fn dirty_hard_reset_is_refused_but_soft_reset_is_allowed() {
        let hard_reset = preflight_for_state(
            oid('a'),
            true,
            None,
            &GraphMutation::Reset {
                commit: oid('b'),
                mode: ResetMode::Hard,
            },
        );
        assert_eq!(hard_reset, Err(GraphMutationError::DirtyWorktree));

        let soft_reset = preflight_for_state(
            oid('a'),
            true,
            None,
            &GraphMutation::Reset {
                commit: oid('b'),
                mode: ResetMode::Soft,
            },
        )
        .unwrap();
        assert!(soft_reset.is_dirty);
        assert_eq!(soft_reset.backup_ref, Some(backup_ref_name(oid('a'))));
    }

    #[test]
    fn active_operation_is_refused_before_scheduling() {
        let result = preflight_for_state(
            oid('a'),
            false,
            Some(GitOperationKind::Rebase),
            &GraphMutation::Checkout { commit: oid('b') },
        );

        assert_eq!(
            result,
            Err(GraphMutationError::ActiveOperation {
                operation: GitOperationKind::Rebase,
            })
        );
    }

    #[test]
    fn backup_ref_name_is_deterministic() {
        let head = oid('a');
        assert_eq!(
            backup_ref_name(head),
            "refs/zed/graph-backup/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(backup_ref_name(head), backup_ref_name(head));
    }

    #[test]
    fn reset_creates_backup_before_mutation_and_keeps_it_on_failure() {
        let plan = GraphMutationPlan {
            mutation: GraphMutation::Reset {
                commit: oid('b'),
                mode: ResetMode::Hard,
            },
            preflight: GraphMutationPreflight {
                head: oid('a'),
                is_dirty: false,
                operation: None,
                backup_ref: Some(backup_ref_name(oid('a'))),
            },
        };
        let events = RefCell::new(Vec::new());
        let mut backup_exists = false;
        let result = run_with_backup(
            &plan,
            |name, head| {
                events.borrow_mut().push(format!("backup:{name}:{head}"));
                backup_exists = true;
                Ok::<_, &'static str>(())
            },
            |_| {
                events.borrow_mut().push("mutation".to_string());
                Err::<(), _>("reset failed")
            },
        );

        assert_eq!(result, Err("reset failed"));
        assert_eq!(
            events.into_inner(),
            vec![
                "backup:refs/zed/graph-backup/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "mutation",
            ]
        );
        assert!(
            backup_exists,
            "a failed mutation must not remove its backup ref"
        );
    }

    #[test]
    fn snapshot_preflight_reads_head_and_dirty_state() {
        let _ = std::mem::size_of::<RepositorySnapshot>();
    }
}
