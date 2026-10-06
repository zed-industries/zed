use std::collections::BTreeMap;

use anyhow::{Context as _, Result};
use cloud_api_client::{
    SyncedSettingsErasureScope, SyncedSettingsGroupTarget, SyncedSettingsHost,
    SyncedSettingsRevision, Timestamp, UpdateSyncedSettingsHostBody,
};
use editor::Editor;
use feature_flags::{FeatureFlagAppExt as _, SettingsSyncFeatureFlag};
use fs::Fs;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Global, PromptLevel, ScrollHandle,
    Subscription, Task, WeakEntity, WindowHandle, actions,
};

use settings_sync::{
    ConflictBatch, FirstSyncChoice, PendingDecision, SettingsSyncEngine, SyncActivity, SyncOp,
    SyncedSettingsHistory,
};
use time::{OffsetDateTime, UtcOffset};
use time_format::TimestampFormat;
use ui::{Modal, ModalFooter, ModalHeader, Section, SectionHeader, prelude::*};
use workspace::{ModalView, MultiWorkspace, Workspace, with_active_or_new_workspace};

actions!(settings_sync, [OpenStatus]);

pub fn init(cx: &mut App) {
    cx.observe_flag::<SettingsSyncFeatureFlag, _>({
        let mut registered = false;
        move |flag, cx| {
            if registered || !*flag {
                return;
            }
            registered = true;
            cx.on_action(|_: &OpenStatus, cx| {
                with_active_or_new_workspace(cx, |workspace, window, cx| {
                    SettingsSyncStatusModal::toggle(workspace, window, cx);
                });
            });
        }
    })
    .detach();

    cx.observe_new(|_: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        if !cx
            .try_global::<DeferredDecisionModal>()
            .is_some_and(|deferred| deferred.0)
        {
            return;
        }
        cx.set_global(DeferredDecisionModal(false));
        cx.defer_in(window, |workspace, window, cx| {
            if let Some(engine) = settings_sync::engine(cx) {
                toggle_decision_modal(workspace, engine, window, cx);
            }
        });
    })
    .detach();
}

pub fn show_decision_modal(engine: Entity<SettingsSyncEngine>, cx: &mut App) {
    let Some(window) = workspace_window(cx) else {
        cx.set_global(DeferredDecisionModal(true));
        return;
    };
    cx.defer(move |cx| {
        window
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace
                    .workspace()
                    .clone()
                    .update(cx, |workspace, cx| {
                        toggle_decision_modal(workspace, engine, window, cx);
                    });
            })
            .ok();
    });
}

pub fn format_sync_value(value: &serde_json::Value) -> String {
    util::truncate_and_trailoff(&value.to_string(), MAX_VALUE_CHARS)
}

const MAX_VALUE_CHARS: usize = 80;
const PRIVACY_POLICY_URL: &str = "https://zed.dev/privacy-policy";

struct DeferredDecisionModal(bool);

impl Global for DeferredDecisionModal {}

fn workspace_window(cx: &App) -> Option<WindowHandle<MultiWorkspace>> {
    cx.active_window()
        .and_then(|window| window.downcast::<MultiWorkspace>())
        .or_else(|| {
            cx.windows()
                .into_iter()
                .find_map(|window| window.downcast::<MultiWorkspace>())
        })
}

fn toggle_decision_modal(
    workspace: &mut Workspace,
    engine: Entity<SettingsSyncEngine>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if engine.read(cx).pending_decision().is_none()
        || workspace
            .active_modal::<SettingsSyncDecisionModal>(cx)
            .is_some()
    {
        return;
    }
    workspace.toggle_modal(window, cx, |_, cx| {
        SettingsSyncDecisionModal::new(engine, cx)
    });
}

fn set_sync_enabled(enabled: bool, cx: &mut App) -> Task<Result<()>> {
    let written = settings::update_settings_file_with_completion(
        <dyn Fs>::global(cx),
        cx,
        move |content, _| {
            content.settings_sync.get_or_insert_default().enabled = Some(enabled);
        },
    );
    cx.background_spawn(async move {
        written
            .await
            .context("the settings file write was dropped")?
            .context("writing settings_sync.enabled to the settings file")
    })
}

fn format_op(op: &SyncOp) -> String {
    match op {
        SyncOp::Set { path, value } => format!("set {path} = {value}"),
        SyncOp::Delete { path } => format!("remove {path}"),
    }
}

fn format_timestamp(timestamp: OffsetDateTime) -> String {
    time_format::format_localized_timestamp(
        timestamp,
        OffsetDateTime::now_utc(),
        UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        TimestampFormat::EnhancedAbsolute,
    )
}

fn format_cloud_timestamp(timestamp: &Timestamp) -> String {
    OffsetDateTime::from_unix_timestamp(timestamp.0.timestamp())
        .map(format_timestamp)
        .unwrap_or_else(|_| timestamp.0.to_rfc3339())
}

fn render_ops(title: &'static str, ops: &[SyncOp], cx: &App) -> impl IntoElement {
    v_flex()
        .gap_0p5()
        .child(Label::new(title).size(LabelSize::Small).color(Color::Muted))
        .when(ops.is_empty(), |container| {
            container.child(Label::new("No changes").size(LabelSize::Small))
        })
        .children(ops.iter().map(|op| {
            Label::new(format_op(op))
                .size(LabelSize::Small)
                .buffer_font(cx)
        }))
}

