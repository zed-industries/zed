use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use gpui::{App, Entity};
use project::{Project, ProjectPath};
use util::rel_path::RelPath;

use crate::match_list::MatchList;
use crate::types::MatchId;

#[derive(Clone)]
pub struct GroupedFileHeader {
    pub project_path: ProjectPath,
    pub file_name: Arc<str>,
    pub parent_path: Arc<str>,
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
    pub collapsed_files: HashSet<ProjectPath>,
    pub multi_worktree: bool,
}

impl GroupedListState {
    pub fn clear(&mut self) {
        self.rows.clear();
        self.collapsed_files.clear();
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

        let mut per_file: HashMap<ProjectPath, Vec<MatchId>> = HashMap::default();
        let mut file_order: Vec<ProjectPath> = Vec::new();
        let mut path_occurrences: HashMap<Arc<RelPath>, usize> = HashMap::default();

        for match_index in 0..visible_matches {
            let Some(match_item) = match_list.item(match_index) else {
                continue;
            };
            let Some(buffer) = match_item.buffer() else {
                continue;
            };
            let Some(file) = buffer.read(cx).file() else {
                continue;
            };
            let project_path = ProjectPath::from_file(file.as_ref(), cx);

            let entry = per_file.entry(project_path.clone()).or_insert_with(|| {
                file_order.push(project_path.clone());
                path_occurrences
                    .entry(project_path.path.clone())
                    .and_modify(|count| *count = count.saturating_add(1))
                    .or_insert(1);
                Vec::new()
            });
            entry.push(match_item.id);
        }

        let mut rows: Vec<GroupedRow> = Vec::with_capacity(visible_matches + per_file.len());
        for project_path in file_order {
            let Some(match_indices) = per_file.get(&project_path) else {
                continue;
            };
            let match_count = match_indices.len();

            let file_name: Arc<str> = project_path
                .path
                .file_name()
                .map(|name| Arc::<str>::from(name.to_string()))
                .unwrap_or_else(|| Arc::<str>::from(project_path.path.as_unix_str().to_string()));
            let parent_path: Arc<str> = project_path
                .path
                .parent()
                .map(|path| Arc::<str>::from(path.as_unix_str().to_string()))
                .unwrap_or_else(|| Arc::<str>::from(""));

            let worktree_name = if self.multi_worktree {
                project
                    .read(cx)
                    .worktree_for_id(project_path.worktree_id, cx)
                    .map(|worktree| Arc::<str>::from(worktree.read(cx).root_name_str().to_string()))
            } else {
                None
            };

            let emphasize_worktree = self.multi_worktree
                && path_occurrences
                    .get(&project_path.path)
                    .copied()
                    .unwrap_or(0)
                    > 1;

            rows.push(GroupedRow::FileHeader(GroupedFileHeader {
                project_path: project_path.clone(),
                file_name,
                parent_path,
                match_count,
                worktree_name,
                emphasize_worktree,
            }));

            if self.collapsed_files.contains(&project_path) {
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
        project_path: &ProjectPath,
        cx: &App,
    ) -> Option<usize> {
        if self.collapsed_files.contains(project_path) {
            self.collapsed_files.remove(project_path);
        } else {
            self.collapsed_files.insert(project_path.clone());
        }
        self.rebuild(match_list, selection, project, cx)
    }

    pub fn toggle_all_groups_collapsed(
        &mut self,
        match_list: &mut MatchList,
        selection: Option<MatchId>,
        project: &Entity<Project>,
        clicked: &ProjectPath,
        cx: &App,
    ) -> Option<usize> {
        let clicked_is_collapsed = self.collapsed_files.contains(clicked);
        if clicked_is_collapsed {
            self.collapsed_files.clear();
        } else {
            self.collapsed_files.clear();
            for row in &self.rows {
                if let GroupedRow::FileHeader(header) = row {
                    self.collapsed_files.insert(header.project_path.clone());
                }
            }
        }
        self.rebuild(match_list, selection, project, cx)
    }
}

