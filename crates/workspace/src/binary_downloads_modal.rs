//! A modal that informs the user that Zed is not allowed to download tool
//! binaries (LSPs, MCP servers, formatters, debug adapters, npm packages...)
//! because of the `allow_binary_downloads` setting.
//!
//! The look and layout mirrors [`crate::security_modal::SecurityModal`], so the
//! two restrictions feel like a coherent set.

use collections::{HashMap, HashSet};
use fs::Fs;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, Subscription, Task,
    WeakEntity,
};
use project::{
    Project,
    binary_downloads::{
        BinaryDownloads, BinaryDownloadsStore, PendingToolInstall, ToolInstallOrigin,
    },
    project_settings::ProjectSettings,
    trusted_worktrees::RemoteHostLocation,
};
use settings::{Settings, SettingsLocation, update_settings_file_with_completion};
use theme::ActiveTheme;
use ui::{AlertModal, ButtonStyle, Checkbox, KeyBinding, ToggleState, WithScrollbar, prelude::*};
use util::rel_path::RelPath;

use crate::{DismissDecision, ModalView, ToggleWorktreeSecurity};

/// Where the `allow_binary_downloads = false` setting is taking effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisabledScope {
    Global,
    Project,
    Both,
}

impl DisabledScope {
    fn override_hint(self) -> &'static str {
        match self {
            DisabledScope::Global => {
                "Project overrides do not enable app-global tools; approve those tools individually or enable global downloads."
            }
            DisabledScope::Project => {
                "To allow downloads for this project, set \"allow_binary_downloads\": true in .zed/settings.json (or remove the project override)."
            }
            DisabledScope::Both => {
                "To allow downloads only for this project while keeping the global default disabled, set \"allow_binary_downloads\": true in .zed/settings.json."
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DismissOutcome {
    Acknowledged,
    EnabledDownloads,
    InstalledTools,
}

pub struct BinaryDownloadsModal {
    scope: DisabledScope,
    focus_handle: FocusHandle,
    decided: Option<DismissOutcome>,
    /// The global binary-downloads store, used to read pending one-off install
    /// requests and to resolve the ones the user selects.
    store: Option<WeakEntity<BinaryDownloadsStore>>,
    project: WeakEntity<Project>,
    /// Tools the user ticked to install once. Empty by default.
    selected: HashSet<PendingToolInstall>,
    entry_focus_handles: HashMap<PendingToolInstall, FocusHandle>,
    enable_focus_handle: FocusHandle,
    cancel_focus_handle: FocusHandle,
    confirm_focus_handle: FocusHandle,
    list_scroll_handle: ScrollHandle,
    approval_error: Option<SharedString>,
    enabling_downloads: bool,
    _enable_downloads: Task<()>,
    /// Keeps the pending-tools list fresh when requests arrive or the setting
    /// flips while the modal is open.
    _store_subscription: Option<Subscription>,
}

impl Focusable for BinaryDownloadsModal {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for BinaryDownloadsModal {}

impl ModalView for BinaryDownloadsModal {
    fn fade_out_background(&self) -> bool {
        true
    }

    fn on_before_dismiss(&mut self, _: &mut Window, _: &mut Context<Self>) -> DismissDecision {
        if self.enabling_downloads {
            return DismissDecision::Pending;
        }
        match self.decided {
            Some(DismissOutcome::Acknowledged) => {
                telemetry::event!("Acknowledge", source = "Binary Downloads Modal");
                DismissDecision::Dismiss(true)
            }
            Some(DismissOutcome::EnabledDownloads) => {
                telemetry::event!("Enable Downloads", source = "Binary Downloads Modal");
                DismissDecision::Dismiss(true)
            }
            // The "Install Tools Once" event is emitted in `confirm_and_dismiss`
            // where the approved tool count is available.
            Some(DismissOutcome::InstalledTools) => DismissDecision::Dismiss(true),
            None => {
                telemetry::event!("Dismiss", source = "Binary Downloads Modal");
                DismissDecision::Dismiss(true)
            }
        }
    }
}

impl BinaryDownloadsModal {
    pub fn new(project: &Entity<Project>, scope: DisabledScope, cx: &mut Context<Self>) -> Self {
        let store_entity = BinaryDownloads::try_get_global(cx);
        let store_subscription = store_entity
            .as_ref()
            .map(|store| cx.observe(store, |_, _, cx| cx.notify()));
        Self {
            scope,
            focus_handle: cx.focus_handle(),
            decided: None,
            store: store_entity.map(|store| store.downgrade()),
            project: project.downgrade(),
            selected: HashSet::default(),
            entry_focus_handles: HashMap::default(),
            enable_focus_handle: cx.focus_handle(),
            cancel_focus_handle: cx.focus_handle(),
            confirm_focus_handle: cx.focus_handle(),
            list_scroll_handle: ScrollHandle::new(),
            approval_error: None,
            enabling_downloads: false,
            _enable_downloads: Task::ready(()),
            _store_subscription: store_subscription,
        }
    }

    /// Pending one-off install requests relevant to this modal's project,
    /// sorted for a stable checkbox order.
    fn pending_tool_installs(&self, cx: &App) -> Vec<PendingToolInstall> {
        let Some(project) = self.project.upgrade() else {
            return Vec::new();
        };
        pending_tool_installs_for_project(project.read(cx), cx)
    }

    pub(crate) fn acknowledge_and_dismiss(&mut self, cx: &mut Context<Self>) {
        if self.enabling_downloads {
            return;
        }
        self.decided = Some(DismissOutcome::Acknowledged);
        cx.emit(DismissEvent);
    }

    /// Approves the ticked tools for a one-off install without touching the
    /// `allow_binary_downloads` setting, then dismisses. With nothing ticked
    /// this is equivalent to acknowledging.
    fn confirm_and_dismiss(&mut self, cx: &mut Context<Self>) {
        if self.enabling_downloads {
            return;
        }
        let pending = self.pending_tool_installs(cx);
        self.selected.retain(|install| pending.contains(install));
        if self.selected.is_empty() {
            self.acknowledge_and_dismiss(cx);
            return;
        }
        self.approval_error = None;
        if let Some(store) = self.store.as_ref().and_then(|store| store.upgrade()) {
            let selected = self.selected.clone();
            let mut approved = 0;
            store.update(cx, |store, cx| {
                for pending in selected {
                    let install = &pending.install;
                    let result = match &pending.origin {
                        ToolInstallOrigin::Local => {
                            store.approve_tool_install(
                                install.worktree_id,
                                install.tool.clone(),
                                cx,
                            );
                            Ok(())
                        }
                        ToolInstallOrigin::Remote(origin) => {
                            store.approve_remote_tool_install(origin, install)
                        }
                    };
                    match result {
                        Ok(()) => {
                            self.selected.remove(&pending);
                            approved += 1;
                        }
                        Err(error) => {
                            self.approval_error = Some(SharedString::from(format!(
                                "Could not approve {}: {error}",
                                install.tool
                            )));
                        }
                    }
                }
            });
            telemetry::event!(
                "Install Tools Once",
                source = "Binary Downloads Modal",
                count = approved
            );
        }
        if self.approval_error.is_some() {
            cx.notify();
        } else {
            self.decided = Some(DismissOutcome::InstalledTools);
            cx.emit(DismissEvent);
        }
    }

    fn toggle_install(&mut self, install: PendingToolInstall, cx: &mut Context<Self>) {
        if self.enabling_downloads {
            return;
        }
        if !self.selected.remove(&install) {
            self.selected.insert(install);
        }
        cx.notify();
    }

    fn move_focus(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.enabling_downloads {
            return;
        }
        let pending = self.pending_tool_installs(cx);
        let mut handles = pending
            .iter()
            .filter_map(|install| self.entry_focus_handles.get(install))
            .collect::<Vec<_>>();
        if !ProjectSettings::get_global(cx).allow_binary_downloads {
            handles.push(&self.enable_focus_handle);
        }
        handles.push(&self.cancel_focus_handle);
        if !pending.is_empty() && !self.selected.is_empty() {
            handles.push(&self.confirm_focus_handle);
        }
        let current = handles.iter().position(|handle| handle.is_focused(window));
        let index = match (current, forward) {
            (Some(index), true) => (index + 1) % handles.len(),
            (Some(0) | None, false) => handles.len() - 1,
            (Some(index), false) => index - 1,
            (None, true) => 0,
        };
        let Some(handle) = handles.get(index) else {
            return;
        };
        handle.focus(window, cx);
        if index < pending.len() {
            self.list_scroll_handle.scroll_to_item(index);
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        window.prevent_default();
        let focused_install = self
            .entry_focus_handles
            .iter()
            .find(|(_, handle)| handle.is_focused(window))
            .map(|(install, _)| install.clone());
        if let Some(install) = focused_install {
            self.toggle_install(install, cx);
        } else if self.enable_focus_handle.is_focused(window) {
            self.enable_and_dismiss(cx);
        } else if self.cancel_focus_handle.is_focused(window) {
            self.acknowledge_and_dismiss(cx);
        } else {
            self.confirm_and_dismiss(cx);
        }
    }

    fn install_label(&self, pending: &PendingToolInstall, cx: &App) -> String {
        let remote_location = || {
            self.project
                .upgrade()
                .and_then(|project| project.read(cx).remote_connection_options(cx))
                .map(RemoteHostLocation::from)
                .map(|host| match host.user_name {
                    Some(user) => format!("{user}@{}", host.host_identifier),
                    None => host.host_identifier.to_string(),
                })
                .unwrap_or_else(|| "Remote host".to_string())
        };
        let location = match &pending.origin {
            ToolInstallOrigin::Local
                if pending.install.worktree_id.is_some()
                    && self
                        .project
                        .upgrade()
                        .is_some_and(|project| !project.read(cx).is_local()) =>
            {
                format!("{}; approved on this computer", remote_location())
            }
            ToolInstallOrigin::Local => "This computer".to_string(),
            ToolInstallOrigin::Remote(_) => remote_location(),
        };
        let worktree = pending.install.worktree_id.and_then(|id| {
            self.project.upgrade().and_then(|project| {
                project
                    .read(cx)
                    .worktree_store()
                    .read(cx)
                    .worktrees()
                    .find(|worktree| worktree.read(cx).id() == id)
                    .map(|worktree| worktree.read(cx).abs_path().display().to_string())
            })
        });
        match worktree {
            Some(path) => format!("{} — {path} ({location})", pending.install.tool),
            None => {
                let scope = match (pending.install.worktree_id, &pending.origin) {
                    (Some(worktree_id), _) => format!("Worktree {}", worktree_id.to_proto()),
                    (None, ToolInstallOrigin::Local) => "App-wide".to_string(),
                    (None, ToolInstallOrigin::Remote(_)) => "Host session".to_string(),
                };
                format!("{} — {scope} ({location})", pending.install.tool)
            }
        }
    }

    fn enable_and_dismiss(&mut self, cx: &mut Context<Self>) {
        if self.enabling_downloads {
            return;
        }
        self.enabling_downloads = true;
        self.approval_error = None;
        let completion =
            update_settings_file_with_completion(<dyn Fs>::global(cx), cx, |settings, _| {
                settings.project.allow_binary_downloads = Some(true);
            });
        self._enable_downloads = cx.spawn(async move |modal, cx| {
            let result = completion
                .await
                .map_err(anyhow::Error::from)
                .and_then(|result| result);
            modal
                .update(cx, |modal, cx| {
                    modal.enabling_downloads = false;
                    match result {
                        Ok(()) => {
                            modal.decided = Some(DismissOutcome::EnabledDownloads);
                            cx.emit(DismissEvent);
                        }
                        Err(error) => {
                            modal.approval_error = Some(SharedString::from(format!(
                                "Could not enable global downloads: {error}"
                            )));
                            cx.notify();
                        }
                    }
                })
                .ok();
        });
        cx.notify();
    }
}

impl Render for BinaryDownloadsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_enable_globally =
            self.enabling_downloads || !ProjectSettings::get_global(cx).allow_binary_downloads;
        let pending = self.pending_tool_installs(cx);
        self.selected.retain(|install| pending.contains(install));
        self.entry_focus_handles
            .retain(|install, _| pending.contains(install));
        for install in &pending {
            self.entry_focus_handles
                .entry(install.clone())
                .or_insert_with(|| cx.focus_handle());
        }
        let project = self.project.upgrade();
        let is_guest = project
            .as_ref()
            .is_some_and(|project| project.read(cx).is_via_collab());
        let scope = project
            .as_ref()
            .and_then(|project| scope_for_project(project.read(cx), cx))
            .unwrap_or(self.scope);
        let install_selected_count = self.selected.len();

        let modal = AlertModal::new("binary-downloads-modal")
            .width(rems(40.))
            .header(
                v_flex()
                    .p_3()
                    .gap_1()
                    .rounded_t_md()
                    .bg(cx.theme().colors().editor_background.opacity(0.5))
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Icon::new(IconName::CloudDownload).color(Color::Warning))
                            .child(Label::new("Tool Permission Required")),
                    )
                    .child(
                        div().pl(IconSize::default().rems() + rems(0.5)).child(
                            Label::new("Review tools that need permission to run or download.")
                                .color(Color::Muted),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Where `allow_binary_downloads` is false, new tool downloads require approval.").color(Color::Muted))
                    .child(Label::new("Installed tools may still need approval to run if they can download dependencies.").color(Color::Muted))
                    .child(Label::new("Approval includes child processes and their downloads; it is not a network sandbox.").color(Color::Muted))
                    .when(!is_guest, |element| element.child(Label::new(scope.override_hint()).color(Color::Muted)))
                    .when(is_guest, |element| element.child(Label::new("Only requests for this computer can be approved here; the host controls its own tools.").color(Color::Muted)))
                    .when(can_enable_globally, |element| element.child(Label::new("Enable Global Downloads changes the default for all projects, including connected remote projects; project overrides stay unchanged.").color(Color::Muted)))
                    .when(!pending.is_empty(), |element| {
                        element.child(
                            v_flex()
                                .pt_2()
                                .gap_1()
                                .border_t_1()
                                .border_color(cx.theme().colors().border_variant)
                                .child(
                                    Label::new("Allow execution and downloads in the selected scopes for this session, then retry:")
                                        .color(Color::Default),
                                )
                                .child(div().vertical_scrollbar_for(&self.list_scroll_handle, window, cx).child(
                                    v_flex()
                                        .id("pending-tool-installs")
                                        .max_h(rems(16.))
                                        .overflow_y_scroll()
                                        .track_scroll(&self.list_scroll_handle)
                                        .children(pending.iter().enumerate().map(|(index, install)| {
                                            let checked = self.selected.contains(install);
                                            let label = self.install_label(install, cx);
                                            let handle = &self.entry_focus_handles[install];
                                            let install = install.clone();
                                            h_flex()
                                                .id(handle)
                                                .flex_shrink_0()
                                                .tab_index(index as isize)
                                                .tab_stop(!self.enabling_downloads)
                                                .track_focus(handle)
                                                .role(gpui::accesskit::Role::CheckBox)
                                                .aria_label(label.clone())
                                                .aria_toggled(if checked { gpui::accesskit::Toggled::True } else { gpui::accesskit::Toggled::False })
                                                .rounded_sm()
                                                .focus_visible(|style| style.bg(cx.theme().colors().element_hover))
                                                .child(Checkbox::new("install-tool-checkbox", ToggleState::from(checked)).label(label).disabled(self.enabling_downloads))
                                                .when(!self.enabling_downloads, |element| element.on_click(cx.listener(move |modal, _, _, cx| {
                                                    modal.toggle_install(install.clone(), cx);
                                                    cx.stop_propagation();
                                                })))
                                        }))
                                )),
                        )
                    })
                    .when_some(self.approval_error.clone(), |element, error| {
                        element.child(Label::new(error).color(Color::Error))
                    }),
            )
            .footer(
                h_flex()
                    .px_3()
                    .pb_3()
                    .gap_1()
                    .justify_end()
                    .flex_wrap()
                    .when(can_enable_globally, |element| {
                        element.child(
                            Button::new("enable-downloads", if self.enabling_downloads { "Enabling Global Downloads…" } else { "Enable Global Downloads" })
                                .loading(self.enabling_downloads)
                                .disabled(self.enabling_downloads)
                                .tab_index(pending.len() as isize)
                                .track_focus(&self.enable_focus_handle)
                                .on_click(cx.listener(|modal, _, _, cx| {
                                    modal.enable_and_dismiss(cx);
                                    cx.stop_propagation();
                                })),
                        )
                    })
                    .child(
                        Button::new("cancel", "Not Now")
                            .disabled(self.enabling_downloads)
                            .tab_index(pending.len() as isize + 1)
                            .track_focus(&self.cancel_focus_handle)
                            .key_binding(KeyBinding::for_action(&ToggleWorktreeSecurity, cx))
                            .on_click(cx.listener(|modal, _, _, cx| {
                                modal.acknowledge_and_dismiss(cx);
                                cx.stop_propagation();
                            })),
                    )
                    .when(!pending.is_empty(), |element| element.child(
                        Button::new("install-selected", "Allow Execution and Downloads")
                            .tab_index(pending.len() as isize + 2)
                            .track_focus(&self.confirm_focus_handle)
                            .disabled(self.enabling_downloads || install_selected_count == 0)
                            .style(ButtonStyle::Filled)
                            .layer(ui::ElevationIndex::ModalSurface)
                            .key_binding(KeyBinding::for_action(&menu::Confirm, cx))
                            .on_click(cx.listener(|modal, _, _, cx| {
                                modal.confirm_and_dismiss(cx);
                                cx.stop_propagation();
                            })),
                    )),
            );
        div()
            .key_context("BinaryDownloadsModal")
            .on_action(cx.listener(|modal, _: &ToggleWorktreeSecurity, _, cx| {
                modal.acknowledge_and_dismiss(cx);
            }))
            .on_action(cx.listener(|modal, _: &menu::Cancel, _, cx| {
                modal.acknowledge_and_dismiss(cx);
            }))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(|modal, _: &menu::SelectNext, window, cx| {
                modal.move_focus(true, window, cx);
            }))
            .on_action(cx.listener(|modal, _: &menu::SelectPrevious, window, cx| {
                modal.move_focus(false, window, cx);
            }))
            .track_focus(&self.focus_handle)
            .tab_group()
            .tab_stop(false)
            .child(modal)
            .into_any_element()
    }
}

pub fn pending_tool_installs_for_project(project: &Project, cx: &App) -> Vec<PendingToolInstall> {
    BinaryDownloads::try_get_global(cx)
        .map(|store| {
            store
                .read(cx)
                .pending_tool_installs_for_project(project, cx)
        })
        .unwrap_or_default()
}

/// Whether the given project has the `allow_binary_downloads` setting
/// effectively turned off (globally and/or via a per-worktree override).
pub fn project_blocks_binary_downloads(project: &Project, cx: &App) -> bool {
    scope_for_project(project, cx).is_some()
}

pub fn scope_for_project(project: &Project, cx: &App) -> Option<DisabledScope> {
    let global_disabled = !ProjectSettings::get_global(cx).allow_binary_downloads;
    let pending = pending_tool_installs_for_project(project, cx);
    if project.is_via_collab() {
        return (global_disabled || !pending.is_empty()).then_some(DisabledScope::Global);
    }

    let mut any_project_disabled = false;
    let mut any_project_enabled = false;
    let worktree_ids: Vec<_> = project
        .worktree_store()
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| worktree.read(cx).id())
        .collect();
    for worktree_id in worktree_ids {
        let worktree_disabled = !ProjectSettings::get(
            Some(SettingsLocation {
                worktree_id,
                path: RelPath::empty(),
            }),
            cx,
        )
        .allow_binary_downloads;
        if worktree_disabled {
            any_project_disabled = true;
        } else {
            any_project_enabled = true;
        }
    }

    match (global_disabled, any_project_disabled, any_project_enabled) {
        (true, false, true) if !pending.is_empty() => Some(DisabledScope::Global),
        (false, false, _) if !pending.is_empty() => Some(DisabledScope::Project),
        // Globally disabled but every visible worktree overrode it back on.
        (true, false, true) => None,
        // No worktrees but global allows downloads.
        (false, false, false) => None,
        // Every worktree allows downloads and so does the global setting.
        (false, false, true) => None,
        // Some worktree disables but global allows.
        (false, true, _) => Some(DisabledScope::Project),
        // Global disables AND at least one worktree keeps it disabled.
        (true, true, _) => Some(DisabledScope::Both),
        // No worktrees but globally disabled.
        (true, false, false) => Some(DisabledScope::Global),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{sync::Arc, time::Duration};

    use fs::FakeFs;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use project::Project;
    use serde_json::json;
    use settings::{LocalSettingsKind, LocalSettingsPath, SettingsStore};
    use util::path;

    use crate::tests::init_test;

    #[gpui::test]
    async fn test_scope_for_project_overrides_and_pending_requests(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        for (global, local, expected) in [
            (true, None, None),
            (false, None, Some(DisabledScope::Both)),
            (true, Some(false), Some(DisabledScope::Project)),
            (false, Some(true), None),
        ] {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(global)
                });
                store
                    .set_local_settings(
                        worktree_id,
                        LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                        LocalSettingsKind::Settings,
                        local
                            .map(|local| format!("{{\"allow_binary_downloads\":{local}}}"))
                            .as_deref(),
                        cx,
                    )
                    .unwrap();
            });
            project.read_with(cx, |project, cx| {
                assert_eq!(scope_for_project(project, cx), expected)
            });
        }
        cx.update(|cx| {
            project::binary_downloads::request_tool_install(None, "global-tool", cx);
        });
        project.read_with(cx, |project, cx| {
            assert_eq!(scope_for_project(project, cx), Some(DisabledScope::Global))
        });
    }

    #[gpui::test]
    async fn test_modal_keyboard_selection_and_cancel(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            project::binary_downloads::init(cx);
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
            cx.bind_keys([
                gpui::KeyBinding::new("tab", menu::SelectNext, None),
                gpui::KeyBinding::new("shift-tab", menu::SelectPrevious, None),
                gpui::KeyBinding::new("enter", menu::Confirm, None),
                gpui::KeyBinding::new("escape", menu::Cancel, None),
                gpui::KeyBinding::new("ctrl-alt-s", ToggleWorktreeSecurity, None),
            ]);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;
        let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
        let receiver = store
            .update(cx, |store, cx| store.request_tool_install(None, "tool", cx))
            .unwrap();

        for cancel_key in ["ctrl-alt-s", "escape", "tab tab enter"] {
            let (modal, cx) = cx.add_window_view(|window, cx| {
                let modal = BinaryDownloadsModal::new(&project, DisabledScope::Both, cx);
                modal.focus_handle.focus(window, cx);
                modal
            });
            cx.simulate_keystrokes("tab");
            press_key(cx, "space");
            modal.read_with(cx, |modal, cx| {
                assert_eq!(
                    modal.selected,
                    modal.pending_tool_installs(cx).into_iter().collect()
                );
            });
            cx.simulate_keystrokes(cancel_key);
            modal.read_with(cx, |modal, _| {
                assert_eq!(modal.decided, Some(DismissOutcome::Acknowledged))
            });
            assert!(!*receiver.borrow());
        }

        let (modal, cx) = cx.add_window_view(|window, cx| {
            let modal = BinaryDownloadsModal::new(&project, DisabledScope::Both, cx);
            modal.focus_handle.focus(window, cx);
            modal
        });
        cx.simulate_keystrokes("tab");
        press_key(cx, "enter");
        modal.read_with(cx, |modal, cx| {
            assert_eq!(
                modal.selected,
                modal.pending_tool_installs(cx).into_iter().collect()
            );
        });
        assert!(!*receiver.borrow());
        let focused = cx.update(|window, cx| window.focused(cx));
        let other_receiver = store
            .update(cx, |store, cx| {
                store.request_tool_install(None, "another-tool", cx)
            })
            .unwrap();
        cx.run_until_parked();
        assert_eq!(cx.update(|window, cx| window.focused(cx)), focused);
        cx.simulate_keystrokes("tab tab tab");
        modal.update_in(cx, |modal, window, _| {
            assert!(modal.confirm_focus_handle.is_focused(window))
        });
        press_key(cx, "enter");
        assert!(*receiver.borrow());
        assert!(!*other_receiver.borrow());

        store.update(cx, |store, cx| {
            for index in 0..40 {
                store.request_tool_install(None, format!("tool-{index:02}"), cx);
            }
        });
        cx.run_until_parked();
        modal.update_in(cx, |modal, window, cx| {
            modal.focus_handle.focus(window, cx);
        });
        for _ in 0..41 {
            cx.simulate_keystrokes("tab");
        }
        modal.update_in(cx, |modal, window, _| {
            assert!(modal.list_scroll_handle.offset().y < px(0.));
            assert!(modal.list_scroll_handle.bounds().size.height <= window.rem_size() * 16.);
        });
        press_key(cx, "space");
        modal.read_with(cx, |modal, _| {
            assert_eq!(
                modal
                    .selected
                    .iter()
                    .map(|pending| pending.install.tool.as_ref())
                    .collect::<Vec<_>>(),
                vec!["tool-39"]
            );
        });
        cx.simulate_keystrokes("escape");
        store.read_with(cx, |store, cx| {
            assert!(!store.tool_download_allowed(None, "tool-39", cx));
        });
    }

    #[gpui::test]
    async fn test_modal_scopes_pending_installs_and_retains_failed_approval(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        cx.update(|cx| {
            project::binary_downloads::init(cx);
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        fs.insert_tree(path!("/other"), json!({ "b.rs": "" })).await;
        let project = Project::test(fs.clone(), [path!("/proj").as_ref()], cx).await;
        let other = Project::test(fs, [path!("/other").as_ref()], cx).await;
        let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
        let (origin, other_origin, worktree_id, other_worktree_id) = cx.update(|cx| {
            let origin = project.read(cx).worktree_store();
            let other_origin = other.read(cx).worktree_store();
            let worktree_id = origin.read(cx).worktrees().next().unwrap().read(cx).id();
            let other_worktree_id = other_origin
                .read(cx)
                .worktrees()
                .next()
                .unwrap()
                .read(cx)
                .id();
            let client = project.read(cx).client().into();
            project::binary_downloads::track_remote_binary_downloads(
                origin.clone(),
                (client, client::ProjectId(1)),
                cx,
            );
            (
                origin.downgrade(),
                other_origin.downgrade(),
                worktree_id,
                other_worktree_id,
            )
        });
        store.update(cx, |store, cx| {
            store.request_tool_install(None, "global", cx);
            store.request_tool_install(Some(worktree_id), "project", cx);
            store.request_tool_install(Some(other_worktree_id), "other-project", cx);
            let install = project::binary_downloads::ToolInstall {
                worktree_id: None,
                tool: "global".into(),
            };
            store.set_remote_pending_installs(origin.clone(), vec![install.clone()], cx);
            store.set_remote_pending_installs(other_origin, vec![install], cx);
        });
        let modal = cx.new(|cx| BinaryDownloadsModal::new(&project, DisabledScope::Both, cx));
        modal.update(cx, |modal, cx| {
            let pending = modal.pending_tool_installs(cx);
            assert_eq!(
                pending
                    .iter()
                    .map(|pending| (pending.install.tool.as_ref(), pending.origin.clone()))
                    .collect::<Vec<_>>(),
                vec![
                    ("global", ToolInstallOrigin::Local),
                    ("global", ToolInstallOrigin::Remote(origin.clone())),
                    ("project", ToolInstallOrigin::Local),
                ]
            );
            assert_eq!(
                modal.install_label(&pending[0], cx),
                "global — App-wide (This computer)"
            );
            assert_eq!(
                modal.install_label(&pending[1], cx),
                "global — Host session (Remote host)"
            );
            assert_eq!(
                modal.install_label(&pending[2], cx),
                format!("project — {} (This computer)", path!("/proj"))
            );
            let mut unavailable_worktree = pending[1].clone();
            unavailable_worktree.install.worktree_id = Some(other_worktree_id);
            assert_eq!(
                modal.install_label(&unavailable_worktree, cx),
                format!(
                    "global — Worktree {} (Remote host)",
                    other_worktree_id.to_proto()
                )
            );
            modal.selected.insert(pending[1].clone());
            modal.confirm_and_dismiss(cx);
            assert!(modal.approval_error.is_some());
            assert_eq!(modal.decided, None);
            assert_eq!(modal.selected, HashSet::from_iter([pending[1].clone()]));
            assert_eq!(modal.pending_tool_installs(cx), pending);
        });
        store.update(cx, |store, cx| {
            store.set_remote_pending_installs(origin, Vec::new(), cx);
        });
        modal.update(cx, |modal, cx| {
            modal.confirm_and_dismiss(cx);
            assert_eq!(modal.decided, Some(DismissOutcome::Acknowledged));
            assert_eq!(modal.selected, HashSet::default());
        });
        store.read_with(cx, |store, cx| {
            assert!(!store.tool_download_allowed(None, "global", cx))
        });
    }

    #[gpui::test]
    async fn test_enable_global_downloads_preserves_project_override(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(project::binary_downloads::init);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        cx.update(|cx| {
            <dyn Fs>::set_global(fs.clone(), cx);
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let project = Project::test(fs.clone(), [path!("/proj").as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store
                .set_local_settings(
                    worktree_id,
                    LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                    LocalSettingsKind::Settings,
                    Some(r#"{ "allow_binary_downloads": false }"#),
                    cx,
                )
                .unwrap();
        });
        cx.update(|cx| {
            update_settings_file_with_completion(fs.clone(), cx, |settings, _| {
                settings.project.allow_binary_downloads = Some(false);
            })
        })
        .await
        .unwrap()
        .unwrap();
        let settings_path = fs
            .paths(false)
            .into_iter()
            .find(|path| path.file_name().is_some_and(|name| name == "settings.json"))
            .unwrap();
        fs.remove_file(&settings_path, fs::RemoveOptions::default())
            .await
            .unwrap();
        fs.insert_tree(&settings_path, json!({})).await;
        let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
        let receiver = store
            .update(cx, |store, cx| {
                store.request_tool_install(Some(worktree_id), "tool", cx)
            })
            .unwrap();
        let (modal, cx) = cx
            .add_window_view(|_, cx| BinaryDownloadsModal::new(&project, DisabledScope::Both, cx));
        modal.update_in(cx, |modal, window, cx| {
            let pending = modal.pending_tool_installs(cx);
            modal.toggle_install(pending[0].clone(), cx);
            modal.enable_and_dismiss(cx);
            assert!(modal.enabling_downloads);
            modal.acknowledge_and_dismiss(cx);
            assert_eq!(modal.decided, None);
            modal.confirm_and_dismiss(cx);
            modal.toggle_install(pending[0].clone(), cx);
            assert_eq!(modal.decided, None);
            assert_eq!(modal.selected, pending.into_iter().collect());
            assert!(!*receiver.borrow());
            assert!(matches!(
                modal.on_before_dismiss(window, cx),
                DismissDecision::Pending
            ));
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();
        modal.update_in(cx, |modal, window, cx| {
            assert!(modal.approval_error.is_some());
            assert!(!modal.enabling_downloads);
            assert_eq!(modal.decided, None);
            assert!(!ProjectSettings::get_global(cx).allow_binary_downloads);
            assert!(matches!(
                modal.on_before_dismiss(window, cx),
                DismissDecision::Dismiss(true)
            ));
        });
        fs.remove_dir(&settings_path, fs::RemoveOptions::default())
            .await
            .unwrap();
        modal.update_in(cx, |modal, window, cx| {
            modal.enable_and_dismiss(cx);
            modal.acknowledge_and_dismiss(cx);
            assert_eq!(modal.decided, None);
            assert!(matches!(
                modal.on_before_dismiss(window, cx),
                DismissDecision::Pending
            ));
        });
        cx.executor().advance_clock(Duration::from_millis(200));
        cx.run_until_parked();
        let fs: Arc<dyn Fs> = fs;
        let settings_text = SettingsStore::load_settings(&fs).await.unwrap();
        modal.update_in(cx, |modal, window, cx| {
            assert!(!modal.enabling_downloads);
            assert!(modal.approval_error.is_none());
            assert_eq!(modal.decided, Some(DismissOutcome::EnabledDownloads));
            assert!(matches!(
                modal.on_before_dismiss(window, cx),
                DismissDecision::Dismiss(true)
            ));
        });
        assert!(!*receiver.borrow());
        let parsed =
            settings::parse_json_with_comments::<serde_json::Value>(&settings_text).unwrap();
        assert_eq!(parsed["allow_binary_downloads"], json!(true));
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.set_user_settings(&settings_text, cx).unwrap()
            });
            assert!(ProjectSettings::get_global(cx).allow_binary_downloads);
            assert!(!BinaryDownloadsStore::allow_binary_downloads(
                Some(worktree_id),
                cx
            ));
        });
    }

    fn press_key(cx: &mut VisualTestContext, key: &str) {
        cx.simulate_keystrokes(key);
        cx.update(|window, cx| {
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent {
                    keystroke: gpui::Keystroke::parse(key).unwrap(),
                }),
                cx,
            );
        });
        cx.run_until_parked();
    }
}
