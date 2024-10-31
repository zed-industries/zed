use gpui::{actions, AppContext, EventEmitter, FocusHandle, FocusableView, Hsla};
use theme::all_theme_colors;
use ui::{
    prelude::*, utils::calculate_contrast_ratio, AudioStatus, Availability, Avatar,
    AvatarAudioStatusIndicator, AvatarAvailabilityIndicator, ElevationIndex, Facepile, TintColor,
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
            .child(Headline::new("Avatars").size(HeadlineSize::Small))
            .child(
                h_flex()
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
                        div().py_3().px_1_5().bg(Self::preview_bg(cx)).child(
                            Facepile::empty()
                                .child(
                                    Avatar::new(AVATAR_URL)
                                        .border_color(Self::preview_bg(cx))
                                        .size(px(26.))
                                        .into_any_element(),
                                )
                                .child(
                                    Avatar::new(AVATAR_URL)
                                        .border_color(Self::preview_bg(cx))
                                        .size(px(26.))
                                        .into_any_element(),
                                )
                                .child(
                                    Avatar::new(AVATAR_URL)
                                        .border_color(Self::preview_bg(cx))
                                        .size(px(26.))
                                        .into_any_element(),
                                )
                                .child(
                                    Avatar::new(AVATAR_URL)
                                        .border_color(Self::preview_bg(cx))
                                        .size(px(26.))
                                        .into_any_element(),
                                ),
                        ),
                    ),
            )
    }

    fn render_buttons(&self, layer: ElevationIndex, cx: &ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(Headline::new("Buttons").size(HeadlineSize::Small))
            .child(
                h_flex()
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

    fn render_theme_layer(
        &self,
        layer: ElevationIndex,
        cx: &ViewContext<Self>,
    ) -> impl IntoElement {
        let bg = layer.bg(cx);

        let label_with_contrast = |label: &str, fg: Hsla| {
            let contrast = calculate_contrast_ratio(fg, bg);
            format!("{} ({:.2})", label, contrast)
        };

        let all_colors = all_theme_colors(cx);

        v_flex()
            .text_color(cx.theme().colors().text)
            .gap_2()
            .child(Headline::new(layer.clone().to_string()).size(HeadlineSize::Medium))
            .child(self.render_avatars(cx))
            .child(self.render_buttons(layer, cx))
            .child(
                v_flex()
                    .w_full()
                    .gap_px()
                    .children(all_colors.into_iter().map(|(color, name)| {
                        let fg = color;
                        let contrast = (calculate_contrast_ratio(fg, bg) * 100.0).round() / 100.0;

                        h_flex()
                            .gap_2()
                            .border_b_1()
                            .text_xs()
                            .border_color(cx.theme().colors().border)
                            .py_1()
                            .child(
                                div()
                                    .flex_none()
                                    .size_8()
                                    .bg(fg)
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .overflow_hidden(),
                            )
                            .child(
                                div()
                                    .w_64()
                                    .flex_none()
                                    .overflow_hidden()
                                    .truncate()
                                    .child(name),
                            )
                            .child(
                                div()
                                    .w_48()
                                    .flex_none()
                                    .overflow_hidden()
                                    .truncate()
                                    .child(fg.to_string()),
                            )
                            .child(
                                div()
                                    .w_16()
                                    .flex_none()
                                    .overflow_hidden()
                                    .truncate()
                                    .child(contrast.to_string()),
                            )
                    })),
            )
            .child(
                v_flex()
                    .bg(layer.bg(cx))
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .child(
                        h_flex().items_start().child(
                            v_flex()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .p_2()
                                .child(
                                    Headline::new("Text")
                                        .size(HeadlineSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    Label::new(label_with_contrast(
                                        "Default Text",
                                        Color::Default.color(cx),
                                    ))
                                    .color(Color::Default),
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
                                ),
                        ),
                    ),
            )
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
            .child(self.render_theme_layer(ElevationIndex::ElevatedSurface, cx))
    }
}
