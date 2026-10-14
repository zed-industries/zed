//! The interactive REPL window: a read-only transcript plus an input line,
//! like Calva's output window or CIDER's REPL buffer.
//!
//! - Enter evaluates the input in the workspace's nREPL session, so `*1`,
//!   `*2`, `*3` and `*e` work and are shared with in-editor evals.
//! - The prompt tracks the namespace the server reports back, so
//!   `(in-ns 'foo)` behaves as it does in a terminal REPL.
//! - Up / Down walk the input history; `nrepl: interrupt` cancels the
//!   running evaluation; `nrepl: clear outputs` clears the transcript.
//! - When an evaluation throws, the analyzed stacktrace (cider-nrepl) is
//!   printed below the error, project frames first.

use std::{cell::RefCell, net::SocketAddr, rc::Rc};

use anyhow::{Context as _, Result};
use editor::{Editor, EditorElement, EditorMode, EditorStyle, SizingBehavior};
use futures::StreamExt as _;
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task, TextStyle, WeakEntity,
    prelude::*,
};
use menu::{Confirm, SelectNext, SelectPrevious};
use project::search_history::{QueryInsertionBehavior, SearchHistory, SearchHistoryCursor};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::{SplitDirection, Workspace, item::Item, item::ItemEvent};

use crate::completion::NreplCompletionProvider;
use crate::nrepl_store::{ConnectionState, NreplConnection, connected_client};
use crate::ops::{self, ExceptionCause};
use crate::{Value, nrepl_sessions_ui::ClearOutputs, nrepl_sessions_ui::Interrupt};

const DEFAULT_NAMESPACE: &str = "user";
/// Stacktrace frames printed per exception cause.
const MAX_FRAMES: usize = 12;

#[derive(Default)]
struct ConnectionTracker {
    printed_starting: bool,
    printed_connecting: Option<SocketAddr>,
    printed_terminal: bool,
    last_jack_in_line_count: usize,
}

pub struct NreplConsole {
    workspace: WeakEntity<Workspace>,
    transcript: Entity<Editor>,
    input: Entity<Editor>,
    namespace: Rc<RefCell<String>>,
    history: SearchHistory,
    history_cursor: SearchHistoryCursor,
    /// `(request id, task)` of the evaluation currently running.
    running: Option<(String, Task<()>)>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
    _connection_subscription: Option<Subscription>,
    _connection_poll_task: Option<Task<()>>,
}

impl NreplConsole {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_workspace = workspace.weak_handle();
        let namespace = Rc::new(RefCell::new(DEFAULT_NAMESPACE.to_string()));

