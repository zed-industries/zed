//! Script Runner panel — a self-contained panel for running arbitrary shell
//! commands and streaming their output live.
//!
//! Ported standalone (no host-app wiring yet): the source runner used to
//! also receive run requests from other panels via a host `AppState`
//! broadcast channel, and notify the same host state when a run finished.
//! Neither exists here — instead, other panels reach this panel directly
//! through its `Entity`: [`ScriptRunnerPanel::run_external`] is what
//! `npm_manager_panel`'s install/remove/update actions call to stream their
//! npm output here (§4.2 of the npm port plan), polling
//! [`ScriptRunnerPanel::is_running`] via `cx.observe` for completion. The
//! panel is also fully usable on its own via its own command input.
//!
//! Runs commands via `pwsh.exe` (falling back to `cmd.exe`) on Windows and
//! `sh -c` elsewhere, and streams
//! stdout into an output log. A kill flag lets a running command be stopped
//! mid-flight. The SPFx debug query-string bar surfaces the workbench URL
//! fragment printed by `gulp serve` / `heft start` for easy copy-paste.

pub mod command;

mod script_runner_panel_settings;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::ffi::OsString;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use gpui::{
    Action, AnyElement, App, AppContext as _, AsyncApp, AsyncWindowContext, ClipboardItem,
    Context, Entity, EventEmitter, FocusHandle, Focusable, Hsla, InteractiveElement as _,
    IntoElement, ParentElement as _, Pixels, Render, SharedString,
    StatefulInteractiveElement as _, Styled as _, WeakEntity, Window, actions, div,
    prelude::FluentBuilder as _, px, svg,
};
use gpui_component::{
    ActiveTheme, Sizable as _,
    h_flex, v_flex,
    input::{Input, InputState},
    spinner::Spinner,
};
pub use script_runner_panel_settings::ScriptRunnerPanelSettings;
use settings::Settings as _;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

const MAX_OUTPUT_LINES: usize = 500;

/// Events streamed from the background runner thread to the GPUI panel.
#[derive(Clone)]
enum RunnerEvent {
    Line(String),
    Done { exit_code: i32, was_killed: bool },
}

/// Detect an SPFx workbench debug query-string in a line of output:
/// `?debug=true&...` printed by `gulp serve` / `heft start`.
fn debug_querystring_in(line: &str) -> Option<String> {
    let start = line.find("?debug")?;
    let rest = &line[start..];
    let end = rest
        .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '<' | '>'))
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

/// Strips ANSI escape sequences — SGR/CSI color and cursor codes
/// (`ESC [ ... <final byte>`) and OSC sequences such as hyperlinks
/// (`ESC ] ... (BEL | ESC \)`) — from a line of subprocess output.
///
/// Tools like Vite assume they're printing to a real terminal and colorize
/// their output accordingly; this output log is a plain text buffer, not a
/// terminal emulator, so left unstripped those escapes show up as literal
/// noise (`[32m[1mVITE[22m`) instead of being interpreted.
fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('[') => {
                chars.next();
                for c in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&c) {
                        break;
                    }
                }
            }
            Some(']') => {
                chars.next();
                loop {
                    match chars.next() {
                        None | Some('\u{7}') => break,
                        Some('\u{1b}') => {
                            if chars.peek() == Some(&'\\') {
                                chars.next();
                            }
                            break;
                        }
                        Some(_) => {}
                    }
                }
            }
            // An unrecognized escape kind: drop just the ESC byte itself
            // rather than guessing how far to skip.
            _ => {}
        }
    }
    out
}

/// The end of the URL starting at byte offset `start` in `line`: scans to
/// the next whitespace/quote/angle-bracket, then trims trailing sentence
/// punctuation — the same as the original Tauri/React version's
/// `URL_TRIM_RE`, so `https://host/,` becomes `https://host/`.
fn url_end(line: &str, start: usize) -> usize {
    let candidate = &line[start..];
    let raw_end = candidate
        .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '<' | '>'))
        .unwrap_or(candidate.len());
    let trimmed = candidate[..raw_end].trim_end_matches([',', '.', ';', ':', '!', '?', ')', ']']);
    start + trimmed.len()
}

