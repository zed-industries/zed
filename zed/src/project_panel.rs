use crate::{project::Project, theme, Settings};
use gpui::{
    action,
    elements::{Label, MouseEventHandler, UniformList, UniformListState},
    Element, ElementBox, Entity, ModelHandle, MutableAppContext, ReadModel, View, ViewContext,
    WeakViewHandle,
};
use postage::watch;
use std::ops::Range;

pub struct ProjectPanel {
    project: ModelHandle<Project>,
    list: UniformListState,
    visible_entries: Vec<Vec<usize>>,
    expanded_dir_ids: Vec<Vec<usize>>,
    settings: watch::Receiver<Settings>,
    handle: WeakViewHandle<Self>,
}

#[derive(Debug, PartialEq, Eq)]
struct EntryDetails {
    filename: String,
    depth: usize,
    is_dir: bool,
    is_expanded: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProjectEntry {
    worktree_ix: usize,
    entry_id: usize,
}

action!(ToggleExpanded, ProjectEntry);
action!(Open, ProjectEntry);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ProjectPanel::toggle_expanded);
}

pub enum Event {}

impl ProjectPanel {
    pub fn new(
        project: ModelHandle<Project>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&project, |this, _, cx| {
            this.update_visible_entries(cx);
            cx.notify();
        })
        .detach();

        let mut this = Self {
            project,
            settings,
            list: Default::default(),
            visible_entries: Default::default(),
            expanded_dir_ids: Default::default(),
            handle: cx.handle().downgrade(),
        };
        this.update_visible_entries(cx);
        this
    }

    fn toggle_expanded(&mut self, action: &ToggleExpanded, cx: &mut ViewContext<Self>) {
        let ProjectEntry {
            worktree_ix,
            entry_id,
        } = action.0;
        let expanded_dir_ids = &mut self.expanded_dir_ids[worktree_ix];
        match expanded_dir_ids.binary_search(&entry_id) {
            Ok(ix) => {
                expanded_dir_ids.remove(ix);
            }
            Err(ix) => {
                expanded_dir_ids.insert(ix, entry_id);
            }
        }
        self.update_visible_entries(cx);
    }

    fn update_visible_entries(&mut self, cx: &mut ViewContext<Self>) {
        let worktrees = self.project.read(cx).worktrees();
        self.visible_entries.clear();
        for (worktree_ix, worktree) in worktrees.iter().enumerate() {
            let snapshot = worktree.read(cx).snapshot();

            if self.expanded_dir_ids.len() <= worktree_ix {
                self.expanded_dir_ids
                    .push(vec![snapshot.root_entry().unwrap().id])
            }

            let expanded_dir_ids = &self.expanded_dir_ids[worktree_ix];
            let mut visible_worktree_entries = Vec::new();
            let mut entry_iter = snapshot.entries(false);
            while let Some(item) = entry_iter.entry() {
                visible_worktree_entries.push(entry_iter.offset());
                if expanded_dir_ids.binary_search(&item.id).is_err() {
                    if entry_iter.advance_to_sibling() {
                        continue;
                    }
                }
                entry_iter.advance();
            }
            self.visible_entries.push(visible_worktree_entries);
        }
    }

    fn append_visible_entries<C: ReadModel, T>(
        &self,
        range: Range<usize>,
        items: &mut Vec<T>,
        cx: &mut C,
        mut render_item: impl FnMut(ProjectEntry, EntryDetails, &mut C) -> T,
    ) {
        let worktrees = self.project.read(cx).worktrees().to_vec();
        let mut total_ix = 0;
        for (worktree_ix, visible_worktree_entries) in self.visible_entries.iter().enumerate() {
            if total_ix >= range.end {
                break;
            }
            if total_ix + visible_worktree_entries.len() <= range.start {
                total_ix += visible_worktree_entries.len();
                continue;
            }

            let expanded_entry_ids = &self.expanded_dir_ids[worktree_ix];
            let snapshot = worktrees[worktree_ix].read(cx).snapshot();
            let mut cursor = snapshot.entries(false);
            for ix in visible_worktree_entries[(range.start - total_ix)..]
                .iter()
                .copied()
            {
                cursor.advance_to_offset(ix);
                if let Some(entry) = cursor.entry() {
                    let details = EntryDetails {
                        filename: entry.path.file_name().map_or_else(
                            || snapshot.root_name().to_string(),
                            |name| name.to_string_lossy().to_string(),
                        ),
                        depth: entry.path.components().count(),
                        is_dir: entry.is_dir(),
                        is_expanded: expanded_entry_ids.binary_search(&entry.id).is_ok(),
                    };
                    let entry = ProjectEntry {
                        worktree_ix,
                        entry_id: entry.id,
                    };
                    items.push(render_item(entry, details, cx));
                }
                total_ix += 1;
            }
        }
    }

    fn render_entry(
        entry: ProjectEntry,
        details: EntryDetails,
        theme: &theme::ProjectPanel,
        cx: &mut ViewContext<Self>,
    ) -> ElementBox {
        let is_dir = details.is_dir;
        MouseEventHandler::new::<Self, _, _, _>(
            (entry.worktree_ix, entry.entry_id),
            cx,
            |state, cx| {
                Label::new(details.filename, theme.entry.clone())
                    .contained()
                    .with_margin_left(details.depth as f32 * 20.)
                    .boxed()
            },
        )
        .on_click(move |cx| {
            if is_dir {
                cx.dispatch_action(ToggleExpanded(entry))
            } else {
                cx.dispatch_action(Open(entry))
            }
        })
        .boxed()
    }
}

