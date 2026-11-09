use std::{fmt, path::Path, sync::Arc};

use gpui::{Entity, EventEmitter, FocusHandle, Focusable};
use project::{ProjectPath, Worktree};
use ui::{
    App, Button, ButtonCommon, ButtonStyle, Clickable, Context, FluentBuilder, InteractiveElement,
    KeyBinding, Label, LabelCommon, LabelSize, ParentElement, Render, SharedString, Styled as _,
    Window, h_flex, v_flex,
};
use zed_actions::workspace::OpenWithSystem;

use crate::{Item, ItemNavHistory, Workspace};

/// A view to display when a certain buffer/image/other item fails to open.
pub struct InvalidItemView {
    /// Which path was attempted to open.
    pub abs_path: Arc<Path>,
    /// An error message, happened when opening the item.
    pub error: SharedString,
    is_local: bool,
    focus_handle: FocusHandle,
    project_identity: Option<(Entity<Worktree>, ProjectPath)>,
    nav_history: Option<ItemNavHistory>,
}

impl InvalidItemView {
    pub fn new(
        abs_path: &Path,
        is_local: bool,
        e: &anyhow::Error,
        _: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            is_local,
            abs_path: Arc::from(abs_path),
            error: format!("{}", e.root_cause()).into(),
            focus_handle: cx.focus_handle(),
            project_identity: None,
            nav_history: None,
        }
    }

    pub(crate) fn with_project_path(
        mut self,
        worktree: Entity<Worktree>,
        project_path: ProjectPath,
    ) -> Self {
        self.project_identity = Some((worktree, project_path));
        self
    }

    pub(crate) fn has_deleted_file(&self, cx: &App) -> bool {
        self.project_identity
            .as_ref()
            .is_some_and(|(worktree, project_path)| {
                Workspace::history_path_is_removed(worktree.read(cx), &project_path.path, cx)
            })
    }
}

impl fmt::Debug for InvalidItemView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InvalidItemView")
            .field("abs_path", &self.abs_path)
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl Item for InvalidItemView {
    type Event = ();

    fn tab_content_text(&self, mut detail: usize, _: &App) -> SharedString {
        // Ensure we always render at least the filename.
        detail += 1;

        let path = self.abs_path.as_ref();

        let mut prefix = path;
        while detail > 0 {
            if let Some(parent) = prefix.parent() {
                prefix = parent;
                detail -= 1;
            } else {
                break;
            }
        }

        let path = if detail > 0 {
            path
        } else {
            path.strip_prefix(prefix).unwrap_or(path)
        };

        SharedString::new(path.to_string_lossy())
    }

    fn active_project_path(&self, _: &App) -> Option<ProjectPath> {
        self.project_identity
            .as_ref()
            .map(|(_, project_path)| project_path.clone())
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut Window, _: &mut Context<Self>) {
        self.nav_history = Some(history);
    }

    fn deactivated(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((_, project_path)) = &self.project_identity
            && let Some(history) = &mut self.nav_history
        {
            history.push_path(project_path.clone(), self.abs_path.to_path_buf(), cx);
        }
    }
}

impl EventEmitter<()> for InvalidItemView {}

impl Focusable for InvalidItemView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InvalidItemView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let abs_path = self.abs_path.clone();
        v_flex()
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .flex_none()
            .justify_center()
            .overflow_hidden()
            .key_context("InvalidItem")
            .child(
                h_flex().size_full().justify_center().child(
                    v_flex()
                        .justify_center()
                        .gap_2()
                        .child(h_flex().justify_center().child("Could not open file"))
                        .child(
                            h_flex()
                                .justify_center()
                                .child(Label::new(self.error.clone()).size(LabelSize::Small)),
                        )
                        .when(self.is_local, |contents| {
                            contents.child(
                                h_flex().justify_center().child(
                                    Button::new("open-with-system", "Open in Default App")
                                        .on_click(move |_, _, cx| {
                                            cx.open_with_system(&abs_path);
                                        })
                                        .style(ButtonStyle::Outlined)
                                        .key_binding(KeyBinding::for_action(&OpenWithSystem, cx)),
                                ),
                            )
                        }),
                ),
            )
    }
}