/// Byte ranges of every `http(s)://` URL match in `line`, leftmost first —
/// mirrors the original version's `/https?:\/\/[^\s"'<>]+/g` regex.
fn url_ranges(line: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut pos = 0;
    while pos < line.len() {
        let rest = &line[pos..];
        let rel_start = match (rest.find("http://"), rest.find("https://")) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => break,
        };
        let start = pos + rel_start;
        let end = url_end(line, start);
        if end > start {
            ranges.push((start, end));
        }
        pos = end.max(start + 1);
    }
    ranges
}

/// Extracts every distinct `http(s)://` URL in `line` (already
/// ANSI-stripped), in order of first appearance.
fn find_urls(line: &str) -> Vec<String> {
    url_ranges(line)
        .into_iter()
        .map(|(start, end)| line[start..end].to_string())
        .collect()
}

/// Renders one line of output, turning the first `http(s)://` URL
/// substring (if any) into a clickable link that opens in the system
/// browser — e.g. the dev-server URL a tool like Vite prints on startup.
/// Every detected link in a run also shows up in the links bar above the
/// output (see `render`), with its own copy button.
fn render_output_line(line: &str, muted: Hsla, link: Hsla) -> AnyElement {
    let Some(&(start, end)) = url_ranges(line).first() else {
        return div()
            .font_family("Cascadia Mono")
            .text_xs()
            .text_color(muted)
            .child(line.to_string())
            .into_any_element();
    };

    let before = line[..start].to_string();
    let url = line[start..end].to_string();
    let after = line[end..].to_string();

    h_flex()
        .font_family("Cascadia Mono")
        .text_xs()
        .child(div().text_color(muted).child(before))
        .child(
            div()
                .id(SharedString::from(format!("runner-link-{url}")))
                .text_color(link)
                .cursor_pointer()
                .hover(|d| d.underline())
                .on_click({
                    let url = url.clone();
                    move |_event, _window, cx: &mut gpui::App| cx.open_url(&url)
                })
                .child(url),
        )
        .child(div().text_color(muted).child(after))
        .into_any_element()
}

/// The workspace's project root — the same "first project directory" Zed's
/// own terminal defaults new shells to (see `default_working_directory` /
/// `first_project_directory` in `terminal_view::terminal_view`, not reused
/// directly here to avoid pulling in the whole terminal-emulator crate for
/// one path lookup). Falls back to this process's own current directory if
/// the workspace has no worktree open yet.
fn workspace_root_directory(workspace: &Workspace, cx: &App) -> String {
    workspace
        .worktrees(cx)
        .next()
        .map(|worktree| {
            let worktree = worktree.read(cx);
            let root = worktree.abs_path();
            if worktree.root_entry().is_some_and(|entry| entry.is_dir()) {
                root.to_path_buf()
            } else {
                root.parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| root.to_path_buf())
            }
        })
        .or_else(|| std::env::current_dir().ok())
        .map(|p| p.display().to_string())
        .unwrap_or_default()
}

actions!(
    script_runner_panel,
    [
        /// Toggles focus on the Script Runner panel.
        ToggleFocus
    ]
);

/// Registers the Script Runner panel's actions on every workspace. Call
/// once at app startup, alongside the other panels' `init` functions.
pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ScriptRunnerPanel>(window, cx);
        });
    })
    .detach();
}

pub struct ScriptRunnerPanel {
    focus_handle: FocusHandle,

    /// Broadcast channel: the background runner thread sends `RunnerEvent`s
    /// here; the subscription in `new()` receives them and updates panel state.
    event_tx: tokio::sync::broadcast::Sender<RunnerEvent>,

    command_input: Option<Entity<InputState>>,
    cwd: String,
    output: Vec<String>,
    running: bool,
    kill_flag: Arc<AtomicBool>,
    /// Environment variables for the run being started, from
    /// [`Self::run_external_with_env`]. Empty outside that call.
    run_env: Vec<(OsString, OsString)>,

    /// Latest SPFx debug query-string seen in this run, if any.
    debug_script: Option<String>,
    /// True for ~1.6 s after the copy button is clicked — swaps icon to ✓.
    debug_copied: bool,

