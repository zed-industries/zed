//! The process list the Node and Python panels show in their Processes
//! section.

use std::path::PathBuf;
use std::time::Duration;

use gpui::{
    AnyElement, AppContext as _, AsyncApp, Context, FontWeight, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, StatefulInteractiveElement as _, Styled as _, Task,
    WeakEntity, Window, div, prelude::FluentBuilder as _,
};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    button::{Button, ButtonCustomVariant, ButtonVariants as _},
    h_flex,
    spinner::Spinner,
    tag::Tag,
    tooltip::Tooltip,
};
use sysinfo::System;

use crate::{RunningProcess, Runtime, scan, stop_tree};

/// How often the list refreshes while it is on screen. It doesn't refresh at
/// all otherwise (see [`ProcessList::set_polling`]).
const TICK: Duration = Duration::from_secs(3);

/// One runtime's running processes: the workspace's own first, everything
/// else (other projects', the IDE's own language servers) folded under
/// "Other". Each one says what it is and can be stopped, with everything it
/// started, except the IDE's own.
pub struct ProcessList {
    runtime: Runtime,
    /// The workspace folder that decides which processes are "this
    /// workspace's".
    root: Option<PathBuf>,
    processes: Vec<RunningProcess>,
    loaded: bool,
    show_other: bool,
    /// The process whose Stop is waiting for confirmation, by pid and start
    /// time.
    confirm_stop: Option<(u32, u64)>,
    /// The process being stopped right now.
    stopping: Option<u32>,
    /// What the last Stop came to.
    notice: Option<Notice>,
    poll: Option<Task<()>>,
}

struct Notice {
    text: String,
    failed: bool,
}

impl ProcessList {
    /// A list of `runtime`'s processes, placed against the workspace at
    /// `root`. It doesn't scan until [`Self::set_polling`] turns it on.
    pub fn new(runtime: Runtime, root: Option<PathBuf>) -> Self {
        Self {
            runtime,
            root,
            processes: Vec::new(),
            loaded: false,
            show_other: false,
            confirm_stop: None,
            stopping: None,
            notice: None,
            poll: None,
        }
    }

