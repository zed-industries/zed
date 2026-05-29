//! A modal that informs the user that Zed is not allowed to download tool
//! binaries (LSPs, MCP servers, formatters, debug adapters, npm packages...)
//! because of the `allow_binary_downloads` setting.
//!
//! The look and layout mirrors [`crate::security_modal::SecurityModal`], so the
//! two restrictions feel like a coherent set.

use collections::HashSet;
use fs::Fs;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, WeakEntity};
use project::{
    Project,
    binary_downloads::{BinaryDownloads, BinaryDownloadsEvent, BinaryDownloadsStore, ToolInstall},
    project_settings::ProjectSettings,
};
use remote::RemoteConnectionOptions;
use settings::{Settings, SettingsLocation, WorktreeId, update_settings_file};
use theme::ActiveTheme;
use ui::{AlertModal, ButtonStyle, Checkbox, KeyBinding, ListBulletItem, ToggleState, prelude::*};
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
                "To allow downloads only for this project, set \"allow_binary_downloads\": true in .zed/settings.json."
            }
            DisabledScope::Project => {
                "To allow downloads for this project, set \"allow_binary_downloads\": true in .zed/settings.json (or remove the project override)."
            }
            DisabledScope::Both => {
                "To allow downloads only for this project while keeping the global default disabled, set \"allow_binary_downloads\": true in .zed/settings.json."
            }
        }
    }

    /// Whether the global `allow_binary_downloads` setting is the one
    /// currently disabling downloads. When `true`, the modal can offer an
    /// "Enable Downloads" shortcut that flips the global value back on.
    fn global_disabled(self) -> bool {
        matches!(self, DisabledScope::Global | DisabledScope::Both)
    }
}

/// Outcome the user explicitly picked before the modal dismisses. Mirrors
/// [`crate::security_modal::SecurityModal`]'s `trusted: Option<bool>`: while it
/// remains `None`, escape/click-outside dismissal is blocked so the user has
/// to make a deliberate choice.
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
    /// Visible worktrees of the project this modal was opened for; pending
    /// installs scoped to other projects are hidden.
    worktree_ids: Vec<WorktreeId>,
    /// Tools the user ticked to install once. Empty by default.
    selected: HashSet<ToolInstall>,
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
            None => DismissDecision::Dismiss(false),
        }
    }
}

impl BinaryDownloadsModal {
    pub fn new(project: &Entity<Project>, scope: DisabledScope, cx: &mut Context<Self>) -> Self {
        let store_entity = BinaryDownloads::try_get_global(cx);
        let store_subscription = store_entity
            .as_ref()
            .map(|store| cx.subscribe(store, |_, _, _: &BinaryDownloadsEvent, cx| cx.notify()));
        let worktree_ids = project
            .read(cx)
            .worktree_store()
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).id())
            .collect();
        Self {
            scope,
            focus_handle: cx.focus_handle(),
            decided: None,
            store: store_entity.map(|store| store.downgrade()),
            worktree_ids,
            selected: HashSet::default(),
            _store_subscription: store_subscription,
        }
    }

    /// Pending one-off install requests relevant to this modal's project,
    /// sorted for a stable checkbox order.
    fn pending_tool_installs(&self, cx: &App) -> Vec<ToolInstall> {
        let Some(store) = self.store.as_ref().and_then(|store| store.upgrade()) else {
            return Vec::new();
        };
        let mut installs = store
            .read(cx)
            .pending_tool_installs()
            .into_iter()
            .filter(|install| match install.worktree_id {
                Some(worktree_id) => self.worktree_ids.contains(&worktree_id),
                None => true,
            })
            .collect::<Vec<_>>();
        installs.sort_by(|a, b| a.tool.cmp(&b.tool));
        installs
    }

    pub(crate) fn acknowledge_and_dismiss(&mut self, cx: &mut Context<Self>) {
        self.decided = Some(DismissOutcome::Acknowledged);
        cx.emit(DismissEvent);
    }

    /// Approves the ticked tools for a one-off install without touching the
    /// `allow_binary_downloads` setting, then dismisses. With nothing ticked
    /// this is equivalent to acknowledging.
    fn confirm_and_dismiss(&mut self, cx: &mut Context<Self>) {
        if self.selected.is_empty() {
            self.acknowledge_and_dismiss(cx);
            return;
        }
        if let Some(store) = self.store.as_ref().and_then(|store| store.upgrade()) {
            let selected = std::mem::take(&mut self.selected);
            telemetry::event!(
                "Install Tools Once",
                source = "Binary Downloads Modal",
                count = selected.len()
            );
            store.update(cx, |store, cx| {
                for install in selected {
                    store.resolve_tool_install(install.worktree_id, install.tool, true, cx);
                }
            });
        }
        self.decided = Some(DismissOutcome::InstalledTools);
        cx.emit(DismissEvent);
    }

    fn enable_and_dismiss(&mut self, cx: &mut Context<Self>) {
        update_settings_file(<dyn Fs>::global(cx), cx, |settings, _| {
            settings.project.allow_binary_downloads = Some(true);
        });
        self.decided = Some(DismissOutcome::EnabledDownloads);
        cx.emit(DismissEvent);
    }
}

