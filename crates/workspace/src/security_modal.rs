//! A UI interface for managing the [`TrustedWorktrees`] data.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, HashSet};
use gpui::{DismissEvent, EventEmitter, FocusHandle, Focusable, WeakEntity};

use project::{
    WorktreeId,
    trusted_worktrees::{PathTrust, RemoteHostLocation, TrustedWorktrees},
    worktree_store::WorktreeStore,
};
use smallvec::SmallVec;
use theme::ActiveTheme;
use ui::{
    AlertModal, Checkbox, FluentBuilder, KeyBinding, ListBulletItem, ToggleState, prelude::*,
};

use crate::{DismissDecision, ModalView, ToggleWorktreeSecurity};

pub struct SecurityModal {
    restricted_paths: HashMap<WorktreeId, RestrictedPath>,
    home_dir: Option<PathBuf>,
    trust_parents: bool,
    worktree_store: WeakEntity<WorktreeStore>,
    remote_host: Option<RemoteHostLocation>,
    focus_handle: FocusHandle,
    trusted: Option<bool>,
}

#[derive(Debug, PartialEq, Eq)]
struct RestrictedPath {
    abs_path: Arc<Path>,
    is_file: bool,
    host: Option<RemoteHostLocation>,
}

impl Focusable for SecurityModal {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for SecurityModal {}

impl ModalView for SecurityModal {
    fn fade_out_background(&self) -> bool {
        true
    }

    fn on_before_dismiss(&mut self, _: &mut Window, _: &mut Context<Self>) -> DismissDecision {
        match self.trusted {
            Some(false) => telemetry::event!("Open in Restricted", source = "Worktree Trust Modal"),
            Some(true) => telemetry::event!("Trust and Continue", source = "Worktree Trust Modal"),
            None => telemetry::event!("Dismissed", source = "Worktree Trust Modal"),
        }
        DismissDecision::Dismiss(true)
    }
}

impl Render for SecurityModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.restricted_paths.is_empty() {
            self.dismiss(cx);
            return v_flex().into_any_element();
        }

        let header_label = if self.restricted_paths.len() == 1 {
            "Unrecognized Project"
        } else {
            "Unrecognized Projects"
        };

        let trust_label = self.build_trust_label();

        AlertModal::new("security-modal")
            .width(rems(40.))
            .key_context("SecurityModal")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(|this, _: &menu::Confirm, _window, cx| {
                this.trust_and_dismiss(cx);
            }))
            .on_action(cx.listener(|security_modal, _: &ToggleWorktreeSecurity, _window, cx| {
                security_modal.trusted = Some(false);
                security_modal.dismiss(cx);
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
                            .child(Icon::new(IconName::Warning).color(Color::Warning))
                            .child(Label::new(header_label)),
                    )
                    .children(self.restricted_paths.values().filter_map(|restricted_path| {
                        let abs_path = if restricted_path.is_file {
                            restricted_path.abs_path.parent()
                        } else {
                            Some(restricted_path.abs_path.as_ref())
                        }?;
                        let label = match &restricted_path.host {
                            Some(remote_host) => match &remote_host.user_name {
                                Some(user_name) => format!(
                                    "{} ({}@{})",
                                    self.shorten_path(abs_path).display(),
                                    user_name,
                                    remote_host.host_identifier
                                ),
                                None => format!(
                                    "{} ({})",
                                    self.shorten_path(abs_path).display(),
                                    remote_host.host_identifier
                                ),
                            },
                            None => self.shorten_path(abs_path).display().to_string(),
                        };
                        Some(h_flex()
                            .pl(IconSize::default().rems() + rems(0.5))
                            .child(Label::new(label).color(Color::Muted)))
                    })),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        v_flex()
                            .child(
                                Label::new(
                                    "Untrusted projects are opened in Restricted Mode to protect your system.",
                                )
                                .color(Color::Muted),
                            )
                            .child(
                                Label::new(
                                    "Review .zed/settings.json for any extensions or commands configured by this project.",
                                )
                                .color(Color::Muted),
                            ),
                    )
                    .child(
                        v_flex()
                            .child(Label::new("Restricted Mode prevents:").color(Color::Muted))
                            .child(ListBulletItem::new("Project settings from being applied"))
                            .child(ListBulletItem::new("Language servers from running"))
                            .child(ListBulletItem::new("MCP Server integrations from installing")),
                    )
                    .map(|this| match trust_label {
                        Some(trust_label) => this.child(
                            Checkbox::new("trust-parents", ToggleState::from(self.trust_parents))
                                .label(trust_label)
                                .on_click(cx.listener(
                                    |security_modal, state: &ToggleState, _, cx| {
                                        security_modal.trust_parents = state.selected();
                                        cx.notify();
                                        cx.stop_propagation();
                                    },
                                )),
                        ),
                        None => this,
                    }),
            )
            .footer(
                h_flex()
                    .px_3()
                    .pb_3()
                    .gap_1()
                    .justify_end()
                    .child(
                        Button::new("rm", "Stay in Restricted Mode")
                            .key_binding(
                                KeyBinding::for_action(
                                    &ToggleWorktreeSecurity,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(move |security_modal, _, _, cx| {
                                security_modal.trusted = Some(false);
                                security_modal.dismiss(cx);
                                cx.stop_propagation();
                            })),
                    )
                    .child(
                        Button::new("tc", "Trust and Continue")
                            .style(ButtonStyle::Filled)
                            .layer(ui::ElevationIndex::ModalSurface)
                            .key_binding(
                                KeyBinding::for_action(&menu::Confirm, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(move |security_modal, _, _, cx| {
                                security_modal.trust_and_dismiss(cx);
                                cx.stop_propagation();
                            })),
                    ),
            )
            .into_any_element()
    }
}

