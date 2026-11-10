#![allow(
    clippy::disallowed_methods,
    reason = "commands run and are waited on from dedicated threads, never the UI thread"
)]

use std::collections::VecDeque;
use std::io::Read;
use std::os::unix::process::ExitStatusExt as _;
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use futures::StreamExt as _;
use futures::channel::mpsc::{UnboundedSender, unbounded};
use gpui::{
    App, Context, ElementId, Entity, FontWeight, Hsla, SharedString, Stateful, Task, Window,
    anchored, deferred, div, prelude::*, px, rgb, rgba, white,
};
use sandbox_linux::{
    Access, AccessRequest, AddressClass, Decision, Denial, FsRule, Handler, NetHost, NetRule,
    OpenFile, Permission, Protocol, RequestedAccess, Responder, SandboxContext, SandboxPolicy,
    UntrustedCommand,
};

/// How long a stopped command gets to exit before it's killed outright.
const STOP_GRACE_PERIOD: Duration = Duration::from_secs(2);

use crate::text_input::TextInput;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GrantMode {
    ReadOnly,
    ReadWrite,
    AskWrite,
    AskReadWrite,
    DenyWrite,
    DenyRead,
    DenyAll,
}

impl GrantMode {
    const ALL: [GrantMode; 7] = [
        GrantMode::ReadOnly,
        GrantMode::ReadWrite,
        GrantMode::AskWrite,
        GrantMode::AskReadWrite,
        GrantMode::DenyWrite,
        GrantMode::DenyRead,
        GrantMode::DenyAll,
    ];

    fn label(self) -> &'static str {
        match self {
            GrantMode::ReadOnly => "read-only",
            GrantMode::ReadWrite => "read & write",
            GrantMode::AskWrite => "read, ask to write",
            GrantMode::AskReadWrite => "ask to read & write",
            GrantMode::DenyWrite => "deny writes",
            GrantMode::DenyRead => "deny reads",
            GrantMode::DenyAll => "deny all",
        }
    }

    /// `(read, write)`. `None` leaves that permission to earlier grants.
    /// `handler` decides `Ask`s.
    fn access(self, handler: &Handler) -> (Option<Access>, Option<Access>) {
        use Access::*;
        let ask = || Some(Ask(handler.clone()));
        match self {
            GrantMode::ReadOnly => (Some(Allow), Some(Deny)),
            GrantMode::ReadWrite => (Some(Allow), Some(Allow)),
            GrantMode::AskWrite => (Some(Allow), ask()),
            GrantMode::AskReadWrite => (ask(), ask()),
            GrantMode::DenyWrite => (None, Some(Deny)),
            GrantMode::DenyRead => (Some(Deny), None),
            GrantMode::DenyAll => (Some(Deny), Some(Deny)),
        }
    }
}

struct Grant {
    id: usize,
    mode: GrantMode,
    path: Entity<TextInput>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NetMode {
    Allow,
    Ask,
    Deny,
}

impl NetMode {
    const ALL: [NetMode; 3] = [NetMode::Allow, NetMode::Ask, NetMode::Deny];

    fn label(self) -> &'static str {
        match self {
            NetMode::Allow => "allow",
            NetMode::Ask => "ask",
            NetMode::Deny => "deny",
        }
    }

    fn access(self, handler: &Handler) -> Access {
        match self {
            NetMode::Allow => Access::Allow,
            NetMode::Ask => Access::Ask(handler.clone()),
            NetMode::Deny => Access::Deny,
        }
    }
}

struct NetGrant {
    /// Shares the numbering of [`Grant::id`], so one `open_menu` serves both.
    id: usize,
    mode: NetMode,
    destination: Entity<TextInput>,
}

enum Status {
    Idle,
    Running,
    Allowed(String),
    Denied(String),
    Stopped(String),
    Error(String),
}

