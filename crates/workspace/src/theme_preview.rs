#![allow(unused, dead_code)]
use gpui::{actions, AppContext, EventEmitter, FocusHandle, FocusableView, Hsla};
use theme::all_theme_colors;
use ui::{
    prelude::*, utils::calculate_contrast_ratio, AudioStatus, Availability, Avatar,
    AvatarAudioStatusIndicator, AvatarAvailabilityIndicator, ButtonLike, ElevationIndex, Facepile,
    TintColor, Tooltip,
};

use crate::{Item, Workspace};

actions!(debug, [OpenThemePreview]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &OpenThemePreview, cx| {
            let theme_preview = cx.new_view(ThemePreview::new);
            workspace.add_item_to_active_pane(Box::new(theme_preview), None, true, cx)
        });
    })
    .detach();
}

struct ThemePreview {
    focus_handle: FocusHandle,
}

impl ThemePreview {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl EventEmitter<()> for ThemePreview {}

impl FocusableView for ThemePreview {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl ThemePreview {}

impl Item for ThemePreview {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(crate::item::ItemEvent)) {}

    fn tab_content_text(&self, cx: &WindowContext) -> Option<SharedString> {
        let name = cx.theme().name.clone();
        Some(format!("{} Preview", name).into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<crate::WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(Self::new))
    }
}

const AVATAR_URL: &str = "https://avatars.githubusercontent.com/u/1714999?v=4";

impl ThemePreview {
    fn preview_bg(cx: &WindowContext) -> Hsla {
        cx.theme().colors().editor_background
    }

    fn render_avatars(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(
                Headline::new("Avatars")
                    .size(HeadlineSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .items_start()
                    .gap_4()
                    .child(Avatar::new(AVATAR_URL).size(px(24.)))
                    .child(Avatar::new(AVATAR_URL).size(px(24.)).grayscale(true))
                    .child(
                        Avatar::new(AVATAR_URL)
                            .size(px(24.))
                            .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Muted)),
                    )
                    .child(
                        Avatar::new(AVATAR_URL)
                            .size(px(24.))
                            .indicator(AvatarAudioStatusIndicator::new(AudioStatus::Deafened)),
                    )
                    .child(
                        Avatar::new(AVATAR_URL)
                            .size(px(24.))
                            .indicator(AvatarAvailabilityIndicator::new(Availability::Free)),
                    )
                    .child(
                        Avatar::new(AVATAR_URL)
                            .size(px(24.))
                            .indicator(AvatarAvailabilityIndicator::new(Availability::Free)),
                    )
                    .child(
                        Facepile::empty()
                            .child(
                                Avatar::new(AVATAR_URL)
                                    .border_color(Self::preview_bg(cx))
                                    .size(px(22.))
                                    .into_any_element(),
                            )
                            .child(
                                Avatar::new(AVATAR_URL)
                                    .border_color(Self::preview_bg(cx))
                                    .size(px(22.))
                                    .into_any_element(),
                            )
                            .child(
                                Avatar::new(AVATAR_URL)
                                    .border_color(Self::preview_bg(cx))
                                    .size(px(22.))
                                    .into_any_element(),
                            )
                            .child(
                                Avatar::new(AVATAR_URL)
                                    .border_color(Self::preview_bg(cx))
                                    .size(px(22.))
                                    .into_any_element(),
                            ),
                    ),
            )
    }