fn render_property(name: &'static str, value: impl Into<SharedString>) -> impl IntoElement {
    h_flex()
        .gap_2()
        .items_start()
        .child(
            div()
                .w(rems(9.))
                .flex_none()
                .child(Label::new(name).size(LabelSize::Small).color(Color::Muted)),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(Label::new(value).size(LabelSize::Small)),
        )
}

struct FirstSyncOption {
    choice: FirstSyncChoice,
    local_changes: Vec<SyncOp>,
    remote_changes: Vec<SyncOp>,
}

struct SettingsSyncDecisionModal {
    engine: Entity<SettingsSyncEngine>,
    decision: Option<PendingDecision>,
    first_sync_options: Vec<FirstSyncOption>,
    exclusions: Vec<String>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    error: Option<SharedString>,
    operation_in_flight: bool,
    operation_task: Task<()>,
    _engine_subscription: Subscription,
}

impl SettingsSyncDecisionModal {
    fn new(engine: Entity<SettingsSyncEngine>, cx: &mut Context<Self>) -> Self {
        let engine_subscription = cx.observe(&engine, |modal, engine, cx| {
            if engine.read(cx).pending_decision().is_none() {
                cx.emit(DismissEvent);
            } else {
                modal.refresh(cx);
            }
        });
        let mut modal = Self {
            engine,
            decision: None,
            first_sync_options: Vec::new(),
            exclusions: Vec::new(),
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            error: None,
            operation_in_flight: false,
            operation_task: Task::ready(()),
            _engine_subscription: engine_subscription,
        };
        modal.refresh(cx);
        modal
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let engine = self.engine.read(cx);
        let decision = engine.pending_decision().cloned();
        if decision == self.decision {
            return;
        }
        self.first_sync_options = match &decision {
            Some(PendingDecision::FirstSync(preview)) => preview
                .choices()
                .into_iter()
                .map(|choice| FirstSyncOption {
                    choice,
                    local_changes: preview.local_changes(choice),
                    remote_changes: preview.remote_changes(choice),
                })
                .collect(),
            _ => Vec::new(),
        };
        self.exclusions = engine.effective_exclusions(cx);
        self.decision = decision;
        self.error = None;
        cx.notify();
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn decline_consent(&mut self, cx: &mut Context<Self>) {
        let written = set_sync_enabled(false, cx);
        let engine = self.engine.clone();
        let task = cx.spawn(async move |_, cx| {
            written.await?;
            engine.update(cx, |engine, cx| engine.clear_pending_decision(cx));
            anyhow::Ok(())
        });
        self.track_operation(task, cx);
    }

    fn accept_consent(&mut self, cx: &mut Context<Self>) {
        let task = self
            .engine
            .update(cx, |engine, cx| engine.accept_consent(cx));
        self.track_operation(task, cx);
    }

    fn join_group(&mut self, target: SyncedSettingsGroupTarget, cx: &mut Context<Self>) {
        let Some(PendingDecision::JoinGroup { account_id, .. }) = self.decision else {
            return;
        };
        let label = settings_sync::default_host_label();
        let task = self.engine.update(cx, |engine, cx| {
            engine.join_group(account_id, target, label, cx)
        });
        self.track_operation(task, cx);
    }

    fn choose_first_sync(&mut self, choice: FirstSyncChoice, cx: &mut Context<Self>) {
        let Some(PendingDecision::FirstSync(preview)) = self.decision.clone() else {
            return;
        };
        self.engine.update(cx, |engine, cx| {
            engine.choose_first_sync(preview, choice, cx)
        });
        cx.emit(DismissEvent);
    }

    fn track_operation<T: 'static>(&mut self, task: Task<Result<T>>, cx: &mut Context<Self>) {
        self.operation_in_flight = true;
        self.error = None;
        self.operation_task = cx.spawn(async move |modal, cx| {
            let result = task.await;
            modal
                .update(cx, |modal, cx| {
                    modal.operation_in_flight = false;
                    if let Err(error) = result {
                        modal.error = Some(SharedString::from(format!("{error:#}")));
                    }
                    cx.notify();
                })
                .ok();
        });
        cx.notify();
    }

