use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use collections::HashSet;
use gpui::{DismissEvent, EventEmitter, Focusable};
use project::trusted_worktrees::TrustedWorktreesStorage;
use ui::{
    BorrowAppContext, Button, ButtonCommon, ButtonStyle, Checkbox, Clickable as _, Color, Context,
    Div, Element, ElevationIndex, Icon, IconName, IconSize, InteractiveElement as _, IntoElement,
    Label, ListSeparator, ParentElement as _, Render, SelectableButton, Styled, StyledExt as _,
    ToggleState, Window, div, h_flex, rems, v_flex,
};

use crate::{DismissDecision, ModalView};

pub struct SecurityModal {
    pub paths: HashSet<PathBuf>,
    home_dir: Option<PathBuf>,
    dismissed: bool,
    trust_parents: bool,
}

impl Focusable for SecurityModal {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        cx.focus_handle()
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
            return div().into_any();
        }

        v_flex()
            .id("security-modal")
            .elevation_3(cx)
            .w(rems(34.))
            .size_full()
            .p_2()
            .child(self.render_header().size_full())
            .child(div().child(ListSeparator).size_full())
            .child(self.render_explanation().size_full())
            .child(div().child(ListSeparator).size_full())
            .child(self.render_footer(cx).size_full())
            .into_any()
    }
}

impl SecurityModal {
    pub fn new(paths: HashSet<PathBuf>) -> Self {
        Self {
            paths,
            dismissed: false,
            trust_parents: false,
            home_dir: std::env::home_dir(),
        }
    }

    fn render_header(&self) -> Div {
        let header_label = if self.paths.len() == 1 {
            "Do you trust the authors of this project?"
        } else {
            "Do you trust the authors of these projects?"
        };
        let mut header = v_flex().child(
            h_flex()
                .gap_1()
                .justify_start()
                .child(Icon::new(IconName::Warning).color(Color::Warning))
                .child(div().child(Label::new(header_label))),
        );
        for path in &self.paths {
            header = header.child(
                h_flex()
                    .gap_1()
                    .justify_start()
                    .child(div().size(IconSize::default().rems()))
                    .child(div().child(Label::new(path.display().to_string()))),
            );
        }
        header
    }

    fn render_explanation(&self) -> Div {
        div().child(Label::new(
            "Untrusted workspaces are opened in Restricted Mode to protect your system.

Restricted mode prevents:
 — Project settings from being applied
 — Language servers from running
 — MCP integrations from installing
",
        ))
    }

    fn render_footer(&self, cx: &mut Context<Self>) -> Div {
        let trust_label = if self.paths.len() == 1 {
            let Some(single_path) = self.paths.iter().next() else {
                return div();
            };
            match single_path.parent().map(|path| match &self.home_dir {
                Some(home_dir) => path
                    .strip_prefix(home_dir)
                    .map(|stripped| Path::new("~").join(stripped))
                    .map(Cow::Owned)
                    .unwrap_or(Cow::Borrowed(path)),
                None => Cow::Borrowed(path),
            }) {
                Some(parent) => Cow::Owned(format!("Trust all projects in the {parent:?} folder")),
                None => Cow::Borrowed("Trust all projects in the parent folders"),
            }
        } else {
            Cow::Borrowed("Trust all projects in the parent folders")
        };

        h_flex()
            .justify_end()
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
                    .child(
                        Button::new("open-in-restricted-mode", "Open in Restricted Mode")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(move |security_modal, _, _, cx| {
                                security_modal.dismiss(cx);
                                cx.stop_propagation();
                            })),
                    )
                    .child(
                        Button::new("trust-and-continue", "Trust and continue")
                            .style(ButtonStyle::Filled)
                            .selected_style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .layer(ElevationIndex::ModalSurface)
                            .on_click(cx.listener(move |security_modal, _, _, cx| {
                                if cx.has_global::<TrustedWorktreesStorage>() {
                                    cx.update_global::<TrustedWorktreesStorage, _>(
                                        |trusted_wortrees_storage, cx| {
                                            let mut paths_to_trust = security_modal.paths.clone();
                                            if security_modal.trust_parents {
                                                paths_to_trust.extend(
                                                    security_modal.paths.iter().filter_map(
                                                        |path| Some(path.parent()?.to_owned()),
                                                    ),
                                                );
                                            }
                                            trusted_wortrees_storage.trust(paths_to_trust, cx);
                                        },
                                    );
                                }

                                security_modal.dismiss(cx);
                                cx.stop_propagation();
                            })),
                    ),
            )
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.dismissed = true;
        cx.emit(DismissEvent);
    }
}
