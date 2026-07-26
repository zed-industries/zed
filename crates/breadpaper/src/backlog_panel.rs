//! The Backlog panel (spec `v6-backlog.md`): a bottom-dock checklist over the
//! vault's `backlog.md`, grouped by Soon and Someday with a collapsed
//! Completed history. Task text is editable inline; checking a task off
//! records it as done in today's daily note and files it under Completed with
//! the date. The file stays the single source of truth: edits go through the
//! open buffer (so an editor tab and the panel never fight) and the panel
//! re-parses on every buffer change.

use anyhow::{Context as _, Result};
use chrono::Local;
use editor::{Editor, EditorEvent, SelectionEffects, scroll::Autoscroll};
use gpui::{
    Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Pixels, Subscription, Task, WeakEntity, Window, actions, div, px,
};
use language::{Buffer, BufferEvent};
use project::Project;
use std::path::PathBuf;
use std::time::Duration;
use text::{Bias, Point};
use ui::prelude::*;
use ui::{Checkbox, Icon, IconButton, IconSize, Label, ToggleState, Tooltip};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{OpenOptions, OpenVisible, Workspace};

use crate::backlog::{
    self, Backlog, BacklogTask, SectionKind, parse_backlog, split_completion,
};
use crate::notes::{EnsureNoteOutcome, NoteKind, ensure_note};
use crate::vault::{Vault, VaultStatus};

const BACKLOG_PANEL_KEY: &str = "BreadPaperBacklogPanel";
const REPARSE_DEBOUNCE: Duration = Duration::from_millis(150);

actions!(
    breadpaper,
    [
        /// Toggles focus on the BreadPaper backlog panel.
        ToggleBacklogFocus
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleBacklogFocus, window, cx| {
            workspace.toggle_panel_focus::<BacklogPanel>(window, cx);
        });
    })
    .detach();
}

/// What the inline editor is editing (spec §6.2, §9.5).
enum EditTarget {
    /// A task addressed by its section, line, and text when editing started —
    /// the line disambiguates duplicate texts and external edits mid-edit are
    /// detected (§6.5).
    Existing {
        section: SectionKind,
        line: u32,
        original_text: String,
    },
    /// The `+` affordance: a brand-new task appended to the section.
    New { section: SectionKind },
}

struct EditState {
    target: EditTarget,
    editor: Entity<Editor>,
    _subscription: Subscription,
}

