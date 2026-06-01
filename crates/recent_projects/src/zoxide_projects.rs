use std::sync::Arc;

use fuzzy_nucleo::{StringMatch, StringMatchCandidate};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task,
    TaskExt, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate, highlighted_match_with_paths::HighlightedMatch};
use ui::{Icon, IconName, ListItem, ListItemSpacing, prelude::*, tooltip_container};
use workspace::{CloseIntent, ModalView, OpenMode, Workspace};

use crate::match_strings_order_insensitive;

pub struct RecentProjectsZoxide {
    pub picker: Entity<Picker<RecentProjectsZoxideDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

impl ModalView for RecentProjectsZoxide {}

impl RecentProjectsZoxide {
    fn new(
        delegate: RecentProjectsZoxideDelegate,
        rem_width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));

        cx.spawn_in(window, async move |this, cx| {
            #[allow(clippy::disallowed_methods)]
            let output = std::process::Command::new("zoxide")
                .args(&["query", "--list"])
                .output();

            let directories = match output {
                Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            };

            this.update_in(cx, move |this, window, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.set_directories(directories);
                    picker.update_matches(picker.query(cx), window, cx)
                })
            })
            .ok()
        })
        .detach();

        Self {
            picker,
            rem_width,
            _subscription,
        }
    }

    pub fn open(
        workspace: &mut Workspace,
        create_new_window: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let weak = cx.entity().downgrade();
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = RecentProjectsZoxideDelegate::new(weak, create_new_window);
            Self::new(delegate, 34., window, cx)
        })
    }
}

impl EventEmitter<DismissEvent> for RecentProjectsZoxide {}

impl Focusable for RecentProjectsZoxide {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentProjectsZoxide {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RecentProjectsZoxide")
            .w(rems(self.rem_width))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), window, cx);
                })
            }))
    }
}

pub struct RecentProjectsZoxideDelegate {
    workspace: WeakEntity<Workspace>,
    directories: Vec<String>,
    selected_match_index: usize,
    matches: Vec<StringMatch>,
    create_new_window: bool,
    reset_selected_match_index: bool,
}

impl RecentProjectsZoxideDelegate {
    fn new(workspace: WeakEntity<Workspace>, create_new_window: bool) -> Self {
        Self {
            workspace,
            directories: Vec::new(),
            selected_match_index: 0,
            matches: Default::default(),
            create_new_window,
            reset_selected_match_index: true,
        }
    }

    pub fn set_directories(&mut self, directories: Vec<String>) {
        self.directories = directories;
    }

    fn format_path_for_display(&self, path: &str) -> String {
        if let Some(home_dir) = std::env::var("HOME").ok() {
            if path.starts_with(&home_dir) {
                return path.replacen(&home_dir, "~", 1);
            }
        }
        path.to_string()
    }
}

impl EventEmitter<DismissEvent> for RecentProjectsZoxideDelegate {}

impl PickerDelegate for RecentProjectsZoxideDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, window: &mut Window, _: &mut App) -> Arc<str> {
        let (create_window, reuse_window) = if self.create_new_window {
            (
                window.keystroke_text_for(&menu::SecondaryConfirm),
                window.keystroke_text_for(&menu::Confirm),
            )
        } else {
            (
                window.keystroke_text_for(&menu::Confirm),
                window.keystroke_text_for(&menu::SecondaryConfirm),
            )
        };
        Arc::from(format!(
            "{reuse_window} reuses this window, {create_window} opens a new one (zoxide)",
        ))
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim_start();
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let candidates = self
            .directories
            .iter()
            .enumerate()
            .map(|(id, path)| StringMatchCandidate::new(id, path))
            .collect::<Vec<_>>();

        self.matches = futures::executor::block_on(match_strings_order_insensitive(
            candidates.as_slice(),
            query,
            smart_case,
            100,
            &Default::default(),
        ));

        self.matches.sort_unstable_by_key(|m| m.candidate_id);

        if self.reset_selected_match_index {
            self.selected_match_index = 0;
        }
        self.reset_selected_match_index = true;
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some((selected_match, workspace)) = self
            .matches
            .get(self.selected_index())
            .zip(self.workspace.upgrade())
        {
            let directory_path = &self.directories[selected_match.candidate_id];
            let path = std::path::PathBuf::from(directory_path);

            #[allow(clippy::disallowed_methods)]
            let _ = std::process::Command::new("zoxide")
                .args(&["add", directory_path])
                .output();

            let replace_current_window = if self.create_new_window {
                !secondary
            } else {
                secondary
            };

            workspace
                .update(cx, |workspace, cx| {
                    let paths = vec![path];
                    if replace_current_window {
                        cx.spawn_in(window, async move |workspace, cx| {
                            let continue_replacing = workspace
                                .update_in(cx, |workspace, window, cx| {
                                    workspace.prepare_to_close(
                                        CloseIntent::ReplaceWindow,
                                        window,
                                        cx,
                                    )
                                })?
                                .await?;
                            if continue_replacing {
                                workspace
                                    .update_in(cx, |workspace, window, cx| {
                                        workspace.open_workspace_for_paths(
                                            OpenMode::Activate,
                                            paths,
                                            window,
                                            cx,
                                        )
                                    })?
                                    .await?;
                            }
                            anyhow::Ok(())
                        })
                    } else {
                        let task = workspace.open_workspace_for_paths(
                            OpenMode::NewWindow,
                            paths,
                            window,
                            cx,
                        );
                        cx.spawn_in(window, async move |_, _| {
                            task.await?;
                            Ok(())
                        })
                    }
                })
                .detach_and_log_err(cx);
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if self.directories.is_empty() {
            "No zoxide directories found. Make sure zoxide is installed and has been used.".into()
        } else {
            "No matches".into()
        };
        Some(text)
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = self.matches.get(ix)?;
        let directory_path = self.directories.get(hit.candidate_id)?;
        let display_path = self.format_path_for_display(directory_path);

        let adjusted_positions = if display_path != *directory_path {
            let display_chars: Vec<char> = display_path.chars().collect();

            let mut offset = 0;
            if let Some(home_dir) = std::env::var("HOME").ok() {
                if directory_path.starts_with(&home_dir) && display_path.starts_with('~') {
                    offset = home_dir.chars().count().saturating_sub(1);
                }
            }

            hit.positions
                .iter()
                .filter_map(|&pos| {
                    if pos >= offset {
                        let adjusted_pos = pos - offset;
                        if adjusted_pos < display_chars.len() {
                            Some(adjusted_pos)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            hit.positions.clone()
        };

        let highlighted_text = HighlightedMatch {
            text: display_path.clone(),
            highlight_positions: adjusted_positions,
            color: Color::Default,
        };

        let tooltip_text = display_path;
        Some(
            ListItem::new(ix)
                .toggle_state(selected)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .flex_grow(1.)
                        .gap_3()
                        .child(Icon::new(IconName::Folder).color(Color::Muted))
                        .child(highlighted_text.render(window, cx)),
                )
                .tooltip(move |_, cx| {
                    cx.new(|_| SimpleTooltip {
                        text: tooltip_text.clone(),
                    })
                    .into()
                }),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        None
    }
}

struct SimpleTooltip {
    text: String,
}

impl Render for SimpleTooltip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(cx, |div, _| div.child(self.text.clone()))
    }
}
