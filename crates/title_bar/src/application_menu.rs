use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, ContextMenu, PopoverMenu, Tooltip};

#[derive(IntoElement)]
pub struct ApplicationMenu;

impl ApplicationMenu {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ApplicationMenu {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
        let font = cx.text_style().font();
        let font_id = cx.text_system().resolve_font(&font);
        let width = cx
            .text_system()
            .typographic_bounds(font_id, ui_font_size, 'm')
            .unwrap()
            .size
            .width
            * 3.0;

        PopoverMenu::new("application-menu")
            .menu(move |cx| {
                let width = width;
                ContextMenu::build(cx, move |menu, _cx| {
                    let width = width;
                    menu.header("Workspace")
                        .action("Open Command Palette", Box::new(command_palette::Toggle))
                        .custom_row(move |cx| {
                            div()
                                .w_full()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .cursor(gpui::CursorStyle::Arrow)
                                .child(Label::new("Buffer Font Size"))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .child(div().w(px(16.0)))
                                        .child(
                                            IconButton::new(
                                                "reset-buffer-zoom",
                                                IconName::RotateCcw,
                                            )
                                            .on_click(
                                                |_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::ResetBufferFontSize,
                                                    ))
                                                },
                                            ),
                                        )
                                        .child(
                                            IconButton::new("--buffer-zoom", IconName::Dash)
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::DecreaseBufferFontSize,
                                                    ))
                                                }),
                                        )
                                        .child(
                                            div()
                                                .w(width)
                                                .flex()
                                                .flex_row()
                                                .justify_around()
                                                .child(Label::new(
                                                    theme::get_buffer_font_size(cx).to_string(),
                                                )),
                                        )
                                        .child(
                                            IconButton::new("+-buffer-zoom", IconName::Plus)
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::IncreaseBufferFontSize,
                                                    ))
                                                }),
                                        ),
                                )
                                .into_any_element()
                        })
                        .custom_row(move |cx| {
                            div()
                                .w_full()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .cursor(gpui::CursorStyle::Arrow)
                                .child(Label::new("UI Font Size"))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .child(
                                            IconButton::new("reset-ui-zoom", IconName::RotateCcw)
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::ResetUiFontSize,
                                                    ))
                                                }),
                                        )
                                        .child(
                                            IconButton::new("--ui-zoom", IconName::Dash).on_click(
                                                |_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::DecreaseUiFontSize,
                                                    ))
                                                },
                                            ),
                                        )
                                        .child(
                                            div()
                                                .w(width)
                                                .flex()
                                                .flex_row()
                                                .justify_around()
                                                .child(Label::new(
                                                    theme::get_ui_font_size(cx).to_string(),
                                                )),
                                        )
                                        .child(
                                            IconButton::new("+-ui-zoom", IconName::Plus).on_click(
                                                |_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::IncreaseUiFontSize,
                                                    ))
                                                },
                                            ),
                                        ),
                                )
                                .into_any_element()
                        })
                        .header("Project")
                        .action(
                            "Add Folder to Project...",
                            Box::new(workspace::AddFolderToProject),
                        )
                        .action("Open a new Project...", Box::new(workspace::Open))
                        .action(
                            "Open Recent Projects...",
                            Box::new(recent_projects::OpenRecent {
                                create_new_window: false,
                            }),
                        )
                        .header("Help")
                        .action("About Zed", Box::new(zed_actions::About))
                        .action("Welcome", Box::new(workspace::Welcome))
                        .link(
                            "Documentation",
                            Box::new(zed_actions::OpenBrowser {
                                url: "https://zed.dev/docs".into(),
                            }),
                        )
                        .action("Give Feedback", Box::new(feedback::GiveFeedback))
                        .action("Check for Updates", Box::new(auto_update::Check))
                        .action("View Telemetry", Box::new(zed_actions::OpenTelemetryLog))
                        .action(
                            "View Dependency Licenses",
                            Box::new(zed_actions::OpenLicenses),
                        )
                        .separator()
                        .action("Quit", Box::new(zed_actions::Quit))
                })
                .into()
            })
            .trigger(
                IconButton::new("application-menu", ui::IconName::Menu)
                    .style(ButtonStyle::Subtle)
                    .tooltip(|cx| Tooltip::text("Open Application Menu", cx))
                    .icon_size(IconSize::Small),
            )
            .into_any_element()
    }
}
