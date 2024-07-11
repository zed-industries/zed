use crate::prelude::*;

/// !!don't use this yet – it's not functional!!
///
/// pub crate until this is functional
///
/// just a placeholder for now for filling out the settings menu stories.
#[derive(Debug, Clone, IntoElement)]
pub(crate) struct DropdownMenu {
    pub id: ElementId,
    current_item: Option<SharedString>,
    // items: Vec<SharedString>,
    full_width: bool,
    disabled: bool,
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>, _cx: &WindowContext) -> Self {
        Self {
            id: id.into(),
            current_item: None,
            // items: Vec::new(),
            full_width: false,
            disabled: false,
        }
    }

    pub fn current_item(mut self, current_item: Option<SharedString>) -> Self {
        self.current_item = current_item;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let disabled = self.disabled;

        h_flex()
            .id(self.id)
            .justify_between()
            .rounded_md()
            .bg(cx.theme().colors().editor_background)
            .pl_2()
            .pr_1p5()
            .py_0p5()
            .gap_2()
            .min_w_20()
            .when_else(
                self.full_width,
                |full_width| full_width.w_full(),
                |auto_width| auto_width.flex_none().w_auto(),
            )
            .when_else(
                disabled,
                |disabled| disabled.cursor_not_allowed(),
                |enabled| enabled.cursor_pointer(),
            )
            .child(
                Label::new(self.current_item.unwrap_or("".into())).color(if disabled {
                    Color::Disabled
                } else {
                    Color::Default
                }),
            )
            .child(
                Icon::new(IconName::ChevronUpDown)
                    .size(IconSize::XSmall)
                    .color(if disabled {
                        Color::Disabled
                    } else {
                        Color::Muted
                    }),
            )
    }
}