    /// Every distinct `http(s)://` URL seen in this run's output so far, in
    /// first-seen order. Rendered as a "links" bar above the output when
    /// there's no SPFx debug script to show instead (see `render`).
    detected_links: Vec<String>,
    /// The link (if any) currently showing "copied" in the links bar —
    /// same ~1.6 s revert pattern as `debug_copied`, keyed by URL since
    /// multiple links can be detected in one run.
    link_copied: Option<String>,
}

impl ScriptRunnerPanel {
    /// Loads the panel for a workspace, following the same
    /// `WeakEntity<Workspace>` + `AsyncWindowContext` convention as the
    /// other dock panels' `load` functions (see `initialize_panels` in
    /// `zed::zed`), so it can be added to the dock alongside them.
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            ScriptRunnerPanel::new(workspace, window, cx)
        })
    }

    pub fn new(
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let cwd = workspace_root_directory(workspace, cx);

        cx.new(|cx| {
            let (event_tx, _) = tokio::sync::broadcast::channel::<RunnerEvent>(256);

            // ── Subscribe to our own event channel ──
            {
                let mut rx = event_tx.subscribe();
                cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
                    loop {
                        let evt = match rx.recv().await {
                            Ok(v) => v,
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                        };
                        let weak = this.clone();
                        let _ = cx.update(|app| {
                            if let Some(panel) = weak.upgrade() {
                                let _ = panel.update(app, |panel, cx| match evt {
                                    RunnerEvent::Line(line) => {
                                        if panel.output.len() >= MAX_OUTPUT_LINES {
                                            panel.output.drain(
                                                0..(panel.output.len() - MAX_OUTPUT_LINES + 1),
                                            );
                                        }
                                        if let Some(qs) = debug_querystring_in(&line) {
                                            panel.debug_script = Some(qs);
                                        }
                                        for url in find_urls(&line) {
                                            if !panel.detected_links.contains(&url) {
                                                panel.detected_links.push(url);
                                            }
                                        }
                                        panel.output.push(line);
                                        cx.notify();
                                    }
                                    RunnerEvent::Done {
                                        exit_code,
                                        was_killed,
                                    } => {
                                        panel.running = false;
                                        panel.kill_flag.store(false, Ordering::SeqCst);
                                        let marker = if was_killed {
                                            "[killed]".to_string()
                                        } else if exit_code == 0 {
                                            format!("[done · exit {exit_code}]")
                                        } else {
                                            format!("[error · exit {exit_code}]")
                                        };
                                        panel.output.push(marker);
                                        // Future wiring point: notify a host app's own
                                        // run-completion channel here, the way the ported
                                        // source notified `AppState::script_done_tx`.
                                        cx.notify();
                                    }
                                });
                            }
                        });
                    }
                })
                .detach();
            }

            ScriptRunnerPanel {
                focus_handle: cx.focus_handle(),
                event_tx,
                command_input: None,
                cwd,
                output: Vec::new(),
                running: false,
                kill_flag: Arc::new(AtomicBool::new(false)),
                run_env: Vec::new(),
                debug_script: None,
                debug_copied: false,
                detected_links: Vec::new(),
                link_copied: None,
            }
        })
    }

    /// Entry point for another panel to trigger a run directly on this
    /// panel's entity (e.g. `entity.update(cx, |panel, cx| panel.run_external(...))`).
    /// Not called automatically by anything yet — see the module doc.
    pub fn run_external(&mut self, command: String, cwd: String, cx: &mut Context<Self>) {
        self.run_external_with_env(command, cwd, Vec::new(), cx);
    }

    /// [`Self::run_external`], with `env` set on the command's process. This
    /// is how a caller runs something under a particular toolchain: the Node
    /// panel passes a `PATH` with its chosen Node.js version first.
    ///
    /// The variables apply to this run only. A command typed into the panel
    /// afterwards runs in the IDE's own environment again.
    pub fn run_external_with_env(
        &mut self,
        command: String,
        cwd: String,
        env: Vec<(OsString, OsString)>,
        cx: &mut Context<Self>,
    ) {
        self.cwd = cwd;
        self.run_env = env;
        self.run(&command, cx);
        self.run_env.clear();
    }

    /// Whether a command is currently executing on this panel. Used by the
    /// npm manager's §4.2 wiring to (a) refuse to pile a second run onto the
    /// single streaming slot and (b) detect completion after the run it
    /// kicked off leaves `running`.
    pub fn is_running(&self) -> bool {
        self.running
    }

    fn run(&mut self, command: &str, cx: &mut Context<Self>) {
        if self.running {
            return;
        }
        let cmd = command.trim().to_string();
        if cmd.is_empty() {
            return;
        }

        self.output.clear();
        self.debug_script = None;
        self.debug_copied = false;
        self.detected_links.clear();
        self.link_copied = None;
        self.running = true;
        self.kill_flag.store(false, Ordering::SeqCst);
        cx.notify();

        let cwd = self.cwd.clone();
        let env = self.run_env.clone();
        let event_tx = self.event_tx.clone();
        let kill_flag = self.kill_flag.clone();

        std::thread::spawn(move || {
            let env = with_ide_runtimes(env, &cwd);
            run_shell_blocking(&cmd, &cwd, &env, event_tx, kill_flag);
        });
    }

    fn stop(&mut self) {
        self.kill_flag.store(true, Ordering::SeqCst);
    }
}

