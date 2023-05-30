use crate::{StatusItemView, Workspace};
use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    elements::*, platform::CursorStyle, platform::MouseButton, Action, AnyViewHandle, AppContext,
    Axis, Entity, Subscription, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use serde::Deserialize;
use std::rc::Rc;
use theme::ThemeSettings;

pub trait Panel: View {
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition) -> bool;
    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>);
    fn size(&self, cx: &WindowContext) -> f32;
    fn set_size(&mut self, size: f32, cx: &mut ViewContext<Self>);
    fn icon_path(&self) -> &'static str;
    fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>);
    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }
    fn should_change_position_on_event(_: &Self::Event) -> bool;
    fn should_zoom_in_on_event(_: &Self::Event) -> bool;
    fn should_zoom_out_on_event(_: &Self::Event) -> bool;
    fn is_zoomed(&self, cx: &WindowContext) -> bool;
    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>);
    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>);
    fn should_activate_on_event(_: &Self::Event) -> bool;
    fn should_close_on_event(_: &Self::Event) -> bool;
    fn has_focus(&self, cx: &WindowContext) -> bool;
    fn is_focus_event(_: &Self::Event) -> bool;
}

pub trait PanelHandle {
    fn id(&self) -> usize;
    fn position(&self, cx: &WindowContext) -> DockPosition;
    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool;
    fn set_position(&self, position: DockPosition, cx: &mut WindowContext);
    fn is_zoomed(&self, cx: &WindowContext) -> bool;
    fn set_zoomed(&self, zoomed: bool, cx: &mut WindowContext);
    fn set_active(&self, active: bool, cx: &mut WindowContext);
    fn size(&self, cx: &WindowContext) -> f32;
    fn set_size(&self, size: f32, cx: &mut WindowContext);
    fn icon_path(&self, cx: &WindowContext) -> &'static str;
    fn icon_tooltip(&self, cx: &WindowContext) -> (String, Option<Box<dyn Action>>);
    fn icon_label(&self, cx: &WindowContext) -> Option<String>;
    fn has_focus(&self, cx: &WindowContext) -> bool;
    fn as_any(&self) -> &AnyViewHandle;
}

impl<T> PanelHandle for ViewHandle<T>
where
    T: Panel,
{
    fn id(&self) -> usize {
        self.id()
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        self.read(cx).position(cx)
    }

    fn position_is_valid(&self, position: DockPosition, cx: &WindowContext) -> bool {
        self.read(cx).position_is_valid(position)
    }

    fn set_position(&self, position: DockPosition, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_position(position, cx))
    }

    fn size(&self, cx: &WindowContext) -> f32 {
        self.read(cx).size(cx)
    }

    fn set_size(&self, size: f32, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_size(size, cx))
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.read(cx).is_zoomed(cx)
    }

    fn set_zoomed(&self, zoomed: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_zoomed(zoomed, cx))
    }

    fn set_active(&self, active: bool, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.set_active(active, cx))
    }

    fn icon_path(&self, cx: &WindowContext) -> &'static str {
        self.read(cx).icon_path()
    }

    fn icon_tooltip(&self, cx: &WindowContext) -> (String, Option<Box<dyn Action>>) {
        self.read(cx).icon_tooltip()
    }

    fn icon_label(&self, cx: &WindowContext) -> Option<String> {
        self.read(cx).icon_label(cx)
    }

    fn has_focus(&self, cx: &WindowContext) -> bool {
        self.read(cx).has_focus(cx)
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }
}

impl From<&dyn PanelHandle> for AnyViewHandle {
    fn from(val: &dyn PanelHandle) -> Self {
        val.as_any().clone()
    }
}

pub struct Dock {
    position: DockPosition,
    panel_entries: Vec<PanelEntry>,
    is_open: bool,
    active_panel_index: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

impl DockPosition {
    fn to_label(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Bottom => "bottom",
            Self::Right => "right",
        }
    }

