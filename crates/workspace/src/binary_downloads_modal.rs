//! A modal that informs the user that Zed is not allowed to download tool
//! binaries (LSPs, MCP servers, formatters, debug adapters, npm packages...)
//! because of the `allow_binary_downloads` setting.
//!
//! The look and layout mirrors [`crate::security_modal::SecurityModal`], so the
//! two restrictions feel like a coherent set.

use fs::Fs;
use gpui::{DismissEvent, EventEmitter, FocusHandle, Focusable};
use project::{Project, project_settings::ProjectSettings};
use settings::{Settings, SettingsLocation, update_settings_file};
use theme::ActiveTheme;
use ui::{AlertModal, ButtonStyle, KeyBinding, ListBulletItem, prelude::*};
use util::rel_path::RelPath;

use crate::{DismissDecision, ModalView, ToggleBinaryDownloadsRestriction};

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
}

pub struct BinaryDownloadsModal {
    scope: DisabledScope,
    focus_handle: FocusHandle,
    decided: Option<DismissOutcome>,
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
            None => DismissDecision::Dismiss(false),
        }
    }
}

impl BinaryDownloadsModal {
    pub fn new(scope: DisabledScope, cx: &mut Context<Self>) -> Self {
        Self {
            scope,
            focus_handle: cx.focus_handle(),
            decided: None,
        }
    }

    pub(crate) fn acknowledge_and_dismiss(&mut self, cx: &mut Context<Self>) {
        self.decided = Some(DismissOutcome::Acknowledged);
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

        AlertModal::new("binary-downloads-modal")
            .width(rems(40.))
            .key_context("BinaryDownloadsModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(
                cx.listener(|this, _: &ToggleBinaryDownloadsRestriction, _, cx| {
                    this.acknowledge_and_dismiss(cx);
                }),
            )
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
                    .child(Label::new(override_hint).color(Color::Muted)),
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
                        Button::new("ok", "OK")
                            .style(ButtonStyle::Filled)
                            .layer(ui::ElevationIndex::ModalSurface)
                            .key_binding(
                                KeyBinding::for_action(&ToggleBinaryDownloadsRestriction, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.acknowledge_and_dismiss(cx);
                                cx.stop_propagation();
                            })),
                    ),
            )
            .into_any_element()
    }
}

/// Whether the given project has the `allow_binary_downloads` setting
/// effectively turned off (globally and/or via a per-worktree override).
pub fn project_blocks_binary_downloads(project: &Project, cx: &App) -> bool {
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
}
