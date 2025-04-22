use gpui::{
    App, Context, EventEmitter, IntoElement, PlatformDisplay, Size, Window,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowKind, WindowOptions,
    linear_color_stop, linear_gradient, point,
};
use release_channel::ReleaseChannel;
use std::rc::Rc;
use theme;
use ui::{Render, prelude::*};

pub struct AgentNotification {
    title: SharedString,
    caption: SharedString,
    icon: IconName,
    project_name: Option<SharedString>,
}

impl AgentNotification {
    pub fn new(
        title: impl Into<SharedString>,
        caption: impl Into<SharedString>,
        icon: IconName,
        project_name: Option<impl Into<SharedString>>,
    ) -> Self {
        Self {
            title: title.into(),
            caption: caption.into(),
            icon,
            project_name: project_name.map(|name| name.into()),
        }
    }

    pub fn window_options(screen: Rc<dyn PlatformDisplay>, cx: &App) -> WindowOptions {
        let size = Size {
            width: px(450.),
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

pub enum AgentNotificationEvent {
    Accepted,
    Dismissed,
}

impl EventEmitter<AgentNotificationEvent> for AgentNotification {}

impl Render for AgentNotification {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);
        let line_height = window.line_height();

        let bg = cx.theme().colors().elevated_surface_background;
        let gradient_overflow = || {
            div()
                .h_full()
                .absolute()
                .w_8()
                .bottom_0()
                .right_0()
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(bg, 1.),
                    linear_color_stop(bg.opacity(0.2), 0.),
                ))
        };

        h_flex()
            .id("agent-notification")
            .size_full()
            .p_3()
            .gap_4()
            .justify_between()
            .elevation_3(cx)
            .text_ui(cx)
            .font(ui_font)
            .border_color(cx.theme().colors().border)
            .rounded_xl()
            .on_click(cx.listener(|_, _, _, cx| {
                cx.emit(AgentNotificationEvent::Accepted);
            }))
            .child(
                h_flex()
                    .items_start()
                    .gap_2()
                    .flex_1()
                    .child(
                        h_flex().h(line_height).justify_center().child(
                            Icon::new(self.icon)
                                .color(Color::Muted)
                                .size(IconSize::Small),
                        ),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .max_w(px(300.))
                            .child(
                                div()
                                    .relative()
                                    .text_size(px(14.))
                                    .text_color(cx.theme().colors().text)
                                    .truncate()
                                    .child(self.title.clone())
                                    .child(gradient_overflow()),
                            )
                            .child(
                                h_flex()
                                    .relative()
                                    .gap_1()
                                    .text_size(px(12.))
                                    .text_color(cx.theme().colors().text_muted)
                                    .truncate()
                                    .when_some(
                                        self.project_name.clone(),
                                        |description, project_name| {
                                            description.child(
                                                h_flex()
                                                    .gap_1()
                                                    .child(
                                                        Icon::new(IconName::Folder)
                                                            .size(IconSize::XSmall)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(
                                                        div()
                                                            .max_w_16()
                                                            .truncate()
                                                            .child(project_name),
                                                    )
                                                    .child(
                                                        div().size(px(3.)).rounded_full().bg(cx
                                                            .theme()
                                                            .colors()
                                                            .text
                                                            .opacity(0.2)),
                                                    ),
                                            )
                                        },
                                    )
                                    .child(self.caption.clone())
                                    .child(gradient_overflow()),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .gap_1()
                    .items_center()
                    .child(
                        Button::new("open", "View Panel")
                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .full_width()
                            .on_click({
                                cx.listener(move |_this, _event, _, cx| {
                                    cx.emit(AgentNotificationEvent::Accepted);
                                })
                            }),
                    )
                    .child(Button::new("dismiss", "Dismiss").full_width().on_click({
                        cx.listener(move |_, _event, _, cx| {
                            cx.emit(AgentNotificationEvent::Dismissed);
                        })
                    })),
            )
    }
}