enum RunEvent {
    Prompt(AccessRequest, Responder),
    Denied(Denial),
    Stdout(String),
    Stderr(String),
    PolicyApplied(Result<Vec<OpenFile>, String>),
    Exited(Result<ExitStatus, String>),
}

/// How the UI steers a running command. The sandbox itself lives on the run
/// thread, so updates are sent there.
struct RunControl {
    stop: Arc<AtomicBool>,
    policy_updates: mpsc::Sender<SandboxPolicy>,
    /// Decides this run's `Ask` grants, including ones added by Apply.
    handler: Handler,
}

enum ApplyMessage {
    Applied(String),
    Failed(String),
}

pub struct Tester {
    context: Result<Arc<SandboxContext>, String>,
    command: Entity<TextInput>,
    grants: Vec<Grant>,
    net_grants: Vec<NetGrant>,
    next_grant: usize,
    /// The grant whose mode dropdown is open.
    open_menu: Option<usize>,
    stdout: String,
    stderr: String,
    status: Status,
    /// Requests waiting for an answer, oldest first. Each must be answered
    /// through its responder (dropping one is only fine once cancelled).
    prompts: VecDeque<(AccessRequest, Responder)>,
    denials: Vec<Denial>,
    /// Present while a command runs.
    control: Option<RunControl>,
    stop_requested: bool,
    apply_message: Option<ApplyMessage>,
    _run_task: Option<Task<()>>,
}