    fn to_resize_handle_side(self) -> HandleSide {
        match self {
            Self::Left => HandleSide::Right,
            Self::Bottom => HandleSide::Top,
            Self::Right => HandleSide::Left,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Horizontal,
            Self::Bottom => Axis::Vertical,
        }
    }
}

struct PanelEntry {
    panel: Rc<dyn PanelHandle>,
    context_menu: ViewHandle<ContextMenu>,
    _subscriptions: [Subscription; 2],
}

pub struct PanelButtons {
    dock: ViewHandle<Dock>,
    workspace: WeakViewHandle<Workspace>,
}

impl Dock {
    pub fn new(position: DockPosition) -> Self {
        Self {
            position,
            panel_entries: Default::default(),
            active_panel_index: 0,
            is_open: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn has_focus(&self, cx: &WindowContext) -> bool {
        self.visible_panel()
            .map_or(false, |panel| panel.has_focus(cx))
    }

    pub fn panel_index_for_type<T: Panel>(&self) -> Option<usize> {
        self.panel_entries
            .iter()
            .position(|entry| entry.panel.as_any().is::<T>())
    }

    pub fn panel_index_for_ui_name(&self, ui_name: &str, cx: &AppContext) -> Option<usize> {
        self.panel_entries.iter().position(|entry| {
            let panel = entry.panel.as_any();
            cx.view_ui_name(panel.window_id(), panel.id()) == Some(ui_name)
        })
    }

    pub fn active_panel_index(&self) -> usize {
        self.active_panel_index
    }

    pub(crate) fn set_open(&mut self, open: bool, cx: &mut ViewContext<Self>) {
        if open != self.is_open {
            self.is_open = open;
            if let Some(active_panel) = self.panel_entries.get(self.active_panel_index) {
                active_panel.panel.set_active(open, cx);
            }

            cx.notify();
        }
    }

    pub fn set_panel_zoomed(
        &mut self,
        panel: &AnyViewHandle,
        zoomed: bool,
        cx: &mut ViewContext<Self>,
    ) {
        for entry in &mut self.panel_entries {
            if entry.panel.as_any() == panel {
                if zoomed != entry.panel.is_zoomed(cx) {
                    entry.panel.set_zoomed(zoomed, cx);
                }
            } else if entry.panel.is_zoomed(cx) {
                entry.panel.set_zoomed(false, cx);
            }
        }

        cx.notify();
    }

    pub fn zoom_out(&mut self, cx: &mut ViewContext<Self>) {
        for entry in &mut self.panel_entries {
            if entry.panel.is_zoomed(cx) {
                entry.panel.set_zoomed(false, cx);
            }
        }
    }

    pub fn add_panel<T: Panel>(&mut self, panel: ViewHandle<T>, cx: &mut ViewContext<Self>) {
        let subscriptions = [
            cx.observe(&panel, |_, _, cx| cx.notify()),
            cx.subscribe(&panel, |this, panel, event, cx| {
                if T::should_activate_on_event(event) {
                    if let Some(ix) = this
                        .panel_entries
                        .iter()
                        .position(|entry| entry.panel.id() == panel.id())
                    {
                        this.set_open(true, cx);
                        this.activate_panel(ix, cx);
                        cx.focus(&panel);
                    }
                } else if T::should_close_on_event(event)
                    && this.visible_panel().map_or(false, |p| p.id() == panel.id())
                {
                    this.set_open(false, cx);
                }
            }),
        ];

        let dock_view_id = cx.view_id();
        self.panel_entries.push(PanelEntry {
            panel: Rc::new(panel),
            context_menu: cx.add_view(|cx| {
                let mut menu = ContextMenu::new(dock_view_id, cx);
                menu.set_position_mode(OverlayPositionMode::Local);
                menu
            }),
            _subscriptions: subscriptions,
        });
        cx.notify()
    }

    pub fn remove_panel<T: Panel>(&mut self, panel: &ViewHandle<T>, cx: &mut ViewContext<Self>) {
        if let Some(panel_ix) = self
            .panel_entries
            .iter()
            .position(|entry| entry.panel.id() == panel.id())
        {
            if panel_ix == self.active_panel_index {
                self.active_panel_index = 0;
                self.set_open(false, cx);
            } else if panel_ix < self.active_panel_index {
                self.active_panel_index -= 1;
            }
            self.panel_entries.remove(panel_ix);
            cx.notify();
        }
    }

    pub fn panels_len(&self) -> usize {
        self.panel_entries.len()
    }

    pub fn activate_panel(&mut self, panel_ix: usize, cx: &mut ViewContext<Self>) {
        if panel_ix != self.active_panel_index {
            if let Some(active_panel) = self.panel_entries.get(self.active_panel_index) {
                active_panel.panel.set_active(false, cx);
            }

            self.active_panel_index = panel_ix;
            if let Some(active_panel) = self.panel_entries.get(self.active_panel_index) {
                active_panel.panel.set_active(true, cx);
            }

            cx.notify();
        }
    }

    pub fn visible_panel(&self) -> Option<&Rc<dyn PanelHandle>> {
        let entry = self.visible_entry()?;
        Some(&entry.panel)
    }

    pub fn active_panel(&self) -> Option<&Rc<dyn PanelHandle>> {
        Some(&self.panel_entries.get(self.active_panel_index)?.panel)
    }

    fn visible_entry(&self) -> Option<&PanelEntry> {
        if self.is_open {
            self.panel_entries.get(self.active_panel_index)
        } else {
            None
        }
    }

    pub fn zoomed_panel(&self, cx: &WindowContext) -> Option<Rc<dyn PanelHandle>> {
        let entry = self.visible_entry()?;
        if entry.panel.is_zoomed(cx) {
            Some(entry.panel.clone())
        } else {
            None
        }
    }

    pub fn panel_size(&self, panel: &dyn PanelHandle, cx: &WindowContext) -> Option<f32> {
        self.panel_entries
            .iter()
            .find(|entry| entry.panel.id() == panel.id())
            .map(|entry| entry.panel.size(cx))
    }

    pub fn active_panel_size(&self, cx: &WindowContext) -> Option<f32> {
        if self.is_open {
            self.panel_entries
                .get(self.active_panel_index)
                .map(|entry| entry.panel.size(cx))
        } else {
            None
        }
    }

    pub fn resize_active_panel(&mut self, size: f32, cx: &mut ViewContext<Self>) {
        if let Some(entry) = self.panel_entries.get_mut(self.active_panel_index) {
            entry.panel.set_size(size, cx);
            cx.notify();
        }
    }

    pub fn render_placeholder(&self, cx: &WindowContext) -> AnyElement<Workspace> {
        if let Some(active_entry) = self.visible_entry() {
            Empty::new()
                .into_any()
                .contained()
                .with_style(self.style(cx))
                .resizable(
                    self.position.to_resize_handle_side(),
                    active_entry.panel.size(cx),
                    |_, _, _| {},
                )
                .into_any()
        } else {
            Empty::new().into_any()
        }
    }

    fn style(&self, cx: &WindowContext) -> ContainerStyle {
        let theme = &settings::get::<ThemeSettings>(cx).theme;
        let style = match self.position {
            DockPosition::Left => theme.workspace.dock.left,
            DockPosition::Bottom => theme.workspace.dock.bottom,
            DockPosition::Right => theme.workspace.dock.right,
        };
        style
    }
}

impl Entity for Dock {
    type Event = ();
}

impl View for Dock {
    fn ui_name() -> &'static str {
        "Dock"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(active_entry) = self.visible_entry() {
            let style = self.style(cx);
            ChildView::new(active_entry.panel.as_any(), cx)
                .contained()
                .with_style(style)
                .resizable(
                    self.position.to_resize_handle_side(),
                    active_entry.panel.size(cx),
                    |dock: &mut Self, size, cx| dock.resize_active_panel(size, cx),
                )
                .into_any()
        } else {
            Empty::new().into_any()
        }
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            if let Some(active_entry) = self.visible_entry() {
                cx.focus(active_entry.panel.as_any());
            } else {
                cx.focus_parent();
            }
        }
    }
}