    fn render_consent(&self, cx: &mut Context<Self>) -> (ModalHeader, Section, AnyElement) {
        let exclusions = if self.exclusions.is_empty() {
            SharedString::from("None")
        } else {
            SharedString::from(self.exclusions.join(", "))
        };
        let header = ModalHeader::new()
            .headline("Enable Settings Sync")
            .show_dismiss_button(true);
        let section = Section::new()
            .child(Label::new(
                "Settings sync uploads your settings.json, minus excluded settings, to Zed Cloud, \
                 where it is stored under your Zed account and synced to your other devices.",
            ))
            .child(Label::new(
                "Your settings may contain values you consider sensitive, such as server \
                 addresses, file paths, or tokens. Add them to settings_sync.exclude to keep \
                 them on this device.",
            ))
            .child(Label::new(
                "Secrets stored in your system keychain are never synced.",
            ))
            .child(Label::new(
                "Zed identifies this device by its existing per-machine system ID, \
                 the same ID that telemetry uses.",
            ))
            .child(render_property("Excluded settings", exclusions))
            .child(
                h_flex().child(
                    Button::new("settings-sync-privacy-policy", "Privacy Policy")
                        .style(ButtonStyle::Subtle)
                        .end_icon(
                            Icon::new(IconName::ArrowUpRight)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .on_click(|_, _, cx| cx.open_url(PRIVACY_POLICY_URL)),
                ),
            );
        let buttons = h_flex()
            .gap_2()
            .child(
                Button::new("settings-sync-consent-cancel", "Cancel")
                    .style(ButtonStyle::Filled)
                    .disabled(self.operation_in_flight)
                    .on_click(cx.listener(|modal, _, _, cx| modal.decline_consent(cx))),
            )
            .child(
                Button::new("settings-sync-consent-enable", "Enable Settings Sync")
                    .style(ButtonStyle::Filled)
                    .loading(self.operation_in_flight)
                    .disabled(self.operation_in_flight)
                    .on_click(cx.listener(|modal, _, _, cx| modal.accept_consent(cx))),
            )
            .into_any_element();
        (header, section, buttons)
    }

    fn render_join_group(
        &self,
        hosts: &[SyncedSettingsHost],
        cx: &mut Context<Self>,
    ) -> (ModalHeader, Section, AnyElement) {
        let mut groups = BTreeMap::<String, Vec<String>>::new();
        for host in hosts {
            groups
                .entry(host.group_id.clone())
                .or_default()
                .push(host.label.clone());
        }
        let header = ModalHeader::new()
            .headline("Choose a Sync Group")
            .description(
                "Settings sync already runs on other devices of your account. \
                 Choose which devices this one syncs with.",
            )
            .show_dismiss_button(true);
        let mut section = Section::new();
        for (ix, (group_id, labels)) in groups.into_iter().enumerate() {
            let labels = labels.join(" + ");
            section = section.child(
                Button::new(
                    ("settings-sync-join-group", ix),
                    format!("Sync with {labels}"),
                )
                .style(ButtonStyle::Outlined)
                .full_width()
                .disabled(self.operation_in_flight)
                .on_click(cx.listener(move |modal, _, _, cx| {
                    modal.join_group(
                        SyncedSettingsGroupTarget::Existing {
                            group_id: group_id.clone(),
                        },
                        cx,
                    );
                })),
            );
        }
        section = section.child(
            Button::new("settings-sync-new-group", "Start a New Sync Group")
                .style(ButtonStyle::Outlined)
                .full_width()
                .disabled(self.operation_in_flight)
                .on_click(cx.listener(|modal, _, _, cx| {
                    modal.join_group(SyncedSettingsGroupTarget::New, cx);
                })),
        );
        let footer = if self.operation_in_flight {
            Label::new("Joining…")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element()
        } else {
            div().into_any_element()
        };
        (header, section, footer)
    }

    fn render_first_sync(&self, cx: &mut Context<Self>) -> (ModalHeader, Section, AnyElement) {
        let empty_group = matches!(
            &self.decision,
            Some(PendingDecision::FirstSync(preview)) if preview.remote_version.is_none()
        );
        let header = if empty_group {
            ModalHeader::new()
                .headline("Upload Your Settings")
                .description(
                    "This sync group has no settings yet. \
                 Nothing is uploaded until you review and confirm what this device uploads.",
                )
        } else {
            ModalHeader::new()
                .headline("Combine Your Settings")
                .description(
                    "Both this device and Zed Cloud have settings. \
                 Nothing changes until you choose how to combine them.",
                )
        }
        .show_dismiss_button(true);
        let mut section = Section::new();
        for (ix, option) in self.first_sync_options.iter().enumerate() {
            let (title, description) = match option.choice {
                FirstSyncChoice::ReplaceRemote if empty_group => (
                    "Upload This Device's Settings",
                    "Start this sync group with this device's settings.",
                ),
                FirstSyncChoice::Merge => (
                    "Merge",
                    "Keep settings from both; where both set a different value, the cloud value wins.",
                ),
                FirstSyncChoice::ReplaceLocal => (
                    "Replace This Device's Settings",
                    "Use the cloud settings on this device. Excluded settings stay as they are.",
                ),
                FirstSyncChoice::ReplaceRemote => (
                    "Replace Cloud Settings",
                    "Upload this device's settings, replacing what your other devices sync.",
                ),
            };
            let choice = option.choice;
            section = section.child(
                v_flex()
                    .gap_1()
                    .p_2()
                    .rounded_sm()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .justify_between()
                            .gap_2()
                            .child(Label::new(title))
                            .child(
                                Button::new(("settings-sync-first-sync", ix), "Choose")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(move |modal, _, _, cx| {
                                        modal.choose_first_sync(choice, cx);
                                    })),
                            ),
                    )
                    .child(
                        Label::new(description)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(render_ops(
                        "Changes to this device's settings.json:",
                        &option.local_changes,
                        cx,
                    ))
                    .child(render_ops(
                        "Changes to the cloud copy:",
                        &option.remote_changes,
                        cx,
                    )),
            );
        }
        (header, section, div().into_any_element())
    }
}

impl EventEmitter<DismissEvent> for SettingsSyncDecisionModal {}

impl Focusable for SettingsSyncDecisionModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for SettingsSyncDecisionModal {}

