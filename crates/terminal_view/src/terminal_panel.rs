use crate::TerminalView;
use gpui::{
    elements::*, AppContext, Entity, ModelHandle, Subscription, View, ViewContext, ViewHandle,
    WeakViewHandle, WindowContext,
};
use project::Project;
use settings::{settings_file::SettingsFile, Settings, TerminalDockPosition, WorkingDirectory};
use util::ResultExt;
use workspace::{
    dock::{DockPosition, Panel},
    pane, DraggedItem, Pane, Workspace,
};

pub fn init(cx: &mut AppContext) {
    cx.add_action(TerminalPanel::add_terminal);
}

pub enum Event {
    Close,
    DockPositionChanged,
    ZoomIn,
    ZoomOut,
}

pub struct TerminalPanel {
    project: ModelHandle<Project>,
    pane: ViewHandle<Pane>,
    workspace: WeakViewHandle<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl TerminalPanel {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let mut old_dock_position = cx.global::<Settings>().terminal_overrides.dock;
        cx.observe_global::<Settings, _>(move |_, cx| {
            let new_dock_position = cx.global::<Settings>().terminal_overrides.dock;
            if new_dock_position != old_dock_position {
                old_dock_position = new_dock_position;
                cx.emit(Event::DockPositionChanged);
            }
        })
        .detach();

        let this = cx.weak_handle();
        let pane = cx.add_view(|cx| {
            let window_id = cx.window_id();
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.app_state().background_actions,
                cx,
            );
            pane.set_can_split(false, cx);
            pane.on_can_drop(move |drag_and_drop, cx| {
                drag_and_drop
                    .currently_dragged::<DraggedItem>(window_id)
                    .map_or(false, |(_, item)| {
                        item.handle.act_as::<TerminalView>(cx).is_some()
                    })
            });
            pane.set_render_tab_bar_buttons(cx, move |_, cx| {
                let this = this.clone();
                Pane::render_tab_bar_button(
                    0,
                    "icons/plus_12.svg",
                    cx,
                    move |_, cx| {
                        let this = this.clone();
                        cx.window_context().defer(move |cx| {
                            if let Some(this) = this.upgrade(cx) {
                                this.update(cx, |this, cx| {
                                    this.add_terminal(&Default::default(), cx);
                                });
                            }
                        })
                    },
                    None,
                )
            });
            pane
        });
        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
        ];
        Self {
            project: workspace.project().clone(),
            pane,
            workspace: workspace.weak_handle(),
            _subscriptions: subscriptions,
        }
    }

    fn handle_pane_event(
        &mut self,
        _pane: ViewHandle<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::Remove => cx.emit(Event::Close),
            pane::Event::ZoomIn => cx.emit(Event::ZoomIn),
            pane::Event::ZoomOut => cx.emit(Event::ZoomOut),
            _ => {}
        }
    }

    fn add_terminal(&mut self, _: &workspace::NewTerminal, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            let working_directory_strategy = cx
                .global::<Settings>()
                .terminal_overrides
                .working_directory
                .clone()
                .unwrap_or(WorkingDirectory::CurrentProjectDirectory);
            let working_directory =
                crate::get_working_directory(workspace.read(cx), cx, working_directory_strategy);
            let window_id = cx.window_id();
            if let Some(terminal) = self.project.update(cx, |project, cx| {
                project
                    .create_terminal(working_directory, window_id, cx)
                    .log_err()
            }) {
                workspace.update(cx, |workspace, cx| {
                    let terminal =
                        Box::new(cx.add_view(|cx| {
                            TerminalView::new(terminal, workspace.database_id(), cx)
                        }));
                    Pane::add_item(workspace, &self.pane, terminal, true, true, None, cx);
                });
            }
        }
    }
}

impl Entity for TerminalPanel {
    type Event = Event;
}

impl View for TerminalPanel {
    fn ui_name() -> &'static str {
        "TerminalPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
        ChildView::new(&self.pane, cx).into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if self.pane.read(cx).items_len() == 0 {
            self.add_terminal(&Default::default(), cx)
        }
    }
}

impl Panel for TerminalPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        let settings = cx.global::<Settings>();
        let dock = settings
            .terminal_overrides
            .dock
            .or(settings.terminal_defaults.dock)
            .unwrap()
            .into();
        match dock {
            settings::TerminalDockPosition::Left => DockPosition::Left,
            settings::TerminalDockPosition::Bottom => DockPosition::Bottom,
            settings::TerminalDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        SettingsFile::update(cx, move |settings| {
            let dock = match position {
                DockPosition::Left => TerminalDockPosition::Left,
                DockPosition::Bottom => TerminalDockPosition::Bottom,
                DockPosition::Right => TerminalDockPosition::Right,
            };
            settings.terminal.dock = Some(dock);
        });
    }

    fn default_size(&self, cx: &WindowContext) -> f32 {
        let settings = &cx.global::<Settings>().terminal_overrides;
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => settings.default_width.unwrap_or(640.),
            DockPosition::Bottom => settings.default_height.unwrap_or(320.),
        }
    }

    fn should_zoom_in_on_event(event: &Event) -> bool {
        matches!(event, Event::ZoomIn)
    }

    fn should_zoom_out_on_event(event: &Event) -> bool {
        matches!(event, Event::ZoomOut)
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn icon_path(&self) -> &'static str {
        "icons/terminal_12.svg"
    }

    fn icon_tooltip(&self) -> String {
        "Terminals".to_string()
    }

    fn icon_label(&self, cx: &AppContext) -> Option<String> {
        let count = self.pane.read(cx).items_len();
        if count == 0 {
            None
        } else {
            Some(count.to_string())
        }
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn should_activate_on_event(&self, _: &Self::Event, _: &AppContext) -> bool {
        false
    }

    fn should_close_on_event(&self, event: &Event, _: &AppContext) -> bool {
        matches!(event, Event::Close)
    }
}