impl PanelButtons {
    pub fn new(
        dock: ViewHandle<Dock>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&dock, |_, _, cx| cx.notify()).detach();
        Self { dock, workspace }
    }
}

impl Entity for PanelButtons {
    type Event = ();
}

impl View for PanelButtons {
    fn ui_name() -> &'static str {
        "PanelButtons"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &settings::get::<ThemeSettings>(cx).theme;
        let tooltip_style = theme.tooltip.clone();
        let theme = &theme.workspace.status_bar.panel_buttons;
        let button_style = theme.button.clone();
        let dock = self.dock.read(cx);
        let active_ix = dock.active_panel_index;
        let is_open = dock.is_open;
        let dock_position = dock.position;
        let group_style = match dock_position {
            DockPosition::Left => theme.group_left,
            DockPosition::Bottom => theme.group_bottom,
            DockPosition::Right => theme.group_right,
        };
        let menu_corner = match dock_position {
            DockPosition::Left => AnchorCorner::BottomLeft,
            DockPosition::Bottom | DockPosition::Right => AnchorCorner::BottomRight,
        };

        let panels = dock
            .panel_entries
            .iter()
            .map(|item| (item.panel.clone(), item.context_menu.clone()))
            .collect::<Vec<_>>();
        Flex::row()
            .with_children(panels.into_iter().enumerate().map(
                |(panel_ix, (view, context_menu))| {
                    let (tooltip, tooltip_action) = view.icon_tooltip(cx);
                    Stack::new()
                        .with_child(
                            MouseEventHandler::<Self, _>::new(panel_ix, cx, |state, cx| {
                                let is_active = is_open && panel_ix == active_ix;
                                let style = button_style.style_for(state, is_active);
                                Flex::row()
                                    .with_child(
                                        Svg::new(view.icon_path(cx))
                                            .with_color(style.icon_color)
                                            .constrained()
                                            .with_width(style.icon_size)
                                            .aligned(),
                                    )
                                    .with_children(if let Some(label) = view.icon_label(cx) {
                                        Some(
                                            Label::new(label, style.label.text.clone())
                                                .contained()
                                                .with_style(style.label.container)
                                                .aligned(),
                                        )
                                    } else {
                                        None
                                    })
                                    .constrained()
                                    .with_height(style.icon_size)
                                    .contained()
                                    .with_style(style.container)
                            })
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_click(MouseButton::Left, {
                                let tooltip_action =
                                    tooltip_action.as_ref().map(|action| action.boxed_clone());
                                move |_, this, cx| {
                                    if let Some(tooltip_action) = &tooltip_action {
                                        let window_id = cx.window_id();
                                        let view_id = this.workspace.id();
                                        let tooltip_action = tooltip_action.boxed_clone();
                                        cx.spawn(|_, mut cx| async move {
                                            cx.dispatch_action(
                                                window_id,
                                                view_id,
                                                &*tooltip_action,
                                            )
                                            .ok();
                                        })
                                        .detach();
                                    }
                                }
                            })
                            .on_click(MouseButton::Right, {
                                let view = view.clone();
                                let menu = context_menu.clone();
                                move |_, _, cx| {
                                    const POSITIONS: [DockPosition; 3] = [
                                        DockPosition::Left,
                                        DockPosition::Right,
                                        DockPosition::Bottom,
                                    ];

                                    menu.update(cx, |menu, cx| {
                                        let items = POSITIONS
                                            .into_iter()
                                            .filter(|position| {
                                                *position != dock_position
                                                    && view.position_is_valid(*position, cx)
                                            })
                                            .map(|position| {
                                                let view = view.clone();
                                                ContextMenuItem::handler(
                                                    format!("Dock {}", position.to_label()),
                                                    move |cx| view.set_position(position, cx),
                                                )
                                            })
                                            .collect();
                                        menu.show(Default::default(), menu_corner, items, cx);
                                    })
                                }
                            })
                            .with_tooltip::<Self>(
                                panel_ix,
                                tooltip,
                                tooltip_action,
                                tooltip_style.clone(),
                                cx,
                            ),
                        )
                        .with_child(ChildView::new(&context_menu, cx))
                },
            ))
            .contained()
            .with_style(group_style)
            .into_any()
    }
}

