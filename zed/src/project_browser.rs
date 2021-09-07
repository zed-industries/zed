use gpui::{Element, Entity, View, elements::{ConstrainedBox, Container, Flex, Label, ParentElement, Text}};
use postage::watch;

use crate::Settings;

pub struct ProjectBrowser {
    settings: watch::Receiver<Settings>
}

impl ProjectBrowser {
    pub fn new(settings: watch::Receiver<Settings>) -> Self { 
        Self { settings }
    }
}

pub enum Event {}

impl Entity for ProjectBrowser {
    type Event = Event;
}

impl View for ProjectBrowser {
    fn ui_name() -> &'static str {
        "ProjectBrowser"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme.project_browser;

        ConstrainedBox::new(
            Container::new(
                Flex::column()
                    .with_child(
                        Container::new(
                            Flex::row()
                                .with_child(
                                    Container::new(
                                        Label::new(
                                            // TODO: Project folder goes here
                                            "zed".to_string(),
                                            theme.headline.clone(),
                                        )
                                        .boxed(),
                                    )
                                    .boxed(),
                                )
                                .with_child(
                                    Container::new(
                                        Text::new(
                                            // TODO: Project path goes here
                                            "~/code/zed".to_string(), 
                                            theme.item.clone()
                                        ).boxed(),
                                    )
                                    .with_margin_left(4.0)
                                    .boxed(),
                                )
                            .boxed()
                            
                        )
                        .with_style(&theme.header)
                        .boxed()
                    )
                    .with_child(
                        Text::new( 
                            // TODO: List of files goes here
                            "wip. static content".to_string(), 
                            theme.item.clone()
                        ).boxed()
                    )
                    .boxed(),
            )
            .with_style(&theme.container)
            .boxed()
        )
        .with_min_width(220.0)
        .boxed()
        
    }
}
