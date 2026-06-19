//! A UI interface for managing the [`TrustedWorktrees`] data.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
};

use collections::{HashMap, HashSet};
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, WeakEntity};

use project::{
    WorktreeId,
    trusted_worktrees::{PathTrust, RemoteHostLocation, TrustedWorktrees},
    worktree_store::WorktreeStore,
};
use smallvec::SmallVec;
use theme::ActiveTheme;
use ui::{
    AlertModal, Checkbox, FluentBuilder, KeyBinding, ListBulletItem, ToggleState, WithScrollbar,
    prelude::*,
};
use ui_input::InputField;

use crate::{DismissDecision, ModalView, ToggleWorktreeSecurity};

pub struct SecurityModal {
    restricted_paths: HashMap<WorktreeId, RestrictedPath>,
    home_dir: Option<PathBuf>,
    trust_parents: bool,
    worktree_store: WeakEntity<WorktreeStore>,
    remote_host: Option<RemoteHostLocation>,
    focus_handle: FocusHandle,
    project_list_scroll_handle: ScrollHandle,
    trusted: Option<bool>,
    /// Editable trust scope shown inline with the checkbox when a single
    /// project is being prompted for; read-only until the checkbox is ticked.
    trust_path_input: Entity<InputField>,
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
            Some(false) => {
                telemetry::event!("Open in Restricted", source = "Worktree Trust Modal");
                DismissDecision::Dismiss(true)
            }
            Some(true) => {
                telemetry::event!("Trust and Continue", source = "Worktree Trust Modal");
                DismissDecision::Dismiss(true)
            }
            // Block dismiss via escape or clicking outside; user must pick an action
            None => DismissDecision::Dismiss(false),
        }
    }
}

impl Render for SecurityModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.restricted_paths.is_empty() {
            self.dismiss(cx);
            return v_flex().into_any_element();
        }

        let restricted_count = self.restricted_paths.len();
        let header_label: SharedString = if restricted_count == 1 {
            "Unrecognized Project".into()
        } else {
            format!("Unrecognized Projects ({})", restricted_count).into()
        };

        let trust_label = self.build_trust_label();

        // The editable trust-scope field is shown only when a single project is
        // being prompted for (Delta opens one worktree per thread).
        let trust_input = self
            .single_trustable_path()
            .is_some()
            .then(|| self.trust_path_input.clone());

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
                            .vertical_scrollbar_for(&self.project_list_scroll_handle, window, cx)
                            .child(
                                v_flex()
                                    .id("paths_container")
                                    .max_h_24()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.project_list_scroll_handle)
                                    .children(
                                        self.restricted_paths.values().filter_map(
                                            |restricted_path| {
                                                let abs_path = if restricted_path.is_file {
                                                    restricted_path.abs_path.parent()
                                                } else {
                                                    Some(restricted_path.abs_path.as_ref())
                                                }?;
                                                let label = match &restricted_path.host {
                                                    Some(remote_host) => {
                                                        match &remote_host.user_name {
                                                            Some(user_name) => format!(
                                                                "{} ({}@{})",
                                                                self.shorten_path(abs_path)
                                                                    .display(),
                                                                user_name,
                                                                remote_host.host_identifier
                                                            ),
                                                            None => format!(
                                                                "{} ({})",
                                                                self.shorten_path(abs_path)
                                                                    .display(),
                                                                remote_host.host_identifier
                                                            ),
                                                        }
                                                    }
                                                    None => self
                                                        .shorten_path(abs_path)
                                                        .display()
                                                        .to_string(),
                                                };
                                                Some(
                                                    h_flex()
                                                        .pl(
                                                            IconSize::default().rems() + rems(0.5),
                                                        )
                                                        .child(
                                                            Label::new(label).color(Color::Muted),
                                                        ),
                                                )
                                            },
                                        ),
                                    ),
                            ),
                    ),
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
                    .map(|this| {
                        let Some(trust_label) = trust_label else {
                            return this;
                        };
                        match trust_input {
                            // Single project: the editable scope field replaces
                            // the static folder name, inline with the checkbox.
                            Some(input) => this.child(
                                // Top-aligned so the field's validation error
                                // grows downward without shifting the checkbox;
                                // the checkbox sits in a fixed-height box matching
                                // the input row.
                                h_flex()
                                    .items_start()
                                    .gap_1p5()
                                    .child(
                                        h_flex().h_8().child(
                                        Checkbox::new(
                                            "trust-parents",
                                            ToggleState::from(self.trust_parents),
                                        )
                                        .label("Trust all projects in")
                                        .on_click(cx.listener(
                                            |security_modal, state: &ToggleState, _, cx| {
                                                let trust_parents = state.selected();
                                                security_modal.trust_parents = trust_parents;
                                                let input =
                                                    security_modal.trust_path_input.clone();
                                                let editor = input.read(cx).editor().clone();
                                                editor.set_read_only(!trust_parents, cx);
                                                if !trust_parents {
                                                    input.update(cx, |input, cx| {
                                                        input.set_error(None::<SharedString>, cx)
                                                    });
                                                }
                                                cx.notify();
                                                cx.stop_propagation();
                                            },
                                        )),
                                        ),
                                    )
                                    .child(input),
                            ),
                            // Zero or several projects: keep the static label.
                            None => this.child(
                                Checkbox::new(
                                    "trust-parents",
                                    ToggleState::from(self.trust_parents),
                                )
                                .label(trust_label)
                                .on_click(cx.listener(
                                    |security_modal, state: &ToggleState, _, cx| {
                                        security_modal.trust_parents = state.selected();
                                        cx.notify();
                                        cx.stop_propagation();
                                    },
                                )),
                            ),
                        }
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let trust_path_input = cx.new(|cx| InputField::new(window, cx, "Folder to trust"));
        let mut this = Self {
            worktree_store,
            remote_host: remote_host.map(|host| host.into()),
            restricted_paths: HashMap::default(),
            focus_handle: cx.focus_handle(),
            project_list_scroll_handle: ScrollHandle::new(),
            trust_parents: false,
            home_dir: std::env::home_dir(),
            trusted: None,
            trust_path_input,
        };
        this.refresh_restricted_paths(cx);

        // Pre-fill with the single project's parent folder (today's static
        // scope), read-only until the checkbox is ticked.
        if let Some(project) = this.single_trustable_path() {
            let default_scope = project.parent().unwrap_or(&project).to_path_buf();
            this.trust_path_input.update(cx, |field, cx| {
                field.set_text(&default_scope.to_string_lossy(), window, cx);
            });
        }
        let editor = this.trust_path_input.read(cx).editor().clone();
        editor.set_read_only(!this.trust_parents, cx);

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

    /// The user-edited trust scope, when an editable field is shown. `Ok(None)`
    /// means fall back to the default per-parent behavior; `Err` is a validation
    /// message to surface on the field.
    fn edited_trust_scope(&self, cx: &App) -> Result<Option<PathBuf>, SharedString> {
        if !self.trust_parents {
            return Ok(None);
        }
        let Some(project) = self.single_trustable_path() else {
            return Ok(None);
        };
        let typed = self.trust_path_input.read(cx).text(cx);
        validate_trust_scope(&typed, &project, self.home_dir.as_deref()).map(Some)
    }

    fn trust_and_dismiss(&mut self, cx: &mut Context<Self>) {
        let scope_override = match self.edited_trust_scope(cx) {
            Ok(scope) => scope,
            Err(error) => {
                // Invalid path: flag the field and keep the modal open.
                self.trust_path_input
                    .update(cx, |input, cx| input.set_error(Some(error), cx));
                return;
            }
        };

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
                    if let Some(scope) = scope_override {
                        paths_to_trust.insert(PathTrust::AbsPath(scope));
                    } else {
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

    /// The sole untrusted, non-file project, when exactly one is being prompted
    /// for. This is the case where the trust scope can be edited (Delta opens
    /// one worktree per thread); with zero or several we keep the static label.
    fn single_trustable_path(&self) -> Option<Arc<Path>> {
        let mut projects = self
            .restricted_paths
            .values()
            .filter(|restricted_path| !restricted_path.is_file)
            .map(|restricted_path| restricted_path.abs_path.clone());
        let only = projects.next()?;
        projects.next().is_none().then_some(only)
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
                    if self.restricted_paths.is_empty() {
                        self.trusted = Some(true);
                        self.dismiss(cx);
                    } else {
                        cx.notify();
                    }
                }
            }
        } else if !self.restricted_paths.is_empty() {
            self.restricted_paths.clear();
            cx.notify();
        }
    }
}

