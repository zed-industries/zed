use ui::{prelude::*, ContextMenu, NumericStepper, PopoverMenu, Tooltip};

#[derive(IntoElement)]
pub struct ApplicationMenu;

impl ApplicationMenu {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ApplicationMenu {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        PopoverMenu::new("application-menu")
            .menu(move |cx| {
                ContextMenu::build(cx, move |menu, cx| {
                    menu.header("Workspace")
                        .action("Open Command Palette", Box::new(command_palette::Toggle))
                        .when_some(cx.focused(), |menu, focused| menu.context(focused))
                        .custom_row(move |cx| {
                            h_flex()
                                .gap_2()
                                .w_full()
                                .justify_between()
                                .cursor(gpui::CursorStyle::Arrow)
                                .child(Label::new("Buffer Font Size"))
                                .child(
                                    NumericStepper::new(
                                        "buffer-font-size",
                                        theme::get_buffer_font_size(cx).to_string(),
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::DecreaseBufferFontSize,
                                            ))
                                        },
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::IncreaseBufferFontSize,
                                            ))
                                        },
                                    )
                                    .reserve_space_for_reset(true)
                                    .when(
                                        theme::has_adjusted_buffer_font_size(cx),
                                        |stepper| {
                                            stepper.on_reset(|_, cx| {
                                                cx.dispatch_action(Box::new(
                                                    zed_actions::ResetBufferFontSize,
                                                ))
                                            })
                                        },
                                    ),
                                )
                                .into_any_element()
                        })
                        .custom_row(move |cx| {
                            h_flex()
                                .gap_2()
                                .w_full()
                                .justify_between()
                                .cursor(gpui::CursorStyle::Arrow)
                                .child(Label::new("UI Font Size"))
                                .child(
                                    NumericStepper::new(
                                        "ui-font-size",
                                        theme::get_ui_font_size(cx).to_string(),
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::DecreaseUiFontSize,
                                            ))
                                        },
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::IncreaseUiFontSize,
                                            ))
                                        },
                                    )
                                    .reserve_space_for_reset(true)
                                    .when(
                                        theme::has_adjusted_ui_font_size(cx),
                                        |stepper| {
                                            stepper.on_reset(|_, cx| {
                                                cx.dispatch_action(Box::new(
                                                    zed_actions::ResetUiFontSize,
                                                ))
                                            })
                                        },
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
