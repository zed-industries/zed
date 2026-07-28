//! The prompt shown when opening a folder while a project is already open, offering to
//! attach it to the current window, open it in a new window, or replace the active project.
//!
//! Shown when `default_open_behavior` is set to `ask`; the other values pick one of these
//! outcomes without prompting.

use std::path::PathBuf;

use gpui::{DismissEvent, EventEmitter, FocusHandle, Focusable, WeakEntity, WindowHandle};
use settings::{DefaultOpenBehavior, Settings as _};
use ui::{AlertModal, Checkbox, KeyBinding, ListItem, ListItemSpacing, ToggleState, prelude::*};
use util::ResultExt as _;

use crate::{CloseIntent, ModalView, MultiWorkspace, OpenMode, Workspace, WorkspaceSettings};

/// Shows the three-way prompt for `paths` when the user has asked to be prompted and
/// this window actually has a project to attach to or replace. Returns whether the
/// prompt was shown; when it wasn't, the caller should apply its own default handling.
pub fn prompt_if_asking(
    workspace: &mut Workspace,
    paths: Vec<PathBuf>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> bool {
    if WorkspaceSettings::get_global(cx).default_open_behavior != DefaultOpenBehavior::Ask {
        return false;
    }
    // With no project open there is nothing to attach to or replace, so all three
    // outcomes are the same and asking would only add a keystroke.
    if workspace
        .project()
        .read(cx)
        .visible_worktrees(cx)
        .next()
        .is_none()
    {
        return false;
    }

    let multi_workspace = window.window_handle().downcast::<MultiWorkspace>();
    let can_attach = multi_workspace
        .and_then(|handle| {
            handle
                .read(cx)
                .ok()
                .map(|multi_workspace| multi_workspace.multi_workspace_enabled(cx))
        })
        .unwrap_or(false);
    let workspace_handle = cx.weak_entity();
    workspace.toggle_modal(window, cx, |_, cx| {
        OpenProjectPrompt::new(paths, can_attach, workspace_handle, multi_workspace, cx)
    });
    true
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpenProjectChoice {
    /// Keep the open projects and add this folder alongside them.
    Attach,
    /// Open the folder in a separate window.
    NewWindow,
    /// Close the active project and open the folder in its place.
    Replace,
}

impl OpenProjectChoice {
    fn label(self) -> &'static str {
        match self {
            Self::Attach => "Attach to Current Window",
            Self::NewWindow => "Open in New Window",
            Self::Replace => "Replace Active Project",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Attach => "Add it alongside the projects already open here",
            Self::NewWindow => "Leave this window as it is",
            Self::Replace => "Close the active project first",
        }
    }

    /// The `default_open_behavior` value equivalent to this choice, if any.
    /// `Replace` is inherently a one-off and so cannot be remembered.
    fn as_default_behavior(self) -> Option<DefaultOpenBehavior> {
        match self {
            Self::Attach => Some(DefaultOpenBehavior::ExistingWindow),
            Self::NewWindow => Some(DefaultOpenBehavior::NewWindow),
            Self::Replace => None,
        }
    }
}

pub struct OpenProjectPrompt {
    paths: Vec<PathBuf>,
    choices: Vec<OpenProjectChoice>,
    selected_index: usize,
    remember: bool,
    workspace: WeakEntity<Workspace>,
    multi_workspace: Option<WindowHandle<MultiWorkspace>>,
    focus_handle: FocusHandle,
}

impl OpenProjectPrompt {
    /// `can_attach` is false when this window cannot host more than one project, in
    /// which case attaching is not offered at all rather than offered and ignored.
    pub fn new(
        paths: Vec<PathBuf>,
        can_attach: bool,
        workspace: WeakEntity<Workspace>,
        multi_workspace: Option<WindowHandle<MultiWorkspace>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut choices = Vec::with_capacity(3);
        if can_attach {
            choices.push(OpenProjectChoice::Attach);
        }
        choices.push(OpenProjectChoice::NewWindow);
        choices.push(OpenProjectChoice::Replace);

        Self {
            paths,
            choices,
            selected_index: 0,
            remember: false,
            workspace,
            multi_workspace,
            focus_handle: cx.focus_handle(),
        }
    }

    fn selected_choice(&self) -> Option<OpenProjectChoice> {
        self.choices.get(self.selected_index).copied()
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        if !self.choices.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.choices.len();
            cx.notify();
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.choices.is_empty() {
            self.selected_index = self
                .selected_index
                .checked_sub(1)
                .unwrap_or(self.choices.len() - 1);
            cx.notify();
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.open(window, cx);
    }

    fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(choice) = self.selected_choice() else {
            return;
        };

        if self.remember {
            if let Some(behavior) = choice.as_default_behavior() {
                let fs = <dyn fs::Fs>::global(cx);
                settings::update_settings_file(fs, cx, move |settings, _cx| {
                    settings.workspace.default_open_behavior = Some(behavior);
                });
            }
        }

        let paths = std::mem::take(&mut self.paths);
        let workspace = self.workspace.clone();
        let multi_workspace = self.multi_workspace;
        cx.emit(DismissEvent);

        cx.spawn_in(window, async move |_, cx| match choice {
            OpenProjectChoice::Attach => {
                let Some(multi_workspace) = multi_workspace else {
                    return;
                };
                if let Some(task) = multi_workspace
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace.open_project(paths, OpenMode::Activate, window, cx)
                    })
                    .log_err()
                {
                    task.await.log_err();
                }
            }
            OpenProjectChoice::NewWindow => {
                if let Some(task) = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_workspace_for_paths(OpenMode::NewWindow, paths, window, cx)
                    })
                    .log_err()
                {
                    task.await.log_err();
                }
            }
            OpenProjectChoice::Replace => {
                let Some(should_continue) = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.prepare_to_close(CloseIntent::ReplaceWindow, window, cx)
                    })
                    .log_err()
                else {
                    return;
                };
                if should_continue.await.log_err() != Some(true) {
                    return;
                }
                if let Some(task) = workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_workspace_for_paths(OpenMode::Activate, paths, window, cx)
                    })
                    .log_err()
                {
                    task.await.log_err();
                }
            }
        })
        .detach();
    }

    fn render_choice(&self, index: usize, cx: &Context<Self>) -> Option<impl IntoElement> {
        let choice = *self.choices.get(index)?;
        let selected = index == self.selected_index;

        Some(
            ListItem::new(("open-project-choice", index))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(
                    Icon::new(if selected {
                        IconName::Check
                    } else {
                        IconName::Circle
                    })
                    .size(IconSize::Small)
                    .color(if selected {
                        Color::Accent
                    } else {
                        Color::Muted
                    }),
                )
                .child(
                    v_flex().child(Label::new(choice.label())).child(
                        Label::new(choice.description())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.selected_index = index;
                    this.open(window, cx);
                })),
        )
    }
}

