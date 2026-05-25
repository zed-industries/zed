//! A UI interface for managing the [`TrustedWorktrees`] data.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, HashSet};
use gpui::{DismissEvent, EventEmitter, FocusHandle, Focusable, ScrollHandle, WeakEntity};

use project::{
    WorktreeId,
    trusted_worktrees::{PathTrust, RemoteHostLocation, ToolTrust, TrustedWorktrees},
    worktree_store::WorktreeStore,
};
use smallvec::SmallVec;
use theme::ActiveTheme;
use ui::{
    AlertModal, Checkbox, FluentBuilder, KeyBinding, ListBulletItem, ToggleState, WithScrollbar,
    prelude::*,
};

use crate::{DismissDecision, ModalView, ToggleWorktreeSecurity};

pub struct SecurityModal {
    restricted_paths: HashMap<WorktreeId, RestrictedPath>,
    restricted_tools: HashSet<ToolTrust>,
    /// Per-tool checkbox state. Defaults to `true` (checked) for every restricted tool.
    /// Tools the user unchecks remain restricted when the user clicks "Trust and Continue".
    tool_selection: HashMap<ToolTrust, bool>,
    home_dir: Option<PathBuf>,
    trust_parents: bool,
    worktree_store: WeakEntity<WorktreeStore>,
    remote_host: Option<RemoteHostLocation>,
    focus_handle: FocusHandle,
    items_scroll_handle: ScrollHandle,
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
            Some(true) => {
                telemetry::event!("Trust and Continue", source = "Worktree Trust Modal");
            }
            // Explicit "Stay in Restricted Mode", ESC, or click-outside all leave the user
            // in restricted mode.
            Some(false) | None => {
                self.trusted = Some(false);
                telemetry::event!("Open in Restricted", source = "Worktree Trust Modal");
            }
        }
        DismissDecision::Dismiss(true)
    }
}

fn decorate_with_host(label: String, host: Option<&RemoteHostLocation>) -> String {
    match host {
        Some(host) => match &host.user_name {
            Some(user_name) => format!("{label} ({user_name}@{})", host.host_identifier),
            None => format!("{label} ({})", host.host_identifier),
        },
        None => label,
    }
}