impl Tester {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let context = SandboxContext::new()
            .map(Arc::new)
            .map_err(|error| format!("{error:#}"));
        let command =
            cx.new(|cx| TextInput::new("command (run with sh -c)", "cat /etc/hostname", cx));
        let mut tester = Self {
            context,
            command,
            grants: Vec::new(),
            net_grants: Vec::new(),
            next_grant: 0,
            open_menu: None,
            stdout: String::new(),
            stderr: String::new(),
            status: Status::Idle,
            prompts: VecDeque::new(),
            denials: Vec::new(),
            control: None,
            stop_requested: false,
            apply_message: None,
            _run_task: None,
        };
        tester.add_grant(GrantMode::ReadOnly, "/", cx);
        tester
    }

    fn add_grant(&mut self, mode: GrantMode, path: &str, cx: &mut Context<Self>) {
        let path = path.to_string();
        let path = cx.new(|cx| TextInput::new("/path, or a glob like /**/*.env", path, cx));
        self.grants.push(Grant {
            id: self.next_grant,
            mode,
            path,
        });
        self.next_grant += 1;
        cx.notify();
    }

    fn add_net_grant(&mut self, mode: NetMode, destination: &str, cx: &mut Context<Self>) {
        let destination = destination.to_string();
        let destination = cx.new(|cx| {
            TextInput::new(
                "[tcp/|udp/]host, *.domain, 10.0.0.0/8, loopback, local or *[:ports]",
                destination,
                cx,
            )
        });
        self.net_grants.push(NetGrant {
            id: self.next_grant,
            mode,
            destination,
        });
        self.next_grant += 1;
        cx.notify();
    }

    fn policy(&self, handler: &Handler, cx: &App) -> Result<SandboxPolicy, String> {
        let mut rules = Vec::new();
        for grant in &self.grants {
            let path = grant.path.read(cx).text().trim();
            if path.is_empty() {
                continue;
            }
            let mut rule =
                FsRule::new(path).map_err(|error| format!("invalid grant `{path}`: {error:#}"))?;
            (rule.read, rule.write) = grant.mode.access(handler);
            rules.push(rule);
        }
        let mut net = Vec::new();
        for grant in &self.net_grants {
            let destination = grant.destination.read(cx).text().trim();
            if destination.is_empty() {
                continue;
            }
            net.push(
                NetRule::parse(destination, grant.mode.access(handler))
                    .map_err(|error| format!("invalid network grant `{destination}`: {error:#}"))?,
            );
        }
        Ok(SandboxPolicy { fs: rules, net })
    }

    fn run(&mut self, cx: &mut Context<Self>) {
        if self.control.is_some() {
            return;
        }
        let context = match &self.context {
            Ok(context) => context.clone(),
            Err(error) => {
                self.status =
                    Status::Error(format!("the sandbox supervisor failed to start: {error}"));
                cx.notify();
                return;
            }
        };
        let (sender, mut receiver) = unbounded();
        let handler = prompt_handler(sender.clone());
        let policy = match self.policy(&handler, cx) {
            Ok(policy) => policy,
            Err(error) => {
                self.status = Status::Error(error);
                cx.notify();
                return;
            }
        };
        let script = self.command.read(cx).text().to_string();

        self.stdout.clear();
        self.stderr.clear();
        self.denials.clear();
        self.deny_pending_prompts();
        self.open_menu = None;
        self.apply_message = None;
        self.stop_requested = false;

        let stop = Arc::new(AtomicBool::new(false));
        let (policy_updates, policy_receiver) = mpsc::channel();
        let spawned = std::thread::Builder::new()
            .name("sandbox-run".to_string())
            .spawn({
                let stop = stop.clone();
                move || run_command(&context, policy, script, &stop, &policy_receiver, sender)
            });
        if let Err(error) = spawned {
            self.status = Status::Error(format!("starting the run thread: {error}"));
            cx.notify();
            return;
        }
        self.control = Some(RunControl {
            stop,
            policy_updates,
            handler,
        });
        self.status = Status::Running;
        self._run_task = Some(cx.spawn(async move |this, cx| {
            while let Some(event) = receiver.next().await {
                if this
                    .update(cx, |this, cx| this.handle_event(event, cx))
                    .is_err()
                {
                    break;
                }
            }
        }));
        cx.notify();
    }

    fn stop(&mut self, cx: &mut Context<Self>) {
        if let Some(control) = &self.control {
            control.stop.store(true, Ordering::SeqCst);
            self.stop_requested = true;
            cx.notify();
        }
    }

    /// Replace the running sandbox's policy with the current grants.
    fn apply(&mut self, cx: &mut Context<Self>) {
        let Some(control) = &self.control else {
            return;
        };
        self.apply_message = match self.policy(&control.handler, cx) {
            Ok(policy) => match control.policy_updates.send(policy) {
                Ok(()) => Some(ApplyMessage::Applied("applying…".to_string())),
                Err(_) => Some(ApplyMessage::Failed(
                    "the command already exited".to_string(),
                )),
            },
            Err(error) => Some(ApplyMessage::Failed(error)),
        };
        cx.notify();
    }

    fn handle_event(&mut self, event: RunEvent, cx: &mut Context<Self>) {
        match event {
            RunEvent::Stdout(text) => self.stdout.push_str(&text),
            RunEvent::Stderr(text) => self.stderr.push_str(&text),
            RunEvent::Prompt(request, responder) => self.prompts.push_back((request, responder)),
            RunEvent::Denied(denial) => self.denials.push(denial),
            RunEvent::PolicyApplied(Ok(still_open)) => {
                self.apply_message = Some(ApplyMessage::Applied(describe_still_open(&still_open)))
            }
            RunEvent::PolicyApplied(Err(error)) => {
                self.apply_message = Some(ApplyMessage::Failed(error))
            }
            RunEvent::Exited(result) => {
                self.deny_pending_prompts();
                self.status = self.final_status(result);
                self.control = None;
            }
        }
        self.forget_cancelled_prompts();
        cx.notify();
    }

    /// Requests whose syscall was abandoned (or timed out) need no answer.
    fn forget_cancelled_prompts(&mut self) {
        self.prompts
            .retain(|(_, responder)| !responder.is_cancelled());
    }

    fn deny_pending_prompts(&mut self) {
        for (_, responder) in self.prompts.drain(..) {
            responder.respond(Decision::deny());
        }
    }

    fn final_status(&self, result: Result<ExitStatus, String>) -> Status {
        let status = match result {
            Ok(status) => status,
            Err(error) => return Status::Error(error),
        };
        let exit = match status.code() {
            Some(code) => format!("exit code: {code}"),
            None => format!("killed by signal {}", status.signal().unwrap_or(0)),
        };
        if self.stop_requested {
            return Status::Stopped(format!("stopped ({exit})"));
        }
        // The helper's own failures (before the command runs) exit with 126.
        if status.code() == Some(126) && self.stderr.contains("zed sandbox:") {
            return Status::Error(format!("sandbox setup failed ({exit}), see stderr"));
        }
        match self.denials.split_first() {
            None => Status::Allowed(format!("allowed ({exit})")),
            Some((first, rest)) => {
                let more = if rest.is_empty() {
                    String::new()
                } else {
                    format!(", and {} more", rest.len())
                };
                Status::Denied(format!(
                    "denied (reason: {}{more}; {exit})",
                    describe(first)
                ))
            }
        }
    }

    fn answer_prompt(&mut self, allow: bool, cx: &mut Context<Self>) {
        self.forget_cancelled_prompts();
        if let Some((_, responder)) = self.prompts.pop_front() {
            responder.respond(if allow {
                Decision::allow()
            } else {
                Decision::deny()
            });
        }
        cx.notify();
    }

    fn render_grant(&self, grant: &Grant, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let id = grant.id;
        let menu_open = self.open_menu == Some(id);
        let menu = div()
            .id(("mode-menu", id))
            .occlude()
            .flex()
            .flex_col()
            .min_w(px(180.))
            .py_1()
            .bg(white())
            .border_1()
            .border_color(rgb(0xbbbbbb))
            .rounded_sm()
            .shadow_md()
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.open_menu = None;
                cx.notify();
            }))
            .children(GrantMode::ALL.into_iter().enumerate().map(|(index, mode)| {
                div()
                    .id(("mode-option", index))
                    .px_2()
                    .py_1()
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0xe8eefc)))
                    .when(mode == grant.mode, |this| {
                        this.font_weight(FontWeight::BOLD)
                    })
                    .child(mode.label())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(grant) = this.grants.iter_mut().find(|grant| grant.id == id) {
                            grant.mode = mode;
                        }
                        this.open_menu = None;
                        cx.notify();
                    }))
            }));

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(180.))
                    .child(
                        button(("mode", id), format!("{} ▾", grant.mode.label()), true).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.open_menu = Some(id);
                                cx.notify();
                            }),
                        ),
                    )
                    .when(menu_open, |this| {
                        this.child(deferred(anchored().child(menu)))
                    }),
            )
            .child(grant.path.clone())
            .child(button(("delete", id), "✕", true).on_click(cx.listener(
                move |this, _, _, cx| {
                    this.grants.retain(|grant| grant.id != id);
                    if this.open_menu == Some(id) {
                        this.open_menu = None;
                    }
                    cx.notify();
                },
            )))
    }

    fn render_net_grant(
        &self,
        grant: &NetGrant,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let id = grant.id;
        let menu_open = self.open_menu == Some(id);
        let menu = div()
            .id(("net-mode-menu", id))
            .occlude()
            .flex()
            .flex_col()
            .min_w(px(180.))
            .py_1()
            .bg(white())
            .border_1()
            .border_color(rgb(0xbbbbbb))
            .rounded_sm()
            .shadow_md()
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.open_menu = None;
                cx.notify();
            }))
            .children(NetMode::ALL.into_iter().enumerate().map(|(index, mode)| {
                div()
                    .id(("net-mode-option", index))
                    .px_2()
                    .py_1()
                    .cursor_pointer()
                    .hover(|style| style.bg(rgb(0xe8eefc)))
                    .when(mode == grant.mode, |this| {
                        this.font_weight(FontWeight::BOLD)
                    })
                    .child(mode.label())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(grant) = this.net_grants.iter_mut().find(|grant| grant.id == id)
                        {
                            grant.mode = mode;
                        }
                        this.open_menu = None;
                        cx.notify();
                    }))
            }));

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(180.))
                    .child(
                        button(("net-mode", id), format!("{} ▾", grant.mode.label()), true)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.open_menu = Some(id);
                                cx.notify();
                            })),
                    )
                    .when(menu_open, |this| {
                        this.child(deferred(anchored().child(menu)))
                    }),
            )
            .child(grant.destination.clone())
            .child(button(("net-delete", id), "✕", true).on_click(cx.listener(
                move |this, _, _, cx| {
                    this.net_grants.retain(|grant| grant.id != id);
                    if this.open_menu == Some(id) {
                        this.open_menu = None;
                    }
                    cx.notify();
                },
            )))
    }

    fn render_prompt(
        &self,
        prompt: &AccessRequest,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let executable = prompt.executable.as_ref().map_or_else(
            || "a process".to_string(),
            |path| path.display().to_string(),
        );
        let waiting = self.prompts.len().saturating_sub(1);
        div()
            .id("prompt-overlay")
            .absolute()
            .inset_0()
            .occlude()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x0000_0066))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_4()
                    .max_w(px(560.))
                    .bg(white())
                    .rounded_md()
                    .shadow_lg()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .child("Sandbox permission request"),
                    )
                    .child(format!("{executable} (pid {}) wants to:", prompt.pid))
                    .children(
                        prompt
                            .items
                            .iter()
                            .map(|item| div().pl_2().child(format!("• {}", describe_item(item)))),
                    )
                    .when(waiting > 0, |this| {
                        this.child(
                            div()
                                .text_color(rgb(0x777777))
                                .child(format!("{waiting} more waiting")),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_end()
                            .gap_2()
                            .child(button("prompt-deny", "Deny", true).on_click(
                                cx.listener(|this, _, _, cx| this.answer_prompt(false, cx)),
                            ))
                            .child(button("prompt-allow", "Allow", true).on_click(
                                cx.listener(|this, _, _, cx| this.answer_prompt(true, cx)),
                            )),
                    ),
            )
    }
}

