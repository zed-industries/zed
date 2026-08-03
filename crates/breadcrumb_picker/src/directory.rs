use std::rc::Rc;
use std::sync::Arc;

use editor::{
    BreadcrumbDirectoryEntry, BreadcrumbDirectoryListingSettings, Editor,
    ErasedBreadcrumbPopoverHandle, breadcrumb_directory_entries,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, MouseButton, ParentElement, Styled, Task,
    WeakEntity, Window, div, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{Project, ProjectPath, WorktreeId};
use settings::Settings;
use ui::{
    ButtonLike, ButtonSize, ButtonStyle, Color, HighlightedLabel, Icon, IconSize, ListItem,
    ListItemSpacing, PopoverMenu, PopoverMenuHandle, prelude::*,
};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::Workspace;

use crate::MAX_BREADCRUMB_MENU_ENTRIES;

/// Bounds how far [`descend_single_child_directories`] walks, guarding against a pathologically
/// deep chain or a symlink cycle.
const MAX_BREADCRUMB_DESCENT_DEPTH: usize = 64;

/// Walks down through directories that hold exactly one child directory, stopping one short of a
/// file so the user still opens it themselves.
fn descend_single_child_directories(
    start: Arc<RelPath>,
    mut child_entries: impl FnMut(&RelPath) -> Vec<(Arc<RelPath>, bool)>,
) -> Arc<RelPath> {
    let mut current = start;
    for _ in 0..MAX_BREADCRUMB_DESCENT_DEPTH {
        let children = child_entries(&current);
        let [(only_child_path, only_child_is_dir)] = children.as_slice() else {
            return current;
        };
        if !only_child_is_dir {
            return current;
        }
        current = only_child_path.clone();
    }
    current
}

fn breadcrumb_directory_children(
    worktree: &Entity<project::Worktree>,
    path: &RelPath,
    cx: &App,
) -> Vec<(Arc<RelPath>, bool)> {
    worktree
        .read(cx)
        .snapshot()
        .child_entries(path)
        .map(|entry| (entry.path.clone(), entry.is_dir()))
        .collect()
}

/// Which icon a listing row uses. With folder icons off a directory falls back to a chevron
/// rather than to nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BreadcrumbEntryIconSource {
    File,
    Folder,
    Chevron,
    None,
}

fn breadcrumb_entry_label_color(
    entry: &BreadcrumbDirectoryEntry,
    git_status_enabled: bool,
    is_active_file: bool,
) -> Color {
    let git_summary = if git_status_enabled {
        entry.git_summary
    } else {
        Default::default()
    };
    editor::items::entry_git_aware_label_color(git_summary, entry.is_ignored, is_active_file)
}

// Diagnostics tint the icon and git status the label, the same split the project panel uses.
fn breadcrumb_entry_icon_color(entry: &BreadcrumbDirectoryEntry) -> Color {
    editor::items::entry_diagnostic_aware_icon_decoration_and_color(entry.diagnostic_severity)
        .map(|(_, color)| color)
        .unwrap_or(Color::Muted)
}

fn breadcrumb_entry_icon_source(
    is_dir: bool,
    show_file_icons: bool,
    show_folder_icons: bool,
) -> BreadcrumbEntryIconSource {
    if is_dir {
        if show_folder_icons {
            BreadcrumbEntryIconSource::Folder
        } else {
            BreadcrumbEntryIconSource::Chevron
        }
    } else if show_file_icons {
        BreadcrumbEntryIconSource::File
    } else {
        BreadcrumbEntryIconSource::None
    }
}

/// The directory dropdown's contents. Choosing a directory navigates the bar into it; choosing a
/// file opens it.
pub struct BreadcrumbDirectoryDelegate {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    current_path: Arc<RelPath>,
    /// The open file's own path, so the row leading to it reads as the current one.
    active_path: Option<Arc<RelPath>>,
    entries: Vec<BreadcrumbDirectoryEntry>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    /// Whether any row draws an icon: reserving the column when every icon is off would indent
    /// the list for nothing.
    show_icons: bool,
    _expand_task: gpui::Task<()>,
}

pub type BreadcrumbDirectoryPicker = Picker<BreadcrumbDirectoryDelegate>;

/// Newtype over the concrete popover handle, so it can implement `ErasedBreadcrumbPopoverHandle`
/// without running into the orphan rule.
pub(crate) struct DirectoryPopoverHandle(pub PopoverMenuHandle<BreadcrumbDirectoryPicker>);

