use file_icons::FileIcons;
use git::{
    repository::RepoPath,
    status::{DiffStat, FileStatus},
};
use gpui::{App, ClickEvent, Div, ElementId, Hsla, Rems, Window};
use project::project_settings::{GitPathStyle, ProjectSettings};
use settings::{Settings as _, StatusStyle};
use ui::{Tooltip, prelude::*};
use util::paths::PathStyle;

use crate::{git_status_icon, worktree_panel_settings::WorktreeRowStyle};

/// The height of a changed-file row.
///
/// Shared so that a panel listing changed files lines up with the git panel
/// rather than drifting from it.
pub(crate) fn git_list_item_height() -> Rems {
    rems(1.75)
}

pub(crate) fn entry_label(label: impl Into<SharedString>, color: Color) -> Label {
    Label::new(label.into()).single_line().color(color)
}

/// Lays out a changed file's name and its directory.
///
/// The file name is drawn at its full width and only the directory truncates,
/// so a narrow panel eats the least identifying part of the path first.
pub(crate) fn path_formatted(
    directory: Option<String>,
    path_color: Color,
    file_name: String,
    label_color: Color,
    path_style: PathStyle,
    git_path_style: GitPathStyle,
    strikethrough: bool,
) -> Div {
    let file_name_first = git_path_style == GitPathStyle::FileNameFirst;
    let file_path_first = git_path_style == GitPathStyle::FilePathFirst;

    let file_name = format!("{} ", file_name);

    h_flex()
        .min_w_0()
        .overflow_hidden()
        .when(file_path_first, |this| this.flex_row_reverse())
        .child(
            div().flex_none().child(
                entry_label(file_name, label_color).when(strikethrough, Label::strikethrough),
            ),
        )
        .when_some(
            directory.filter(|directory| !directory.is_empty()),
            |this, directory| {
                let path_name = if file_name_first {
                    directory
                } else {
                    format!("{directory}{}", path_style.primary_separator())
                };

                this.child(
                    entry_label(path_name, path_color)
                        .truncate_start()
                        .when(strikethrough, Label::strikethrough),
                )
            },
        )
}

const SELECTED_BG_ALPHA: f32 = 0.08;
const STATE_OPACITY_STEP: f32 = 0.04;

/// The base, hover and active backgrounds for a row in a list of changed files,
/// matching the git panel's selection treatment.
pub(crate) fn row_backgrounds(selected: bool, cx: &App) -> (Hsla, Hsla, Hsla) {
    let info_color = cx.theme().status().info;
    if selected {
        (
            info_color.alpha(SELECTED_BG_ALPHA),
            info_color.alpha(SELECTED_BG_ALPHA + STATE_OPACITY_STEP),
            info_color.alpha(SELECTED_BG_ALPHA + STATE_OPACITY_STEP * 2.0),
        )
    } else {
        (
            cx.theme().colors().ghost_element_background,
            cx.theme().colors().ghost_element_hover,
            cx.theme().colors().ghost_element_active,
        )
    }
}

/// Which way a row's staging control moves the file.
///
/// A row's direction comes from the section it is listed under rather than
/// from the file's own state, so a partially staged file — which is listed
/// under both — moves the way its section says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StageAction {
    Stage,
    Unstage,
}

/// Groups a row so its staging control can appear only while the row is
/// hovered.
///
/// The same name on every row is what makes this work: group lookup resolves
/// to the innermost enclosing group, which is the button's own row.
const ROW_GROUP: &str = "git-status-list-item";

/// Lets a test find the staging control, which has no stable position of its
/// own. A no-op outside of tests.
const STAGE_BUTTON_SELECTOR: &str = "git-status-list-item-stage-button";