        let transcript = cx.new(|cx| {
            let mut editor = Editor::multi_line(window, cx);
            editor.set_mode(EditorMode::Full {
                scale_ui_elements_with_buffer_font_size: true,
                show_active_line_background: false,
                sizing_behavior: SizingBehavior::ExcludeOverscrollMargin,
            });
            editor.set_read_only(true);
            editor.set_show_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_bookmarks(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor.set_autoindent(false);
            editor.set_input_enabled(false);
            editor.set_use_autoclose(false);
            editor.set_use_modal_editing(false);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor
        });

        let input = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Evaluate Clojure (Enter)", window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            let namespace = namespace.clone();
            editor.set_completion_provider(Some(Rc::new(NreplCompletionProvider::standalone(
                weak_workspace.clone(),
                Rc::new(move |_, _| namespace.borrow().clone()),
            ))));
            editor
        });

        Self::set_clojure_language(workspace, &[&transcript, &input], cx);

        let focus_handle = cx.focus_handle();
        let subscriptions = vec![cx.on_focus(&focus_handle, window, |this, window, cx| {
            this.input.focus_handle(cx).focus(window, cx);
        })];

        let mut this = Self {
            workspace: weak_workspace,
            transcript,
            input,
            namespace,
            history: SearchHistory::new(None, QueryInsertionBehavior::ReplacePreviousIfContains),
            history_cursor: SearchHistoryCursor::default(),
            running: None,
            focus_handle,
            _subscriptions: subscriptions,
            _connection_subscription: None,
            _connection_poll_task: None,
        };
        this.append(
            ";; nREPL console — Enter evaluates, ↑/↓ browse history.\n\
             ;; *1 *2 *3 and *e are shared with evaluations from the editor.\n",
            Some(window),
            cx,
        );
        this
    }

    /// Highlights the transcript and input as Clojure, once the language
    /// (provided by the Clojure extension) is loaded.
    fn set_clojure_language(workspace: &Workspace, editors: &[&Entity<Editor>], cx: &mut App) {
        let languages = workspace.project().read(cx).languages().clone();
        let buffers: Vec<_> = editors
            .iter()
            .filter_map(|editor| editor.read(cx).buffer().read(cx).as_singleton())
            .collect();
        let language = languages.language_for_name("Clojure");
        cx.spawn(async move |cx| {
            let Some(language) = language.await.log_err() else {
                return;
            };
            cx.update(|cx| {
                for buffer in buffers {
                    buffer.update(cx, |buffer, cx| {
                        buffer.set_language(Some(language.clone()), cx)
                    });
                }
            });
        })
        .detach();
    }

    pub fn namespace(&self) -> String {
        self.namespace.borrow().clone()
    }

    /// Sets the prompt namespace (e.g. after `nrepl: switch namespace`).
    pub fn set_namespace(&mut self, namespace: String, cx: &mut Context<Self>) {
        *self.namespace.borrow_mut() = namespace;
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    pub fn append(&mut self, text: &str, window: Option<&mut Window>, cx: &mut Context<Self>) {
        if text.is_empty() {
            return;
        }
        self.transcript.update(cx, |editor, cx| {
            editor.set_read_only(false);
            if let Some(window) = window {
                editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
                editor.insert(text, window, cx);
            } else {
                let multibuffer = editor.buffer().clone();
                multibuffer.update(cx, |multibuffer, cx| {
                    let len = multibuffer.len(cx);
                    multibuffer.edit([(len..len, text)], None, cx);
                });
            }
            editor.set_read_only(true);
        });
    }

    /// Appends `text`, making sure it starts on a fresh line.
    pub fn append_line(
        &mut self,
        text: &str,
        mut window: Option<&mut Window>,
        cx: &mut Context<Self>,
    ) {
        let needs_newline = !self.transcript.read(cx).text(cx).ends_with('\n');
        if needs_newline {
            self.append("\n", window.as_deref_mut(), cx);
        }
        self.append(text, window.as_deref_mut(), cx);
        if !text.ends_with('\n') {
            self.append("\n", window, cx);
        }
    }

    fn evaluate(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let code = self.input.read(cx).text(cx);
        if code.trim().is_empty() {
            return;
        }
        self.input.update(cx, |editor, cx| editor.clear(window, cx));
        self.history.add(&mut self.history_cursor, code.clone());
        self.history_cursor.reset();
        self.submit_with_echo(code, window, cx);
    }

    /// Echoes `code` after the prompt, then evaluates it.
    pub fn submit_with_echo(&mut self, code: String, window: &mut Window, cx: &mut Context<Self>) {
        let prompt = format!("{}=> {}", self.namespace(), code.trim());
        self.append_line(&prompt, Some(window), cx);
        if let Err(err) = self.submit(code, window, cx) {
            self.append_line(&format!(";; {err:#}"), Some(window), cx);
        }
    }

    /// Evaluates `code` in the console's namespace, streaming the reply
    /// into the transcript. Also used by "send form to REPL" from editors.
    pub fn submit(
        &mut self,
        code: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let workspace = self.workspace.upgrade().context("workspace dropped")?;
        let (client, session) = connected_client(workspace.entity_id(), cx)?;
        let namespace = self.namespace();
        let stream = client.send(ops::eval_request(&code, Some(&namespace), &session))?;
        let request_id = stream.id().to_string();

        let task = cx.spawn_in(window, async move |this, cx| {
            let mut stream = stream;
            let mut failed = false;
            while let Some(message) = stream.next().await {
                let statuses = ops::statuses(&message);
                failed |= statuses.contains(&"eval-error");
                let keep_going = this
                    .update_in(cx, |this, window, cx| {
                        this.handle_reply(&message, &statuses, Some(window), cx);
                    })
                    .is_ok();
                if !keep_going {
                    return;
                }
            }
            if failed {
                let causes = ops::last_stacktrace(&client, &session).await.log_err();
                this.update_in(cx, |this, window, cx| {
                    if let Some(causes) = causes {
                        let text = format_stacktrace(&causes);
                        this.append_line(&text, Some(window), cx);
                    }
                })
                .log_err();
            }
            this.update(cx, |this, _| this.running = None).log_err();
        });
        self.running = Some((request_id, task));
        Ok(())
    }

    pub fn handle_reply(
        &mut self,
        message: &Value,
        statuses: &[&str],
        mut window: Option<&mut Window>,
        cx: &mut Context<Self>,
    ) {
        if let Some(out) = message.get("out").and_then(Value::as_str) {
            self.append(out, window.as_deref_mut(), cx);
        }
        if let Some(err) = message.get("err").and_then(Value::as_str) {
            self.append(err, window.as_deref_mut(), cx);
        }
        if let Some(ex) = message
            .get("ex")
            .and_then(Value::as_str)
            .or_else(|| message.get("root-ex").and_then(Value::as_str))
        {
            self.append_line(&format!(";; {ex}"), window.as_deref_mut(), cx);
        }
        if let Some(value) = message.get("value").and_then(Value::as_str) {
            self.append_line(value, window.as_deref_mut(), cx);
        }
        if let Some(ns) = message.get("ns").and_then(Value::as_str)
            && ns != self.namespace.borrow().as_str()
        {
            self.set_namespace(ns.to_string(), cx);
        }
        if statuses.contains(&"interrupted") {
            self.append_line(";; interrupted", window.as_deref_mut(), cx);
        }
        if statuses.contains(&"namespace-not-found") {
            let message = format!(";; namespace {} not found", self.namespace());
            self.append_line(&message, window, cx);
        }
    }

    pub fn watch_connection(
        &mut self,
        connection: &Entity<NreplConnection>,
        cx: &mut Context<Self>,
    ) {
        let tracker = Rc::new(RefCell::new(ConnectionTracker::default()));

        let handle_update = {
            let tracker = tracker.clone();
            move |this: &mut Self, conn: Entity<NreplConnection>, cx: &mut Context<Self>| {
                let mut tracker = tracker.borrow_mut();
                let (new_lines, state) = {
                    let conn_ref = conn.read(cx);
                    let new_lines = if let Some(output) = conn_ref.jack_in_output() {
                        let lines = output.lines();
                        if lines.len() > tracker.last_jack_in_line_count {
                            let chunk = lines[tracker.last_jack_in_line_count..].to_vec();
                            tracker.last_jack_in_line_count = lines.len();
                            Some(chunk)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    (new_lines, conn_ref.state().clone())
                };

                if let Some(lines) = new_lines {
                    for line in &lines {
                        this.append_line(line, None, cx);
                    }
                }

                match state {
                    ConnectionState::Resolving => {}
                    ConnectionState::Starting { command } => {
                        if !tracker.printed_starting {
                            tracker.printed_starting = true;
                            this.append_line(
                                &format!(";; Starting nREPL server: {command}"),
                                None,
                                cx,
                            );
                        }
                    }
                    ConnectionState::Connecting { addr } => {
                        if tracker.printed_connecting != Some(addr) {
                            tracker.printed_connecting = Some(addr);
                            this.append_line(&format!(";; Connecting to {addr}…"), None, cx);
                        }
                    }
                    ConnectionState::Connected { addr, session, .. } => {
                        if !tracker.printed_terminal {
                            tracker.printed_terminal = true;
                            this.append_line(
                                &format!(";; Connected to {addr} (session {session})"),
                                None,
                                cx,
                            );
                        }
                    }
                    ConnectionState::Failed { error } => {
                        if !tracker.printed_terminal {
                            tracker.printed_terminal = true;
                            this.append_line(&format!(";; Connection failed: {error}"), None, cx);
                        }
                    }
                }
            }
        };

        handle_update(self, connection.clone(), cx);

        let poll_task = cx.spawn({
            let connection = connection.downgrade();
            async move |this, cx| {
                loop {
                    let is_starting = cx.update(|cx| {
                        connection
                            .upgrade()
                            .map(|conn| {
                                matches!(conn.read(cx).state(), ConnectionState::Starting { .. })
                            })
                            .unwrap_or(false)
                    });

                    if !is_starting {
                        break;
                    }

                    let keep_going = this
                        .update(cx, |this, cx| {
                            if let Some(conn) = connection.upgrade() {
                                let mut tracker = tracker.borrow_mut();
                                let new_lines = {
                                    let conn_ref = conn.read(cx);
                                    if let Some(output) = conn_ref.jack_in_output() {
                                        let lines = output.lines();
                                        if lines.len() > tracker.last_jack_in_line_count {
                                            let chunk =
                                                lines[tracker.last_jack_in_line_count..].to_vec();
                                            tracker.last_jack_in_line_count = lines.len();
                                            Some(chunk)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                };
                                if let Some(lines) = new_lines {
                                    for line in &lines {
                                        this.append_line(line, None, cx);
                                    }
                                }
                            }
                        })
                        .is_ok();

                    if !keep_going {
                        break;
                    }

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(100))
                        .await;
                }
            }
        });

        self._connection_subscription = Some(cx.observe(connection, handle_update));
        self._connection_poll_task = Some(poll_task);
    }

    fn interrupt(&mut self, _: &Interrupt, _window: &mut Window, cx: &mut Context<Self>) {
        let Some((request_id, _)) = &self.running else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        if let Some((client, session)) = connected_client(workspace.entity_id(), cx).log_err()
            && let Some(stream) = client.interrupt(&session, request_id).log_err()
        {
            cx.background_spawn(ops::collect(stream)).detach();
        }
    }

    fn clear(&mut self, _: &ClearOutputs, window: &mut Window, cx: &mut Context<Self>) {
        self.transcript.update(cx, |editor, cx| {
            editor.set_read_only(false);
            editor.clear(window, cx);
            editor.set_read_only(true);
        });
    }

    fn previous_input(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        let current = self.input.read(cx).text(cx);
        if let Some(previous) = self.history.previous(&mut self.history_cursor, &current) {
            let previous = previous.to_string();
            self.input
                .update(cx, |editor, cx| editor.set_text(previous, window, cx));
        }
    }

    fn next_input(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        let next = self
            .history
            .next(&mut self.history_cursor)
            .map(str::to_string)
            .unwrap_or_else(|| {
                self.history_cursor.reset();
                String::new()
            });
        self.input
            .update(cx, |editor, cx| editor.set_text(next, window, cx));
    }

    fn editor_style(editor: &Entity<Editor>, cx: &App) -> EditorStyle {
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme();
        let text = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                theme.colors().text_muted
            } else {
                theme.colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
            ..Default::default()
        };
        EditorStyle {
            background: theme.colors().editor_background,
            local_player: theme.players().local(),
            text,
            syntax: theme.syntax().clone(),
            ..Default::default()
        }
    }
}

/// Renders analyzed exception causes, CIDER style.
pub fn format_stacktrace(causes: &[ExceptionCause]) -> String {
    let mut text = String::new();
    for cause in causes {
        text.push_str(&format!(";; {}: {}\n", cause.class, cause.message.trim()));
        let project_frames: Vec<_> = cause.frames.iter().filter(|frame| frame.project).collect();
        let frames: Vec<_> = if project_frames.is_empty() {
            cause.frames.iter().take(MAX_FRAMES).collect()
        } else {
            project_frames.into_iter().take(MAX_FRAMES).collect()
        };
        for frame in frames {
            let location = match (&frame.file_url, &frame.file, frame.line) {
                (Some(url), _, Some(line)) => match ops::local_path_from_file_url(url) {
                    Some(path) => format!("{}:{line}", path.display()),
                    None => format!("{}:{line}", frame.file.as_deref().unwrap_or("?")),
                },
                (_, Some(file), Some(line)) => format!("{file}:{line}"),
                _ => String::new(),
            };
            text.push_str(&format!(";;   at {} {location}\n", frame.name));
        }
    }
    text
}

impl EventEmitter<ItemEvent> for NreplConsole {}

impl Focusable for NreplConsole {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for NreplConsole {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("REPL · {}", self.namespace()).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Terminal))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("nREPL Console Opened")
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }
}