/// `env` with a `PATH` that has the IDE's own runtimes (`ide_env`: the
/// built-in Node.js, …) first, so a command run from `cwd` finds them however
/// it was started: typed into the panel or sent by another one. A caller
/// that passed its own `PATH` keeps it.
///
/// Reads the disk, so this runs on the command's thread, not the main one.
fn with_ide_runtimes(mut env: Vec<(OsString, OsString)>, cwd: &str) -> Vec<(OsString, OsString)> {
    let has_path = env
        .iter()
        .any(|(name, _)| name.to_str().is_some_and(ide_env::is_path_variable));
    if !has_path {
        let directory = (!cwd.is_empty()).then(|| Path::new(cwd));
        if let Some(path) = ide_env::path_env(directory) {
            env.push((OsString::from("PATH"), path));
        }
    }
    env
}

/// Force-kills a process tree by PID. Used both by the kill-watcher below and
/// as the fallback if the normal `child.kill()` path can't run because the
/// reader thread is blocked inside a line read (see `run_shell_blocking`'s
/// doc comment for why a plain `child.kill()` in the read loop isn't enough).
#[cfg(target_os = "windows")]
fn kill_process_tree(pid: u32) {
    let mut c = gpui_util::new_std_command("taskkill.exe");
    c.args(["/F", "/T", "/PID", &pid.to_string()]);
    let _ = c.output();
}

/// Off Windows the shell is started as the leader of its own process group
/// (see `run_shell_blocking`), so `pid` is also the group's id. Killing the
/// group (a negative id) takes everything the command started with it — a
/// dev server and its workers, not just the `sh` that launched them.
#[cfg(not(target_os = "windows"))]
fn kill_process_tree(pid: u32) {
    use std::process::Command;
    let group = format!("-{pid}");
    let killed_group = Command::new("kill")
        .args(["-9", "--", &group])
        .output()
        .is_ok_and(|output| output.status.success());
    // A shell that already exited leaves no group to signal; whatever is
    // left is then at most the process itself.
    if !killed_group {
        let _ = Command::new("kill").args(["-9", &pid.to_string()]).output();
    }
}