    /// How many processes are listed, the folded ones included.
    pub fn len(&self) -> usize {
        self.processes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    /// Places the processes against another workspace folder: a project
    /// opened after the panel was. Rescans at once when the list is running.
    pub fn set_root(&mut self, root: Option<PathBuf>, cx: &mut Context<Self>) {
        if self.root == root {
            return;
        }
        self.root = root;
        if self.poll.is_some() {
            self.loaded = false;
            self.poll = Some(self.spawn_poll(cx));
        }
        cx.notify();
    }

    /// Runs the scans while the list is on screen and only then: the panel
    /// shown and its Processes section open. Turning it on scans at once, so
    /// the list is current as soon as it appears; turning it off ends the
    /// scans. Nothing else reads the list, so nothing misses an update.
    pub fn set_polling(&mut self, on: bool, cx: &mut Context<Self>) {
        if !on {
            self.poll = None;
        } else if self.poll.is_none() {
            self.poll = Some(self.spawn_poll(cx));
        }
    }

    /// Every scan walks the whole process table, which takes long enough to
    /// stall the UI, so it runs in the background and only the rows come
    /// back to the main thread.
    fn spawn_poll(&self, cx: &mut Context<Self>) -> Task<()> {
        let runtime = self.runtime;
        let root = self.root.clone();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut sys = cx.background_spawn(async { System::new() }).await;
            loop {
                let root = root.clone();
                let (scanned, processes) = cx
                    .background_spawn(async move {
                        let processes = scan(&mut sys, runtime, root.as_deref());
                        (sys, processes)
                    })
                    .await;
                sys = scanned;

                let alive = this
                    .update(cx, |this, cx| {
                        // A confirmation for a process that has since exited
                        // has nothing left to confirm.
                        if let Some((pid, start_time)) = this.confirm_stop
                            && !processes
                                .iter()
                                .any(|proc| proc.pid == pid && proc.start_time == start_time)
                        {
                            this.confirm_stop = None;
                        }
                        this.processes = processes;
                        this.loaded = true;
                        cx.notify();
                    })
                    .is_ok();
                if !alive {
                    break;
                }

                cx.background_executor().timer(TICK).await;
            }
        })
    }

    /// Stops the process and everything it started, once the user has
    /// confirmed it, and says how that went.
    fn stop(&mut self, pid: u32, start_time: u64, cx: &mut Context<Self>) {
        self.confirm_stop = None;
        self.stopping = Some(pid);
        self.notice = None;
        cx.notify();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { stop_tree(pid, start_time) })
                .await;
            this.update(cx, |this, cx| {
                this.stopping = None;
                if result.is_ok() {
                    // Gone now, not only on the next scan.
                    this.processes.retain(|proc| proc.pid != pid);
                }
                this.notice = Some(match result {
                    Ok(text) => Notice {
                        text,
                        failed: false,
                    },
                    Err(text) => Notice { text, failed: true },
                });
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn render_notice(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let notice = self.notice.as_ref()?;
        let color = if notice.failed {
            cx.theme().danger
        } else {
            cx.theme().success
        };
        Some(
            h_flex()
                .gap_1()
                .px_3()
                .py_0p5()
                .text_xs()
                .text_color(color)
                .child(div().flex_1().child(notice.text.clone()))
                .child(
                    Button::new("runtime-procs-notice-close")
                        .ghost()
                        .xsmall()
                        .icon(IconName::Close)
                        .tooltip("Dismiss")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.notice = None;
                            cx.notify();
                        })),
                )
                .into_any_element(),
        )
    }

    /// One process: what it is (full command line on hover), its id, CPU and
    /// memory, and Stop. Stop asks first, in the row itself. The IDE's own
    /// processes have no Stop: ending a language server breaks an editor
    /// feature, and the IDE would only start it again.
    fn render_row(&self, proc: &RunningProcess, cx: &Context<Self>) -> AnyElement {
        let pid = proc.pid;
        let start_time = proc.start_time;
        let mono = |text: String| div().font_family("Cascadia Mono").child(text);
        let command_line = proc.command_line.clone();

        let what = h_flex()
            .id(("runtime-proc-what", pid as usize))
            .flex_1()
            .min_w_0()
            .gap_1()
            .child(mono(proc.label.clone()).truncate())
            .when(proc.ide_owned, |el| {
                el.child(
                    Tag::custom(
                        cx.theme().info.opacity(0.2),
                        cx.theme().info,
                        cx.theme().info.opacity(0.3),
                    )
                    .xsmall()
                    .child("IDE"),
                )
            })
            .when(!command_line.is_empty(), |el| {
                el.tooltip(move |window, cx| Tooltip::new(command_line.clone()).build(window, cx))
            });

        let action: AnyElement = if proc.ide_owned {
            div().w_6().into_any_element()
        } else if self.stopping == Some(pid) {
            div()
                .w_6()
                .flex()
                .justify_center()
                .child(Spinner::new().xsmall())
                .into_any_element()
        } else if self.confirm_stop == Some((pid, start_time)) {
            h_flex()
                .gap_1()
                .child(
                    Button::new(("runtime-proc-stop-confirm", pid as usize))
                        .danger()
                        .xsmall()
                        .label("Stop")
                        .tooltip("Stop this process and everything it started")
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.stop(pid, start_time, cx)),
                        ),
                )
                .child(
                    Button::new(("runtime-proc-stop-cancel", pid as usize))
                        .ghost()
                        .xsmall()
                        .label("Cancel")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.confirm_stop = None;
                            cx.notify();
                        })),
                )
                .into_any_element()
        } else {
            Button::new(("runtime-proc-stop", pid as usize))
                .custom(ButtonCustomVariant::new(cx).foreground(cx.theme().muted_foreground))
                .xsmall()
                .icon(IconName::CircleX)
                .tooltip("Stop\u{2026}")
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.confirm_stop = Some((pid, start_time));
                    cx.notify();
                }))
                .into_any_element()
        };

        h_flex()
            .gap_2()
            .px_3()
            .py_0p5()
            .text_xs()
            .text_color(cx.theme().foreground)
            .child(what)
            .child(mono(pid.to_string()).w_12().text_right())
            .child(
                mono(format!("{:.1}%", proc.cpu_percent))
                    .w_12()
                    .text_right(),
            )
            .child(mono(format!("{:.0}MB", proc.memory_mb)).w_16().text_right())
            .child(action)
            .into_any_element()
    }
}

impl Render for ProcessList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut col = div()
            .w_full()
            .flex()
            .flex_col()
            .children(self.render_notice(cx));

        if !self.loaded {
            return col.child(
                h_flex()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(Spinner::new().xsmall())
                    .child("Loading processes\u{2026}"),
            );
        }
        if self.processes.is_empty() {
            return col.child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!("No {} processes running", self.runtime.name)),
            );
        }

        // Both keep the scan's heaviest-first order.
        let (mine, others): (Vec<&RunningProcess>, Vec<&RunningProcess>) =
            self.processes.iter().partition(|proc| proc.in_workspace);

        col = col.child(
            h_flex()
                .gap_2()
                .px_3()
                .py_0p5()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(cx.theme().muted_foreground)
                .child(div().flex_1().child("What"))
                .child(div().w_12().text_right().child("PID"))
                .child(div().w_12().text_right().child("CPU"))
                .child(div().w_16().text_right().child("Mem"))
                .child(div().w_6()),
        );

        if mine.is_empty() {
            col = col.child(
                div()
                    .px_3()
                    .py_1()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("None started from this workspace"),
            );
        }
        for proc in &mine {
            col = col.child(self.render_row(proc, cx));
        }

        if !others.is_empty() {
            let open = self.show_other;
            col = col.child(
                h_flex()
                    .id("runtime-procs-other-toggle")
                    .gap_1()
                    .px_3()
                    .py_0p5()
                    .text_xs()
                    .cursor_pointer()
                    .text_color(cx.theme().muted_foreground)
                    .hover(|el| el.bg(cx.theme().list_hover))
                    .child(
                        Icon::new(if open {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .xsmall(),
                    )
                    .child(format!("Other ({})", others.len()))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.show_other = !this.show_other;
                        cx.notify();
                    })),
            );
            if open {
                for proc in &others {
                    col = col.child(self.render_row(proc, cx));
                }
            }
        }

        col
    }
}