impl Render for SecurityModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project_labels = self.project_labels();
        let has_projects = !project_labels.is_empty();
        let has_tools = !self.restricted_tools.is_empty();
        if !has_projects && !has_tools {
            self.dismiss(cx);
            return v_flex().into_any_element();
        }

        let header_label = self.header_label(project_labels.len(), self.restricted_tools.len());
        let trust_label = self.build_trust_label();

        // Sorted snapshot so tool order is stable across renders.
        let mut tools = self.restricted_tools.iter().cloned().collect::<Vec<_>>();
        tools.sort_by(|a, b| {
            a.namespace
                .cmp(&b.namespace)
                .then_with(|| a.name.cmp(&b.name))
        });
        let any_tool_selected = tools
            .iter()
            .any(|tool| *self.tool_selection.get(tool).unwrap_or(&true));

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
                    .child(
                        div()
                            .size_full()
                            .vertical_scrollbar_for(&self.items_scroll_handle, window, cx)
                            .child(
                                v_flex()
                                    .id("trust_items_container")
                                    .max_h_32()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.items_scroll_handle)
                                    .children(project_labels.iter().map(|label| {
                                        h_flex()
                                            .pl(IconSize::default().rems() + rems(0.5))
                                            .child(Label::new(label.clone()).color(Color::Muted))
                                    }))
                                    .children(tools.into_iter().map(|tool| {
                                        let selected = self
                                            .tool_selection
                                            .get(&tool)
                                            .copied()
                                            .unwrap_or(true);
                                        let id = ElementId::from(SharedString::from(format!(
                                            "tool-{}-{}",
                                            tool.namespace, tool.name
                                        )));
                                        let label = decorate_with_host(
                                            format!("{}: {}", tool.namespace, tool.name),
                                            self.remote_host.as_ref(),
                                        );
                                        h_flex().pl(rems(0.25)).child(
                                            Checkbox::new(id, ToggleState::from(selected))
                                                .label(label)
                                                .on_click(cx.listener(
                                                    move |security_modal,
                                                          state: &ToggleState,
                                                          _,
                                                          cx| {
                                                        security_modal
                                                            .tool_selection
                                                            .insert(tool.clone(), state.selected());
                                                        cx.notify();
                                                        cx.stop_propagation();
                                                    },
                                                )),
                                        )
                                    })),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .when(has_projects, |this| {
                        this.child(
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
                    })
                    .when(has_tools && !has_projects, |this| {
                        this.child(
                            Label::new(
                                "These Zed-managed tools were requested but have not been allowed yet. \
                                Tick a row to allow that tool to download and run on this host across all your sessions.",
                            )
                            .color(Color::Muted),
                        )
                    })
                    .when(has_tools && has_projects, |this| {
                        this.child(
                            Label::new(
                                "Untick any Zed-managed tools you do not want to allow on this host.",
                            )
                            .color(Color::Muted),
                        )
                    })
                    .child(
                        v_flex()
                            .child(Label::new("Restricted Mode prevents:").color(Color::Muted))
                            .when(has_projects, |this| {
                                this.child(ListBulletItem::new(
                                    "Project settings from being applied",
                                ))
                                .child(ListBulletItem::new(
                                    "MCP Server integrations from installing",
                                ))
                            })
                            .child(ListBulletItem::new("Language servers from running"))
                            .when(has_tools, |this| {
                                this.child(ListBulletItem::new(
                                    "Zed-managed tools from downloading or running",
                                ))
                            }),
                    )
                    .when_some(trust_label, |this, trust_label| {
                        this.child(
                            Checkbox::new("trust-parents", ToggleState::from(self.trust_parents))
                                .label(trust_label)
                                .on_click(cx.listener(
                                    |security_modal, state: &ToggleState, _, cx| {
                                        security_modal.trust_parents = state.selected();
                                        cx.notify();
                                        cx.stop_propagation();
                                    },
                                )),
                        )
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
                                KeyBinding::for_action(&ToggleWorktreeSecurity, cx)
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
                            // Disable when there's literally nothing to trust (only happens if
                            // the user unticks every tool in a tools-only modal).
                            .disabled(!has_projects && !any_tool_selected)
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
            restricted_tools: HashSet::default(),
            tool_selection: HashMap::default(),
            focus_handle: cx.focus_handle(),
            items_scroll_handle: ScrollHandle::new(),
            trust_parents: false,
            home_dir: std::env::home_dir(),
            trusted: None,
        };
        this.refresh_restricted_paths(cx);

        this
    }

    fn header_label(&self, project_count: usize, tool_count: usize) -> SharedString {
        match (project_count, tool_count) {
            (0, 1) => "Unrecognized Tool".into(),
            (0, n) => format!("Unrecognized Tools ({n})").into(),
            (1, 0) => "Unrecognized Project".into(),
            (n, 0) => format!("Unrecognized Projects ({n})").into(),
            (p, 1) => format!("Unrecognized Projects ({p}) and Tool").into(),
            (1, t) => format!("Unrecognized Project and Tools ({t})").into(),
            (p, t) => format!("Unrecognized Projects ({p}) and Tools ({t})").into(),
        }
    }

    fn project_labels(&self) -> Vec<String> {
        self.restricted_paths
            .values()
            .filter_map(|restricted_path| {
                let abs_path = if restricted_path.is_file {
                    restricted_path.abs_path.parent()
                } else {
                    Some(restricted_path.abs_path.as_ref())
                }?;
                Some(decorate_with_host(
                    self.shorten_path(abs_path).display().to_string(),
                    restricted_path.host.as_ref(),
                ))
            })
            .collect()
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
        if let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) {
            trusted_worktrees.update(cx, |trusted_worktrees, cx| {
                if let Some(worktree_store) = self.worktree_store.upgrade() {
                    let mut paths_to_trust = self
                        .restricted_paths
                        .keys()
                        .copied()
                        .map(PathTrust::Worktree)
                        .collect::<HashSet<_>>();
                    if self.trust_parents {
                        paths_to_trust.extend(self.restricted_paths.values().filter_map(
                            |restricted_path| {
                                if restricted_path.is_file {
                                    None
                                } else {
                                    Some(PathTrust::AbsPath(
                                        restricted_path.abs_path.parent()?.to_owned(),
                                    ))
                                }
                            },
                        ));
                    }
                    if !paths_to_trust.is_empty() {
                        trusted_worktrees.trust(&worktree_store, paths_to_trust, cx);
                    }
                }
                let tools_to_trust = self
                    .restricted_tools
                    .iter()
                    .filter(|tool| *self.tool_selection.get(*tool).unwrap_or(&true))
                    .cloned()
                    .collect::<HashSet<_>>();
                if !tools_to_trust.is_empty() {
                    trusted_worktrees.trust_tools(self.remote_host.clone(), tools_to_trust, cx);
                }
            });
        }

        self.trusted = Some(true);
        self.dismiss(cx);
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    pub fn refresh_restricted_paths(&mut self, cx: &mut Context<Self>) {
        let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) else {
            if !self.restricted_paths.is_empty() || !self.restricted_tools.is_empty() {
                self.restricted_paths.clear();
                self.restricted_tools.clear();
                self.tool_selection.clear();
                cx.notify();
            }
            return;
        };

        let new_restricted_worktrees = if let Some(worktree_store) = self.worktree_store.upgrade() {
            trusted_worktrees
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
                .collect::<HashMap<_, _>>()
        } else {
            HashMap::default()
        };
        let new_restricted_tools = trusted_worktrees
            .read(cx)
            .restricted_tools(self.remote_host.clone());

        let paths_changed = self.restricted_paths != new_restricted_worktrees;
        let tools_changed = self.restricted_tools != new_restricted_tools;
        if paths_changed || tools_changed {
            if paths_changed {
                self.trust_parents = false;
            }
            // Drop selection state for tools that are no longer restricted;
            // newly-restricted tools default to checked.
            self.tool_selection
                .retain(|tool, _| new_restricted_tools.contains(tool));
            self.restricted_paths = new_restricted_worktrees;
            self.restricted_tools = new_restricted_tools;
            if self.restricted_paths.is_empty() && self.restricted_tools.is_empty() {
                self.trusted = Some(true);
                self.dismiss(cx);
            } else {
                cx.notify();
            }
        }
    }
}
