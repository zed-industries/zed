mod base_keymap_picker;
mod base_keymap_setting;

use client::{telemetry::Telemetry, TelemetrySettings};
use gpui::{
    svg, uniform_list, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, ParentElement, Render, Styled, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use ui::{prelude::*, Checkbox};
use vim::VimModeSetting;
use workspace::{
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new, AppState, Extensions, Workspace, WorkspaceId,
};

pub use base_keymap_setting::BaseKeymap;

pub fn init(cx: &mut AppContext) {
    BaseKeymap::register(cx);

    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &Extensions, cx| {
            let welcome_page = ExtensionsPage::new(workspace, cx);
            workspace.add_item(Box::new(welcome_page), cx)
        });
    })
    .detach();

    base_keymap_picker::init(cx);
}

pub fn show_extensions_view(app_state: &Arc<AppState>, cx: &mut AppContext) {
    open_new(&app_state, cx, |workspace, cx| {
        workspace.toggle_dock(DockPosition::Left, cx);
        let welcome_page = ExtensionsPage::new(workspace, cx);
        workspace.add_item_to_center(Box::new(welcome_page.clone()), cx);
        cx.focus_view(&welcome_page);
        cx.notify();
    })
    .detach();
}
pub struct Extension {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub repository: String,
    pub download_url: String,
    pub installed: bool,
}

pub struct ExtensionsPage {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    telemetry: Arc<Telemetry>,
    extensions_entries: Vec<Extension>,
}

impl Render for ExtensionsPage {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .full()
            .bg(cx.theme().colors().editor_background)
            .track_focus(&self.focus_handle)
            .child(
                v_flex()
                    .w_96()
                    .gap_4()
                    .mx_auto()
                    .child(
                        svg()
                            .path("icons/logo_96.svg")
                            .text_color(gpui::white())
                            .w(px(96.))
                            .h(px(96.))
                            .mx_auto(),
                    )
                    .child(
                        h_flex()
                            .justify_center()
                            .child(Label::new("Extensions Page")),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Button::new("choose-theme", "Choose a theme")
                                    .full_width()
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.telemetry.report_app_event(
                                            "welcome page: change theme".to_string(),
                                        );
                                        this.workspace
                                            .update(cx, |workspace, cx| {
                                                theme_selector::toggle(
                                                    workspace,
                                                    &Default::default(),
                                                    cx,
                                                )
                                            })
                                            .ok();
                                    })),
                            )
                            .child(
                                Button::new("choose-keymap", "Choose a keymap")
                                    .full_width()
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.telemetry.report_app_event(
                                            "welcome page: change keymap".to_string(),
                                        );
                                        this.workspace
                                            .update(cx, |workspace, cx| {
                                                base_keymap_picker::toggle(
                                                    workspace,
                                                    &Default::default(),
                                                    cx,
                                                )
                                            })
                                            .ok();
                                    })),
                            )
                            .child(
                                Button::new("install-cli", "Install the CLI")
                                    .full_width()
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.telemetry.report_app_event(
                                            "welcome page: install cli".to_string(),
                                        );
                                        cx.app_mut()
                                            .spawn(|cx| async move {
                                                install_cli::install_cli(&cx).await
                                            })
                                            .detach_and_log_err(cx);
                                    })),
                            ),
                    )
                    .child(self.render_extensions(cx))
                    .child(
                        v_flex()
                            .p_3()
                            .gap_2()
                            .bg(cx.theme().colors().elevated_surface_background)
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_md()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Checkbox::new(
                                            "enable-vim",
                                            if VimModeSetting::get_global(cx).0 {
                                                ui::Selection::Selected
                                            } else {
                                                ui::Selection::Unselected
                                            },
                                        )
                                        .on_click(
                                            cx.listener(move |this, selection, cx| {
                                                this.telemetry.report_app_event(
                                                    "welcome page: toggle vim".to_string(),
                                                );
                                                this.update_settings::<VimModeSetting>(
                                                    selection,
                                                    cx,
                                                    |setting, value| *setting = Some(value),
                                                );
                                            }),
                                        ),
                                    )
                                    .child(Label::new("Enable vim mode")),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Checkbox::new(
                                            "enable-telemetry",
                                            if TelemetrySettings::get_global(cx).metrics {
                                                ui::Selection::Selected
                                            } else {
                                                ui::Selection::Unselected
                                            },
                                        )
                                        .on_click(
                                            cx.listener(move |this, selection, cx| {
                                                this.telemetry.report_app_event(
                                                    "welcome page: toggle metric telemetry"
                                                        .to_string(),
                                                );
                                                this.update_settings::<TelemetrySettings>(
                                                    selection,
                                                    cx,
                                                    {
                                                        let telemetry = this.telemetry.clone();

                                                        move |settings, value| {
                                                            settings.metrics = Some(value);

                                                            telemetry.report_setting_event(
                                                                "metric telemetry",
                                                                value.to_string(),
                                                            );
                                                        }
                                                    },
                                                );
                                            }),
                                        ),
                                    )
                                    .child(Label::new("Send anonymous usage data")),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Checkbox::new(
                                            "enable-crash",
                                            if TelemetrySettings::get_global(cx).diagnostics {
                                                ui::Selection::Selected
                                            } else {
                                                ui::Selection::Unselected
                                            },
                                        )
                                        .on_click(
                                            cx.listener(move |this, selection, cx| {
                                                this.telemetry.report_app_event(
                                                    "welcome page: toggle diagnostic telemetry"
                                                        .to_string(),
                                                );
                                                this.update_settings::<TelemetrySettings>(
                                                    selection,
                                                    cx,
                                                    {
                                                        let telemetry = this.telemetry.clone();

                                                        move |settings, value| {
                                                            settings.diagnostics = Some(value);

                                                            telemetry.report_setting_event(
                                                                "diagnostic telemetry",
                                                                value.to_string(),
                                                            );
                                                        }
                                                    },
                                                );
                                            }),
                                        ),
                                    )
                                    .child(Label::new("Send crash reports")),
                            ),
                    ),
            )
    }
}

