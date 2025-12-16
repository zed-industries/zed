use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use gpui::{App, Entity};
use project::{Project, ProjectPath};
use util::rel_path::RelPath;

use crate::match_list::MatchList;
use crate::types::{GroupHeader, GroupKey, MatchId};

#[derive(Clone)]
pub struct GroupedFileHeader {
    pub key: GroupKey,
    pub header: GroupHeader,
    pub match_count: usize,
    pub worktree_name: Option<Arc<str>>,
    pub emphasize_worktree: bool,
}

#[derive(Clone)]
pub enum GroupedRow {
    FileHeader(GroupedFileHeader),
    LineMatch { match_id: MatchId },
}

#[derive(Default)]
pub struct GroupedListState {
    pub rows: Vec<GroupedRow>,
    pub collapsed_groups: HashSet<GroupKey>,
    pub multi_worktree: bool,
}

impl GroupedListState {
    pub fn clear(&mut self) {
        self.rows.clear();
        self.collapsed_groups.clear();
        self.multi_worktree = false;
    }

    pub fn rebuild(
        &mut self,
        match_list: &mut MatchList,
        selection: Option<MatchId>,
        project: &Entity<Project>,
        cx: &App,
    ) -> Option<usize> {
        let visible_matches = match_list.match_count();
        self.multi_worktree = project.read(cx).visible_worktrees(cx).count() > 1;
        if visible_matches == 0 {
            self.rows.clear();
            return None;
        }

        let mut per_group: HashMap<GroupKey, Vec<MatchId>> = HashMap::default();
        let mut group_order: Vec<GroupKey> = Vec::new();
        let mut group_headers: HashMap<GroupKey, GroupHeader> = HashMap::default();
        let mut group_project_paths: HashMap<GroupKey, ProjectPath> = HashMap::default();
        let mut path_occurrences: HashMap<Arc<RelPath>, usize> = HashMap::default();

        for match_index in 0..visible_matches {
            let Some(match_item) = match_list.item(match_index) else {
                continue;
            };
            let Some(group) = match_item.group.as_deref() else {
                continue;
            };
            let key = group.key;

            let entry = per_group.entry(key).or_insert_with(|| {
                group_order.push(key);
                group_headers.insert(key, group.header.clone());
                if let Some(project_path) = match_item.project_path().cloned() {
                    path_occurrences
                        .entry(project_path.path.clone())
                        .and_modify(|count| *count = count.saturating_add(1))
                        .or_insert(1);
                    group_project_paths.insert(key, project_path);
                }
                Vec::new()
            });
            entry.push(match_item.id);
        }

        let mut rows: Vec<GroupedRow> = Vec::with_capacity(visible_matches + per_group.len());
        for key in group_order {
            let Some(match_indices) = per_group.get(&key) else {
                continue;
            };
            let match_count = match_indices.len();

            let header = match group_headers.get(&key) {
                Some(h) => h.clone(),
                None => continue,
            };
            let project_path = group_project_paths.get(&key);

            let worktree_name = if self.multi_worktree {
                project_path.and_then(|project_path| {
                    project
                        .read(cx)
                        .worktree_for_id(project_path.worktree_id, cx)
                        .map(|worktree| Arc::<str>::from(worktree.read(cx).root_name_str().to_string()))
                })
            } else {
                None
            };

            let emphasize_worktree = self.multi_worktree
                && project_path.is_some_and(|project_path| {
                    path_occurrences
                        .get(&project_path.path)
                        .copied()
                        .unwrap_or(0)
                        > 1
                });

            rows.push(GroupedRow::FileHeader(GroupedFileHeader {
                key,
                header,
                match_count,
                worktree_name,
                emphasize_worktree,
            }));

            if self.collapsed_groups.contains(&key) {
                continue;
            }
            for &match_id in match_indices {
                rows.push(GroupedRow::LineMatch { match_id });
            }
        }

        self.rows = rows;

        selection.and_then(|id| self.row_index_for_match_id(id))
    }

    pub fn row_index_for_match_id(&self, id: MatchId) -> Option<usize> {
        self.rows.iter().position(|row| {
            matches!(row, GroupedRow::LineMatch { match_id } if *match_id == id)
        })
    }

    pub fn toggle_group_collapsed(
        &mut self,
        match_list: &mut MatchList,
        selection: Option<MatchId>,
        project: &Entity<Project>,
        key: GroupKey,
        cx: &App,
    ) -> Option<usize> {
        if self.collapsed_groups.contains(&key) {
            self.collapsed_groups.remove(&key);
        } else {
            self.collapsed_groups.insert(key);
        }
        self.rebuild(match_list, selection, project, cx)
    }

    pub fn toggle_all_groups_collapsed(
        &mut self,
        match_list: &mut MatchList,
        selection: Option<MatchId>,
        project: &Entity<Project>,
        clicked: GroupKey,
        cx: &App,
    ) -> Option<usize> {
        let clicked_is_collapsed = self.collapsed_groups.contains(&clicked);
        if clicked_is_collapsed {
            self.collapsed_groups.clear();
        } else {
            self.collapsed_groups.clear();
            for row in &self.rows {
                if let GroupedRow::FileHeader(header) = row {
                    self.collapsed_groups.insert(header.key);
                }
            }
        }
        self.rebuild(match_list, selection, project, cx)
    }
}

