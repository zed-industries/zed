use dap::{DebugAdapterConfig, DebugRequestType};
use gpui::{App, EventEmitter, FocusHandle, Focusable};
use ui::{
    div, h_flex, v_flex, Button, ButtonCommon, ButtonStyle, Clickable, Context, ContextMenu,
    DropdownMenu, Element, InteractiveElement, ParentElement, Render, SharedString, Styled, Window,
};

pub(super) struct InertState {
    focus_handle: FocusHandle,
    selected_debugger: Option<SharedString>,
}

impl InertState {
    pub(super) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            selected_debugger: None,
        }
    }
}
impl Focusable for InertState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub(crate) enum InertEvent {
    Spawned { config: DebugAdapterConfig },
}

impl EventEmitter<InertEvent> for InertState {}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");
impl Render for InertState {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<'_, Self>,
    ) -> impl ui::IntoElement {
        let weak = cx.weak_entity();
        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .gap_1()
            .p_1()
            .child(
                h_flex().child(DropdownMenu::new(
                    "dap-adapter-picker",
                    self.selected_debugger
                        .as_ref()
                        .unwrap_or_else(|| &SELECT_DEBUGGER_LABEL)
                        .clone(),
                    ContextMenu::build(window, cx, move |this, _, _| {
                        let setter_for_name = |name: &'static str| {
                            let weak = weak.clone();
                            move |_: &mut Window, cx: &mut App| {
                                let name = name;
                                (&weak)
                                    .update(cx, move |this, _| {
                                        this.selected_debugger = Some(name.into());
                                    })
                                    .ok();
                            }
                        };
                        this.entry("GDB", None, setter_for_name("GDB"))
                            .entry("Delve", None, setter_for_name("Delve"))
                            .entry("LLDB", None, setter_for_name("LLDB"))
                    }),
                )),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("launch-dap", "Launch")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(InertEvent::Spawned {
                                    config: DebugAdapterConfig {
                                        label: "hard coded".into(),
                                        kind: dap::DebugAdapterKind::Python(task::TCPHost {
                                            port: None,
                                            host: None,
                                            timeout: None,
                                        }),
                                        request: DebugRequestType::Launch,
                                        program: Some(
                                            "/Users/hiro/Projects/zed/test_debug_file.py".into(),
                                        ),
                                        cwd: None,
                                        initialize_args: None,
                                        supports_attach: false,
                                    },
                                });
                            })),
                    )
                    .child(Button::new("attach-dap", "Attach").style(ButtonStyle::Filled)),
            )
    }
}
