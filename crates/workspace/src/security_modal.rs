//! TODO kb
use std::path::PathBuf;

use collections::HashSet;
use gpui::{DismissEvent, EventEmitter, Focusable};
use ui::{
    Color, Context, Div, Element, Icon, IconName, IconSize, InteractiveElement as _, IntoElement,
    Label, ListSeparator, ParentElement as _, Render, Styled, StyledExt as _, Window, div, h_flex,
    v_flex,
};

use crate::ModalView;

pub struct SecurityModal {
    pub paths: HashSet<PathBuf>,
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
    ) -> crate::DismissDecision {
        // TODO kb
        crate::DismissDecision::Dismiss(true)
    }

    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Render for SecurityModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.paths.is_empty() {
            cx.emit(DismissEvent);
            return div().into_any();
        }

        v_flex()
            .id("security-modal")
            .elevation_3(cx)
            .child(self.render_header().size_full().border_b_1())
            .child(ListSeparator)
            .child(self.render_explanation().size_full())
            .child(ListSeparator)
            .child(self.render_footer().size_full().border_t_1())
            .into_any()
    }
}

impl SecurityModal {
    fn render_header(&self) -> Div {
        let mut parent = v_flex().child(
            h_flex()
                .gap_1()
                .justify_start()
                .child(Icon::new(IconName::Warning).color(Color::Warning))
                .child(div().child(Label::new("Do you trust the authors of this project?"))),
        );
        for path in &self.paths {
            parent = parent.child(
                h_flex()
                    .gap_1()
                    .justify_start()
                    .child(div().size(IconSize::default().rems()))
                    .child(div().child(Label::new(path.display().to_string()))),
            );
        }
        parent
    }

    fn render_explanation(&self) -> Div {
        div().child(Label::new(
            "Untrusted workspaces are opened in Restricted Mode to protect your system.

Restricted mode prevents:
 — Project settings from being applied
",
        ))
    }

    fn render_footer(&self) -> Div {
        // Trust all projects in the "@@@" folder
        // Open in Restricted Mode
        // Trust and continue
        div().child(Label::new("TODO kb footer"))
    }
}
