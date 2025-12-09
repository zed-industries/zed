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
    AlertModal, Button, ButtonCommon as _, ButtonStyle, Checkbox, Clickable as _, Color, Context,
    FluentBuilder, Headline, HeadlineSize, Icon, IconName, IconSize, IntoElement, KeyBinding,
    Label, LabelCommon as _, ListBulletItem, ParentElement as _, Render, Styled, ToggleState,
    Window, h_flex, rems, v_flex,
};

use crate::{DismissDecision, ModalView, ToggleWorktreeSecurity};

pub struct SecurityModal {
    restricted_paths: HashMap<Option<WorktreeId>, RestrictedPath>,
    home_dir: Option<PathBuf>,
    dismissed: bool,
    trust_parents: bool,
    worktree_store: WeakEntity<WorktreeStore>,
    remote_host: Option<RemoteHostLocation>,
    focus_handle: FocusHandle,
}

#[derive(Debug, PartialEq, Eq)]
struct RestrictedPath {
    abs_path: Option<Arc<Path>>,
    is_file: bool,
    host: Option<RemoteHostLocation>,
}

impl Focusable for SecurityModal {
    fn focus_handle(&self, _: &ui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for SecurityModal {}

impl ModalView for SecurityModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> DismissDecision {
        DismissDecision::Dismiss(self.dismissed)
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

// TODO kb keyboard navigation (ESC to dismiss?)
impl Render for SecurityModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.restricted_paths.is_empty() {
            self.dismiss(cx);
            return v_flex().into_any_element();
        }

        let header_label = if self.restricted_paths.len() == 1 {
            "Unrecognized Workspace"
        } else {
            "Unrecognized Workspaces"
        };

        let trust_label = self.build_trust_label();
        let focus_handle = self.focus_handle(cx);

        AlertModal::new("security-modal")
            .header(
                v_flex()
                    .p_3()
                    .bg(cx.theme().colors().background)
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::Warning).color(Color::Warning))
                            .child(Headline::new(header_label).size(HeadlineSize::Small)),
                    )
                    .children(self.restricted_paths.values().map(|restricted_path| {
                        let abs_path = restricted_path.abs_path.as_ref().and_then(|abs_path| {
                            if restricted_path.is_file {
                                abs_path.parent()
                            } else {
                                Some(abs_path.as_ref())
                            }
                        });

                        let label = match abs_path {
                            Some(abs_path) => match &restricted_path.host {
                                Some(remote_host) => match &remote_host.user_name {
                                    Some(user_name) => format!(
                                        "{} ({}@{})",
                                        self.shorten_path(abs_path).display(),
                                        user_name,
                                        remote_host.host_name
                                    ),
                                    None => format!(
                                        "{} ({})",
                                        self.shorten_path(abs_path).display(),
                                        remote_host.host_name
                                    ),
                                },
                                None => self.shorten_path(abs_path).display().to_string(),
                            },
                            None => match &restricted_path.host {
                                Some(remote_host) => match &remote_host.user_name {
                                    Some(user_name) => format!(
                                        "Empty project ({}@{})",
                                        user_name, remote_host.host_name
                                    ),
                                    None => {
                                        format!("Empty project ({})", remote_host.host_name)
                                    }
                                },
                                None => "Empty project".to_string(),
                            },
                        };
                        h_flex()
                            .pl(IconSize::default().rems() + rems(0.5))
                            .child(Label::new(label).color(Color::Muted))
                    })),
            )
            .child(
                "Untrusted workspaces are opened in Restricted Mode to protect your system.
Review .zed/settings.json for any extensions or commands configured by this project.",
            )
            .child(
                v_flex()
                    .mt_2()
                    .child(Label::new("Restricted mode prevents:").color(Color::Muted))
                    .child(ListBulletItem::new("Project settings from being applied"))
                    .child(ListBulletItem::new("Language servers from running"))
                    .child(ListBulletItem::new("MCP integrations from installing")),
            )
            .footer(
                h_flex()
                    .p_3()
                    .map(|div| match trust_label {
                        Some(trust_label) => div.justify_between().child(
                            Checkbox::new("trust-parents", ToggleState::from(self.trust_parents))
                                .label(trust_label)
                                .on_click(cx.listener(
                                    |security_modal, state: &ToggleState, _, cx| {
                                        security_modal.trust_parents = state.selected();
                                        cx.notify();
                                    },
                                )),
                        ),
                        None => div.justify_end(),
                    })
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new("open-in-restricted-mode", "Restricted Mode")
                                    .key_binding(KeyBinding::for_action_in(
                                        &ToggleWorktreeSecurity,
                                        &focus_handle,
                                        cx,
                                    ))
                                    .color(Color::Muted)
                                    .on_click(cx.listener(move |security_modal, _, _, cx| {
                                        security_modal.dismiss(cx);
                                        cx.stop_propagation();
                                    })),
                            )
                            .child(
                                Button::new("trust-and-continue", "Trust and Continue")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(move |security_modal, _, _, cx| {
                                        security_modal.trust_and_dismiss(cx);
                                    })),
                            ),
                    ),
            )
            .width(rems(40.))
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
            dismissed: false,
            trust_parents: false,
            home_dir: std::env::home_dir(),
        };
        this.refresh_restricted_paths(cx);

        this
    }

    fn build_trust_label(&self) -> Option<Cow<'static, str>> {
        let available_parents = self
            .restricted_paths
            .values()
            .filter(|restricted_path| !restricted_path.is_file)
            .filter_map(|restricted_path| restricted_path.abs_path.as_ref()?.parent())
            .collect::<SmallVec<[_; 2]>>();
        match available_parents.len() {
            0 => None,
            1 => Some(Cow::Owned(format!(
                "Trust all projects in the {:?} folder",
                self.shorten_path(available_parents[0])
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
        if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                let mut paths_to_trust = self
                    .restricted_paths
                    .keys()
                    .map(|worktree_id| match worktree_id {
                        Some(worktree_id) => PathTrust::Worktree(*worktree_id),
                        None => PathTrust::Global(self.remote_host.clone()),
                    })
                    .collect::<HashSet<_>>();
                if self.trust_parents {
                    paths_to_trust.extend(self.restricted_paths.values().filter_map(
                        |restricted_paths| {
                            if restricted_paths.is_file {
                                return None;
                            }
                            let parent_abs_path =
                                restricted_paths.abs_path.as_ref()?.parent()?.to_owned();
                            Some(PathTrust::AbsPath(
                                parent_abs_path,
                                restricted_paths.host.clone(),
                            ))
                        },
                    ));
                }

                trusted_worktrees.trust(paths_to_trust, cx);
            });
        }

        self.dismiss(cx);
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.dismissed = true;
        cx.emit(DismissEvent);
    }

    pub fn refresh_restricted_paths(&mut self, cx: &mut Context<Self>) {
        if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
            if let Some(worktree_store) = self.worktree_store.upgrade() {
                let mut new_restricted_worktrees = trusted_worktrees
                    .read(cx)
                    .restricted_paths(worktree_store.read(cx), self.remote_host.clone(), cx)
                    .into_iter()
                    .filter_map(|restricted_path| {
                        let restricted_path = match restricted_path {
                            Some((worktree_id, abs_path)) => {
                                let worktree =
                                    worktree_store.read(cx).worktree_for_id(worktree_id, cx)?;
                                (
                                    Some(worktree_id),
                                    RestrictedPath {
                                        abs_path: Some(abs_path),
                                        is_file: worktree.read(cx).is_single_file(),
                                        host: self.remote_host.clone(),
                                    },
                                )
                            }
                            None => (
                                None,
                                RestrictedPath {
                                    abs_path: None,
                                    is_file: false,
                                    host: self.remote_host.clone(),
                                },
                            ),
                        };
                        Some(restricted_path)
                    })
                    .collect::<HashMap<_, _>>();
                // Do not clutter the UI: agreeing on local events assumes the global are agreed to either, on the same host.
                if new_restricted_worktrees.len() > 1 {
                    new_restricted_worktrees.remove(&None);
                }

                if self.restricted_paths != new_restricted_worktrees {
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