impl StatusItemView for PanelButtons {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn crate::ItemHandle>,
        _: &mut ViewContext<Self>,
    ) {
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use gpui::{ViewContext, WindowContext};

    pub enum TestPanelEvent {
        PositionChanged,
        Activated,
        Closed,
        ZoomIn,
        ZoomOut,
        Focus,
    }

    pub struct TestPanel {
        pub position: DockPosition,
        pub zoomed: bool,
        pub active: bool,
        pub has_focus: bool,
        pub size: f32,
    }

    impl TestPanel {
        pub fn new(position: DockPosition) -> Self {
            Self {
                position,
                zoomed: false,
                active: false,
                has_focus: false,
                size: 300.,
            }
        }
    }

    impl Entity for TestPanel {
        type Event = TestPanelEvent;
    }

    impl View for TestPanel {
        fn ui_name() -> &'static str {
            "TestPanel"
        }

        fn render(&mut self, _: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self> {
            Empty::new().into_any()
        }

        fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
            self.has_focus = true;
            cx.emit(TestPanelEvent::Focus);
        }

        fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
            self.has_focus = false;
        }
    }

    impl Panel for TestPanel {
        fn position(&self, _: &gpui::WindowContext) -> super::DockPosition {
            self.position
        }

        fn position_is_valid(&self, _: super::DockPosition) -> bool {
            true
        }

        fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
            self.position = position;
            cx.emit(TestPanelEvent::PositionChanged);
        }

        fn is_zoomed(&self, _: &WindowContext) -> bool {
            self.zoomed
        }

        fn set_zoomed(&mut self, zoomed: bool, _cx: &mut ViewContext<Self>) {
            self.zoomed = zoomed;
        }

        fn set_active(&mut self, active: bool, _cx: &mut ViewContext<Self>) {
            self.active = active;
        }

        fn size(&self, _: &WindowContext) -> f32 {
            self.size
        }

        fn set_size(&mut self, size: f32, _: &mut ViewContext<Self>) {
            self.size = size;
        }

        fn icon_path(&self) -> &'static str {
            "icons/test_panel.svg"
        }

        fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>) {
            ("Test Panel".into(), None)
        }

        fn should_change_position_on_event(event: &Self::Event) -> bool {
            matches!(event, TestPanelEvent::PositionChanged)
        }

        fn should_zoom_in_on_event(event: &Self::Event) -> bool {
            matches!(event, TestPanelEvent::ZoomIn)
        }

        fn should_zoom_out_on_event(event: &Self::Event) -> bool {
            matches!(event, TestPanelEvent::ZoomOut)
        }

        fn should_activate_on_event(event: &Self::Event) -> bool {
            matches!(event, TestPanelEvent::Activated)
        }

        fn should_close_on_event(event: &Self::Event) -> bool {
            matches!(event, TestPanelEvent::Closed)
        }

        fn has_focus(&self, _cx: &WindowContext) -> bool {
            self.has_focus
        }

        fn is_focus_event(event: &Self::Event) -> bool {
            matches!(event, TestPanelEvent::Focus)
        }
    }
}