impl Focusable for OpenProjectPrompt {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for OpenProjectPrompt {}
impl ModalView for OpenProjectPrompt {}

impl Render for OpenProjectPrompt {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let folder_name = self
            .paths
            .first()
            .and_then(|path| path.file_name())
            .map(|name| name.to_string_lossy().to_string());
        let prompt: SharedString = match (&folder_name, self.paths.len()) {
            (Some(name), 1) => format!("How would you like to open “{name}”?").into(),
            _ => format!(
                "How would you like to open these {} folders?",
                self.paths.len()
            )
            .into(),
        };

        // `Replace` has no persisted equivalent, so remembering it is not offered.
        let can_remember = self
            .selected_choice()
            .and_then(OpenProjectChoice::as_default_behavior)
            .is_some();

        let mut choice_rows = Vec::with_capacity(self.choices.len());
        for index in 0..self.choices.len() {
            choice_rows.extend(self.render_choice(index, cx));
        }

        AlertModal::new("open-project-prompt")
            .title("Open Project")
            .width(rems(30.))
            .key_context("OpenProjectPrompt")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .child(Label::new(prompt).color(Color::Muted))
            .child(v_flex().pt_1().children(choice_rows))
            .footer(
                h_flex()
                    .p_3()
                    .gap_1()
                    .items_center()
                    .justify_between()
                    .child(
                        Checkbox::new("remember", ToggleState::from(self.remember))
                            .label("Remember my choice")
                            .disabled(!can_remember)
                            .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                                this.remember = state.selected();
                                cx.notify();
                                cx.stop_propagation();
                            })),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("cancel", "Cancel")
                                    .color(Color::Muted)
                                    .key_binding(
                                        KeyBinding::for_action(&menu::Cancel, cx)
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .on_click(cx.listener(|_, _, _, cx| {
                                        cx.emit(DismissEvent);
                                        cx.stop_propagation();
                                    })),
                            )
                            .child(
                                Button::new("open", "Open")
                                    .style(ButtonStyle::Filled)
                                    .layer(ui::ElevationIndex::ModalSurface)
                                    .key_binding(
                                        KeyBinding::for_action(&menu::Confirm, cx)
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open(window, cx);
                                        cx.stop_propagation();
                                    })),
                            ),
                    ),
            )
    }
}
