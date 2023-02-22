use gpui::{
    actions,
    elements::{Flex, Label, ParentElement},
    Element, Entity, MutableAppContext, View,
};
use settings::Settings;
use workspace::{item::Item, Workspace};

actions!(welcome, [ShowWelcome]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|workspace: &mut Workspace, _: &ShowWelcome, cx| {
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
        Label::new("Welcome page", theme.editor.hover_popover.prose.clone()).boxed()
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