impl SecurityModal {
    pub fn new(
        worktree_store: WeakEntity<WorktreeStore>,
        remote_host: Option<impl Into<RemoteHostLocation>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            worktree_store,
            remote_host: remote_host.map(|host| host.into()),
            restricted_paths: HashMap::default(),
            focus_handle: cx.focus_handle(),
            trust_parents: false,
            home_dir: std::env::home_dir(),
            trusted: None,
        };
        this.refresh_restricted_paths(cx);

        this
    }

    fn build_trust_label(&self) -> Option<Cow<'static, str>> {
        let mut has_restricted_files = false;
        let available_parents = self
            .restricted_paths
            .values()
            .filter(|restricted_path| {
                has_restricted_files |= restricted_path.is_file;
                !restricted_path.is_file
            })
            .filter_map(|restricted_path| restricted_path.abs_path.parent())
            .collect::<SmallVec<[_; 2]>>();
        match available_parents.len() {
            0 => {
                if has_restricted_files {
                    Some(Cow::Borrowed("Trust all single files"))
                } else {
                    None
                }
            }
            1 => Some(Cow::Owned(format!(
                "Trust all projects in the {:} folder",
                self.shorten_path(available_parents[0]).display()
            ))),
            _ => Some(Cow::Borrowed("Trust all projects in the parent folders")),
        }
    }

    fn shorten_path<'a>(&self, path: &'a Path) -> Cow<'a, Path> {
        match &self.home_dir {
            Some(home_dir) => path
                .strip_prefix(home_dir)
                .map(|stripped| Path::new("~").join(stripped))
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed(path)),
            None => Cow::Borrowed(path),
        }
    }

    fn trust_and_dismiss(&mut self, cx: &mut Context<Self>) {
        if let Some((trusted_worktrees, worktree_store)) =
            TrustedWorktrees::try_get_global(cx).zip(self.worktree_store.upgrade())
        {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                let mut paths_to_trust = self
                    .restricted_paths
                    .keys()
                    .copied()
                    .map(PathTrust::Worktree)
                    .collect::<HashSet<_>>();
                if self.trust_parents {
                    paths_to_trust.extend(self.restricted_paths.values().filter_map(
                        |restricted_paths| {
                            if restricted_paths.is_file {
                                None
                            } else {
                                let parent_abs_path =
                                    restricted_paths.abs_path.parent()?.to_owned();
                                Some(PathTrust::AbsPath(parent_abs_path))
                            }
                        },
                    ));
                }
                trusted_worktrees.trust(&worktree_store, paths_to_trust, cx);
            });
        }

        self.trusted = Some(true);
        self.dismiss(cx);
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    pub fn refresh_restricted_paths(&mut self, cx: &mut Context<Self>) {
        if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
            if let Some(worktree_store) = self.worktree_store.upgrade() {
                let new_restricted_worktrees = trusted_worktrees
                    .read(cx)
                    .restricted_worktrees(&worktree_store, cx)
                    .into_iter()
                    .filter_map(|(worktree_id, abs_path)| {
                        let worktree = worktree_store.read(cx).worktree_for_id(worktree_id, cx)?;
                        Some((
                            worktree_id,
                            RestrictedPath {
                                abs_path,
                                is_file: worktree.read(cx).is_single_file(),
                                host: self.remote_host.clone(),
                            },
                        ))
                    })
                    .collect::<HashMap<_, _>>();

                if self.restricted_paths != new_restricted_worktrees {
                    self.trust_parents = false;
                    self.restricted_paths = new_restricted_worktrees;
                    cx.notify();
                }
            }
        } else if !self.restricted_paths.is_empty() {
            self.restricted_paths.clear();
            cx.notify();
        }
    }
}