    fn render_buttons(&self, layer: ElevationIndex, cx: &ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(
                Headline::new("Buttons")
                    .size(HeadlineSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .items_start()
                    .gap_px()
                    .child(
                        IconButton::new("icon_button_transparent", IconName::Check)
                            .style(ButtonStyle::Transparent),
                    )
                    .child(
                        IconButton::new("icon_button_subtle", IconName::Check)
                            .style(ButtonStyle::Subtle),
                    )
                    .child(
                        IconButton::new("icon_button_filled", IconName::Check)
                            .style(ButtonStyle::Filled),
                    )
                    .child(
                        IconButton::new("icon_button_selected_accent", IconName::Check)
                            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                            .selected(true),
                    )
                    .child(IconButton::new("icon_button_selected", IconName::Check).selected(true))
                    .child(
                        IconButton::new("icon_button_positive", IconName::Check)
                            .style(ButtonStyle::Tinted(TintColor::Positive)),
                    )
                    .child(
                        IconButton::new("icon_button_warning", IconName::Check)
                            .style(ButtonStyle::Tinted(TintColor::Warning)),
                    )
                    .child(
                        IconButton::new("icon_button_negative", IconName::Check)
                            .style(ButtonStyle::Tinted(TintColor::Negative)),
                    ),
            )
            .child(
                h_flex()
                    .gap_px()
                    .child(
                        Button::new("button_transparent", "Transparent")
                            .style(ButtonStyle::Transparent),
                    )
                    .child(Button::new("button_subtle", "Subtle").style(ButtonStyle::Subtle))
                    .child(Button::new("button_filled", "Filled").style(ButtonStyle::Filled))
                    .child(
                        Button::new("button_selected", "Selected")
                            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                            .selected(true),
                    )
                    .child(
                        Button::new("button_selected_tinted", "Selected (Tinted)")
                            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                            .selected(true),
                    )
                    .child(
                        Button::new("button_positive", "Tint::Positive")
                            .style(ButtonStyle::Tinted(TintColor::Positive)),
                    )
                    .child(
                        Button::new("button_warning", "Tint::Warning")
                            .style(ButtonStyle::Tinted(TintColor::Warning)),
                    )
                    .child(
                        Button::new("button_negative", "Tint::Negative")
                            .style(ButtonStyle::Tinted(TintColor::Negative)),
                    ),
            )
    }

    fn render_text(&self, layer: ElevationIndex, cx: &ViewContext<Self>) -> impl IntoElement {
        let bg = layer.bg(cx);

        let label_with_contrast = |label: &str, fg: Hsla| {
            let contrast = calculate_contrast_ratio(fg, bg);
            format!("{} ({:.2})", label, contrast)
        };

        v_flex()
            .gap_1()
            .child(Headline::new("Text").size(HeadlineSize::Small).color(Color::Muted))
            .child(
                h_flex()
                    .items_start()
                    .gap_4()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Headline::new("Headline Sizes").size(HeadlineSize::Small).color(Color::Muted))
                            .child(Headline::new("XLarge Headline").size(HeadlineSize::XLarge))
                            .child(Headline::new("Large Headline").size(HeadlineSize::Large))
                            .child(Headline::new("Medium Headline").size(HeadlineSize::Medium))
                            .child(Headline::new("Small Headline").size(HeadlineSize::Small))
                            .child(Headline::new("XSmall Headline").size(HeadlineSize::XSmall)),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Headline::new("Text Colors").size(HeadlineSize::Small).color(Color::Muted))
                            .child(
                                Label::new(label_with_contrast(
                                    "Default Text",
                                    Color::Default.color(cx),
                                ))
                                .color(Color::Default),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Accent Text",
                                    Color::Accent.color(cx),
                                ))
                                .color(Color::Accent),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Conflict Text",
                                    Color::Conflict.color(cx),
                                ))
                                .color(Color::Conflict),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Created Text",
                                    Color::Created.color(cx),
                                ))
                                .color(Color::Created),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Deleted Text",
                                    Color::Deleted.color(cx),
                                ))
                                .color(Color::Deleted),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Disabled Text",
                                    Color::Disabled.color(cx),
                                ))
                                .color(Color::Disabled),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Error Text",
                                    Color::Error.color(cx),
                                ))
                                .color(Color::Error),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Hidden Text",
                                    Color::Hidden.color(cx),
                                ))
                                .color(Color::Hidden),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Hint Text",
                                    Color::Hint.color(cx),
                                ))
                                .color(Color::Hint),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Ignored Text",
                                    Color::Ignored.color(cx),
                                ))
                                .color(Color::Ignored),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Info Text",
                                    Color::Info.color(cx),
                                ))
                                .color(Color::Info),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Modified Text",
                                    Color::Modified.color(cx),
                                ))
                                .color(Color::Modified),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Muted Text",
                                    Color::Muted.color(cx),
                                ))
                                .color(Color::Muted),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Placeholder Text",
                                    Color::Placeholder.color(cx),
                                ))
                                .color(Color::Placeholder),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Selected Text",
                                    Color::Selected.color(cx),
                                ))
                                .color(Color::Selected),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Success Text",
                                    Color::Success.color(cx),
                                ))
                                .color(Color::Success),
                            )
                            .child(
                                Label::new(label_with_contrast(
                                    "Warning Text",
                                    Color::Warning.color(cx),
                                ))
                                .color(Color::Warning),
                            )
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(Headline::new("Wrapping Text").size(HeadlineSize::Small).color(Color::Muted))
                            .child(
                                div().max_w(px(200.)).child(
                                "This is a longer piece of text that should wrap to multiple lines. It demonstrates how text behaves when it exceeds the width of its container."
                            ))
                    )
            )
    }

    fn render_colors(&self, layer: ElevationIndex, cx: &ViewContext<Self>) -> impl IntoElement {
        let bg = layer.bg(cx);
        let all_colors = all_theme_colors(cx);

        v_flex()
            .gap_1()
            .child(
                Headline::new("Colors")
                    .size(HeadlineSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_1()
                    .children(all_colors.into_iter().map(|(color, name)| {
                        let id = ElementId::Name(format!("{:?}-preview", color).into());
                        let name = name.clone();
                        div().size_8().flex_none().child(
                            ButtonLike::new(id)
                                .child(
                                    div()
                                        .size_8()
                                        .bg(color)
                                        .border_1()
                                        .border_color(cx.theme().colors().border)
                                        .overflow_hidden(),
                                )
                                .size(ButtonSize::None)
                                .style(ButtonStyle::Transparent)
                                .tooltip(move |cx| {
                                    let name = name.clone();
                                    Tooltip::with_meta(name, None, format!("{:?}", color), cx)
                                }),
                        )
                    })),
            )
    }

    fn render_theme_layer(
        &self,
        layer: ElevationIndex,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        v_flex()
            .p_4()
            .bg(layer.bg(cx))
            .text_color(cx.theme().colors().text)
            .gap_2()
            .child(Headline::new(layer.clone().to_string()).size(HeadlineSize::Medium))
            .child(self.render_avatars(cx))
            .child(self.render_buttons(layer, cx))
            .child(self.render_text(layer, cx))
            .child(self.render_colors(layer, cx))
    }
}

impl Render for ThemePreview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl ui::IntoElement {
        v_flex()
            .id("theme-preview")
            .key_context("ThemePreview")
            .overflow_scroll()
            .size_full()
            .max_h_full()
            .p_4()
            .track_focus(&self.focus_handle)
            .bg(Self::preview_bg(cx))
            .gap_4()
            .child(self.render_theme_layer(ElevationIndex::Background, cx))
            .child(self.render_theme_layer(ElevationIndex::Surface, cx))
            .child(self.render_theme_layer(ElevationIndex::EditorSurface, cx))
            .child(self.render_theme_layer(ElevationIndex::ElevatedSurface, cx))
    }
}