impl Render for BinaryDownloadsModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let override_hint = self.scope.override_hint();
        let can_enable_globally = self.scope.global_disabled();
        let pending = self.pending_tool_installs(cx);
        let install_selected_count = self.selected.len();

        AlertModal::new("binary-downloads-modal")
            .width(rems(40.))
            .key_context("BinaryDownloadsModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(|this, _: &ToggleWorktreeSecurity, _, cx| {
                this.confirm_and_dismiss(cx);
            }))
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
                            .child(Label::new("Binary Downloads Disabled")),
                    )
                    .child(
                        div().pl(IconSize::default().rems() + rems(0.5)).child(
                            Label::new("`allow_binary_downloads` is disabled in the settings.")
                                .color(Color::Muted),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Label::new("Zed will not download or install any new tool binaries.")
                            .color(Color::Muted),
                    )
                    .child(
                        v_flex()
                            .child(Label::new("This blocks:").color(Color::Muted))
                            .child(ListBulletItem::new(
                                "Language servers, formatters, prettier",
                            ))
                            .child(ListBulletItem::new(
                                "Managed Node.js and npm package installs",
                            ))
                            .child(ListBulletItem::new("Debug adapters (CodeLLDB, Delve, JS)"))
                            .child(ListBulletItem::new("MCP servers installed via extensions"))
                            .child(ListBulletItem::new(
                                "Files fetched via `download_file` from extensions",
                            )),
                    )
                    .child(Label::new("Already installed tools will be run.").color(Color::Muted))
                    .child(Label::new(override_hint).color(Color::Muted))
                    .when(!pending.is_empty(), |this| {
                        let modal = cx.entity().downgrade();
                        this.child(
                            v_flex()
                                .pt_2()
                                .gap_1()
                                .border_t_1()
                                .border_color(cx.theme().colors().border_variant)
                                .child(
                                    Label::new("Or allow installing just these tools once:")
                                        .color(Color::Default),
                                )
                                .children(pending.iter().enumerate().map(|(ix, install)| {
                                    let checked = self.selected.contains(install);
                                    let install = install.clone();
                                    let modal = modal.clone();
                                    Checkbox::new(
                                        SharedString::from(format!("install-tool-{ix}")),
                                        ToggleState::from(checked),
                                    )
                                    .label(install.tool.clone())
                                    .on_click(
                                        move |state, _, cx| {
                                            let selected = *state == ToggleState::Selected;
                                            let install = install.clone();
                                            modal
                                                .update(cx, |modal, cx| {
                                                    if selected {
                                                        modal.selected.insert(install);
                                                    } else {
                                                        modal.selected.remove(&install);
                                                    }
                                                    cx.notify();
                                                })
                                                .ok();
                                        },
                                    )
                                })),
                        )
                    }),
            )
            .footer(
                h_flex()
                    .px_3()
                    .pb_3()
                    .gap_1()
                    .justify_end()
                    .when(can_enable_globally, |this| {
                        this.child(
                            Button::new("enable-downloads", "Enable Downloads").on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.enable_and_dismiss(cx);
                                    cx.stop_propagation();
                                }),
                            ),
                        )
                    })
                    .child(
                        Button::new(
                            "ok",
                            if install_selected_count > 0 {
                                "Install Selected"
                            } else {
                                "OK"
                            },
                        )
                        .style(ButtonStyle::Filled)
                        .layer(ui::ElevationIndex::ModalSurface)
                        .key_binding(
                            KeyBinding::for_action(&ToggleWorktreeSecurity, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.confirm_and_dismiss(cx);
                            cx.stop_propagation();
                        })),
                    ),
            )
            .into_any_element()
    }
}

/// Whether the given project has the `allow_binary_downloads` setting
/// effectively turned off (globally and/or via a per-worktree override).
///
/// Returns `false` for collab guests (whose settings can't take effect on the
/// host) and Docker projects (sandboxed in a disposable container) — both
/// cases skip the indicator and the modal.
pub fn project_blocks_binary_downloads(project: &Project, cx: &App) -> bool {
    if project.is_via_collab() {
        return false;
    }
    if let Some(RemoteConnectionOptions::Docker(_)) = project.remote_connection_options(cx) {
        return false;
    }
    scope_for_project(project, cx).is_some()
}