/// Validates a user-edited trust-scope path. Returns the absolute folder to
/// trust when `typed` is an ancestor of (or equal to) `project`; otherwise an
/// error message suitable for display. A leading `~` is expanded via `home_dir`.
fn validate_trust_scope(
    typed: &str,
    project: &Path,
    home_dir: Option<&Path>,
) -> Result<PathBuf, SharedString> {
    let trimmed = typed.trim();
    if trimmed.is_empty() {
        return Err("Enter a folder to trust".into());
    }
    let expanded = match (trimmed.strip_prefix('~'), home_dir) {
        (Some(rest), Some(home_dir)) => {
            home_dir.join(rest.strip_prefix(std::path::MAIN_SEPARATOR).unwrap_or(rest))
        }
        _ => PathBuf::from(trimmed),
    };
    if !expanded.is_absolute() {
        return Err("Enter an absolute folder path".into());
    }
    if !project.starts_with(&expanded) {
        return Err("Must be a parent folder of the project".into());
    }
    Ok(expanded)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn accepts_ancestor_or_equal() {
        let project = Path::new("/Users/me/dev/delta/wt/t1");
        assert_eq!(
            validate_trust_scope("/Users/me/dev/delta/wt", project, None).unwrap(),
            PathBuf::from("/Users/me/dev/delta/wt"),
        );
        // Equal to the project itself is allowed.
        assert_eq!(
            validate_trust_scope("/Users/me/dev/delta/wt/t1", project, None).unwrap(),
            PathBuf::from("/Users/me/dev/delta/wt/t1"),
        );
        // A distant ancestor is allowed.
        assert!(validate_trust_scope("/Users/me/dev", project, None).is_ok());
    }

    #[test]
    fn rejects_non_ancestor_relative_or_empty() {
        let project = Path::new("/Users/me/dev/delta/wt/t1");
        assert!(validate_trust_scope("/Users/other", project, None).is_err());
        assert!(validate_trust_scope("relative/path", project, None).is_err());
        assert!(validate_trust_scope("   ", project, None).is_err());
        // Deeper than the project is not an ancestor.
        assert!(validate_trust_scope("/Users/me/dev/delta/wt/t1/sub", project, None).is_err());
    }

    #[test]
    fn expands_leading_tilde() {
        let home = Path::new("/Users/me");
        let project = Path::new("/Users/me/dev/wt/t1");
        assert_eq!(
            validate_trust_scope("~/dev/wt", project, Some(home)).unwrap(),
            PathBuf::from("/Users/me/dev/wt"),
        );
        assert_eq!(
            validate_trust_scope("~", project, Some(home)).unwrap(),
            PathBuf::from("/Users/me"),
        );
    }
}
