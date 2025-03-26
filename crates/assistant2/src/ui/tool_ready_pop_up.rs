use gpui::{
    point, App, Context, EventEmitter, IntoElement, PlatformDisplay, Size, Window,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowKind, WindowOptions,
};
use release_channel::ReleaseChannel;
use std::rc::Rc;
use theme;
use ui::{prelude::*, Render};

pub struct ToolReadyPopUp {
    caption: SharedString,
}

impl ToolReadyPopUp {
    pub fn new(caption: impl Into<SharedString>) -> Self {
        Self {
            caption: caption.into(),
        }
    }

    pub fn window_options(screen: Rc<dyn PlatformDisplay>, cx: &App) -> WindowOptions {
        let size = Size {
            width: px(440.),
            height: px(72.),
        };

        let notification_margin_width = px(16.);
        let notification_margin_height = px(-48.);

        let bounds = gpui::Bounds::<Pixels> {
            origin: screen.bounds().top_right()
                - point(
                    size.width + notification_margin_width,
                    notification_margin_height,
                ),
            size,
        };

        let app_id = ReleaseChannel::global(cx).app_id();

        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: None,
            focus: false,
            show: true,
            kind: WindowKind::PopUp,
            is_movable: false,
            display_id: Some(screen.id()),
            window_background: WindowBackgroundAppearance::Transparent,
            app_id: Some(app_id.to_owned()),
            window_min_size: None,
            window_decorations: Some(WindowDecorations::Client),
        }
    }
}

pub enum ToolReadyPopupEvent {
    Accepted,
    Dismissed,
}

impl EventEmitter<ToolReadyPopupEvent> for ToolReadyPopUp {}

impl Render for ToolReadyPopUp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);
        let line_height = window.line_height();

        h_flex()
            .size_full()
            .p_3()
            .gap_4()
            .justify_between()
            .elevation_3(cx)
            .text_ui(cx)
            .font(ui_font)
            .border_color(cx.theme().colors().border)
            .rounded_xl()
            .child(
                h_flex()
                    .items_start()
                    .gap_2()
                    .child(
                        h_flex().h(line_height).justify_center().child(
                            Icon::new(IconName::Info)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(
                        v_flex()
                            .child(Headline::new("Agent Panel").size(HeadlineSize::XSmall))
                            .child(Label::new(self.caption.clone()).color(Color::Muted)),
                    ),
            )
            .child(
                h_flex()
                    .gap_0p5()
                    .child(
                        Button::new("open", "View Panel")
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .on_click({
                                cx.listener(move |_this, _event, _, cx| {
                                    cx.emit(ToolReadyPopupEvent::Accepted);
                                })
                            }),
                    )
                    .child(Button::new("dismiss", "Dismiss").on_click({
                        cx.listener(move |_, _event, _, cx| {
                            cx.emit(ToolReadyPopupEvent::Dismissed);
                        })
                    })),
            )
    }
}