impl Render for SettingsSyncDecisionModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let max_height = window.viewport_size().height - px(200.);
        let (header, section, end_slot) = match self.decision.clone() {
            Some(PendingDecision::Consent) | None => self.render_consent(cx),
            Some(PendingDecision::JoinGroup { hosts, .. }) => self.render_join_group(&hosts, cx),
            Some(PendingDecision::FirstSync(_)) => self.render_first_sync(cx),
        };
        v_flex()
            .id("settings-sync-decision-modal")
            .key_context("SettingsSyncDecisionModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w(rems(36.))
            .max_h(max_height)
            .overflow_hidden()
            .child(
                Modal::new("settings-sync-decision", Some(self.scroll_handle.clone()))
                    .header(header)
                    .section(section)
                    .footer(
                        ModalFooter::new()
                            .start_slot::<Label>(self.error.clone().map(|error| {
                                Label::new(error).size(LabelSize::Small).color(Color::Error)
                            }))
                            .end_slot(end_slot),
                    ),
            )
    }
}

struct Confirmation {
    message: String,
    detail: String,
    confirm: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LoadedFor {
    account_id: Option<u64>,
    group_id: Option<String>,
    server_version: Option<u64>,
    hosts_revision: u64,
}

impl LoadedFor {
    fn current(engine: &Entity<SettingsSyncEngine>, cx: &App) -> Self {
        let engine = engine.read(cx);
        Self {
            account_id: engine.account_id(),
            group_id: engine.state().and_then(|state| state.group_id.clone()),
            server_version: engine.status().server_version,
            hosts_revision: engine.hosts_revision(),
        }
    }
}

struct SettingsSyncStatusModal {
    engine: Entity<SettingsSyncEngine>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    label_editor: Entity<Editor>,
    hosts: Option<Vec<SyncedSettingsHost>>,
    hosts_account_id: Option<u64>,
    history: Option<SyncedSettingsHistory>,
    loaded_for: Option<LoadedFor>,
    load_error: Option<SharedString>,
    operation_error: Option<SharedString>,
    loading: bool,
    operation_in_flight: bool,
    load_task: Task<()>,
    operation_task: Task<()>,
    _engine_subscription: Subscription,
}

impl SettingsSyncStatusModal {
    fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let Some(engine) = settings_sync::engine(cx) else {
            return;
        };
        let workspace_handle = cx.weak_entity();
        workspace.toggle_modal(window, cx, |window, cx| {
            Self::new(engine, workspace_handle, window, cx)
        });
    }

