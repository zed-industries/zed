use acp_thread::{AgentModelIcon, AgentModelInfo};
use agent_client_protocol as acp;
use gpui::prelude::*;
use ui::prelude::*;

/// An ephemeral overlay that displays favourite models with numbered hotkeys.
/// This overlay is shown when the user holds Ctrl for 500ms in the agent panel.
/// It allows quick model selection using Ctrl+1 through Ctrl+9 and Ctrl+0.
#[derive(IntoElement)]
pub struct FavouritesOverlay {
    models: Vec<AgentModelInfo>,
    current_model_id: Option<acp::ModelId>,
}

impl FavouritesOverlay {
    pub fn new(models: Vec<AgentModelInfo>, current_model_id: Option<acp::ModelId>) -> Self {
        Self {
            models,
            current_model_id,
        }
    }

    fn number_for_index(index: usize) -> Option<char> {
        match index {
            0 => Some('1'),
            1 => Some('2'),
            2 => Some('3'),
            3 => Some('4'),
            4 => Some('5'),
            5 => Some('6'),
            6 => Some('7'),
            7 => Some('8'),
            8 => Some('9'),
            9 => Some('0'),
            _ => None,
        }
    }
}

impl RenderOnce for FavouritesOverlay {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();

        if self.models.is_empty() {
            return div()
                .p_2()
                .rounded_md()
                .bg(colors.elevated_surface_background)
                .border_1()
                .border_color(colors.border)
                .shadow_md()
                .child(
                    Label::new("No favourite models")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element();
        }

        let items = self
            .models
            .into_iter()
            .enumerate()
            .filter_map(|(index, model)| {
                let number = Self::number_for_index(index)?;
                let is_selected = self
                    .current_model_id
                    .as_ref()
                    .is_some_and(|id| *id == model.id);

                Some(FavouriteModelRow::new(number, model, is_selected))
            })
            .collect::<Vec<_>>();

        div()
            .p_1()
            .rounded_md()
            .bg(colors.elevated_surface_background)
            .border_1()
            .border_color(colors.border)
            .shadow_md()
            .min_w_48()
            .max_w_72()
            .child(
                div().px_2().py_1().child(
                    Label::new("Quick Select (Ctrl+#)")
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .child(v_flex().gap_px().children(items))
            .into_any_element()
    }
}

#[derive(IntoElement)]
struct FavouriteModelRow {
    number: char,
    model: AgentModelInfo,
    is_selected: bool,
}

impl FavouriteModelRow {
    fn new(number: char, model: AgentModelInfo, is_selected: bool) -> Self {
        Self {
            number,
            model,
            is_selected,
        }
    }
}

impl RenderOnce for FavouriteModelRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();

        let text_color = if self.is_selected {
            Color::Accent
        } else {
            Color::Default
        };

        let icon_color = if self.is_selected {
            Color::Accent
        } else {
            Color::Muted
        };

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .rounded_sm()
            .when(self.is_selected, |this| this.bg(colors.element_selected))
            .child(
                div()
                    .w_5()
                    .h_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .bg(colors.element_background)
                    .border_1()
                    .border_color(colors.border_variant)
                    .child(
                        Label::new(self.number.to_string())
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
            .when_some(self.model.icon, |this, icon| {
                this.child(
                    match icon {
                        AgentModelIcon::Path(path) => Icon::from_external_svg(path),
                        AgentModelIcon::Named(icon_name) => Icon::new(icon_name),
                    }
                    .color(icon_color)
                    .size(IconSize::Small),
                )
            })
            .child(
                Label::new(self.model.name.clone())
                    .size(LabelSize::Small)
                    .color(text_color)
                    .truncate(),
            )
            .when(self.is_selected, |this| {
                this.child(
                    Icon::new(IconName::Check)
                        .color(Color::Accent)
                        .size(IconSize::Small),
                )
            })
    }
}