impl ErasedBreadcrumbPopoverHandle for DirectoryPopoverHandle {
    fn hide(&self, cx: &mut App) {
        self.0.hide(cx);
    }

    fn show(&self, window: &mut Window, cx: &mut App) {
        self.0.show(window, cx);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl BreadcrumbDirectoryDelegate {
    fn picker(
        editor: WeakEntity<Editor>,
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        current_path: Arc<RelPath>,
        active_path: Option<Arc<RelPath>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<BreadcrumbDirectoryPicker> {
        cx.new(|cx| {
            let delegate = Self {
                editor,
                workspace,
                worktree_id,
                current_path,
                active_path,
                entries: Vec::new(),
                matches: Vec::new(),
                selected_index: 0,
                show_icons: false,
                _expand_task: gpui::Task::ready(()),
            };
            let mut picker = Picker::uniform_list(delegate, window, cx)
                .popover()
                // Narrower than the picker default, which is sized for modals: this lists file
                // names beside their own segment.
                .initial_width(rems(15.));
            picker.delegate.reload_entries(cx);
            picker.delegate.expand_current_path(window, cx);
            picker.delegate.select_active_path();
            picker
        })
    }

    fn project(&self, cx: &App) -> Option<Entity<Project>> {
        Some(self.workspace.upgrade()?.read(cx).project().clone())
    }

    fn worktree(&self, cx: &App) -> Option<Entity<project::Worktree>> {
        self.project(cx)?
            .read(cx)
            .worktree_for_id(self.worktree_id, cx)
    }

    fn reload_entries(&mut self, cx: &App) {
        let (Some(project), Some(worktree)) = (self.project(cx), self.worktree(cx)) else {
            self.entries = Vec::new();
            return;
        };
        self.entries = breadcrumb_directory_entries(&project, &worktree, &self.current_path, cx);

        let settings = BreadcrumbDirectoryListingSettings::get_global(cx);
        self.show_icons = self.entries.iter().any(|entry| {
            breadcrumb_entry_icon_source(entry.is_dir, settings.file_icons, settings.folder_icons)
                != BreadcrumbEntryIconSource::None
        });
    }

    /// Starts on the row leading to the open file, so the dropdown opens where the user already is
    /// rather than at the top of an unrelated directory.
    fn select_active_path(&mut self) {
        let Some(active_path) = self.active_path.as_ref() else {
            return;
        };
        self.selected_index = self
            .matches
            .iter()
            .position(|entry_match| {
                self.entries
                    .get(entry_match.candidate_id)
                    .is_some_and(|entry| active_path.starts_with(&entry.path))
            })
            .unwrap_or(0);
    }

    /// Triggers the same worktree scan the project panel makes when a directory is expanded.
    /// Gitignored directories are never scanned proactively, so without this the dropdown lists
    /// nothing.
    fn expand_current_path(
        &mut self,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) {
        let Some(project) = self.project(cx) else {
            return;
        };
        let Some(entry_id) = self
            .worktree(cx)
            .and_then(|worktree| worktree.read(cx).entry_for_path(&self.current_path))
            .map(|entry| entry.id)
        else {
            return;
        };
        let Some(expand) = project.update(cx, |project, cx| {
            project.expand_entry(self.worktree_id, entry_id, cx)
        }) else {
            return;
        };

        self._expand_task = cx.spawn_in(window, async move |picker, cx| {
            expand.await.log_err();
            picker
                .update_in(cx, |picker, window, cx| picker.refresh(window, cx))
                .ok();
        });
    }

    fn entry_at(&self, index: usize) -> Option<&BreadcrumbDirectoryEntry> {
        self.entries.get(self.matches.get(index)?.candidate_id)
    }
}

impl PickerDelegate for BreadcrumbDirectoryDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "breadcrumb directory picker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) {
        self.selected_index = index;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search this folder…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(if self.entries.is_empty() {
            "Empty directory".into()
        } else {
            "No matches".into()
        })
    }

    fn editor_position(&self) -> PickerEditorPosition {
        // Below the list, so a folder with three entries still reads as a menu rather than as a
        // search dialog that happens to have results.
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Task<()> {
        // Re-read rather than filtered from the previous listing: a pending scan or an edit
        // elsewhere may have changed what this directory holds.
        self.reload_entries(cx);
        let candidates = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| StringMatchCandidate::new(index, &entry.name))
            .collect::<Vec<_>>();

        if query.is_empty() {
            self.matches = candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    string: candidate.string,
                    positions: Vec::new(),
                    score: 0.,
                })
                .collect();
            self.select_active_path();
            cx.notify();
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                MAX_BREADCRUMB_MENU_ENTRIES,
                &Default::default(),
                executor,
            )
            .await;
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) {
        let Some(entry) = self.entry_at(self.selected_index) else {
            return;
        };
        let entry_path = entry.path.clone();

        if !entry.is_dir {
            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_path(
                            ProjectPath {
                                worktree_id: self.worktree_id,
                                path: entry_path,
                            },
                            None,
                            true,
                            window,
                            cx,
                        )
                        .detach_and_log_err(cx);
                });
            }
            cx.emit(DismissEvent);
            return;
        }

        // Descending happens on confirm rather than on open: a segment's own dropdown lists its
        // children verbatim, and only choosing a row walks through single-child directories.
        let Some(worktree) = self.worktree(cx) else {
            return;
        };
        let resolved_path = descend_single_child_directories(entry_path, |path| {
            breadcrumb_directory_children(&worktree, path, cx)
        });

        // `current_path` isn't updated in place: the popover is dismissed and reopened under the
        // resolved directory's own segment.
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_breadcrumb_to(self.worktree_id, resolved_path, window, cx);
            });
        }
    }

    // Some rather than None when handled: None lets the keystroke fall through to cursor
    // movement in the query editor.
    fn select_child(
        &mut self,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Option<String> {
        if self.entry_at(self.selected_index)?.is_dir {
            self.confirm(false, window, cx);
            return Some(String::new());
        }
        None
    }

    fn select_parent(
        &mut self,
        window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Option<String> {
        let parent = self.current_path.parent()?.into_arc();
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_breadcrumb_to(self.worktree_id, parent, window, cx);
            });
        }
        Some(String::new())
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<BreadcrumbDirectoryPicker>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbDirectoryPicker>,
    ) -> Option<Self::ListItem> {
        let entry = self.entry_at(index)?;
        let listing_settings = BreadcrumbDirectoryListingSettings::get_global(cx);

        let leads_to_active_path = entry.is_dir
            && self
                .active_path
                .as_ref()
                .is_some_and(|active_path| active_path.starts_with(&entry.path));
        let is_active_file =
            !entry.is_dir && self.active_path.as_deref() == Some(entry.path.as_ref());

        let icon_path = match breadcrumb_entry_icon_source(
            entry.is_dir,
            listing_settings.file_icons,
            listing_settings.folder_icons,
        ) {
            BreadcrumbEntryIconSource::File => {
                file_icons::FileIcons::get_icon(entry.path.as_std_path(), cx)
            }
            BreadcrumbEntryIconSource::Folder => file_icons::FileIcons::get_folder_icon(
                leads_to_active_path,
                entry.path.as_std_path(),
                cx,
            ),
            BreadcrumbEntryIconSource::Chevron => {
                // These rows aren't expandable in place — choosing a directory navigates into it
                // rather than expanding it inline — so there's no expanded state to reflect.
                file_icons::FileIcons::get_chevron_icon(false, cx)
            }
            BreadcrumbEntryIconSource::None => None,
        };
        let icon = icon_path.map(Icon::from_path).map(|icon| {
            icon.color(breadcrumb_entry_icon_color(entry))
                .size(IconSize::Small)
                .into_any_element()
        });

        let label_color =
            breadcrumb_entry_label_color(entry, listing_settings.git_status, is_active_file);

        Some(
            ListItem::new(SharedString::from(format!(
                "breadcrumb-directory-entry-{index}"
            )))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .when(self.show_icons, |this| {
                this.start_slot(
                    div()
                        .flex_none()
                        .size(IconSize::Small.rems())
                        .children(icon),
                )
            })
            .child(
                HighlightedLabel::new(entry.name.clone(), self.matches[index].positions.clone())
                    .color(label_color),
            )
            .into_any_element(),
        )
    }
}