    fn new(
        engine: Entity<SettingsSyncEngine>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let label_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Name for this device", window, cx);
            editor
        });
        let engine_subscription = cx.observe_in(&engine, window, |modal, _, window, cx| {
            if !modal.loading
                && !modal.operation_in_flight
                && modal.loaded_for != Some(LoadedFor::current(&modal.engine, cx))
            {
                modal.reload(window, cx);
            }
            cx.notify();
        });
        let mut modal = Self {
            engine,
            workspace,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            label_editor,
            hosts: None,
            hosts_account_id: None,
            history: None,
            loaded_for: None,
            load_error: None,
            operation_error: None,
            loading: false,
            operation_in_flight: false,
            load_task: Task::ready(()),
            operation_task: Task::ready(()),
            _engine_subscription: engine_subscription,
        };
        modal.reload(window, cx);
        modal
    }

    fn reload(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let hosts = self.engine.update(cx, |engine, cx| engine.list_hosts(cx));
        let history = self.engine.update(cx, |engine, cx| engine.list_history(cx));
        let requested_for = LoadedFor::current(&self.engine, cx);
        self.loaded_for = Some(requested_for.clone());
        self.loading = true;
        self.load_task = cx.spawn_in(window, async move |modal, cx| {
            let (hosts, history) = futures::join!(hosts, history);
            modal
                .update_in(cx, |modal, window, cx| {
                    modal.loading = false;
                    if LoadedFor::current(&modal.engine, cx) != requested_for {
                        modal.reload(window, cx);
                        return;
                    }
                    let mut errors = Vec::new();
                    match hosts {
                        Ok(hosts) => {
                            modal.hosts_account_id = Some(hosts.account_id);
                            modal.set_hosts(hosts.hosts, window, cx);
                        }
                        Err(error) => errors.push(format!("Could not load devices: {error:#}")),
                    }
                    match history {
                        Ok(history) => {
                            modal.history = Some(history);
                        }
                        Err(error) => errors.push(format!("Could not load history: {error:#}")),
                    }
                    modal.load_error =
                        (!errors.is_empty()).then(|| SharedString::from(errors.join("\n")));
                    cx.notify();
                })
                .ok();
        });
        cx.notify();
    }

    fn set_hosts(
        &mut self,
        hosts: Vec<SyncedSettingsHost>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let this_system_id = self.engine.read(cx).this_system_id();
        let this_label = hosts
            .iter()
            .find(|host| Some(&host.system_id) == this_system_id.as_ref())
            .map(|host| host.label.clone());
        if let Some(label) = this_label {
            self.label_editor.update(cx, |editor, cx| {
                if editor.text(cx).is_empty() {
                    editor.set_text(label, window, cx);
                }
            });
        }
        self.hosts = Some(hosts);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if self.label_editor.focus_handle(cx).is_focused(window) {
            self.rename_this_device(window, cx);
        }
    }

    fn run_operation<T: 'static>(
        &mut self,
        confirmation: Option<Confirmation>,
        window: &mut Window,
        cx: &mut Context<Self>,
        start: impl FnOnce(&mut SettingsSyncEngine, &mut Context<SettingsSyncEngine>) -> Task<Result<T>>
        + 'static,
    ) {
        if self.operation_in_flight {
            return;
        }
        let answer = confirmation.map(|confirmation| {
            window.prompt(
                PromptLevel::Warning,
                &confirmation.message,
                Some(&confirmation.detail),
                &[confirmation.confirm, "Cancel"],
                cx,
            )
        });
        let engine = self.engine.clone();
        self.operation_in_flight = true;
        self.operation_error = None;
        self.operation_task = cx.spawn_in(window, async move |modal, cx| {
            let confirmed = match answer {
                Some(answer) => answer.await.ok() == Some(0),
                None => true,
            };
            let result = if confirmed {
                Some(engine.update(cx, start).await)
            } else {
                None
            };
            modal
                .update_in(cx, |modal, window, cx| {
                    modal.operation_in_flight = false;
                    match result {
                        Some(Ok(_)) => modal.reload(window, cx),
                        Some(Err(error)) => {
                            modal.operation_error = Some(SharedString::from(format!("{error:#}")));
                        }
                        None => {}
                    }
                    cx.notify();
                })
                .ok();
        });
        cx.notify();
    }

    fn rename_this_device(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(system_id) = self.engine.read(cx).this_system_id() else {
            return;
        };
        let Some(account_id) = self.hosts_account_id else {
            return;
        };
        let label = self.label_editor.read(cx).text(cx).trim().to_string();
        if label.is_empty() {
            self.operation_error = Some(SharedString::from("Enter a name for this device"));
            cx.notify();
            return;
        }
        self.run_operation(None, window, cx, move |engine, cx| {
            engine.update_host(
                account_id,
                system_id,
                UpdateSyncedSettingsHostBody {
                    label: Some(label),
                    group: None,
                },
                cx,
            )
        });
    }

    fn move_host(
        &mut self,
        host: SyncedSettingsHost,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(account_id) = self.hosts_account_id else {
            return;
        };
        let label = &host.label;
        let confirmation = Confirmation {
            message: format!("Move {label} to this device's sync group?"),
            detail: String::from(
                "That device stops syncing with its current group. If its settings differ from \
                 this group's, it asks how to combine them before changing anything.",
            ),
            confirm: "Move",
        };
        self.run_operation(Some(confirmation), window, cx, move |engine, cx| {
            engine.update_host(
                account_id,
                host.system_id,
                UpdateSyncedSettingsHostBody {
                    label: None,
                    group: Some(SyncedSettingsGroupTarget::Existing { group_id }),
                },
                cx,
            )
        });
    }

    fn move_this_device(
        &mut self,
        target: SyncedSettingsGroupTarget,
        target_label: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(account_id) = self.hosts_account_id else {
            return;
        };
        let confirmation = Confirmation {
            message: format!("Move this device to {target_label}?"),
            detail: String::from(
                "This device stops syncing with its current group. If its settings differ from \
                 the new group's, you choose how to combine them before anything changes.",
            ),
            confirm: "Move",
        };
        let label = self.label_editor.read(cx).text(cx).trim().to_string();
        let label = if label.is_empty() {
            settings_sync::default_host_label()
        } else {
            label
        };
        self.run_operation(Some(confirmation), window, cx, move |engine, cx| {
            engine.join_group(account_id, target, label, cx)
        });
    }

    fn revert_conflicts(
        &mut self,
        batch: ConflictBatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.run_operation(None, window, cx, move |engine, cx| {
            engine.revert_conflicts(batch, cx)
        });
    }

    fn remove_host(
        &mut self,
        host: SyncedSettingsHost,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(account_id) = self.hosts_account_id else {
            return;
        };
        let label = &host.label;
        let confirmation = Confirmation {
            message: format!("Remove {label} from settings sync?"),
            detail: String::from(
                "That device stops syncing until it joins a sync group again. \
                 Its settings file is not changed.",
            ),
            confirm: "Remove",
        };
        self.run_operation(Some(confirmation), window, cx, move |engine, cx| {
            engine.remove_host(account_id, host.system_id, cx)
        });
    }

    fn restore_revision(
        &mut self,
        revision: SyncedSettingsRevision,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((account_id, group_id)) = self
            .history
            .as_ref()
            .and_then(|history| Some((history.account_id, history.group_id.clone()?)))
        else {
            return;
        };
        let version = revision.version;
        let confirmation = Confirmation {
            message: format!("Restore settings from version {version}?"),
            detail: String::from(
                "Synced settings in your settings.json are replaced with that version; \
                 excluded settings stay as they are. The result then syncs to your other devices.",
            ),
            confirm: "Restore",
        };
        self.run_operation(Some(confirmation), window, cx, move |engine, cx| {
            engine.restore_revision(revision, account_id, group_id, cx)
        });
    }

    fn erase(
        &mut self,
        scope: SyncedSettingsErasureScope,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(account_id) = self.hosts_account_id else {
            return;
        };
        let this_system_id = self.engine.read(cx).this_system_id();
        let expected_group_id = self
            .hosts
            .iter()
            .flatten()
            .find(|host| Some(&host.system_id) == this_system_id.as_ref())
            .map(|host| host.group_id.clone());
        let confirmation = match scope {
            SyncedSettingsErasureScope::Group => Confirmation {
                message: String::from("Erase this sync group's settings from Zed Cloud?"),
                detail: String::from(
                    "The synced copy and history of this sync group are deleted. \
                     Settings sync turns off on this device; your local settings stay in place.",
                ),
                confirm: "Erase",
            },
            SyncedSettingsErasureScope::Account => Confirmation {
                message: String::from("Erase all your synced settings from Zed Cloud?"),
                detail: String::from(
                    "The synced copies and history of every sync group on your account are deleted. \
                     Settings sync turns off on this device; your local settings stay in place.",
                ),
                confirm: "Erase All",
            },
        };
        self.run_operation(Some(confirmation), window, cx, move |engine, cx| {
            engine.erase_cloud_data(scope, account_id, expected_group_id, cx)
        });
    }

    fn retry(&mut self, cx: &mut Context<Self>) {
        self.engine.update(cx, |engine, cx| engine.unpause(cx));
    }

    fn set_enabled(&mut self, enabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.run_operation(None, window, cx, move |_, cx| set_sync_enabled(enabled, cx));
    }

    fn review_pending_decision(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        let engine = self.engine.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |workspace, cx| {
                    toggle_decision_modal(workspace, engine, window, cx);
                })
                .ok();
        });
    }

    fn render_status(&self, cx: &mut Context<Self>) -> Section {
        let engine = self.engine.read(cx);
        let status = engine.status().clone();
        let activity = engine.activity(cx);
        let has_pending_decision = engine.pending_decision().is_some();
        let exclusions = engine.effective_exclusions(cx);
        let group_id = status
            .group_id
            .clone()
            .or_else(|| engine.state().and_then(|state| state.group_id.clone()));
        let enabled = activity != SyncActivity::Off;
        let can_retry = matches!(activity, SyncActivity::Paused | SyncActivity::Failing);
        let state = match activity {
            SyncActivity::Off => "Off",
            SyncActivity::FeatureUnavailable => "Not available for this account yet",
            SyncActivity::SignedOut => "Waiting for sign-in",
            SyncActivity::Paused => "Paused",
            SyncActivity::WaitingForDecision => "Waiting for your choice",
            SyncActivity::Failing => "Failing, see the last error",
            SyncActivity::On => "On",
        };
        let last_synced = status
            .last_synced_at
            .map(|time| format_timestamp(OffsetDateTime::from(time)))
            .unwrap_or_else(|| String::from("Not since Zed started"));
        let version = status
            .server_version
            .map(|version| version.to_string())
            .unwrap_or_else(|| String::from("Unknown"));
        let exclusions = if exclusions.is_empty() {
            String::from("None")
        } else {
            exclusions.join(", ")
        };
        Section::new()
            .header(SectionHeader::new("Status"))
            .child(render_property("Settings sync", state))
            .child(render_property("Last synced", last_synced))
            .child(render_property("Cloud version", version))
            .child(render_property(
                "Sync group",
                group_id.unwrap_or_else(|| String::from("None")),
            ))
            .when_some(status.last_error, |section, error| {
                section.child(render_property("Last error", error))
            })
            .child(render_property("Excluded settings", exclusions))
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new(
                            "settings-sync-toggle",
                            if enabled {
                                "Turn Off Settings Sync"
                            } else {
                                "Turn On Settings Sync"
                            },
                        )
                        .style(ButtonStyle::Outlined)
                        .disabled(self.operation_in_flight)
                        .on_click(cx.listener(
                            move |modal, _, window, cx| modal.set_enabled(!enabled, window, cx),
                        )),
                    )
                    .when(can_retry, |buttons| {
                        buttons.child(
                            Button::new("settings-sync-retry", "Retry")
                                .style(ButtonStyle::Filled)
                                .on_click(cx.listener(|modal, _, _, cx| modal.retry(cx))),
                        )
                    })
                    .when(has_pending_decision, |buttons| {
                        buttons.child(
                            Button::new("settings-sync-review-decision", "Review Pending Choice")
                                .style(ButtonStyle::Filled)
                                .on_click(cx.listener(|modal, _, window, cx| {
                                    modal.review_pending_decision(window, cx)
                                })),
                        )
                    }),
            )
    }

    fn render_conflicts(&self, cx: &mut Context<Self>) -> Option<Section> {
        let batch = self.engine.read(cx).recent_conflicts()?.clone();
        let disabled = self.operation_in_flight;
        let mut section = Section::new().header(SectionHeader::new("Recent Conflicts"));
        for (ix, conflict) in batch.conflicts.iter().enumerate() {
            let path = &conflict.path;
            let kept = conflict
                .remote
                .as_ref()
                .map(|value| format!("kept {value}"))
                .unwrap_or_else(|| String::from("removed"));
            let yours = conflict
                .local
                .as_ref()
                .map(|value| format!("yours was {value}"))
                .unwrap_or_else(|| String::from("you had it unset"));
            let changed_by = conflict
                .changed_by
                .as_ref()
                .map(|host| format!(", changed on {host}"))
                .unwrap_or_default();
            let single = ConflictBatch {
                conflicts: Vec::from([conflict.clone()]),
                ..batch.clone()
            };
            section = section.child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .child(
                        v_flex()
                            .min_w_0()
                            .child(
                                Label::new(path.to_string())
                                    .size(LabelSize::Small)
                                    .buffer_font(cx),
                            )
                            .child(
                                Label::new(format!("{kept}; {yours}{changed_by}"))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(
                        Button::new(("settings-sync-use-mine", ix), "Use Mine")
                            .style(ButtonStyle::Outlined)
                            .label_size(LabelSize::Small)
                            .disabled(disabled)
                            .on_click(cx.listener(move |modal, _, window, cx| {
                                modal.revert_conflicts(single.clone(), window, cx)
                            })),
                    ),
            );
        }
        Some(section)
    }

    fn render_devices(&self, this_system_id: Option<&str>, cx: &mut Context<Self>) -> Section {
        let section = Section::new().header(SectionHeader::new("Devices"));
        let Some(hosts) = self.hosts.clone() else {
            return section.child(
                Label::new("Loading…")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        };
        if hosts.is_empty() {
            return section.child(
                Label::new("No devices sync settings yet.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        }
        let this_group = hosts
            .iter()
            .find(|host| Some(host.system_id.as_str()) == this_system_id)
            .map(|host| host.group_id.clone());
        let mut groups = BTreeMap::<String, Vec<SyncedSettingsHost>>::new();
        for host in hosts {
            groups.entry(host.group_id.clone()).or_default().push(host);
        }
        let move_targets = groups
            .iter()
            .filter(|(group_id, _)| this_group.as_deref() != Some(group_id.as_str()))
            .map(|(group_id, group_hosts)| {
                let labels = group_hosts
                    .iter()
                    .map(|host| host.label.as_str())
                    .collect::<Vec<_>>()
                    .join(" + ");
                (
                    SyncedSettingsGroupTarget::Existing {
                        group_id: group_id.clone(),
                    },
                    format!("the group of {labels}"),
                )
            })
            .chain([(
                SyncedSettingsGroupTarget::New,
                String::from("a new sync group"),
            )])
            .collect::<Vec<_>>();
        let mut section = section;
        let mut ix = 0;
        for (group_id, group_hosts) in groups {
            let heading = if this_group.as_deref() == Some(group_id.as_str()) {
                format!("Sync group {group_id} (this device's group)")
            } else {
                format!("Sync group {group_id}")
            };
            section = section.child(
                Label::new(heading)
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1(),
            );
            for host in group_hosts {
                section = section.child(self.render_host(
                    ix,
                    host,
                    this_system_id,
                    this_group.as_deref(),
                    &move_targets,
                    cx,
                ));
                ix += 1;
            }
        }
        section
    }

    fn render_host(
        &self,
        ix: usize,
        host: SyncedSettingsHost,
        this_system_id: Option<&str>,
        this_group: Option<&str>,
        move_targets: &[(SyncedSettingsGroupTarget, String)],
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let is_this_device = this_system_id == Some(host.system_id.as_str());
        let label = &host.label;
        let name = if is_this_device {
            format!("{label} (this device)")
        } else {
            label.clone()
        };
        let last_synced = host
            .last_synced_at
            .as_ref()
            .map(|time| {
                let time = format_cloud_timestamp(time);
                format!("Last synced {time}")
            })
            .unwrap_or_else(|| String::from("Never synced"));
        let details = match host.last_synced_version {
            Some(version) => format!("{last_synced}, version {version}"),
            None => last_synced,
        };
        let move_target = this_group
            .filter(|group_id| !is_this_device && *group_id != host.group_id)
            .map(String::from);
        let disabled = self.operation_in_flight;

        let row = h_flex()
            .justify_between()
            .gap_2()
            .child(
                v_flex()
                    .min_w_0()
                    .child(Label::new(name).size(LabelSize::Small))
                    .child(
                        Label::new(details)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .flex_none()
                    .when_some(move_target, |buttons, group_id| {
                        let host = host.clone();
                        buttons.child(
                            Button::new(("settings-sync-move-host", ix), "Move to This Group")
                                .style(ButtonStyle::Outlined)
                                .label_size(LabelSize::Small)
                                .disabled(disabled)
                                .on_click(cx.listener(move |modal, _, window, cx| {
                                    modal.move_host(host.clone(), group_id.clone(), window, cx)
                                })),
                        )
                    })
                    .when(!is_this_device, |buttons| {
                        let host = host.clone();
                        buttons.child(
                            Button::new(("settings-sync-remove-host", ix), "Remove")
                                .style(ButtonStyle::Outlined)
                                .label_size(LabelSize::Small)
                                .disabled(disabled)
                                .on_click(cx.listener(move |modal, _, window, cx| {
                                    modal.remove_host(host.clone(), window, cx)
                                })),
                        )
                    }),
            );

        v_flex()
            .gap_1()
            .py_1()
            .child(row)
            .when(is_this_device, |container| {
                container.child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded_sm()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .child(self.label_editor.clone()),
                        )
                        .child(
                            Button::new("settings-sync-rename-device", "Rename")
                                .style(ButtonStyle::Outlined)
                                .label_size(LabelSize::Small)
                                .disabled(disabled)
                                .on_click(cx.listener(|modal, _, window, cx| {
                                    modal.rename_this_device(window, cx)
                                })),
                        ),
                )
            })
            .when(is_this_device, |container| {
                container.child(
                    h_flex()
                        .flex_wrap()
                        .gap_1()
                        .children(move_targets.iter().enumerate().map(
                            |(target_ix, (target, target_label))| {
                                let target = target.clone();
                                let target_label = target_label.clone();
                                Button::new(
                                    ("settings-sync-move-this-device", target_ix),
                                    format!("Move to {target_label}"),
                                )
                                .style(ButtonStyle::Outlined)
                                .label_size(LabelSize::Small)
                                .disabled(disabled)
                                .on_click(cx.listener(
                                    move |modal, _, window, cx| {
                                        modal.move_this_device(
                                            target.clone(),
                                            target_label.clone(),
                                            window,
                                            cx,
                                        )
                                    },
                                ))
                            },
                        )),
                )
            })
            .into_any_element()
    }

    fn render_history(&self, this_system_id: Option<&str>, cx: &mut Context<Self>) -> Section {
        let section = Section::new().header(SectionHeader::new("Recent History"));
        let Some(history) = self
            .history
            .as_ref()
            .map(|history| history.revisions.clone())
        else {
            return section.child(
                Label::new("Loading…")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        };
        if history.is_empty() {
            return section.child(
                Label::new("No synced versions yet.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        }
        let labels = self
            .hosts
            .iter()
            .flatten()
            .map(|host| (host.system_id.as_str(), host.label.as_str()))
            .collect::<BTreeMap<_, _>>();
        let disabled = self.operation_in_flight;
        let mut section = section;
        for (ix, revision) in history.into_iter().enumerate() {
            let device = match revision.system_id.as_deref() {
                Some(system_id) if Some(system_id) == this_system_id => "this device",
                Some(system_id) => labels.get(system_id).copied().unwrap_or("unknown device"),
                None => "unknown device",
            };
            let version = revision.version;
            let time = format_cloud_timestamp(&revision.created_at);
            section = section.child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .child(
                        Label::new(format!("Version {version}, {time}, from {device}"))
                            .size(LabelSize::Small)
                            .truncate(),
                    )
                    .child(
                        Button::new(("settings-sync-restore", ix), "Restore")
                            .style(ButtonStyle::Outlined)
                            .label_size(LabelSize::Small)
                            .disabled(disabled)
                            .on_click(cx.listener(move |modal, _, window, cx| {
                                modal.restore_revision(revision.clone(), window, cx)
                            })),
                    ),
            );
        }
        section
    }

    fn render_erase(&self, has_group: bool, cx: &mut Context<Self>) -> Section {
        let disabled = self.operation_in_flight;
        Section::new()
            .header(SectionHeader::new("Cloud Data"))
            .child(
                h_flex()
                    .gap_2()
                    .when(has_group, |buttons| {
                        buttons.child(
                            Button::new(
                                "settings-sync-erase-group",
                                "Erase this group's cloud copy",
                            )
                            .style(ButtonStyle::Outlined)
                            .color(Color::Error)
                            .disabled(disabled)
                            .on_click(cx.listener(
                                |modal, _, window, cx| {
                                    modal.erase(SyncedSettingsErasureScope::Group, window, cx)
                                },
                            )),
                        )
                    })
                    .child(
                        Button::new(
                            "settings-sync-erase-account",
                            "Erase all my synced settings",
                        )
                        .style(ButtonStyle::Outlined)
                        .color(Color::Error)
                        .disabled(disabled)
                        .on_click(cx.listener(|modal, _, window, cx| {
                            modal.erase(SyncedSettingsErasureScope::Account, window, cx)
                        })),
                    ),
            )
    }
}

impl EventEmitter<DismissEvent> for SettingsSyncStatusModal {}

impl Focusable for SettingsSyncStatusModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for SettingsSyncStatusModal {}

impl Render for SettingsSyncStatusModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let max_height = window.viewport_size().height - px(200.);
        let this_system_id = self.engine.read(cx).this_system_id();
        let has_group = self
            .hosts
            .iter()
            .flatten()
            .any(|host| Some(&host.system_id) == this_system_id.as_ref());
        let errors = self
            .operation_error
            .iter()
            .chain(self.load_error.iter())
            .cloned()
            .collect::<Vec<_>>();
        let busy = self.loading || self.operation_in_flight;

        v_flex()
            .id("settings-sync-status-modal")
            .key_context("SettingsSyncStatusModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_3(cx)
            .w(rems(40.))
            .max_h(max_height)
            .overflow_hidden()
            .child(
                Modal::new("settings-sync-status", Some(self.scroll_handle.clone()))
                    .header(
                        ModalHeader::new()
                            .headline("Settings Sync")
                            .show_dismiss_button(true),
                    )
                    .section(self.render_status(cx))
                    .when_some(self.render_conflicts(cx), |modal, section| {
                        modal.section(section)
                    })
                    .section(self.render_devices(this_system_id.as_deref(), cx))
                    .section(self.render_history(this_system_id.as_deref(), cx))
                    .section(self.render_erase(has_group, cx))
                    .footer(
                        ModalFooter::new()
                            .start_slot(v_flex().children(errors.into_iter().map(|error| {
                                Label::new(error).size(LabelSize::Small).color(Color::Error)
                            })))
                            .end_slot::<Label>(busy.then(|| {
                                Label::new("Working…")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            })),
                    ),
            )
    }
}
