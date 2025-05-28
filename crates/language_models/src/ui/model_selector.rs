use gpui::{prelude::*, Context, Window};
use ui::{prelude::*, Label, LabelSize};

/// Simple model selector component for language models UI
pub struct ModelSelector {
    selected_model: Option<String>,
}

impl ModelSelector {
    pub fn new() -> Self {
        Self {
            selected_model: None,
        }
    }

    pub fn with_selected_model(mut self, model: String) -> Self {
        self.selected_model = Some(model);
        self
    }
}

impl Render for ModelSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected_model_text = self.selected_model
            .as_deref()
            .unwrap_or("No model selected")
            .to_string();
        let has_model = self.selected_model.is_some();

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                Label::new("Model:")
                    .size(LabelSize::Small)
            )
            .child(
                Label::new(selected_model_text)
                .size(LabelSize::Small)
                .color(if has_model {
                    Color::Default
                } else {
                    Color::Muted
                })
            )
    }
} 