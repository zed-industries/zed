mod base_keymap_picker;
mod base_keymap_setting;

use client::TelemetrySettings;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    svg, AnyElement, AppContext, Div, EventEmitter, FocusHandle, Focusable, FocusableView,
    InteractiveElement, ParentElement, Render, Styled, Subscription, View, ViewContext,
    VisualContext, WeakView, WindowContext,
};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use ui::{prelude::*, Checkbox};
use workspace::{
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new, AppState, Welcome, Workspace, WorkspaceId,
};

pub use base_keymap_setting::BaseKeymap;

pub const FIRST_OPEN: &str = "first_open";

pub fn init(cx: &mut AppContext) {
    BaseKeymap::register(cx);

    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &Welcome, cx| {
            let welcome_page = cx.build_view(|cx| WelcomePage::new(workspace, cx));
            workspace.add_item(Box::new(welcome_page), cx)
        });
    })
    .detach();

    base_keymap_picker::init(cx);
}

pub fn show_welcome_view(app_state: &Arc<AppState>, cx: &mut AppContext) {
    open_new(&app_state, cx, |workspace, cx| {
        workspace.toggle_dock(DockPosition::Left, cx);
        let welcome_page = cx.build_view(|cx| WelcomePage::new(workspace, cx));
        workspace.add_item_to_center(Box::new(welcome_page.clone()), cx);
        cx.focus_view(&welcome_page);
        cx.notify();
    })
    .detach();

    db::write_and_log(cx, || {
        KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
    });
}

pub struct WelcomePage {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    _settings_subscription: Subscription,
}

impl Render for WelcomePage {
    type Element = Focusable<Div>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        h_stack().full().track_focus(&self.focus_handle).child(
            v_stack()
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
                .child(Label::new("Code at the speed of thought"))
                .child(
                    v_stack()
                        .gap_2()
                        .child(
                            Button::new("choose-theme", "Choose a theme")
                                .full_width()
                                .on_click(cx.listener(|this, _, cx| {
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
                                .on_click(cx.listener(|_, _, cx| {
                                    cx.app_mut()
                                        .spawn(
                                            |cx| async move { install_cli::install_cli(&cx).await },
                                        )
                                        .detach_and_log_err(cx);
                                })),
                        ),
                )
                .child(
                    v_stack()
                        .p_3()
                        .gap_2()
                        .bg(cx.theme().colors().elevated_surface_background)
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .rounded_md()
                        .child(
                            // todo!("vim setting")
                            h_stack()
                                .gap_2()
                                .child(
                                    Checkbox::new(
                                        "enable-vim",
                                        if false
                                        /* VimSettings::get_global(cx).enabled */
                                        {
                                            ui::Selection::Selected
                                        } else {
                                            ui::Selection::Unselected
                                        },
                                    ),
                                    // .on_click(cx.listener(
                                    //     move |this, selection, cx| {
                                    //         this.update_settings::<VimSettings>(
                                    //             selection,
                                    //             cx,
                                    //             |settings, value| settings.enabled = value,
                                    //         );
                                    //     },
                                    // )),
                                )
                                .child(Label::new("Enable vim mode")),
                        )
                        .child(
                            h_stack()
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
                                    .on_click(cx.listener(
                                        move |this, selection, cx| {
                                            this.update_settings::<TelemetrySettings>(
                                                selection,
                                                cx,
                                                |settings, value| settings.metrics = Some(value),
                                            );
                                        },
                                    )),
                                )
                                .child(Label::new("Send anonymous usage data")),
                        )
                        .child(
                            h_stack()
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
                                    .on_click(cx.listener(
                                        move |this, selection, cx| {
                                            this.update_settings::<TelemetrySettings>(
                                                selection,
                                                cx,
                                                |settings, value| {
                                                    settings.diagnostics = Some(value)
                                                },
                                            );
                                        },
                                    )),
                                )
                                .child(Label::new("Send crash reports")),
                        ),
                ),
        )
    }
}

impl WelcomePage {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        WelcomePage {
            focus_handle: cx.focus_handle(),
            workspace: workspace.weak_handle(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        }
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

impl EventEmitter<ItemEvent> for WelcomePage {}

impl FocusableView for WelcomePage {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for WelcomePage {
    type Event = ItemEvent;

    fn tab_content(&self, _: Option<usize>, selected: bool, _: &WindowContext) -> AnyElement {
        Label::new("Welcome to Zed!")
            .color(if selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        Some(cx.build_view(|cx| WelcomePage {
            focus_handle: cx.focus_handle(),
            workspace: self.workspace.clone(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
