use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use collections::HashSet;
use gpui::{BorrowAppContext, DismissEvent, EventEmitter, FocusHandle, Focusable};
use project::trusted_worktrees::TrustedWorktreesStorage;
use theme::ActiveTheme;
use ui::{
    AlertModal, App, Button, ButtonCommon as _, ButtonStyle, Checkbox, Clickable as _, Color,
    Context, Headline, HeadlineSize, Icon, IconName, IconSize, IntoElement, KeyBinding, Label,
    LabelCommon as _, ListBulletItem, ParentElement as _, Render, Styled, ToggleState, Window,
    h_flex, rems, v_flex,
};

use crate::{DismissDecision, ModalView, ToggleWorktreeSecurity};

pub struct SecurityModal {
    pub paths: HashSet<PathBuf>,
    home_dir: Option<PathBuf>,
    dismissed: bool,
    trust_parents: bool,
    focus_handle: FocusHandle,
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

impl Render for SecurityModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.paths.is_empty() {
            self.dismiss(cx);
            return v_flex().into_any_element();
        }

        let header_label = if self.paths.len() == 1 {
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
                    .children(self.paths.iter().map(|path| {
                        h_flex().pl(IconSize::default().rems() + rems(0.5)).child(
                            Label::new(self.shorten_path(path).display().to_string())
                                .color(Color::Muted),
                        )
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
                    .justify_between()
                    .child(
                        Checkbox::new("trust-parents", ToggleState::from(self.trust_parents))
                            .label(trust_label)
                            .on_click(cx.listener(|security_modal, state: &ToggleState, _, cx| {
                                security_modal.trust_parents = state.selected();
                                cx.notify();
                            })),
                    )
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
    pub fn new(paths: HashSet<PathBuf>, cx: &App) -> Self {
        Self {
            paths,
            focus_handle: cx.focus_handle(),
            dismissed: false,
            trust_parents: false,
            home_dir: std::env::home_dir(),
        }
    }

    fn build_trust_label(&self) -> Cow<'static, str> {
        if self.paths.len() == 1 {
            let Some(single_path) = self.paths.iter().next() else {
                return Cow::Borrowed("Trust all projects in the parent folders");
            };
            match single_path.parent().map(|path| self.shorten_path(path)) {
                Some(parent) => Cow::Owned(format!("Trust all projects in the {parent:?} folder")),
                None => Cow::Borrowed("Trust all projects in the parent folders"),
            }
        } else {
            Cow::Borrowed("Trust all projects in the parent folders")
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
        if cx.has_global::<TrustedWorktreesStorage>() {
            cx.update_global::<TrustedWorktreesStorage, _>(|trusted_worktrees_storage, cx| {
                let mut paths_to_trust = self.paths.clone();
                if self.trust_parents {
                    paths_to_trust.extend(
                        self.paths
                            .iter()
                            .filter_map(|path| Some(path.parent()?.to_owned())),
                    );
                }

                trusted_worktrees_storage.trust(paths_to_trust, cx);
            });
        }

        self.dismiss(cx);
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.dismissed = true;
        cx.emit(DismissEvent);
    }
}