/// Spawns a shell command, reads stdout line by line, sends each line and the
/// final exit code to `tx`. On Windows uses `pwsh.exe` first, falling back to
/// `cmd.exe`; elsewhere uses `sh -c`.
///
/// Two things this specifically works around:
/// - Python (and other interpreters) fully block-buffer stdout when it's not
///   a real console, so a piped `python app.py` can run for a long time
///   without producing any lines here at all. `PYTHONUNBUFFERED=1` forces
///   line-buffered/unbuffered output so lines show up as they're printed.
/// - `BufReader::lines()` blocks until a line (or EOF) arrives, so checking
///   `kill_flag` only *between* lines never fires for a process that isn't
///   printing anything — e.g. a running Flask/FastAPI server the user wants
///   to Stop. A separate watcher thread polls `kill_flag` independently of
///   the read loop and force-kills the whole process tree (`taskkill /T`,
///   since the direct child is pwsh/cmd, not the interpreter itself) the
///   moment Stop is clicked; killing the process closes its stdout pipe,
///   which unblocks the read loop on its own.
fn run_shell_blocking(
    cmd: &str,
    cwd: &str,
    env: &[(OsString, OsString)],
    tx: tokio::sync::broadcast::Sender<RunnerEvent>,
    kill_flag: Arc<AtomicBool>,
) {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;

    #[cfg(target_os = "windows")]
    let child = {
        let merged_cmd = format!("& {{ {cmd} }} *>&1");
        let mut c = gpui_util::new_std_command("pwsh.exe");
        c.args(["-NoProfile", "-NonInteractive", "-Command", &merged_cmd])
            .current_dir(cwd)
            .envs(env.iter().map(|(name, value)| (name, value)))
            .env("PYTHONUNBUFFERED", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        // Fallback to cmd.exe if pwsh not found
        c.spawn().or_else(|_| {
            let fallback_cmd = format!("{cmd} 2>&1");
            let mut c = gpui_util::new_std_command("cmd.exe");
            c.args(["/C", &fallback_cmd])
                .current_dir(cwd)
                .envs(env.iter().map(|(name, value)| (name, value)))
                .env("PYTHONUNBUFFERED", "1")
                .stdout(Stdio::piped())
                .stderr(Stdio::null());
            c.spawn()
        })
    };

    // No PowerShell/cmd off Windows: run through the POSIX shell, with the
    // same "stderr merged into the one stdout pipe" shape as above.
    #[cfg(not(target_os = "windows"))]
    let child = {
        use std::os::unix::process::CommandExt as _;
        let merged_cmd = format!("{{ {cmd}\n}} 2>&1");
        let mut c = gpui_util::new_std_command("sh");
        c.args(["-c", &merged_cmd])
            .current_dir(cwd)
            .envs(env.iter().map(|(name, value)| (name, value)))
            .env("PYTHONUNBUFFERED", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            // Its own process group, so Stop can kill everything the command
            // started (`kill_process_tree`), as `taskkill /T` does on Windows.
            .process_group(0);
        c.spawn()
    };

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.send(RunnerEvent::Line(format!("Failed to start shell: {e}")));
            let _ = tx.send(RunnerEvent::Done {
                exit_code: -1,
                was_killed: false,
            });
            return;
        }
    };

    let pid = child.id();
    let finished = Arc::new(AtomicBool::new(false));
    {
        let kill_flag = kill_flag.clone();
        let finished = finished.clone();
        std::thread::spawn(move || {
            while !finished.load(Ordering::SeqCst) {
                if kill_flag.load(Ordering::SeqCst) {
                    kill_process_tree(pid);
                    return;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }

    let stdout = match child.stdout.take() {
        Some(s) => s,
        None => {
            finished.store(true, Ordering::SeqCst);
            let _ = tx.send(RunnerEvent::Line("No stdout".to_string()));
            let _ = tx.send(RunnerEvent::Done {
                exit_code: -1,
                was_killed: false,
            });
            return;
        }
    };

    for line in BufReader::new(stdout).lines() {
        if kill_flag.load(Ordering::SeqCst) {
            break;
        }
        match line {
            Ok(l) => {
                if tx.send(RunnerEvent::Line(strip_ansi(&l))).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let was_killed = kill_flag.load(Ordering::SeqCst);
    if was_killed {
        kill_process_tree(pid);
    }
    let exit_code = child.wait().ok().and_then(|s| s.code()).unwrap_or(-1);
    finished.store(true, Ordering::SeqCst);
    let _ = tx.send(RunnerEvent::Done {
        exit_code,
        was_killed,
    });
}

impl Focusable for ScriptRunnerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for ScriptRunnerPanel {}

impl Panel for ScriptRunnerPanel {
    fn persistent_name() -> &'static str {
        "Script Runner Panel"
    }

    fn panel_key() -> &'static str {
        "ScriptRunnerPanel"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        ScriptRunnerPanelSettings::get_global(cx).dock.into()
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        settings::update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
            settings.script_runner_panel.get_or_insert_default().dock = Some(position.into());
        });
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<ui::IconName> {
        ScriptRunnerPanelSettings::get_global(cx).button.then_some(ui::IconName::Runner)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Script Runner")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        23
    }

    fn hide_button_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.script_runner_panel.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Render for ScriptRunnerPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Lazily create the command InputState (needs Window).
        if self.command_input.is_none() {
            self.command_input =
                Some(cx.new(|cx| InputState::new(window, cx).placeholder("Enter a command…")));
        }

        let command_val = self
            .command_input
            .as_ref()
            .map(|i| i.read(cx).value().to_string())
            .unwrap_or_default();
        let can_run = !command_val.trim().is_empty() && !self.running;

        // ── Header strip ──
        let header = h_flex()
            .gap_2()
            .items_center()
            .px_3()
            .py_1p5()
            .flex_shrink_0()
            .border_b_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(
                svg()
                    .path("icons/runner.svg")
                    .size(px(14.0))
                    .text_color(cx.theme().foreground),
            )
            .child(
                div()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_xs()
                    .text_color(cx.theme().foreground)
                    .child("Script Runner"),
            )
            .child(div().flex_1())
            // Status badge
            .child(
                h_flex()
                    .gap_1()
                    .items_center()
                    .when(self.running, |r| r.child(Spinner::new().xsmall()))
                    .child(
                        div()
                            .text_xs()
                            .text_color(if self.running {
                                cx.theme().primary
                            } else {
                                cx.theme().muted_foreground
                            })
                            .child(if self.running { "Running…" } else { "Idle" }),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!("{} lines", self.output.len())),
            );

        // ── Command entry row ──
        let command_row = h_flex()
            .gap_2()
            .items_center()
            .px_3()
            .py_1()
            .flex_shrink_0()
            .border_b_1()
            .border_color(cx.theme().border)
            .when_some(self.command_input.as_ref(), |row, input| {
                row.child(div().flex_1().min_w_0().child(Input::new(input).xsmall()))
            })
            .child(
                div()
                    .id("runner-run-btn")
                    .px_3()
                    .py_0p5()
                    .rounded_md()
                    .text_xs()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .cursor_pointer()
                    .bg(if can_run {
                        cx.theme().success
                    } else {
                        cx.theme().secondary
                    })
                    .text_color(if can_run {
                        cx.theme().background
                    } else {
                        cx.theme().muted_foreground
                    })
                    .child("Run")
                    .on_click(cx.listener(|this, _e, _w, cx| {
                        let cmd = this
                            .command_input
                            .as_ref()
                            .map(|i| i.read(cx).value().to_string())
                            .unwrap_or_default();
                        this.run(&cmd, cx);
                    })),
            )
            .when(self.running, |row| {
                row.child(
                    div()
                        .id("runner-stop-btn")
                        .px_3()
                        .py_0p5()
                        .rounded_md()
                        .text_xs()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .cursor_pointer()
                        .bg(cx.theme().danger)
                        .text_color(cx.theme().background)
                        .child("Stop")
                        .on_click(cx.listener(|this, _e, _w, _cx| {
                            this.stop();
                        })),
                )
            });

        // ── cwd strip ──
        let cwd_row = h_flex()
            .gap_1()
            .items_center()
            .px_3()
            .py_0p5()
            .flex_shrink_0()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("cwd"),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .font_family("Cascadia Mono")
                    .text_xs()
                    .text_color(cx.theme().foreground)
                    .child(self.cwd.clone()),
            );

        // ── SPFx debug-script bar (only when present) ──
        let debug_bar = self.debug_script.clone().map(|qs| {
            let display: String = if qs.chars().count() > 72 {
                qs.chars().take(72).collect::<String>() + "\u{2026}"
            } else {
                qs.clone()
            };
            let copied = self.debug_copied;
            let copy_color = if copied {
                cx.theme().success
            } else {
                cx.theme().muted_foreground
            };

            h_flex()
                .gap_2()
                .items_center()
                .px_3()
                .py_0p5()
                .flex_shrink_0()
                .border_b_1()
                .border_color(cx.theme().border)
                .child(
                    div()
                        .flex_shrink_0()
                        .text_xs()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(cx.theme().primary)
                        .child("Debug script"),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .font_family("Cascadia Mono")
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(display),
                )
                .child(
                    div()
                        .id("runner-debug-copy")
                        .flex()
                        .items_center()
                        .gap_1()
                        .flex_shrink_0()
                        .px_2()
                        .py_0p5()
                        .rounded_sm()
                        .cursor_pointer()
                        .text_color(copy_color)
                        .hover(|d| d.bg(cx.theme().list_hover).text_color(cx.theme().foreground))
                        .child(
                            svg()
                                .path(if copied { "icons/check.svg" } else { "icons/copy.svg" })
                                .size(px(12.0))
                                .text_color(copy_color),
                        )
                        .child(if copied { "Copied" } else { "Copy" })
                        .on_click(cx.listener(move |this, _e, _w, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(qs.clone()));
                            this.debug_copied = true;
                            cx.notify();
                            cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
                                cx.background_executor()
                                    .timer(Duration::from_millis(1600))
                                    .await;
                                let _ = cx.update(|app| {
                                    if let Some(panel) = this.upgrade() {
                                        let _ = panel.update(app, |panel, cx| {
                                            panel.debug_copied = false;
                                            cx.notify();
                                        });
                                    }
                                });
                            })
                            .detach();
                        })),
                )
        });

        // ── Detected-links bar (only when there's no SPFx debug script to
        // show instead — same either/or as the original Tauri/React version) ──
        let links_bar = (self.debug_script.is_none() && !self.detected_links.is_empty()).then(
            || {
                h_flex()
                    .gap_2()
                    .items_center()
                    .flex_wrap()
                    .px_3()
                    .py_0p5()
                    .flex_shrink_0()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        div()
                            .flex_shrink_0()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Links"),
                    )
                    .children(self.detected_links.iter().cloned().map(|url| {
                        let copied = self.link_copied.as_deref() == Some(url.as_str());
                        let copy_color = if copied {
                            cx.theme().success
                        } else {
                            cx.theme().muted_foreground
                        };

                        h_flex()
                            .gap_1()
                            .items_center()
                            .px_2()
                            .py_0p5()
                            .rounded_sm()
                            .bg(cx.theme().secondary)
                            .child(
                                div()
                                    .id(SharedString::from(format!("runner-open-link-{url}")))
                                    .cursor_pointer()
                                    .font_family("Cascadia Mono")
                                    .text_xs()
                                    .text_color(cx.theme().link)
                                    .hover(|d| d.underline())
                                    .child(url.clone())
                                    .on_click({
                                        let url = url.clone();
                                        move |_event, _window, cx: &mut gpui::App| {
                                            cx.open_url(&url)
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .id(SharedString::from(format!("runner-copy-link-{url}")))
                                    .cursor_pointer()
                                    .text_color(copy_color)
                                    .child(
                                        svg()
                                            .path(if copied {
                                                "icons/check.svg"
                                            } else {
                                                "icons/copy.svg"
                                            })
                                            .size(px(12.0))
                                            .text_color(copy_color),
                                    )
                                    .on_click(cx.listener({
                                        let url = url.clone();
                                        move |this, _e, _w, cx| {
                                            cx.write_to_clipboard(ClipboardItem::new_string(
                                                url.clone(),
                                            ));
                                            this.link_copied = Some(url.clone());
                                            cx.notify();
                                            let reset_url = url.clone();
                                            cx.spawn(async move |this: WeakEntity<Self>,
                                                                  cx: &mut AsyncApp| {
                                                cx.background_executor()
                                                    .timer(Duration::from_millis(1600))
                                                    .await;
                                                let _ = cx.update(|app| {
                                                    if let Some(panel) = this.upgrade() {
                                                        let _ = panel.update(app, |panel, cx| {
                                                            if panel.link_copied.as_deref()
                                                                == Some(reset_url.as_str())
                                                            {
                                                                panel.link_copied = None;
                                                            }
                                                            cx.notify();
                                                        });
                                                    }
                                                });
                                            })
                                            .detach();
                                        }
                                    })),
                            )
                    }))
            },
        );

        // ── Output area ──
        let output_area = div()
            .id("runner-output-scroll")
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_y_scroll()
            .bg(cx.theme().secondary)
            .px_3()
            .py_1()
            .children(
                self.output.iter().rev().take(200).rev().map(|line| {
                    render_output_line(line, cx.theme().muted_foreground, cx.theme().link)
                }),
            );

        v_flex()
            .id("script-runner-panel")
            .track_focus(&self.focus_handle(cx))
            .w_full()
            .h_full()
            .overflow_hidden()
            .bg(cx.theme().background)
            .border_t_1()
            .border_color(cx.theme().border)
            .child(header)
            .child(command_row)
            .child(cwd_row)
            .children(debug_bar)
            .children(links_bar)
            .child(output_area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_querystring_is_cut_at_whitespace_and_quotes() {
        let line = "To debug, append ?debug=true&noredir=true&debugManifestsFile=https://localhost:4321/temp/manifests.js to the URL";
        assert_eq!(
            debug_querystring_in(line).as_deref(),
            Some("?debug=true&noredir=true&debugManifestsFile=https://localhost:4321/temp/manifests.js")
        );
        assert_eq!(
            debug_querystring_in("href=\"/workbench.aspx?debug=true&x=1\" rel").as_deref(),
            Some("?debug=true&x=1")
        );
        assert_eq!(debug_querystring_in("ends with ?debug=true").as_deref(), Some("?debug=true"));
    }

    #[test]
    fn debug_querystring_absent() {
        assert_eq!(debug_querystring_in("Server started on port 4321"), None);
        assert_eq!(debug_querystring_in(""), None);
        assert_eq!(debug_querystring_in("https://example.test/?page=2"), None);
    }

    #[test]
    fn strip_ansi_removes_color_codes() {
        assert_eq!(strip_ansi("\u{1b}[32m\u{1b}[1mVITE\u{1b}[22m v5.0.0\u{1b}[39m"), "VITE v5.0.0");
        assert_eq!(strip_ansi("\u{1b}[2K\u{1b}[1Gprogress 50%"), "progress 50%");
        assert_eq!(strip_ansi("plain text"), "plain text");
        assert_eq!(strip_ansi(""), "");
    }

    #[test]
    fn strip_ansi_removes_osc_sequences() {
        // Hyperlink terminated by BEL.
        assert_eq!(
            strip_ansi("\u{1b}]8;;https://example.test\u{7}link\u{1b}]8;;\u{7} done"),
            "link done"
        );
        // Window title terminated by ST (ESC \).
        assert_eq!(strip_ansi("\u{1b}]0;my title\u{1b}\\ready"), "ready");
    }

    #[test]
    fn strip_ansi_tolerates_malformed_sequences() {
        // An unknown escape kind drops only the ESC byte.
        assert_eq!(strip_ansi("a\u{1b}Zb"), "aZb");
        // Unterminated sequences run to the end of the line without panicking.
        assert_eq!(strip_ansi("x\u{1b}[31"), "x");
        assert_eq!(strip_ansi("y\u{1b}]0;never closed"), "y");
        assert_eq!(strip_ansi("z\u{1b}"), "z");
    }

    #[test]
    fn strip_ansi_keeps_non_ascii_text() {
        assert_eq!(strip_ansi("\u{1b}[32m✓\u{1b}[0m héllo → done"), "✓ héllo → done");
    }

    #[test]
    fn urls_are_found_and_trailing_punctuation_trimmed() {
        assert_eq!(
            find_urls("  Local: http://localhost:5173/, Network: https://10.0.0.2:5173/."),
            vec!["http://localhost:5173/", "https://10.0.0.2:5173/"]
        );
        assert_eq!(find_urls("see https://example.test/docs!"), vec!["https://example.test/docs"]);
        assert_eq!(
            find_urls("<a href=\"https://example.test/x?y=1\">x</a>"),
            vec!["https://example.test/x?y=1"]
        );
    }

    #[test]
    fn url_ranges_are_byte_offsets_into_the_line() {
        assert_eq!(url_ranges("go https://a.io now"), vec![(3, 15)]);

        // Offsets stay valid slice boundaries after multi-byte characters.
        let line = "→ ready at https://a.io/é and http://b.io";
        let urls: Vec<&str> = url_ranges(line).into_iter().map(|(s, e)| &line[s..e]).collect();
        assert_eq!(urls, vec!["https://a.io/é", "http://b.io"]);
    }

    #[test]
    fn lines_without_urls() {
        assert!(find_urls("no links here").is_empty());
        assert!(find_urls("").is_empty());
        assert!(find_urls("ftp://example.test/file").is_empty());
        assert!(url_ranges("httpx://nope").is_empty());
    }
}
