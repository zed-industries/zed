use ui::prelude::*;

use crate::story::Story;

#[derive(Element, Default)]
pub struct KitchenSinkStory {}

impl KitchenSinkStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        Story::container(cx)
            .child(Story::title(cx, "Kitchen Sink"))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .overflow_y_scroll(ScrollState::default())
                    .child(crate::stories::elements::avatar::AvatarStory::default())
                    .child(crate::stories::elements::button::ButtonStory::default())
                    .child(crate::stories::elements::icon::IconStory::default())
                    .child(crate::stories::elements::input::InputStory::default())
                    .child(crate::stories::elements::label::LabelStory::default())
                    .child(
                        crate::stories::components::assistant_panel::AssistantPanelStory::default(),
                    )
                    .child(crate::stories::components::breadcrumb::BreadcrumbStory::default())
                    .child(crate::stories::components::buffer::BufferStory::default())
                    .child(crate::stories::components::chat_panel::ChatPanelStory::default())
                    .child(crate::stories::components::collab_panel::CollabPanelStory::default())
                    .child(crate::stories::components::facepile::FacepileStory::default())
                    .child(crate::stories::components::keybinding::KeybindingStory::default())
                    .child(crate::stories::components::palette::PaletteStory::default())
                    .child(crate::stories::components::panel::PanelStory::default())
                    .child(crate::stories::components::project_panel::ProjectPanelStory::default())
                    .child(crate::stories::components::status_bar::StatusBarStory::default())
                    .child(crate::stories::components::tab::TabStory::default())
                    .child(crate::stories::components::tab_bar::TabBarStory::default())
                    .child(crate::stories::components::terminal::TerminalStory::default())
                    .child(crate::stories::components::title_bar::TitleBarStory::default())
                    .child(crate::stories::components::toolbar::ToolbarStory::default())
                    .child(
                        crate::stories::components::traffic_lights::TrafficLightsStory::default(),
                    )
                    .child(crate::stories::components::context_menu::ContextMenuStory::default()),
            )
    }
}
