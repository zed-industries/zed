use gpui::{IntoElement, Window};
use ui::Label;
use ui::{CheckboxWithLabel, prelude::*};

pub(crate) struct InspectorOptions {
    pub open_code_on_inspect: bool,
}

impl Render for InspectorOptions {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        CheckboxWithLabel::new(
            "open-code-on-inspect",
            Label::new("Open code"),
            self.open_code_on_inspect.into(),
            cx.listener(|this, selection: &ToggleState, _, _| {
                this.open_code_on_inspect = selection.selected();
            }),
        )
    }
}