impl ExtensionsPage {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let project = workspace.project().clone();
        let extensions_panel = cx.new_view(|cx: &mut ViewContext<Self>| {
            // cx.observe(&project, |this, _, cx| {
            //     this.get_extensions_from_server(cx);
            //     cx.notify();
            // });
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, Self::focus_in).detach();

            cx.on_release(|this: &mut Self, _, _| {
                this.telemetry
                    .report_app_event("welcome page: close".to_string());
                println!("Extensions page closed");
            })
            .detach();
            let this = Self {
                focus_handle: cx.focus_handle(),
                workspace: workspace.weak_handle(),
                telemetry: workspace.client().telemetry().clone(),
                extensions_entries: Vec::new(),
            };

            this
        });

        extensions_panel
    }
    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        // if !self.focus_handle.contains_focused(cx) {
        //     cx.emit(Event::Focus);
        // }
        println!("Extensions page focused");
        self.get_extensions_from_server();
    }

    fn get_extensions_from_server(&mut self) {
        let extensions = vec![
            Extension {
                name: "Vim Mode".to_string(),
                version: "1.0.0".to_string(),
                author: "Zed".to_string(),
                description: "Zed extension for lang rust".to_string(),
                repository: "https://github.com/zed-industries/zed-extensions".to_string(),
                download_url: "https://download-url".to_string(),
                installed: false,
            },
            Extension {
                name: "Zed Mode".to_string(),
                version: "1.0.0".to_string(),
                author: "Zed".to_string(),
                description: "Zed extension for lang rust".to_string(),
                repository: "https://github.com/zed-industries/zed-extensions".to_string(),
                download_url: "https://download-url".to_string(),
                installed: false,
            },
            Extension {
                name: "Lang Mode".to_string(),
                version: "1.0.0".to_string(),
                author: "Zed".to_string(),
                description: "Zed extension for lang rust".to_string(),
                repository: "https://github.com/zed-industries/zed-extensions".to_string(),
                download_url: "https://download-url".to_string(),
                installed: false,
            },
        ];

        self.extensions_entries = extensions;
    }
    fn render_extensions(&self, cx: &mut ViewContext<Self>) -> Div {
        let mut entries = div().h_full().w_full().child(Label::new("Extensions"));
        println!(
            "extensions len: {:?}",
            self.extensions_entries.len().to_string()
        );
        for extension in &self.extensions_entries {
            println!("extension: {:?}", extension.name.to_string());
            entries = entries.child(self.render_entry(extension, cx));
        }
        entries
    }
    fn render_entry(&self, entry: &Extension, cx: &mut ViewContext<Self>) -> Div {
        // let installed = entry.installed;
        let name = &entry.name;
        // let version = entry.version;
        // let author = entry.author;
        // let description = entry.description;
        // let repository = entry.repository;
        // let download_url = entry.download_url;

        // let mut button = Button::new("install", "Install")
        //     .on_click(cx.listener(move |this, _, cx| {
        //         this.telemetry
        //             .report_app_event("welcome page: install extension".to_string());
        //         // this.install_extension(&name, &download_url, cx);
        //     }))
        //     .disabled(installed);

        // if installed {
        //     button = button.disabled(true).child(Label::new("Installed"));
        // }

        div().h_full().w_full().child(
            v_flex()
                .p_3()
                .gap_2()
                .bg(cx.theme().colors().elevated_surface_background)
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_md()
                .child(Label::new(name.to_string())),
        )
    }
    fn update_settings<T: Settings>(
        &mut self,
        selection: &Selection,
        cx: &mut ViewContext<Self>,
        callback: impl 'static + Send + Fn(&mut T::FileContent, bool),
    ) {
        if let Some(workspace) = self.workspace.upgrade() {
            let fs = workspace.read(cx).app_state().fs.clone();
            let selection = *selection;
            settings::update_settings_file::<T>(fs, cx, move |settings| {
                let value = match selection {
                    Selection::Unselected => false,
                    Selection::Selected => true,
                    _ => return,
                };

                callback(settings, value)
            });
        }
    }
}

impl EventEmitter<ItemEvent> for ExtensionsPage {}

impl FocusableView for ExtensionsPage {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ExtensionsPage {
    type Event = ItemEvent;

    fn tab_content(&self, _: Option<usize>, selected: bool, _: &WindowContext) -> AnyElement {
        Label::new("Extensions")
            .color(if selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("extensions page")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        Some(cx.new_view(|cx| ExtensionsPage {
            focus_handle: cx.focus_handle(),
            workspace: self.workspace.clone(),
            telemetry: self.telemetry.clone(),
            extensions_entries: Default::default(),
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