/// A path segment whose dropdown lists `path`'s direct children, directories first then
/// alphabetical. Opening it only marks the segment active; single-child chains are resolved only
/// once a row is chosen.
pub(crate) fn render_breadcrumb_directory_segment(
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    path: Arc<RelPath>,
    active_path: Option<Arc<RelPath>>,
    is_active_segment: bool,
    shared_popover_handle: Rc<dyn ErasedBreadcrumbPopoverHandle>,
    label: gpui::AnyElement,
    index: usize,
) -> gpui::AnyElement {
    let trigger = ButtonLike::new(("breadcrumb-directory", index))
        .style(ButtonStyle::Transparent)
        .size(ButtonSize::None)
        .height(rems_from_px(22.).into())
        .tooltip(ui::Tooltip::text("Double-Click to Reveal in Project Panel"))
        .child(label);

    // Only the active segment's popover carries the shared handle `Editor::navigate_breadcrumb_to`
    // reopens through; the rest get a throwaway one.
    let popover_handle = if is_active_segment {
        shared_popover_handle
            .as_any()
            .downcast_ref::<DirectoryPopoverHandle>()
            .map(|handle| handle.0.clone())
            .unwrap_or_default()
    } else {
        PopoverMenuHandle::default()
    };

    let reveal_workspace = workspace.clone();
    let reveal_path = path.clone();

    let menu = PopoverMenu::new(("breadcrumb-directory-menu", index))
        .with_handle(popover_handle)
        .trigger(trigger)
        .menu(move |window, cx| {
            let workspace_entity = workspace.upgrade()?;
            workspace_entity
                .read(cx)
                .project()
                .read(cx)
                .worktree_for_id(worktree_id, cx)?;

            if let Some(editor_entity) = editor.upgrade() {
                editor_entity.update(cx, |editor, cx| {
                    editor.open_breadcrumb_navigation(worktree_id, path.clone(), cx);
                });
            }

            let picker = BreadcrumbDirectoryDelegate::picker(
                editor.clone(),
                workspace.clone(),
                worktree_id,
                path.clone(),
                active_path.clone(),
                window,
                cx,
            );
            if let Some(editor_entity) = editor.upgrade() {
                editor_entity.update(cx, |editor, cx| {
                    editor.watch_breadcrumb_dismissal(&picker, worktree_id, path.clone(), cx);
                });
            }
            Some(picker)
        });

    // Double clicking a segment reveals its directory in the project panel, the way IntelliJ's
    // navigation bar does. Capture phase, and on mouse down rather than on click: the first
    // click opens the popover, whose window-level dismiss handler then swallows the second
    // mouse down on the trigger (`PopoverMenu::paint` stops its propagation), so neither a
    // click handler nor a bubble-phase mouse down handler ever sees a double click.
    div()
        .capture_any_mouse_down(move |event, _, cx| {
            if event.button != MouseButton::Left || event.click_count < 2 {
                return;
            }
            reveal_breadcrumb_directory_in_project_panel(
                &reveal_workspace,
                worktree_id,
                &reveal_path,
                cx,
            );
        })
        .child(menu)
        .into_any_element()
}

