use gpui::Render;
use story::{StoryContainer, StoryItem, StorySection};

use crate::{prelude::*, Tooltip};
use crate::{IconButton, IconPath};

pub struct IconButtonStory;

impl Render for IconButtonStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let default_button = StoryItem::new(
            "Default",
            IconButton::new("default_icon_button", IconPath::Hash),
        )
        .description("Displays an icon button.")
        .usage(
            r#"
            IconButton::new("default_icon_button", Icon::Hash)
        "#,
        );

        let selected_button = StoryItem::new(
            "Selected",
            IconButton::new("selected_icon_button", IconPath::Hash).selected(true),
        )
        .description("Displays an icon button that is selected.")
        .usage(
            r#"
            IconButton::new("selected_icon_button", Icon::Hash).selected(true)
        "#,
        );

        let selected_with_selected_icon = StoryItem::new(
            "Selected with `selected_icon`",
            IconButton::new("selected_with_selected_icon_button", IconPath::AudioOn)
                .selected(true)
                .selected_icon(IconPath::AudioOff),
        )
        .description(
            "Displays an icon button that is selected and shows a different icon when selected.",
        )
        .usage(
            r#"
            IconButton::new("selected_with_selected_icon_button", Icon::AudioOn)
                .selected(true)
                .selected_icon(Icon::AudioOff)
        "#,
        );

        let disabled_button = StoryItem::new(
            "Disabled",
            IconButton::new("disabled_icon_button", IconPath::Hash).disabled(true),
        )
        .description("Displays an icon button that is disabled.")
        .usage(
            r#"
            IconButton::new("disabled_icon_button", Icon::Hash).disabled(true)
        "#,
        );

        let with_on_click_button = StoryItem::new(
            "With `on_click`",
            IconButton::new("with_on_click_button", IconPath::Ai).on_click(|_event, _cx| {
                println!("Clicked!");
            }),
        )
        .description("Displays an icon button which triggers an event on click.")
        .usage(
            r#"
            IconButton::new("with_on_click_button", Icon::Ai).on_click(|_event, _cx| {
                println!("Clicked!");
            })
        "#,
        );

        let with_tooltip_button = StoryItem::new(
            "With `tooltip`",
            IconButton::new("with_tooltip_button", IconPath::MessageBubbles)
                .tooltip(|cx| Tooltip::text("Open messages", cx)),
        )
        .description("Displays an icon button that has a tooltip when hovered.")
        .usage(
            r#"
            IconButton::new("with_tooltip_button", Icon::MessageBubbles)
                .tooltip(|cx| Tooltip::text("Open messages", cx))
        "#,
        );

        let selected_with_tooltip_button = StoryItem::new(
            "Selected with `tooltip`",
            IconButton::new("selected_with_tooltip_button", IconPath::InlayHint)
                .selected(true)
                .tooltip(|cx| Tooltip::text("Toggle inlay hints", cx)),
        )
        .description("Displays a selected icon button with tooltip.")
        .usage(
            r#"
            IconButton::new("selected_with_tooltip_button", Icon::InlayHint)
                .selected(true)
                .tooltip(|cx| Tooltip::text("Toggle inlay hints", cx))
        "#,
        );

        let buttons = vec![
            default_button,
            selected_button,
            selected_with_selected_icon,
            disabled_button,
            with_on_click_button,
            with_tooltip_button,
            selected_with_tooltip_button,
        ];

        StoryContainer::new(
            "Icon Button",
            "crates/ui2/src/components/stories/icon_button.rs",
        )
        .children(vec![StorySection::new().children(buttons)])
        .into_element()

        // Story::container()
        //     .child(Story::title_for::<IconButton>())
        //     .child(Story::label("Default"))
        //     .child(div().w_8().child(IconButton::new("icon_a", Icon::Hash)))
        //     .child(Story::label("Selected"))
        //     .child(
        //         div()
        //             .w_8()
        //             .child(IconButton::new("icon_a", Icon::Hash).selected(true)),
        //     )
        //     .child(Story::label("Selected with `selected_icon`"))
        //     .child(
        //         div().w_8().child(
        //             IconButton::new("icon_a", Icon::AudioOn)
        //                 .selected(true)
        //                 .selected_icon(Icon::AudioOff),
        //         ),
        //     )
        //     .child(Story::label("Disabled"))
        //     .child(
        //         div()
        //             .w_8()
        //             .child(IconButton::new("icon_a", Icon::Hash).disabled(true)),
        //     )
        //     .child(Story::label("With `on_click`"))
        //     .child(
        //         div()
        //             .w_8()
        //             .child(
        //                 IconButton::new("with_on_click", Icon::Ai).on_click(|_event, _cx| {
        //                     println!("Clicked!");
        //                 }),
        //             ),
        //     )
        //     .child(Story::label("With `tooltip`"))
        //     .child(
        //         div().w_8().child(
        //             IconButton::new("with_tooltip", Icon::MessageBubbles)
        //                 .tooltip(|cx| Tooltip::text("Open messages", cx)),
        //         ),
        //     )
        //     .child(Story::label("Selected with `tooltip`"))
        //     .child(
        //         div().w_8().child(
        //             IconButton::new("selected_with_tooltip", Icon::InlayHint)
        //                 .selected(true)
        //                 .tooltip(|cx| Tooltip::text("Toggle inlay hints", cx)),
        //         ),
        //     )
    }
}