/// A single changed file, rendered as a list row.
///
/// The git panel renders equivalent rows inline; this is a standalone
/// component so new panels can show a changed file without depending on the
/// git panel's entry state.
#[derive(IntoElement)]
pub(crate) struct GitStatusListItem {
    id: ElementId,
    repo_path: RepoPath,
    status: FileStatus,
    path_style: PathStyle,
    style: Option<WorktreeRowStyle>,
    diff_stat: Option<DiffStat>,
    stage_action: Option<StageAction>,
    selected: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_stage: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl GitStatusListItem {
    pub fn new(
        id: impl Into<ElementId>,
        repo_path: RepoPath,
        status: FileStatus,
        path_style: PathStyle,
    ) -> Self {
        Self {
            id: id.into(),
            repo_path,
            status,
            path_style,
            style: None,
            diff_stat: None,
            stage_action: None,
            selected: false,
            on_click: None,
            on_stage: None,
        }
    }

    pub fn style(mut self, style: WorktreeRowStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn diff_stat(mut self, diff_stat: Option<DiffStat>) -> Self {
        self.diff_stat = diff_stat;
        self
    }

    /// Shows a staging control, revealed while the row is hovered.
    pub fn stage_action(mut self, stage_action: StageAction) -> Self {
        self.stage_action = Some(stage_action);
        self
    }

    pub fn on_stage(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_stage = Some(Box::new(handler));
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

/// Splits a repo path into the parent directory and the file name, so a row
/// can dim the directory and keep the file name legible when the panel is
/// narrow.
///
/// The directory carries no trailing separator; `path_formatted` adds one when
/// the configured path style calls for it.
///
/// Returns `None` for the directory when the file sits at the worktree root.
pub(crate) fn split_display_path(
    repo_path: &RepoPath,
    path_style: PathStyle,
) -> (Option<String>, String) {
    let file_name = repo_path
        .file_name()
        .map(|name| name.to_owned())
        .unwrap_or_else(|| repo_path.display(path_style).into_owned());
    let parent_dir = repo_path
        .parent()
        .filter(|parent| !parent.is_empty())
        .map(|parent| parent.display(path_style).into_owned());
    (parent_dir, file_name)
}

/// The label colour for a changed file, matching the git panel's
/// `label_color` scheme.
pub(crate) fn status_label_color(status: FileStatus) -> Color {
    if status.is_conflicted() {
        Color::VersionControlConflict
    } else if status.is_deleted() {
        // Deleted files are dimmed rather than red, to avoid a wall of red
        // labels in a list.
        Color::Disabled
    } else if status.is_modified() {
        Color::VersionControlModified
    } else {
        Color::VersionControlAdded
    }
}

impl RenderOnce for GitStatusListItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = self.style.unwrap_or_else(|| WorktreeRowStyle::get(cx));
        let git_path_style = ProjectSettings::get_global(cx).git.path_style;
        let (parent_dir, file_name) = split_display_path(&self.repo_path, self.path_style);
        let is_deleted = self.status.is_deleted();
        let path_color = if is_deleted {
            Color::Disabled
        } else {
            Color::Muted
        };
        // Mirrors the git panel: the status is carried either by a leading icon
        // or by the colour of the file name, never both.
        let uses_label_color = style.status_style == StatusStyle::LabelColor;
        let label_color = if uses_label_color {
            status_label_color(self.status)
        } else {
            Color::Default
        };
        let file_icon = style
            .file_icons
            .then(|| FileIcons::get_icon(self.repo_path.as_std_path(), cx))
            .flatten();

        let (base_bg, hover_bg, active_bg) = row_backgrounds(self.selected, cx);

        let stage_button = self.stage_action.map(|stage_action| {
            let (icon, tooltip) = match stage_action {
                StageAction::Stage => (IconName::Plus, "Stage File"),
                StageAction::Unstage => (IconName::Dash, "Unstage File"),
            };
            IconButton::new("stage-file", icon)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                // Hidden rather than absent, so the file name has the same
                // width whether or not the row is hovered.
                .visible_on_hover(ROW_GROUP)
                .tooltip(move |_window, cx| Tooltip::simple(tooltip, cx))
                // `IconButton` stops propagation itself, so this click does not
                // also reach the row and open the diff.
                .when_some(self.on_stage, |this, handler| this.on_click(handler))
        });

        let name_row = h_flex()
            .min_w_0()
            .flex_1()
            .gap_1()
            .when_some(file_icon, |this, icon| {
                this.child(Icon::from_path(icon).size(IconSize::Small).color(Color::Muted))
            })
            .when(!uses_label_color, |this| {
                this.child(git_status_icon(self.status))
            })
            .child(path_formatted(
                parent_dir,
                path_color,
                file_name,
                label_color,
                self.path_style,
                git_path_style,
                is_deleted,
            ));

        h_flex()
            .id(self.id)
            .group(ROW_GROUP)
            .h(git_list_item_height())
            // The panel lists rows in a flex column, which would otherwise
            // shrink them below their height once the list overflows.
            .flex_none()
            .w_full()
            .pl_2p5()
            .pr_1()
            .gap_1p5()
            .border_1()
            .border_r_2()
            .bg(base_bg)
            .hover(|this| this.bg(hover_bg))
            .active(|this| this.bg(active_bg))
            .child(name_row)
            // The staging control sits inboard of the line counts, so the
            // counts stay flush with the right edge instead of leaving a gap
            // there for a button that is usually hidden.
            .when_some(stage_button, |this, stage_button| {
                // Deliberately not `occlude`: an occluding hitbox ends the hit
                // test above the row, so the row would stop counting as
                // hovered the moment the pointer reached the button — hiding
                // the button out from under the click. The button's own
                // handler stops propagation instead.
                this.child(
                    div()
                        .flex_none()
                        .debug_selector(|| STAGE_BUTTON_SELECTOR.to_owned())
                        .child(stage_button),
                )
            })
            .when_some(
                self.diff_stat.filter(|_| style.diff_stats),
                |this, diff_stat| {
                    this.child(ui::DiffStat::new(
                        "diff-stat",
                        diff_stat.added as usize,
                        diff_stat.deleted as usize,
                    ))
                },
            )
            .when_some(self.on_click, |this, handler| this.on_click(handler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git::status::StatusCode;
    use gpui::{Modifiers, TestAppContext, px};
    use std::{cell::Cell, rc::Rc};
    use util::rel_path::rel_path;

    fn repo_path(path: &str) -> RepoPath {
        RepoPath::from_rel_path(rel_path(path))
    }

    #[test]
    fn test_split_display_path_at_the_worktree_root() {
        assert_eq!(
            split_display_path(&repo_path("main.rs"), PathStyle::Unix),
            (None, "main.rs".to_owned())
        );
    }

    #[test]
    fn test_split_display_path_in_a_directory() {
        assert_eq!(
            split_display_path(&repo_path("src/main.rs"), PathStyle::Unix),
            (Some("src".to_owned()), "main.rs".to_owned())
        );
        assert_eq!(
            split_display_path(&repo_path("crates/git_ui/src/main.rs"), PathStyle::Unix),
            (Some("crates/git_ui/src".to_owned()), "main.rs".to_owned())
        );
    }

    #[test]
    fn test_split_display_path_uses_the_platform_separator() {
        assert_eq!(
            split_display_path(&repo_path("src/main.rs"), PathStyle::Windows),
            (Some("src".to_owned()), "main.rs".to_owned())
        );
        assert_eq!(
            split_display_path(&repo_path("a/b/main.rs"), PathStyle::Windows),
            (Some("a\\b".to_owned()), "main.rs".to_owned())
        );
    }

    /// The staging control is only painted while its row is hovered, so
    /// anything that stops the row counting as hovered makes the control
    /// vanish as the pointer arrives on it — leaving no way to stage a file.
    struct StagingRow {
        staged: Rc<Cell<bool>>,
        opened: Rc<Cell<bool>>,
    }

    impl Render for StagingRow {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let staged = self.staged.clone();
            let opened = self.opened.clone();
            div().w(px(300.)).child(
                GitStatusListItem::new(
                    "row",
                    repo_path("main.rs"),
                    FileStatus::Untracked,
                    PathStyle::Unix,
                )
                .stage_action(StageAction::Stage)
                .on_stage(move |_, _, _| staged.set(true))
                .on_click(move |_, _, _| opened.set(true)),
            )
        }
    }

    #[gpui::test]
    async fn test_the_staging_control_survives_the_pointer_reaching_it(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });

        let staged = Rc::new(Cell::new(false));
        let opened = Rc::new(Cell::new(false));
        let (_row, cx) = cx.add_window_view({
            let staged = staged.clone();
            let opened = opened.clone();
            |_window, _cx| StagingRow { staged, opened }
        });
        cx.run_until_parked();

        let button = cx
            .debug_bounds(STAGE_BUTTON_SELECTOR)
            .expect("the row should lay out a staging control");

        // Arriving on the control is the moment that used to hide it.
        cx.simulate_mouse_move(button.center(), None, Modifiers::none());
        cx.run_until_parked();
        cx.simulate_click(button.center(), Modifiers::none());

        assert!(staged.get(), "clicking the staging control should stage");
        assert!(
            !opened.get(),
            "the click should not also fall through to the row"
        );
    }

    #[test]
    fn test_status_label_colors() {
        assert_eq!(
            status_label_color(FileStatus::worktree(StatusCode::Modified)),
            Color::VersionControlModified
        );
        assert_eq!(
            status_label_color(FileStatus::worktree(StatusCode::Deleted)),
            Color::Disabled
        );
        assert_eq!(
            status_label_color(FileStatus::Untracked),
            Color::VersionControlAdded
        );
    }
}