/// Returns the scope that disabled downloads for the given project, or `None`
/// when downloads are effectively allowed everywhere in the project.
pub fn scope_for_project(project: &Project, cx: &App) -> Option<DisabledScope> {
    let global_disabled = !ProjectSettings::get_global(cx).allow_binary_downloads;

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

    use std::sync::Arc;

    use fs::FakeFs;
    use gpui::{TestAppContext, UpdateGlobal};
    use project::Project;
    use serde_json::json;
    use settings::{LocalSettingsKind, LocalSettingsPath, SettingsStore};
    use util::path;

    use crate::tests::init_test;

    #[gpui::test]
    async fn test_scope_for_project_default_allows_downloads(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;

        project.read_with(cx, |project, cx| {
            assert_eq!(scope_for_project(project, cx), None);
            assert_eq!(project_blocks_binary_downloads(project, cx), false);
        });
    }

    #[gpui::test]
    async fn test_scope_for_project_global_disabled(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;

        project.read_with(cx, |project, cx| {
            assert_eq!(scope_for_project(project, cx), Some(DisabledScope::Both));
            assert_eq!(project_blocks_binary_downloads(project, cx), true);
        });
    }

    #[gpui::test]
    async fn test_scope_for_project_global_disabled_but_project_overrides(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store
                .set_local_settings(
                    worktree_id,
                    LocalSettingsPath::InWorktree(Arc::from(util::rel_path::RelPath::empty())),
                    LocalSettingsKind::Settings,
                    Some(r#"{ "allow_binary_downloads": true }"#),
                    cx,
                )
                .unwrap();
        });

        project.read_with(cx, |project, cx| {
            assert_eq!(scope_for_project(project, cx), None);
            assert_eq!(project_blocks_binary_downloads(project, cx), false);
        });
    }

    #[gpui::test]
    async fn test_scope_for_project_only_project_disabled(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store
                .set_local_settings(
                    worktree_id,
                    LocalSettingsPath::InWorktree(Arc::from(util::rel_path::RelPath::empty())),
                    LocalSettingsKind::Settings,
                    Some(r#"{ "allow_binary_downloads": false }"#),
                    cx,
                )
                .unwrap();
        });

        project.read_with(cx, |project, cx| {
            assert_eq!(scope_for_project(project, cx), Some(DisabledScope::Project));
            assert_eq!(project_blocks_binary_downloads(project, cx), true);
        });
    }

    #[gpui::test]
    async fn test_modal_lists_and_approves_pending_tool_installs(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| project::binary_downloads::init(cx));
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
        let receiver = store
            .update(cx, |store, cx| {
                store.request_tool_install(Some(worktree_id), "rust-analyzer", cx)
            })
            .expect("a waiter is returned while downloads are disabled");

        let modal = cx
            .update(|cx| cx.new(|cx| BinaryDownloadsModal::new(&project, DisabledScope::Both, cx)));

        let pending = modal.read_with(cx, |modal, cx| modal.pending_tool_installs(cx));
        assert_eq!(
            pending
                .iter()
                .map(|install| install.tool.to_string())
                .collect::<Vec<_>>(),
            vec!["rust-analyzer".to_string()],
            "the modal should list the pending one-off install"
        );

        // Nothing is ticked by default, so the install stays pending.
        modal.read_with(cx, |modal, _| assert_eq!(modal.selected.is_empty(), true));

        modal.update(cx, |modal, cx| {
            modal.selected.insert(pending[0].clone());
            modal.confirm_and_dismiss(cx);
        });
        cx.run_until_parked();

        assert_eq!(
            *receiver.borrow(),
            true,
            "approving a ticked tool should fire its install waiter"
        );
        let still_pending = store.read_with(cx, |store, _| store.pending_tool_installs());
        assert_eq!(
            still_pending.is_empty(),
            true,
            "approved tools are removed from the pending list"
        );
    }

    #[gpui::test]
    async fn test_modal_refreshes_when_new_install_requested(cx: &mut TestAppContext) {
        use std::{cell::RefCell, rc::Rc};

        init_test(cx);
        cx.update(|cx| project::binary_downloads::init(cx));
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.allow_binary_downloads = Some(false);
                });
            });
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/proj"), json!({ "a.rs": "" })).await;
        let project = Project::test(fs, [path!("/proj").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());

        let modal = cx
            .update(|cx| cx.new(|cx| BinaryDownloadsModal::new(&project, DisabledScope::Both, cx)));
        modal.read_with(cx, |modal, cx| {
            assert_eq!(modal.pending_tool_installs(cx).is_empty(), true)
        });

        let notifications = Rc::new(RefCell::new(0usize));
        cx.update({
            let notifications = notifications.clone();
            |cx| {
                cx.observe(&modal, move |_, _| *notifications.borrow_mut() += 1)
                    .detach();
            }
        });

        store.update(cx, |store, cx| {
            store.request_tool_install(Some(worktree_id), "gopls", cx)
        });
        cx.run_until_parked();

        assert_eq!(
            *notifications.borrow() >= 1,
            true,
            "the open modal should be notified when a new install is requested"
        );
        let pending = modal.read_with(cx, |modal, cx| {
            modal
                .pending_tool_installs(cx)
                .into_iter()
                .map(|install| install.tool.to_string())
                .collect::<Vec<_>>()
        });
        assert_eq!(pending, vec!["gopls".to_string()]);
    }
}