pub struct BacklogPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    position: DockPosition,
    vault_status: VaultStatus,
    /// The open `backlog.md` buffer — panel writes go through it so an open
    /// editor tab and the panel stay coherent (spec §6.4). `None` while the
    /// workspace isn't a vault or the file doesn't exist yet.
    buffer: Option<Entity<Buffer>>,
    /// The path `buffer` was resolved for (kept even when the file is
    /// missing, to notice `[backlog]` config changes).
    buffer_path: Option<PathBuf>,
    _buffer_subscription: Option<Subscription>,
    backlog: Backlog,
    completed_expanded: bool,
    edit_state: Option<EditState>,
    /// A mark-done is running its two ordered writes; checkboxes are disabled
    /// until it settles (spec §6.3).
    mark_in_flight: bool,
    load_buffer_task: Option<Task<()>>,
    reparse_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl BacklogPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            BacklogPanel::new(workspace, window, cx)
        })
    }

    pub fn new(
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let weak_workspace = workspace.weak_handle();
        cx.new(|cx| {
            let project_subscription =
                cx.subscribe(&project, |this: &mut Self, _, event, cx| {
                    if matches!(
                        event,
                        project::Event::WorktreeAdded(_)
                            | project::Event::WorktreeRemoved(_)
                            | project::Event::WorktreeUpdatedEntries(..)
                    ) {
                        this.refresh_vault_status(cx);
                    }
                });
            let mut this = Self {
                workspace: weak_workspace,
                project,
                focus_handle: cx.focus_handle(),
                position: DockPosition::Bottom,
                vault_status: VaultStatus::NotAVault,
                buffer: None,
                buffer_path: None,
                _buffer_subscription: None,
                backlog: Backlog::default(),
                completed_expanded: false,
                edit_state: None,
                mark_in_flight: false,
                load_buffer_task: None,
                reparse_task: None,
                _subscriptions: vec![project_subscription],
            };
            this.vault_status = this.detect_vault_status(cx);
            this.ensure_buffer(cx);
            this
        })
    }

    fn vault(&self) -> Option<&Vault> {
        match &self.vault_status {
            VaultStatus::Valid(vault) => Some(vault),
            _ => None,
        }
    }

    fn detect_vault_status(&self, cx: &App) -> VaultStatus {
        match self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        {
            Some(root) => Vault::detect(&root),
            None => VaultStatus::NotAVault,
        }
    }

    fn refresh_vault_status(&mut self, cx: &mut Context<Self>) {
        let status = self.detect_vault_status(cx);
        if status != self.vault_status {
            self.vault_status = status;
            cx.notify();
        }
        // Re-resolve the buffer either way: a worktree event may mean
        // `backlog.md` just appeared (created by a wrap skill or by hand).
        self.ensure_buffer(cx);
    }

    /// Points the panel's buffer at the vault's current backlog path, opening
    /// the file's buffer when it exists. Idempotent; called on every vault /
    /// worktree change.
    fn ensure_buffer(&mut self, cx: &mut Context<Self>) {
        let Some(path) = self.vault().map(Vault::backlog_path) else {
            if self.buffer.is_some() || self.buffer_path.is_some() {
                self.buffer = None;
                self.buffer_path = None;
                self._buffer_subscription = None;
                self.backlog = Backlog::default();
                self.edit_state = None;
                self.load_buffer_task = None;
                cx.notify();
            }
            return;
        };
        if self.buffer.is_some() && self.buffer_path.as_ref() == Some(&path) {
            // Cancel any in-flight load for a superseded path, or its late
            // completion would repoint the panel away from the current config.
            self.load_buffer_task = None;
            return;
        }
        let project = self.project.clone();
        // Loads are idempotent and read-only until the final assignment, so
        // replacing an in-flight load with a newer one is safe.
        self.load_buffer_task = Some(cx.spawn(async move |this, cx| {
            let exists = cx
                .background_spawn({
                    let path = path.clone();
                    async move { path.is_file() }
                })
                .await;
            let buffer = if exists {
                project
                    .update(cx, |project, cx| project.open_local_buffer(&path, cx))
                    .await
                    .log_err()
            } else {
                None
            };
            this.update(cx, |this, cx| {
                this.buffer_path = Some(path);
                this._buffer_subscription = buffer.as_ref().map(|buffer| {
                    cx.subscribe(buffer, |this, _, event: &BufferEvent, cx| {
                        if matches!(
                            event,
                            BufferEvent::Edited { .. } | BufferEvent::Reloaded
                        ) {
                            this.schedule_reparse(cx);
                        }
                    })
                });
                this.buffer = buffer;
                this.reparse(cx);
                cx.notify();
            })
            .log_err();
        }));
    }

    fn schedule_reparse(&mut self, cx: &mut Context<Self>) {
        self.reparse_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(REPARSE_DEBOUNCE).await;
            this.update(cx, |this, cx| this.reparse(cx)).log_err();
        }));
    }

    fn reparse(&mut self, cx: &mut Context<Self>) {
        let backlog = match &self.buffer {
            Some(buffer) => parse_backlog(&buffer.read(cx).text()),
            None => Backlog::default(),
        };
        if backlog != self.backlog {
            self.backlog = backlog;
            cx.notify();
        }
    }

    fn show_error(&self, message: String, cx: &mut Context<Self>) {
        // Deferred: this can be reached synchronously from action handlers
        // whose callers hold the workspace lease (the V5 double-lease trap).
        let workspace = self.workspace.clone();
        cx.defer(move |cx| {
            workspace
                .update(cx, |workspace, cx| workspace.show_error(message, cx))
                .log_err();
        });
    }

    /// Applies `edits` to the backlog buffer and saves it. All edit ranges
    /// address the buffer's current text (`Buffer::edit` resolves the shifts).
    fn write_edits(
        &mut self,
        buffer: Entity<Buffer>,
        mut edits: Vec<backlog::Edit>,
        error_context: &'static str,
        cx: &mut Context<Self>,
    ) {
        edits.sort_by_key(|edit| edit.range.start);
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                edits
                    .into_iter()
                    .map(|edit| (edit.range, edit.new_text)),
                None,
                cx,
            );
        });
        // Re-parse now rather than waiting for the debounced Edited event, so
        // the panel never renders (or accepts gestures against) the pre-edit
        // state.
        self.reparse(cx);
        let save = self
            .project
            .update(cx, |project, cx| project.save_buffer(buffer, cx));
        cx.spawn(async move |this, cx| {
            if let Err(error) = save.await {
                this.update(cx, |this, cx| {
                    this.show_error(format!("{error_context}: {error}"), cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    // --- Inline editing (spec §6.2) ---

    fn start_edit(
        &mut self,
        target: EditTarget,
        initial_text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(initial_text, window, cx);
            if let EditTarget::New { .. } = target {
                editor.set_placeholder_text("New task", window, cx);
            }
            editor
        });
        editor.update(cx, |editor, cx| {
            editor.select_all(&editor::actions::SelectAll, window, cx);
        });
        let subscription = cx.subscribe_in(
            &editor,
            window,
            |this, _, event: &EditorEvent, window, cx| {
                if matches!(event, EditorEvent::Blurred) {
                    this.commit_edit(window, cx);
                }
            },
        );
        editor.read(cx).focus_handle(cx).focus(window, cx);
        self.edit_state = Some(EditState {
            target,
            editor,
            _subscription: subscription,
        });
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_edit(window, cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.edit_state.take().is_some() {
            self.focus_handle.focus(window, cx);
            cx.notify();
        }
    }

    fn commit_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(state) = self.edit_state.take() else {
            return;
        };
        cx.notify();
        let was_focused = state.editor.read(cx).focus_handle(cx).is_focused(window);
        if was_focused {
            self.focus_handle.focus(window, cx);
        }
        let new_text = state.editor.read(cx).text(cx).trim().to_string();
        match state.target {
            EditTarget::Existing {
                section,
                line,
                original_text,
            } => {
                // Committing an empty string is a revert, not a delete (§6.2).
                if new_text.is_empty() || new_text == original_text {
                    return;
                }
                let Some(buffer) = self.buffer.clone() else {
                    self.show_error(
                        "backlog.md is no longer open, so the edit wasn't applied.".to_string(),
                        cx,
                    );
                    return;
                };
                let text = buffer.read(cx).text();
                let backlog = parse_backlog(&text);
                let Some(task) = backlog.locate_task(section, line, &original_text) else {
                    // The line changed under the edit; dropping beats guessing
                    // (spec §6.5).
                    self.show_error(
                        "That task changed outside the panel, so the edit wasn't applied."
                            .to_string(),
                        cx,
                    );
                    return;
                };
                let edit = backlog::rename_task_edit(task, &new_text);
                self.write_edits(buffer, vec![edit], "Couldn't update backlog.md", cx);
            }
            EditTarget::New { section } => {
                if new_text.is_empty() {
                    return;
                }
                self.append_new_task(section, &new_text, cx);
            }
        }
    }

    fn append_new_task(&mut self, section: SectionKind, text: &str, cx: &mut Context<Self>) {
        let block = backlog::new_task_block(text);
        if let Some(buffer) = self.buffer.clone() {
            let current = buffer.read(cx).text();
            let edit = backlog::append_to_section_edit(&current, section, &block);
            self.write_edits(buffer, vec![edit], "Couldn't add the task to backlog.md", cx);
            return;
        }
        // No file yet: create it around the new task (create-on-first-write,
        // spec §6.5), then adopt its buffer.
        let Some(path) = self.vault().map(Vault::backlog_path) else {
            return;
        };
        let write = cx.background_spawn(async move {
            let current = match std::fs::read_to_string(&path) {
                Ok(current) => current,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    backlog::DEFAULT_BACKLOG.to_string()
                }
                Err(error) => {
                    return Err(error).with_context(|| format!("reading {}", path.display()));
                }
            };
            let edit = backlog::append_to_section_edit(&current, section, &block);
            let new_text = backlog::apply_edits(&current, vec![edit]);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating {}", parent.display()))?;
            }
            std::fs::write(&path, new_text)
                .with_context(|| format!("writing {}", path.display()))?;
            Ok(())
        });
        cx.spawn(async move |this, cx| {
            let result = write.await;
            this.update(cx, |this, cx| match result {
                Ok(()) => this.ensure_buffer(cx),
                Err(error) => {
                    this.show_error(format!("Couldn't create backlog.md: {error}"), cx);
                }
            })
            .log_err();
        })
        .detach();
    }

    // --- Mark done (spec §6.3) ---

    /// Checking a task runs two ordered writes: append `- [x] …` to today's
    /// daily note (created from template if missing), then move the task to
    /// the backlog's Completed section. If the note write fails the backlog
    /// is left untouched, so it never claims a completion no note records.
    fn mark_done(
        &mut self,
        section: SectionKind,
        line: u32,
        task_text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.mark_in_flight || self.buffer.is_none() {
            return;
        }
        let Some(vault) = self.vault().cloned() else {
            return;
        };
        self.mark_in_flight = true;
        cx.notify();

        let project = self.project.clone();
        let heading = vault.config.day_planner.heading.clone();
        let now = Local::now();
        let today = now.date_naive();
        let time = now.time();
        let note_line_text = task_text.clone();
        let ensure =
            cx.background_spawn(async move { ensure_note(&vault, NoteKind::Daily, today, time) });
        cx.spawn_in(window, async move |this, cx| {
            let note_result = async {
                let (note_path, outcome) = ensure.await?;
                if outcome == EnsureNoteOutcome::CreatedWithoutTemplate {
                    this.update(cx, |this, cx| {
                        this.show_error(
                            "The daily template is missing, so today's note was created empty."
                                .to_string(),
                            cx,
                        );
                    })?;
                }
                let note_buffer = project
                    .update(cx, |project, cx| project.open_local_buffer(&note_path, cx))
                    .await?;
                note_buffer.update(cx, |buffer, cx| {
                    let edit =
                        backlog::append_done_to_note_edit(&buffer.text(), &heading, &note_line_text);
                    buffer.edit([(edit.range, edit.new_text)], None, cx);
                });
                project
                    .update(cx, |project, cx| project.save_buffer(note_buffer, cx))
                    .await?;
                anyhow::Ok(())
            }
            .await;
            if let Err(error) = note_result {
                this.update(cx, |this, cx| {
                    this.mark_in_flight = false;
                    this.show_error(
                        format!("Couldn't record the task in today's note: {error}"),
                        cx,
                    );
                    cx.notify();
                })
                .log_err();
                return;
            }

            let backlog_save = this.update(cx, |this, cx| {
                let buffer = this
                    .buffer
                    .clone()
                    .context("backlog.md is no longer open")?;
                let text = buffer.read(cx).text();
                let backlog = parse_backlog(&text);
                let task = backlog
                    .locate_task(section, line, &task_text)
                    .context("the task changed while it was being completed")?;
                let mut edits = backlog::complete_task_edits(&text, task, today);
                edits.sort_by_key(|edit| edit.range.start);
                buffer.update(cx, |buffer, cx| {
                    buffer.edit(
                        edits
                            .into_iter()
                            .map(|edit| (edit.range, edit.new_text)),
                        None,
                        cx,
                    );
                });
                this.reparse(cx);
                anyhow::Ok(
                    this.project
                        .update(cx, |project, cx| project.save_buffer(buffer, cx)),
                )
            });
            let backlog_result = match backlog_save {
                Ok(Ok(save)) => save.await,
                Ok(Err(error)) | Err(error) => Err(error),
            };
            this.update(cx, |this, cx| {
                this.mark_in_flight = false;
                if let Err(error) = backlog_result {
                    // Re-checking is safe: the panel re-renders from the file,
                    // which still shows the task open (spec §6.5).
                    this.show_error(
                        format!(
                            "The task was recorded in today's note, but the backlog couldn't \
                             be updated: {error}"
                        ),
                        cx,
                    );
                }
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    /// Moves a task to the other open section (Soon ↔ Someday).
    fn move_task(
        &mut self,
        section: SectionKind,
        line: u32,
        task_text: String,
        cx: &mut Context<Self>,
    ) {
        let destination = match section {
            SectionKind::Soon => SectionKind::Someday,
            SectionKind::Someday => SectionKind::Soon,
            SectionKind::Completed => return,
        };
        let Some(buffer) = self.buffer.clone() else {
            return;
        };
        let text = buffer.read(cx).text();
        let backlog = parse_backlog(&text);
        let Some(task) = backlog.locate_task(section, line, &task_text) else {
            self.show_error(
                "That task changed outside the panel, so it wasn't moved.".to_string(),
                cx,
            );
            return;
        };
        let edits = backlog::move_task_edits(&text, task, destination);
        self.write_edits(buffer, edits, "Couldn't update backlog.md", cx);
    }

    /// Opens `backlog.md` in the editor at the task's line (the Day Planner's
    /// reveal pattern) for anything beyond a text tweak.
    fn reveal_task(&mut self, line: u32, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.vault().map(Vault::backlog_path) else {
            return;
        };
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let item = workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.open_abs_path(
                        path,
                        OpenOptions {
                            visible: Some(OpenVisible::All),
                            ..Default::default()
                        },
                        window,
                        cx,
                    )
                })?
                .await?;
            if let Some(editor) = item.downcast::<Editor>() {
                editor.update_in(cx, |editor, window, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let point = snapshot.clip_point(Point::new(line, 0), Bias::Left);
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()).nav_history(true),
                        window,
                        cx,
                        |selections| selections.select_ranges([point..point]),
                    );
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    // --- Rendering ---

    fn render_hint(&self, text: impl Into<SharedString>) -> Div {
        v_flex().p_3().child(
            Label::new(text.into())
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
    }

    fn is_editing(&self, section: SectionKind, line: u32, task_text: &str) -> bool {
        self.edit_state.as_ref().is_some_and(|state| {
            matches!(
                &state.target,
                EditTarget::Existing {
                    section: editing_section,
                    line: editing_line,
                    original_text,
                } if *editing_section == section
                    && *editing_line == line
                    && original_text == task_text
            )
        })
    }

    fn is_adding_to(&self, section: SectionKind) -> bool {
        self.edit_state.as_ref().is_some_and(|state| {
            matches!(&state.target, EditTarget::New { section: adding } if *adding == section)
        })
    }

    fn render_editor_row(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let state = self.edit_state.as_ref()?;
        Some(
            div()
                .px_2()
                .py_0p5()
                .border_1()
                .border_color(cx.theme().colors().border_focused)
                .rounded_sm()
                .child(state.editor.clone())
                .into_any_element(),
        )
    }

    fn render_open_task_row(
        &self,
        section: SectionKind,
        index: usize,
        task: &BacklogTask,
        cx: &Context<Self>,
    ) -> AnyElement {
        if self.is_editing(section, task.line, &task.text) {
            if let Some(editor_row) = self.render_editor_row(cx) {
                return editor_row;
            }
        }
        let section_key = section.heading();
        let task_text = task.text.clone();
        let task_line = task.line;
        let (move_icon, move_tooltip) = match section {
            SectionKind::Someday => (IconName::ArrowUp, "Move to Soon"),
            _ => (IconName::ArrowDown, "Move to Someday"),
        };
        h_flex()
            .w_full()
            .gap_1()
            .px_1()
            .py_0p5()
            .child(
                Checkbox::new(
                    ElementId::Name(format!("backlog-check-{section_key}-{index}").into()),
                    ToggleState::Unselected,
                )
                .disabled(self.mark_in_flight)
                .on_click(cx.listener({
                    let task_text = task_text.clone();
                    move |this, _, window, cx| {
                        this.mark_done(section, task_line, task_text.clone(), window, cx);
                    }
                })),
            )
            .child(
                div()
                    .id(ElementId::Name(
                        format!("backlog-task-{section_key}-{index}").into(),
                    ))
                    .flex_1()
                    .min_w_0()
                    .cursor_text()
                    .child(
                        Label::new(task.text.clone())
                            .size(LabelSize::Small)
                            .truncate(),
                    )
                    .on_click(cx.listener({
                        let task_text = task_text.clone();
                        move |this, _, window, cx| {
                            this.start_edit(
                                EditTarget::Existing {
                                    section,
                                    line: task_line,
                                    original_text: task_text.clone(),
                                },
                                &task_text,
                                window,
                                cx,
                            );
                        }
                    })),
            )
            .child(
                IconButton::new(
                    ElementId::Name(format!("backlog-move-{section_key}-{index}").into()),
                    move_icon,
                )
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text(move_tooltip))
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.move_task(section, task_line, task_text.clone(), cx);
                })),
            )
            .child(
                IconButton::new(
                    ElementId::Name(format!("backlog-reveal-{section_key}-{index}").into()),
                    IconName::FileMarkdown,
                )
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text("Reveal in backlog.md"))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.reveal_task(task_line, window, cx);
                })),
            )
            .into_any_element()
    }

    fn render_open_section(&self, section: SectionKind, cx: &Context<Self>) -> AnyElement {
        let colors = cx.theme().colors();
        // Open groups list open tasks (spec §6.1); a hand-checked `- [x]`
        // left in Soon/Someday stays in the file but isn't rendered.
        let tasks: Vec<&BacklogTask> = self
            .backlog
            .section(section)
            .iter()
            .filter(|task| !task.checked)
            .collect();
        let section_key = section.heading();
        let header = h_flex()
            .justify_between()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(colors.border_variant)
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new(section.heading()).size(LabelSize::Small))
                    .child(
                        Label::new(tasks.len().to_string())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
            .child(
                IconButton::new(
                    ElementId::Name(format!("backlog-add-{section_key}").into()),
                    IconName::Plus,
                )
                .icon_size(IconSize::XSmall)
                .icon_color(Color::Muted)
                .tooltip(Tooltip::text(match section {
                    SectionKind::Soon => "Add to Soon",
                    _ => "Add to Someday",
                }))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.start_edit(EditTarget::New { section }, "", window, cx);
                })),
            );

        let mut list = v_flex().py_0p5().gap_0p5();
        for (index, task) in tasks.into_iter().enumerate() {
            list = list.child(self.render_open_task_row(section, index, task, cx));
        }
        if self.is_adding_to(section)
            && let Some(editor_row) = self.render_editor_row(cx)
        {
            list = list.child(editor_row);
        }

        v_flex()
            .flex_1()
            .min_w_0()
            .h_full()
            .border_r_1()
            .border_color(colors.border_variant)
            .child(header)
            .child(
                div()
                    .id(ElementId::Name(
                        format!("backlog-section-{section_key}").into(),
                    ))
                    .flex_1()
                    .overflow_y_scroll()
                    .child(list),
            )
            .into_any_element()
    }

    fn render_completed_section(&self, cx: &Context<Self>) -> AnyElement {
        let colors = cx.theme().colors();
        let completed = &self.backlog.completed;
        let header = h_flex()
            .id("backlog-completed-header")
            .justify_between()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(colors.border_variant)
            .cursor_pointer()
            .child(
                h_flex()
                    .gap_1()
                    .child(Label::new("Completed").size(LabelSize::Small))
                    .child(
                        Label::new(completed.len().to_string())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
            .child(
                Icon::new(if self.completed_expanded {
                    IconName::ChevronDown
                } else {
                    IconName::ChevronRight
                })
                .size(IconSize::XSmall)
                .color(Color::Muted),
            )
            .on_click(cx.listener(|this, _, _window, cx| {
                this.completed_expanded = !this.completed_expanded;
                cx.notify();
            }));

        let mut column = v_flex().flex_1().min_w_0().h_full().child(header);
        if self.completed_expanded {
            let mut list = v_flex().py_0p5().gap_0p5();
            for task in completed.iter() {
                let (label, date) = split_completion(&task.text);
                let mut row = h_flex()
                    .w_full()
                    .gap_1()
                    .px_1()
                    .py_0p5()
                    .child(
                        Icon::new(IconName::TodoComplete)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        div().flex_1().min_w_0().child(
                            Label::new(label.to_string())
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .strikethrough()
                                .truncate(),
                        ),
                    );
                if let Some(date) = date {
                    row = row.child(
                        Label::new(date.to_string())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    );
                }
                list = list.child(row);
            }
            column = column.child(
                div()
                    .id("backlog-completed-list")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(list),
            );
        }
        column.into_any_element()
    }

    fn render_body(&self, cx: &Context<Self>) -> AnyElement {
        match &self.vault_status {
            VaultStatus::NotAVault => self
                .render_hint("Open a BreadPaper vault to use the backlog.")
                .into_any_element(),
            VaultStatus::Invalid { .. } => self
                .render_hint("This vault's config couldn't be read, so the backlog is unavailable.")
                .into_any_element(),
            VaultStatus::Valid(_) => {
                let mut body = v_flex().size_full();
                if self.buffer.is_none() {
                    body = body.child(
                        self.render_hint(
                            "No backlog.md yet — it will be created the first time a task \
                             is added.",
                        )
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant),
                    );
                } else if self.backlog.is_empty() {
                    body = body.child(
                        self.render_hint(
                            "Nothing in the backlog. Wrap skills can move unfinished tasks here.",
                        )
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant),
                    );
                }
                body.child(
                    h_flex()
                        .flex_1()
                        .min_h_0()
                        .items_stretch()
                        .child(self.render_open_section(SectionKind::Soon, cx))
                        .child(self.render_open_section(SectionKind::Someday, cx))
                        .child(self.render_completed_section(cx)),
                )
                .into_any_element()
            }
        }
    }
}

impl Render for BacklogPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("BreadPaperBacklogPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .size_full()
            .child(self.render_body(cx))
    }
}

impl EventEmitter<PanelEvent> for BacklogPanel {}

impl Focusable for BacklogPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for BacklogPanel {
    fn persistent_name() -> &'static str {
        "BreadPaper Backlog Panel"
    }

    fn panel_key() -> &'static str {
        BACKLOG_PANEL_KEY
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Bottom)
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Archive)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Backlog Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        ToggleBacklogFocus.boxed_clone()
    }

    fn activation_priority(&self) -> u32 {
        // Must be unique across all panels; 0-9 are taken (0-3 and 5-7
        // upstream, 4 Timeline, 8 Day Planner, 9 Agent).
        10
    }
}