impl Render for Tester {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (status_text, status_color): (SharedString, Hsla) = match &self.status {
            Status::Idle => ("not run yet".into(), rgb(0x777777).into()),
            Status::Running => ("running…".into(), rgb(0x2255aa).into()),
            Status::Allowed(text) => (text.clone().into(), rgb(0x1a7f37).into()),
            Status::Denied(text) => (text.clone().into(), rgb(0xcf222e).into()),
            Status::Stopped(text) => (text.clone().into(), rgb(0x777777).into()),
            Status::Error(text) => (format!("error: {text}").into(), rgb(0x9a6700).into()),
        };

        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .gap_3()
            .p_3()
            .bg(rgb(0xf6f6f6))
            .text_color(rgb(0x1f2328))
            .text_size(px(14.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(div().w(px(80.)).child("Command"))
                    .child(self.command.clone())
                    .child(if self.control.is_some() {
                        button("stop", "Stop", !self.stop_requested)
                            .on_click(cx.listener(|this, _, _, cx| this.stop(cx)))
                    } else {
                        button("run", "Run", true)
                            .on_click(cx.listener(|this, _, _, cx| this.run(cx)))
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .child("Grants (applied in order, last match wins)"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .when_some(self.apply_message.as_ref(), |this, message| {
                                let (text, color) = match message {
                                    ApplyMessage::Applied(text) => (text.clone(), rgb(0x1a7f37)),
                                    ApplyMessage::Failed(text) => {
                                        (format!("apply failed: {text}"), rgb(0xcf222e))
                                    }
                                };
                                this.child(div().text_color(color).child(text))
                            })
                            .when(self.control.is_some(), |this| {
                                this.child(
                                    button("apply", "Apply", true)
                                        .on_click(cx.listener(|this, _, _, cx| this.apply(cx))),
                                )
                            })
                            .child(button("add-grant", "Add grant", true).on_click(cx.listener(
                                |this, _, _, cx| this.add_grant(GrantMode::ReadWrite, "", cx),
                            ))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(self.grants.iter().map(|grant| self.render_grant(grant, cx))),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .child("Network grants (applied in order, last match wins)"),
                    )
                    .child(button("add-net-grant", "Add network grant", true).on_click(
                        cx.listener(|this, _, _, cx| this.add_net_grant(NetMode::Allow, "", cx)),
                    )),
            )
            .child(
                div().flex().flex_col().gap_1().children(
                    self.net_grants
                        .iter()
                        .map(|grant| self.render_net_grant(grant, cx)),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(div().font_weight(FontWeight::BOLD).child("Status:"))
                    .child(div().text_color(status_color).child(status_text)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h(px(0.))
                    .gap_2()
                    .child(output_pane("stdout", self.stdout.clone()))
                    .child(output_pane("stderr", self.stderr.clone())),
            )
            .when_some(self.prompts.front(), |this, (request, _)| {
                this.child(self.render_prompt(request, cx))
            })
    }
}

fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    enabled: bool,
) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .px_3()
        .py_1()
        .border_1()
        .border_color(rgb(0xbbbbbb))
        .rounded_sm()
        .bg(white())
        .child(label.into())
        .when(enabled, |this| {
            this.cursor_pointer().hover(|style| style.bg(rgb(0xe8eefc)))
        })
        .when(!enabled, |this| this.text_color(rgb(0xaaaaaa)))
}

fn output_pane(title: &'static str, text: String) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w(px(0.))
        .gap_1()
        .child(div().font_weight(FontWeight::BOLD).child(title))
        .child(
            div()
                .id(title)
                .flex_1()
                .overflow_y_scroll()
                .p_2()
                .bg(white())
                .border_1()
                .border_color(rgb(0xbbbbbb))
                .rounded_sm()
                .text_size(px(12.))
                .child(text),
        )
}

fn verb(permission: Permission) -> &'static str {
    match permission {
        Permission::Read => "read",
        Permission::Write => "write",
    }
}

fn describe_still_open(still_open: &[OpenFile]) -> String {
    if still_open.is_empty() {
        return "applied".to_string();
    }
    let files = still_open
        .iter()
        .map(|file| {
            let access = match (file.read, file.write) {
                (true, true) => "read & write",
                (true, false) => "read",
                _ => "write",
            };
            format!("{} ({access}, pid {})", file.path.display(), file.pid)
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("applied, but still open under the old grants: {files}")
}

fn describe_item(item: &RequestedAccess) -> String {
    match item {
        RequestedAccess::Fs { permission, path } => {
            format!("{} {}", verb(*permission), path.display())
        }
        RequestedAccess::Net {
            host,
            port,
            protocol,
        } => {
            let protocol = match protocol {
                Protocol::Tcp => "TCP",
                Protocol::Udp => "UDP",
            };
            let host = match host {
                NetHost::Domain(name) => name.clone(),
                NetHost::Ip(address) => match AddressClass::of(*address) {
                    AddressClass::Loopback => format!("{address} (this machine)"),
                    AddressClass::LocalNetwork => format!("{address} (local network)"),
                    AddressClass::Public => address.to_string(),
                },
            };
            format!("connect over {protocol} to {host} port {port}")
        }
    }
}

/// Forwards this run's `Ask` requests to the UI.
fn prompt_handler(sender: UnboundedSender<RunEvent>) -> Handler {
    Handler::new(move |request, responder| {
        // The UI went away: nobody can answer, so deny.
        if let Err(error) = sender.unbounded_send(RunEvent::Prompt(request, responder))
            && let RunEvent::Prompt(_, responder) = error.into_inner()
        {
            responder.respond(Decision::deny());
        }
    })
}

fn describe(denial: &Denial) -> String {
    let requests = denial
        .requests
        .iter()
        .map(describe_item)
        .collect::<Vec<_>>()
        .join(", ");
    match denial.executable.as_ref().and_then(|path| path.file_name()) {
        Some(name) => format!("{requests} by {}", name.to_string_lossy()),
        None => requests,
    }
}

/// Runs on its own thread: create a sandbox, run the command in it, and
/// stream everything back to the UI.
fn run_command(
    context: &SandboxContext,
    policy: SandboxPolicy,
    script: String,
    stop: &AtomicBool,
    policy_updates: &mpsc::Receiver<SandboxPolicy>,
    sender: UnboundedSender<RunEvent>,
) {
    let result = (|| {
        let sandbox = context
            .create_sandbox(policy)
            .map_err(|error| format!("creating the sandbox: {error:#}"))?;
        sandbox.on_denial({
            let sender = sender.clone();
            move |denial| {
                sender.unbounded_send(RunEvent::Denied(denial)).ok();
            }
        });
        let cwd = std::env::current_dir()
            .map_err(|error| format!("reading the working directory: {error}"))?;
        let mut prepared = sandbox
            .prepare(&UntrustedCommand {
                cmd: "sh".to_string(),
                args: vec!["-c".to_string(), script],
                env: std::env::vars().collect(),
                cwd,
            })
            .map_err(|error| format!("preparing the command: {error:#}"))?;
        prepared
            .command_mut()
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = prepared
            .spawn()
            .map_err(|error| format!("starting the command: {error}"))?;

        let mut readers = Vec::new();
        if let Some(stdout) = child.stdout.take() {
            readers.push(forward(stdout, sender.clone(), RunEvent::Stdout));
        }
        if let Some(stderr) = child.stderr.take() {
            readers.push(forward(stderr, sender.clone(), RunEvent::Stderr));
        }
        let mut stop_sent_at: Option<Instant> = None;
        let status = loop {
            if let Some(status) = child
                .try_wait()
                .map_err(|error| format!("waiting for the command: {error}"))?
            {
                break status;
            }
            while let Ok(policy) = policy_updates.try_recv() {
                let result = sandbox
                    .update_policy(&policy)
                    .map_err(|error| format!("{error:#}"));
                sender.unbounded_send(RunEvent::PolicyApplied(result)).ok();
            }
            if stop.load(Ordering::SeqCst) {
                match stop_sent_at {
                    // The helper stops the command and everything it spawned.
                    // The child isn't reaped yet, so its pid can't be reused.
                    None => {
                        // SAFETY: plain kill.
                        unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM) };
                        stop_sent_at = Some(Instant::now());
                    }
                    Some(sent_at) if sent_at.elapsed() > STOP_GRACE_PERIOD => {
                        if let Err(error) = child.kill() {
                            eprintln!("killing the sandbox helper failed: {error}");
                        }
                    }
                    Some(_) => {}
                }
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        for reader in readers.into_iter().flatten() {
            reader.join().ok();
        }
        // Destroying the sandbox round-trips through the supervisor's
        // control channel, so every denial reported before the command
        // exited has reached the UI by the time `Exited` does.
        drop(sandbox);
        Ok(status)
    })();
    sender.unbounded_send(RunEvent::Exited(result)).ok();
}

fn forward(
    mut reader: impl Read + Send + 'static,
    sender: UnboundedSender<RunEvent>,
    wrap: fn(String) -> RunEvent,
) -> Option<std::thread::JoinHandle<()>> {
    let spawned = std::thread::Builder::new()
        .name("sandbox-output".to_string())
        .spawn(move || {
            let mut buffer = [0u8; 8192];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(length) => {
                        let text = String::from_utf8_lossy(&buffer[..length]).into_owned();
                        if sender.unbounded_send(wrap(text)).is_err() {
                            break;
                        }
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(error) => {
                        eprintln!("reading command output failed: {error}");
                        break;
                    }
                }
            }
        });
    match spawned {
        Ok(handle) => Some(handle),
        Err(error) => {
            eprintln!("starting an output reader failed: {error}");
            None
        }
    }
}