impl Render for NreplConsole {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let running = self.running.is_some();
        v_flex()
            .track_focus(&self.focus_handle)
            .key_context("NreplConsole")
            .on_action(cx.listener(Self::evaluate))
            .on_action(cx.listener(Self::previous_input))
            .on_action(cx.listener(Self::next_input))
            .on_action(cx.listener(Self::interrupt))
            .on_action(cx.listener(Self::clear))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(div().flex_1().px_2().pt_1().child(EditorElement::new(
                &self.transcript,
                Self::editor_style(&self.transcript, cx),
            )))
            .child(
                h_flex()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new(format!("{}=>", self.namespace()))
                            .color(if running { Color::Info } else { Color::Accent })
                            .buffer_font(cx),
                    )
                    .child(div().flex_1().child(EditorElement::new(
                        &self.input,
                        Self::editor_style(&self.input, cx),
                    ))),
            )
    }
}

/// Opens (or focuses) the workspace's REPL console and returns it.
pub fn open_console(
    workspace: &mut Workspace,
    focus: bool,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Entity<NreplConsole> {
    let previous_pane = workspace.active_pane().clone();
    let previous_item = previous_pane.read(cx).active_item();

    let console = if let Some(existing) = existing_console(workspace, cx) {
        workspace.activate_item(&existing, focus, focus, window, cx);
        existing
    } else {
        let console = cx.new(|cx| NreplConsole::new(workspace, window, cx));
        workspace.split_item(SplitDirection::Right, Box::new(console.clone()), window, cx);
        console
    };

    if !focus {
        if let Some(item) = previous_item {
            workspace.activate_item(&*item, true, true, window, cx);
        } else {
            previous_pane.update(cx, |pane, cx| {
                window.focus(&pane.focus_handle(cx), cx);
            });
        }
    }

    console
}

/// Returns the workspace's REPL console if one is open.
pub fn existing_console(workspace: &Workspace, cx: &App) -> Option<Entity<NreplConsole>> {
    workspace.items_of_type::<NreplConsole>(cx).next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::StackFrame;

    #[test]
    fn stacktrace_prefers_project_frames() {
        let frame = |name: &str, project| StackFrame {
            name: name.into(),
            file: Some("core.clj".into()),
            file_url: None,
            line: Some(1),
            project,
        };
        let text = format_stacktrace(&[ExceptionCause {
            class: "java.lang.ArithmeticException".into(),
            message: "Divide by zero".into(),
            frames: vec![frame("clojure.core/+", false), frame("my.app/f", true)],
        }]);
        assert!(text.contains("Divide by zero"));
        assert!(text.contains("my.app/f"));
        assert!(!text.contains("clojure.core/+"));
    }
}
