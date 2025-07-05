use component::{example_group_with_title, single_example};
use gpui::StatefulInteractiveElement as _;
use gpui::{AnyElement, App, ClickEvent, IntoElement, RenderOnce, Window};
use ui::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct CheckboxRow {
    label: SharedString,
    description: Option<SharedString>,
    checked: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl CheckboxRow {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            description: None,
            checked: false,
            on_click: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for CheckboxRow {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let checked = self.checked;
        let on_click = self.on_click;

        let checkbox = gpui::div()
            .w_4()
            .h_4()
            .rounded_sm()
            .border_1()
            .border_color(cx.theme().colors().border)
            .when(checked, |this| {
                this.bg(cx.theme().colors().element_selected)
                    .border_color(cx.theme().colors().border_selected)
            })
            .hover(|this| this.bg(cx.theme().colors().element_hover))
            .child(gpui::div().when(checked, |this| {
                this.size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::Check))
            }));

        let main_row = if let Some(on_click) = on_click {
            gpui::div()
                .id("checkbox-row")
                .h_flex()
                .gap_2()
                .items_center()
                .child(checkbox)
                .child(Label::new(self.label))
                .cursor_pointer()
                .on_click(move |_event, window, cx| on_click(window, cx))
        } else {
            gpui::div()
                .id("checkbox-row")
                .h_flex()
                .gap_2()
                .items_center()
                .child(checkbox)
                .child(Label::new(self.label))
        };

        v_flex()
            .px_5()
            .py_1()
            .gap_1()
            .child(main_row)
            .when_some(self.description, |this, desc| {
                this.child(
                    gpui::div()
                        .ml_6()
                        .child(Label::new(desc).size(LabelSize::Small).color(Color::Muted)),
                )
            })
    }
}

impl Component for CheckboxRow {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn sort_name() -> &'static str {
        "RowCheckbox"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let examples = example_group_with_title(
            "CheckboxRow Examples",
            vec![
                single_example(
                    "Unchecked",
                    CheckboxRow::new("Enable Vim Mode").into_any_element(),
                ),
                single_example(
                    "Checked",
                    CheckboxRow::new("Send Crash Reports")
                        .checked(true)
                        .into_any_element(),
                ),
                single_example(
                    "With Description",
                    CheckboxRow::new("Send Telemetry")
                        .description("Help improve Zed by sending anonymous usage data")
                        .checked(true)
                        .into_any_element(),
                ),
            ],
        );

        Some(v_flex().p_4().gap_4().child(examples).into_any_element())
    }
}