impl View for ProjectPanel {
    fn ui_name() -> &'static str {
        "ProjectPanel"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let settings = self.settings.clone();
        let handle = self.handle.clone();
        UniformList::new(
            self.list.clone(),
            self.visible_entries.len(),
            move |range, items, cx| {
                let theme = &settings.borrow().theme.project_panel;
                let this = handle.upgrade(cx).unwrap();
                this.update(cx.app, |this, cx| {
                    this.append_visible_entries(range, items, cx, |entry, details, cx| {
                        Self::render_entry(entry, details, theme, cx)
                    });
                })
            },
        )
        .contained()
        .with_style(self.settings.borrow().theme.project_panel.container)
        .boxed()
    }
}

impl Entity for ProjectPanel {
    type Event = Event;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_app_state;
    use gpui::{TestAppContext, ViewHandle};
    use serde_json::json;
    use std::{collections::HashSet, path::Path};

    #[gpui::test]
    async fn test_visible_list(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        let settings = app_state.settings.clone();
        let fs = app_state.fs.as_fake();

        fs.insert_tree(
            "/root1",
            json!({
                ".dockerignore": "",
                ".git": {
                    "HEAD": "",
                },
                "a": {
                    "0": { "q": "", "r": "", "s": "" },
                    "1": { "t": "", "u": "" },
                    "2": { "v": "", "w": "", "x": "", "y": "" },
                },
                "b": {
                    "3": { "Q": "" },
                    "4": { "R": "", "S": "", "T": "", "U": "" },
                },
                "c": {
                    "5": {},
                    "6": { "V": "", "W": "" },
                    "7": { "X": "" },
                    "8": { "Y": {}, "Z": "" }
                }
            }),
        )
        .await;

        let project = cx.add_model(|_| Project::new(&app_state));
        let worktree = project
            .update(&mut cx, |project, cx| {
                project.add_local_worktree("/root1".as_ref(), cx)
            })
            .await
            .unwrap();
        worktree
            .read_with(&cx, |t, _| t.as_local().unwrap().scan_complete())
            .await;

        let (_, panel) = cx.add_window(|cx| ProjectPanel::new(project, settings, cx));
        assert_eq!(
            visible_entry_details(&panel, 0..50, &mut cx),
            &[
                EntryDetails {
                    filename: "root1".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                },
                EntryDetails {
                    filename: ".dockerignore".to_string(),
                    depth: 1,
                    is_dir: false,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "a".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "b".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "c".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                },
            ]
        );

        toggle_expand_dir(&panel, "root1/b", &mut cx);
        assert_eq!(
            visible_entry_details(&panel, 0..50, &mut cx),
            &[
                EntryDetails {
                    filename: "root1".to_string(),
                    depth: 0,
                    is_dir: true,
                    is_expanded: true,
                },
                EntryDetails {
                    filename: ".dockerignore".to_string(),
                    depth: 1,
                    is_dir: false,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "a".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "b".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: true,
                },
                EntryDetails {
                    filename: "3".to_string(),
                    depth: 2,
                    is_dir: true,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "4".to_string(),
                    depth: 2,
                    is_dir: true,
                    is_expanded: false,
                },
                EntryDetails {
                    filename: "c".to_string(),
                    depth: 1,
                    is_dir: true,
                    is_expanded: false,
                },
            ]
        );

        fn toggle_expand_dir(
            panel: &ViewHandle<ProjectPanel>,
            path: impl AsRef<Path>,
            cx: &mut TestAppContext,
        ) {
            let path = path.as_ref();
            panel.update(cx, |panel, cx| {
                for (worktree_ix, worktree) in panel.project.read(cx).worktrees().iter().enumerate()
                {
                    let worktree = worktree.read(cx);
                    if let Ok(relative_path) = path.strip_prefix(worktree.root_name()) {
                        let entry_id = worktree.entry_for_path(relative_path).unwrap().id;
                        panel.toggle_expanded(
                            &ToggleExpanded(ProjectEntry {
                                worktree_ix,
                                entry_id,
                            }),
                            cx,
                        );
                        return;
                    }
                }
                panic!("no worktree for path {:?}", path);
            });
        }

        fn visible_entry_details(
            panel: &ViewHandle<ProjectPanel>,
            range: Range<usize>,
            cx: &mut TestAppContext,
        ) -> Vec<EntryDetails> {
            let mut result = Vec::new();
            let mut project_entries = HashSet::new();
            panel.update(cx, |panel, cx| {
                panel.append_visible_entries(
                    range,
                    &mut result,
                    cx,
                    |project_entry, details, _| {
                        assert!(
                            project_entries.insert(project_entry),
                            "duplicate project entry {:?} {:?}",
                            project_entry,
                            details
                        );
                        details
                    },
                );
            });

            result
        }
    }
}