/// Selects `path` in the project panel, expanding whatever is needed to show it.
fn reveal_breadcrumb_directory_in_project_panel(
    workspace: &WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    path: &RelPath,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        return;
    };
    let project = workspace.read(cx).project().clone();
    let Some(entry_id) = project
        .read(cx)
        .entry_for_path(
            &ProjectPath {
                worktree_id,
                path: path.into(),
            },
            cx,
        )
        .map(|entry| entry.id)
    else {
        return;
    };
    project.update(cx, |_, cx| {
        // Opened explicitly rather than relying on the reveal to do it: the panel only activates
        // itself when the reveal succeeds, and a closed panel should open either way, which is
        // what IntelliJ does.
        cx.emit(project::Event::ActivateProjectPanel);
        cx.emit(project::Event::RevealInProjectPanel(entry_id));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    use editor::Editor;
    use gpui::{Focusable, Render, TestAppContext, VisualTestContext};
    use std::cell::RefCell;
    use workspace::Workspace;

    /// Selects the row for `path` in an open directory picker and confirms it, standing in for a
    /// click on that row.
    fn confirm_breadcrumb_row(
        picker: &Entity<BreadcrumbDirectoryPicker>,
        path: &str,
        cx: &mut VisualTestContext,
    ) {
        use util::rel_path::rel_path;
        picker.update_in(cx, |picker, window, cx| {
            let index = picker
                .delegate
                .matches
                .iter()
                .position(|entry_match| {
                    picker.delegate.entries[entry_match.candidate_id]
                        .path
                        .as_ref()
                        == rel_path(path)
                })
                .expect("row is listed");
            picker.delegate.selected_index = index;
            picker.delegate.confirm(false, window, cx);
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let _ = workspace::AppState::test(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    fn test_entry(git_summary: git::status::GitSummary) -> BreadcrumbDirectoryEntry {
        BreadcrumbDirectoryEntry {
            name: "file.txt".into(),
            path: util::rel_path::rel_path("file.txt").into_arc(),
            is_dir: false,
            is_ignored: false,
            git_summary,
            diagnostic_severity: None,
        }
    }

    #[test]
    fn test_breadcrumb_entry_label_color_honors_git_status_setting() {
        let entry = test_entry(git::status::GitSummary::UNTRACKED);

        // On: an untracked file reads as "created", same as the project panel.
        assert_eq!(
            breadcrumb_entry_label_color(&entry, true, false),
            Color::Created
        );

        // Off: falls back to the status-less color, ignoring the entry's own git summary — same
        // as an unselected, untracked project panel entry with `git_status` off.
        assert_eq!(
            breadcrumb_entry_label_color(&entry, false, false),
            Color::Muted
        );
    }

    #[test]
    fn test_breadcrumb_entry_icon_color_follows_diagnostic_severity() {
        let mut entry = test_entry(git::status::GitSummary::UNCHANGED);
        assert_eq!(breadcrumb_entry_icon_color(&entry), Color::Muted);

        entry.diagnostic_severity = Some(language::DiagnosticSeverity::WARNING);
        assert_eq!(breadcrumb_entry_icon_color(&entry), Color::Warning);

        entry.diagnostic_severity = Some(language::DiagnosticSeverity::ERROR);
        assert_eq!(breadcrumb_entry_icon_color(&entry), Color::Error);
    }

    #[test]
    fn test_breadcrumb_entry_icon_source() {
        assert_eq!(
            breadcrumb_entry_icon_source(true, true, true),
            BreadcrumbEntryIconSource::Folder
        );
        assert_eq!(
            breadcrumb_entry_icon_source(true, false, true),
            BreadcrumbEntryIconSource::Folder
        );
        assert_eq!(
            breadcrumb_entry_icon_source(true, true, false),
            BreadcrumbEntryIconSource::Chevron
        );
        assert_eq!(
            breadcrumb_entry_icon_source(true, false, false),
            BreadcrumbEntryIconSource::Chevron
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, true, true),
            BreadcrumbEntryIconSource::File
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, true, false),
            BreadcrumbEntryIconSource::File
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, false, true),
            BreadcrumbEntryIconSource::None
        );
        assert_eq!(
            breadcrumb_entry_icon_source(false, false, false),
            BreadcrumbEntryIconSource::None
        );
    }

    #[test]
    fn test_descend_single_child_directories_stops_at_fork() {
        use util::rel_path::rel_path;

        let tree: collections::HashMap<&str, Vec<(&str, bool)>> =
            collections::HashMap::from_iter([
                ("a", vec![("a/b", true)]),
                ("a/b", vec![("a/b/c", true), ("a/b/d", true)]),
            ]);

        let result = descend_single_child_directories(rel_path("a").into_arc(), |path| {
            tree.get(path.as_unix_str())
                .into_iter()
                .flatten()
                .map(|(child, is_dir)| (rel_path(child).into_arc(), *is_dir))
                .collect()
        });

        assert_eq!(result, rel_path("a/b").into_arc());
    }

    #[test]
    fn test_descend_single_child_directories_stops_short_of_lone_file() {
        use util::rel_path::rel_path;

        let tree: collections::HashMap<&str, Vec<(&str, bool)>> = collections::HashMap::from_iter(
            [("repository", vec![("repository/Repositories.kt", false)])],
        );

        let result = descend_single_child_directories(rel_path("repository").into_arc(), |path| {
            tree.get(path.as_unix_str())
                .into_iter()
                .flatten()
                .map(|(child, is_dir)| (rel_path(child).into_arc(), *is_dir))
                .collect()
        });

        // Stops at `repository` rather than descending into the file it alone contains — the
        // user still clicks `Repositories.kt` themselves.
        assert_eq!(result, rel_path("repository").into_arc());
    }

    #[test]
    fn test_descend_single_child_directories_caps_depth() {
        use util::rel_path::rel_path;

        // Each directory has exactly one child, forever — simulates a symlink cycle or a
        // pathologically deep chain. The cap must stop the walk rather than looping forever.
        let result = descend_single_child_directories(rel_path("a").into_arc(), |path| {
            vec![(
                rel_path(&format!("{}/x", path.as_unix_str())).into_arc(),
                true,
            )]
        });

        // The walk must terminate rather than loop forever; the cap bounds how many `/x` segments
        // it can add on top of the starting `a`.
        assert_eq!(
            result.as_unix_str().matches('/').count(),
            MAX_BREADCRUMB_DESCENT_DEPTH
        );
    }

    #[test]
    fn test_descend_single_child_directories_stops_at_empty_directory() {
        use util::rel_path::rel_path;

        let result = descend_single_child_directories(rel_path("empty").into_arc(), |_| Vec::new());

        assert_eq!(result, rel_path("empty").into_arc());
    }

    /// Without the fix, choosing a directory row panics: `choose` runs while the browser entity
    /// is leased by `cx.listener`, and re-anchoring the popover synchronously updates that same
    /// leased entity again ("cannot update ... while it is already being updated").
    #[gpui::test]
    async fn test_choosing_breadcrumb_directory_row_does_not_double_lease_browser(
        cx: &mut TestAppContext,
    ) {
        use editor::MultiBuffer;
        use editor::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;

        // `PopoverMenu` only wires itself up during a real layout pass, so this needs an honest
        // `Render` mounted as a window root rather than a bare drawn element. The `Editor` is
        // created inside this same window too, since the re-anchor's deferred work is associated
        // with the window it starts on and nothing else here drains another one.
        struct Harness {
            handle: PopoverMenuHandle<BreadcrumbDirectoryPicker>,
            editor: Entity<Editor>,
            workspace: WeakEntity<Workspace>,
            worktree_id: WorktreeId,
            captured_browser: Rc<RefCell<Option<Entity<BreadcrumbDirectoryPicker>>>>,
        }

        impl Render for Harness {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                let editor = self.editor.downgrade();
                let workspace = self.workspace.clone();
                let worktree_id = self.worktree_id;
                let captured_browser = self.captured_browser.clone();
                PopoverMenu::new("test-breadcrumb-directory-menu")
                    .with_handle(self.handle.clone())
                    .trigger(ButtonLike::new("trigger").child(div()))
                    .menu(move |window, cx| {
                        // Marking the segment active must happen in the same opening sequence
                        // that creates the browser below, not as a separate step — a separate
                        // call would dismiss the already-open browser via `handle` first.
                        if let Some(editor_entity) = editor.upgrade() {
                            editor_entity.update(cx, |editor, cx| {
                                editor.open_breadcrumb_navigation(
                                    worktree_id,
                                    RelPath::empty().into(),
                                    cx,
                                );
                            });
                        }
                        let browser = BreadcrumbDirectoryDelegate::picker(
                            editor.clone(),
                            workspace.clone(),
                            worktree_id,
                            RelPath::empty().into(),
                            None,
                            window,
                            cx,
                        );
                        *captured_browser.borrow_mut() = Some(browser.clone());
                        Some(browser)
                    })
            }
        }

        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "dir_a": {
                    "child1.txt": "",
                    "child2.txt": "",
                },
                "file.txt": "",
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let captured_browser: Rc<RefCell<Option<Entity<BreadcrumbDirectoryPicker>>>> =
            Rc::default();

        let harness_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            // The real handle `Editor::navigate_breadcrumb_to` re-anchors, exactly like
            // `render_breadcrumb_directory_segment` uses for the active segment — not a fresh
            // handle of the test's own, which `navigate_breadcrumb_to` would have no way to reach.
            let handle = editor
                .read(cx)
                .breadcrumb_popover_handle()
                .expect("breadcrumb_picker::init registered the renderers")
                .as_any()
                .downcast_ref::<DirectoryPopoverHandle>()
                .expect("the registered handle constructor is this crate's own")
                .0
                .clone();
            Harness {
                handle,
                editor,
                workspace: workspace.downgrade(),
                worktree_id,
                captured_browser: captured_browser.clone(),
            }
        });
        let editor = harness_window
            .read_with(cx, |harness, _| harness.editor.clone())
            .unwrap();
        let handle = harness_window
            .read_with(cx, |harness, _| harness.handle.clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*harness_window, cx);

        // Wires `handle` up to the popover's state, like a real breadcrumb bar render pass does.
        cx.update(|window, cx| {
            let _ = window.draw(cx);
        });

        // Opening through `handle.show` makes `browser` the same entity `handle` holds internally
        // — required for the repro, since the panic is about reaching back into a leased entity.
        cx.update(|window, cx| handle.show(window, cx));
        let browser = captured_browser.borrow().clone().expect("popover opened");
        assert!(handle.is_deployed());
        editor.read_with(cx, |editor, _| {
            assert!(
                editor.breadcrumb_navigation().is_some(),
                "opening the popover marked this segment active"
            );
        });

        // Choosing while `browser` is still leased by this `update` call is the actual repro.
        confirm_breadcrumb_row(&browser, "dir_a", cx);

        editor.read_with(cx, |editor, _| {
            let navigation = editor
                .breadcrumb_navigation()
                .expect("navigate_breadcrumb_to set a session");
            assert_eq!(navigation.active_path.as_unix_str(), "dir_a");
            assert!(navigation.navigated);
            assert!(
                editor.breadcrumb_reanchoring(),
                "re-anchor is still in flight — the popover isn't back open yet"
            );
        });
        assert!(
            !handle.is_deployed(),
            "the pre-navigation popover was dismissed synchronously by the defer"
        );

        // Re-anchoring hides then re-shows the popover once the new active segment lays out; by
        // then `browser` is no longer leased.
        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.reanchor_breadcrumb_popover(window, cx);
            });
        });

        editor.read_with(cx, |editor, _| {
            assert!(
                !editor.breadcrumb_reanchoring(),
                "re-anchor finishes within a few frames"
            );
        });
        assert!(
            handle.is_deployed(),
            "the popover reopened under the resolved directory's own segment"
        );
    }

    /// The whole flow driven by `menu::` actions rather than by simulated keystrokes: move the
    /// selection, submit it, and end up inside the chosen directory.
    #[gpui::test]
    async fn test_breadcrumb_directory_picker_navigates_from_the_keyboard(cx: &mut TestAppContext) {
        use editor::MultiBuffer;
        use editor::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;

        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "alpha": { "one.txt": "", "two.txt": "" },
                "beta": { "three.txt": "" },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        struct Harness {
            picker: Entity<BreadcrumbDirectoryPicker>,
            editor: Entity<Editor>,
        }
        impl Render for Harness {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.picker.clone()
            }
        }

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let harness_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(buffer, window, cx));
            let picker = BreadcrumbDirectoryDelegate::picker(
                editor.downgrade(),
                workspace.downgrade(),
                worktree_id,
                RelPath::empty().into(),
                None,
                window,
                cx,
            );
            Harness { picker, editor }
        });
        let (picker, editor) = harness_window
            .read_with(cx, |harness, _| {
                (harness.picker.clone(), harness.editor.clone())
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*harness_window, cx);
        cx.run_until_parked();

        picker.update_in(cx, |picker, window, cx| {
            window.focus(&picker.focus_handle(cx), cx);
        });
        cx.run_until_parked();

        // The listing opens on `alpha`; one step down lands on `beta`.
        cx.dispatch_action(menu::SelectNext);
        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .entry_at(picker.delegate.selected_index)
                    .map(|entry| entry.name.as_ref()),
                Some("beta"),
            );
        });

        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();

        editor.read_with(cx, |editor, _| {
            let navigation = editor
                .breadcrumb_navigation()
                .expect("confirming a directory row navigates the bar into it");
            assert_eq!(navigation.active_path.as_unix_str(), "beta");
            assert!(navigation.navigated);
        });
    }

    /// Double clicking a segment has to reach the project panel, which listens for this event
    /// rather than being called directly.
    #[gpui::test]
    async fn test_revealing_a_breadcrumb_directory_emits_for_the_project_panel(
        cx: &mut TestAppContext,
    ) {
        use project::{FakeFs, Project};
        use serde_json::json;
        use std::sync::Arc as StdArc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use util::{path, rel_path::rel_path};

        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/root"), json!({ "alpha": { "one.txt": "" } }))
            .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let revealed = StdArc::new(AtomicUsize::new(0));
        let activated = StdArc::new(AtomicUsize::new(0));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&project, {
                let revealed = revealed.clone();
                let activated = activated.clone();
                move |_, event, _| match event {
                    project::Event::RevealInProjectPanel(_) => {
                        revealed.fetch_add(1, Ordering::AcqRel);
                    }
                    project::Event::ActivateProjectPanel => {
                        activated.fetch_add(1, Ordering::AcqRel);
                    }
                    _ => {}
                }
            })
        });

        cx.update(|cx| {
            reveal_breadcrumb_directory_in_project_panel(
                &workspace.downgrade(),
                worktree_id,
                rel_path("alpha"),
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(revealed.load(Ordering::Acquire), 1);
        assert_eq!(
            activated.load(Ordering::Acquire),
            1,
            "a closed panel has to open, not just have its selection moved"
        );

        cx.update(|cx| {
            reveal_breadcrumb_directory_in_project_panel(
                &workspace.downgrade(),
                worktree_id,
                rel_path("nope"),
                cx,
            );
        });
        cx.run_until_parked();
        assert_eq!(
            revealed.load(Ordering::Acquire),
            1,
            "a path with no entry reveals nothing rather than panicking"
        );
        assert_eq!(activated.load(Ordering::Acquire), 1);
    }

    /// Worktrees never scan gitignored directories proactively, so without the expansion call a
    /// dropdown that only reads the snapshot lists nothing. One level per dropdown opened.
    #[gpui::test]
    async fn test_breadcrumb_directory_browser_expands_nested_gitignored_directories(
        cx: &mut TestAppContext,
    ) {
        use editor::MultiBuffer;
        use editor::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::{path, rel_path::rel_path};

        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                ".gitignore": "ignored_dir\n",
                "ignored_dir": { "nested": { "file.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree = project.update(cx, |project, cx| project.worktrees(cx).next().unwrap());
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
        cx.run_until_parked();

        let entries = cx.update(|cx| {
            breadcrumb_directory_entries(&project, &worktree, rel_path("ignored_dir"), cx)
        });
        assert!(
            entries.is_empty(),
            "nothing under a gitignored directory is scanned until something asks for it"
        );

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let open_dropdown_at = |path: &'static str, cx: &mut VisualTestContext| {
            let browser = editor_window
                .update(cx, |_, window, cx| {
                    BreadcrumbDirectoryDelegate::picker(
                        editor.downgrade(),
                        workspace.downgrade(),
                        worktree_id,
                        rel_path(path).into_arc(),
                        None,
                        window,
                        cx,
                    )
                })
                .unwrap();
            cx.run_until_parked();
            let entries = cx.update(|_, cx| {
                breadcrumb_directory_entries(&project, &worktree, rel_path(path), cx)
            });
            drop(browser);
            entries
                .into_iter()
                .map(|entry| entry.name.to_string())
                .collect::<Vec<_>>()
        };

        assert_eq!(
            open_dropdown_at("ignored_dir", cx),
            vec!["nested".to_string()],
            "opening the dropdown scans one level into the gitignored directory"
        );
        assert_eq!(
            open_dropdown_at("ignored_dir/nested", cx),
            vec!["file.txt".to_string()],
            "and the level below it once that one is opened too"
        );
    }

    #[gpui::test]
    async fn test_breadcrumb_directory_browser_choose_descends_single_child_directories(
        cx: &mut TestAppContext,
    ) {
        use editor::MultiBuffer;
        use editor::test::build_editor;
        use project::{FakeFs, Project};
        use serde_json::json;
        use util::path;

        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a": { "b": { "c.txt": "" } },
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = workspace_window.root(cx).unwrap();

        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window = cx.add_window(|window, cx| build_editor(buffer, window, cx));
        let editor = editor_window.root(cx).unwrap();
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        // `a`'s only child is `b`, whose only child is the file `c.txt` — a single-child chain —
        // so choosing `a` descends straight to `a/b`, stopping short of the file.
        let browser = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbDirectoryDelegate::picker(
                    editor.downgrade(),
                    workspace.downgrade(),
                    worktree_id,
                    RelPath::empty().into(),
                    None,
                    window,
                    cx,
                )
            })
            .unwrap();
        confirm_breadcrumb_row(&browser, "a", cx);
        editor.read_with(cx, |editor, _| {
            assert_eq!(
                editor
                    .breadcrumb_navigation()
                    .expect("navigate_breadcrumb_to set a session")
                    .active_path
                    .as_unix_str(),
                "a/b",
            );
        });
    }
}
