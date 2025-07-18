mod vulnerability;

use gpui::{
    actions, div, prelude::*, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render,
    SharedString, Subscription, Task, WeakEntity, Window,
};
use std::process::Command;
use vulnerability::Report;
use workspace::{
    item::{Item, ItemEvent},
    prelude::*,
    ui::{Button, Icon, IconName, Label},
    Workspace,
};

actions!(security, [Deploy]);

pub fn init(cx: &mut App) {
    cx.observe_new(SecurityPanel::register).detach();
}

pub struct SecurityPanel {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    vulnerabilities: Option<Report>,
    scan_task: Option<Task<()>>,
    _subscription: Subscription,
}

impl Render for SecurityPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("SecurityPanel")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .on_action(cx.listener(Self::deploy))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .child(
                        Button::new("scan-button", "Scan for vulnerabilities")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.scan_for_vulnerabilities(window, cx);
                            })),
                    )
                    .children(self.vulnerabilities.as_ref().map(|report| {
                        div().flex().flex_col().children(
                            report
                                .vulnerabilities
                                .list
                                .iter()
                                .map(|v| Label::new(v.advisory.title.clone())),
                        )
                    })),
            )
    }
}

impl SecurityPanel {
    fn register(
        workspace: &mut Workspace,
        _: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(Self::deploy);
    }

    fn new(workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let subscription = cx.subscribe(&workspace.id(), |_, event, cx| {
            // Handle workspace events if needed
        });

        Self {
            workspace,
            focus_handle,
            vulnerabilities: None,
            scan_task: None,
            _subscription: subscription,
        }
    }

    fn deploy(
        workspace: &mut Workspace,
        _: &Deploy,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(existing) = workspace.item_of_type::<SecurityPanel>(cx) {
            let is_active = workspace
                .active_item(cx)
                .is_some_and(|item| item.item_id() == existing.item_id());
            workspace.activate_item(&existing, true, !is_active, window, cx);
        } else {
            let workspace_handle = cx.entity().downgrade();
            let view = cx.new(|cx| SecurityPanel::new(workspace_handle, cx));
            workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
        }
    }

    fn scan_for_vulnerabilities(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.scan_task = Some(cx.spawn(|this, mut cx| async move {
            let output = Command::new("cargo")
                .arg("audit")
                .arg("--json")
                .output()
                .expect("failed to execute process");

            if output.status.success() {
                let report: Report =
                    serde_json::from_slice(&output.stdout).expect("failed to parse json");
                this.update(&mut cx, |this, cx| {
                    this.vulnerabilities = Some(report);
                    cx.notify();
                });
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("cargo audit failed: {}", stderr);
            }
        }));
    }
}

impl Focusable for SecurityPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for SecurityPanel {
    type Event = ItemEvent;

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some("Security".into())
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, _: &App) -> AnyElement {
        h_flex()
            .gap_1()
            .child(Icon::new(IconName::Shield).color(params.text_color()))
            .child(Label::new("Security").color(params.text_color()))
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Security Panel Opened")
    }
}
