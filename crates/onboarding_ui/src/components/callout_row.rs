use component::{example_group_with_title, single_example};
use gpui::{AnyElement, App, IntoElement, RenderOnce, Window};
use smallvec::SmallVec;
use ui::{Label, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct CalloutRow {
    title: SharedString,
    lines: SmallVec<[SharedString; 4]>,
}

impl CalloutRow {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            lines: SmallVec::new(),
        }
    }

    pub fn line(mut self, line: impl Into<SharedString>) -> Self {
        self.lines.push(line.into());
        self
    }
}

impl RenderOnce for CalloutRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().px_2().child(
            v_flex()
                .p_3()
                .gap_1()
                .bg(cx.theme().colors().surface_background)
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .rounded_md()
                .child(Label::new(self.title).weight(gpui::FontWeight::MEDIUM))
                .children(
                    self.lines
                        .into_iter()
                        .map(|line| Label::new(line).size(LabelSize::Small).color(Color::Muted)),
                ),
        )
    }
}

impl Component for CalloutRow {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn sort_name() -> &'static str {
        "RowCallout"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let examples = example_group_with_title(
            "CalloutRow Examples",
            vec![
                single_example(
                    "Privacy Notice",
                    CalloutRow::new("We don't use your code to train AI models")
                        .line("You choose which providers you enable, and they have their own privacy policies.")
                        .line("Read more about our privacy practices in our Privacy Policy.")
                        .into_any_element(),
                ),
                single_example(
                    "Single Line",
                    CalloutRow::new("Important Notice")
                        .line("This is a single line of information.")
                        .into_any_element(),
                ),
                single_example(
                    "Multi Line",
                    CalloutRow::new("Getting Started")
                        .line("Welcome to Zed! Here are some things to know:")
                        .line("• Use Cmd+P to quickly open files")
                        .line("• Use Cmd+Shift+P to access the command palette")
                        .line("• Check out the documentation for more tips")
                        .into_any_element(),
                ),
            ],
        );

        Some(v_flex().p_4().gap_4().child(examples).into_any_element())
    }
}
