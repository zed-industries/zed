use assistant_tooling::ToolRegistry;
use client::User;
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{FontStyle, FontWeight, TextStyle, View, WhiteSpace};
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{prelude::*, Avatar, Tooltip};

use crate::{Submit, SubmitMode};

#[derive(IntoElement)]
pub struct Composer {
    editor: View<Editor>,
    player: Option<Arc<User>>,
    can_submit: bool,
    tool_registry: Arc<ToolRegistry>,
}

impl Composer {
    pub fn new(
        editor: View<Editor>,
        player: Option<Arc<User>>,
        can_submit: bool,
        tool_registry: Arc<ToolRegistry>,
    ) -> Self {
        Self {
            editor,
            player,
            can_submit,
            tool_registry,
        }
    }
}

impl RenderOnce for Composer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let mut player_avatar = div().into_any_element();
        if let Some(player) = self.player.clone() {
            player_avatar = Avatar::new(player.avatar_uri.clone())
                .size(rems(20.0 / 16.0))
                .into_any_element();
        }

        let font_size = rems(0.875);
        let line_height = font_size.to_pixels(cx.rem_size()) * 1.3;

        h_flex()
            .w_full()
            .items_start()
            .gap_3()
            .child(player_avatar)
            .child(
                v_flex()
                    .size_full()
                    .gap_1()
                    .child(
                        v_flex()
                            .w_full()
                            .p_4()
                            .bg(cx.theme().colors().editor_background)
                            .rounded_lg()
                            .child(
                                v_flex()
                                    .justify_between()
                                    .w_full()
                                    .gap_1()
                                    .min_h(line_height * 4 + px(74.0))
                                    .child({
                                        let settings = ThemeSettings::get_global(cx);
                                        let text_style = TextStyle {
                                            color: cx.theme().colors().editor_foreground,
                                            font_family: settings.buffer_font.family.clone(),
                                            font_features: settings.buffer_font.features.clone(),
                                            font_size: font_size.into(),
                                            font_weight: FontWeight::NORMAL,
                                            font_style: FontStyle::Normal,
                                            line_height: line_height.into(),
                                            background_color: None,
                                            underline: None,
                                            strikethrough: None,
                                            white_space: WhiteSpace::Normal,
                                        };

                                        EditorElement::new(
                                            &self.editor,
                                            EditorStyle {
                                                background: cx.theme().colors().editor_background,
                                                local_player: cx.theme().players().local(),
                                                text: text_style,
                                                ..Default::default()
                                            },
                                        )
                                    })
                                    .child(
                                        h_flex()
                                            .flex_none()
                                            .gap_2()
                                            .justify_between()
                                            .w_full()
                                            .child(
                                                h_flex().gap_1().child(
                                                    // IconButton/button
                                                    // Toggle - if enabled, .selected(true).selected_style(IconButtonStyle::Filled)
                                                    //
                                                    // match status
                                                    // Tooltip::with_meta("some label explaining project index + status", "click to enable")
                                                    IconButton::new(
                                                        "add-context",
                                                        IconName::FileDoc,
                                                    )
                                                    .icon_color(Color::Muted),
                                                ), // .child(
                                                   //     IconButton::new(
                                                   //         "add-context",
                                                   //         IconName::Plus,
                                                   //     )
                                                   //     .icon_color(Color::Muted),
                                                   // ),
                                            )
                                            .child(
                                                Button::new("send-button", "Send")
                                                    .style(ButtonStyle::Filled)
                                                    .disabled(!self.can_submit)
                                                    .on_click(|_, cx| {
                                                        cx.dispatch_action(Box::new(Submit(
                                                            SubmitMode::Codebase,
                                                        )))
                                                    })
                                                    .tooltip(|cx| {
                                                        Tooltip::for_action(
                                                            "Submit message",
                                                            &Submit(SubmitMode::Codebase),
                                                            cx,
                                                        )
                                                    }),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .child(Button::new("switch-model", "gpt-4-turbo").color(Color::Muted))
                            .children(self.tool_registry.status_views().iter().cloned()),
                    ),
            )
    }
}
