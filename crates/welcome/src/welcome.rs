use gpui::{
    color::Color,
    elements::{Flex, Label, ParentElement, Svg},
    Element, Entity, MutableAppContext, View,
};
use settings::Settings;
use workspace::{item::Item, Welcome, Workspace};

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|workspace: &mut Workspace, _: &Welcome, cx| {
        let welcome_page = cx.add_view(|_cx| WelcomePage);
        workspace.add_item(Box::new(welcome_page), cx)
    })
}

struct WelcomePage;

impl Entity for WelcomePage {
    type Event = ();
}

impl View for WelcomePage {
    fn ui_name() -> &'static str {
        "WelcomePage"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let theme = &cx.global::<Settings>().theme;

        Flex::new(gpui::Axis::Vertical)
            .with_children([
                Flex::new(gpui::Axis::Horizontal)
                    .with_children([
                        Svg::new("icons/terminal_16.svg")
                            .with_color(Color::red())
                            .constrained()
                            .with_width(100.)
                            .with_height(100.)
                            .aligned()
                            .contained()
                            .boxed(),
                        Label::new("Zed", theme.editor.hover_popover.prose.clone()).boxed(),
                    ])
                    .boxed(),
                Label::new(
                    "Code at the speed of thought",
                    theme.editor.hover_popover.prose.clone(),
                )
                .boxed(),
            ])
            .boxed()
    }
}

impl Item for WelcomePage {
    fn tab_content(
        &self,
        _detail: Option<usize>,
        style: &theme::Tab,
        _cx: &gpui::AppContext,
    ) -> gpui::ElementBox {
        Flex::row()
            .with_child(
                Label::new("Welcome to Zed!", style.label.clone())
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .boxed()
    }
}
