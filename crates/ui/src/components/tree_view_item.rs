use std::sync::Arc;

use gpui::{AnyElement, AnyView, ClickEvent, MouseButton, MouseDownEvent};

use crate::{Disclosure, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct TreeViewItem {
    id: ElementId,
    group_name: Option<SharedString>,
    label: SharedString,
    expanded: bool,
    selected: bool,
    disabled: bool,
    focused: bool,
    default_expanded: bool,
    root_item: bool,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_secondary_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    tab_index: Option<isize>,
    focus_handle: Option<gpui::FocusHandle>,
}

impl TreeViewItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            group_name: None,
            label: label.into(),
            expanded: false,
            selected: false,
            disabled: false,
            focused: false,
            default_expanded: false,
            root_item: false,
            tooltip: None,
            on_click: None,
            on_hover: None,
            on_toggle: None,
            on_secondary_mouse_down: None,
            tab_index: None,
            focus_handle: None,
        }
    }

    pub fn group_name(mut self, group_name: impl Into<SharedString>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Some(Box::new(handler));
        self
    }

    pub fn on_secondary_mouse_down(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_secondary_mouse_down = Some(Box::new(handler));
        self
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = Some(tab_index);
        self
    }

    pub fn expanded(mut self, toggle: bool) -> Self {
        self.expanded = toggle;
        self
    }

    pub fn default_expanded(mut self, default_expanded: bool) -> Self {
        self.default_expanded = default_expanded;
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(on_toggle));
        self
    }

    pub fn root_item(mut self, root_item: bool) -> Self {
        self.root_item = root_item;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn track_focus(mut self, focus_handle: &gpui::FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle.clone());
        self
    }
}

impl Disableable for TreeViewItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for TreeViewItem {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for TreeViewItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected_bg = cx.theme().colors().element_active.opacity(0.5);

        let transparent_border = cx.theme().colors().border.opacity(0.);
        let selected_border = cx.theme().colors().border.opacity(0.4);
        let focused_border = cx.theme().colors().border_focused;

        let item_size = rems_from_px(28.);
        let indentation_line = h_flex().size(item_size).flex_none().justify_center().child(
            div()
                .w_px()
                .h_full()
                .bg(cx.theme().colors().border.opacity(0.5)),
        );

        h_flex()
            .id(self.id)
            .when_some(self.group_name, |this, group| this.group(group))
            .w_full()
            .child(
                h_flex()
                    .id("inner_tree_view_item")
                    .cursor_pointer()
                    .size_full()
                    .h(item_size)
                    .rounded_sm()
                    .border_1()
                    .border_color(transparent_border)
                    .focus(|s| s.border_color(focused_border))
                    .when(self.selected, |this| {
                        this.border_color(selected_border).bg(selected_bg)
                    })
                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                    .map(|this| {
                        let label = self.label;

                        if self.root_item {
                            this.px_1()
                                .gap_2p5()
                                .child(
                                    Disclosure::new("toggle", self.expanded)
                                        .when_some(
                                            self.on_toggle.clone(),
                                            |disclosure, on_toggle| {
                                                disclosure.on_toggle_expanded(on_toggle)
                                            },
                                        )
                                        .opened_icon(IconName::ChevronDown)
                                        .closed_icon(IconName::ChevronRight),
                                )
                                .child(
                                    Label::new(label)
                                        .when(!self.selected, |this| this.color(Color::Muted)),
                                )
                        } else {
                            this.child(indentation_line).child(
                                h_flex()
                                    .id("nested_inner_tree_view_item")
                                    .w_full()
                                    .flex_grow()
                                    .px_1()
                                    .child(
                                        Label::new(label)
                                            .when(!self.selected, |this| this.color(Color::Muted)),
                                    ),
                            )
                        }
                    })
                    .when_some(self.focus_handle, |this, handle| this.track_focus(&handle))
                    .when_some(self.tab_index, |this, index| this.tab_index(index))
                    .when_some(self.on_hover, |this, on_hover| this.on_hover(on_hover))
                    .when_some(
                        self.on_click.filter(|_| !self.disabled),
                        |this, on_click| this.on_click(on_click),
                    )
                    .when_some(self.on_secondary_mouse_down, |this, on_mouse_down| {
                        this.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                            (on_mouse_down)(event, window, cx)
                        })
                    })
                    .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip)),
            )
    }
}

impl Component for TreeViewItem {
    fn scope() -> ComponentScope {
        ComponentScope::Navigation
    }

    fn description() -> Option<&'static str> {
        Some(
            "A hierarchical list of items that may have a parent-child relationship where children can be toggled into view by expanding or collapsing their parent item.",
        )
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || {
            v_flex()
                .p_2()
                .w_64()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().panel_background)
        };

        Some(
            example_group(vec![
                single_example(
                    "Basic Tree View",
                    container()
                        .child(
                            TreeViewItem::new("index-1", "Tree Item Root #1")
                                .root_item(true)
                                .toggle_state(true),
                        )
                        .child(TreeViewItem::new("index-2", "Tree Item #2"))
                        .child(TreeViewItem::new("index-3", "Tree Item #3"))
                        .child(TreeViewItem::new("index-4", "Tree Item Root #2").root_item(true))
                        .child(TreeViewItem::new("index-5", "Tree Item #5"))
                        .child(TreeViewItem::new("index-6", "Tree Item #6"))
                        .into_any_element(),
                ),
                single_example(
                    "Active Child",
                    container()
                        .child(TreeViewItem::new("index-1", "Tree Item Root #1").root_item(true))
                        .child(TreeViewItem::new("index-2", "Tree Item #2").toggle_state(true))
                        .child(TreeViewItem::new("index-3", "Tree Item #3"))
                        .into_any_element(),
                ),
                single_example(
                    "Focused Parent",
                    container()
                        .child(
                            TreeViewItem::new("index-1", "Tree Item Root #1")
                                .root_item(true)
                                .focused(true)
                                .toggle_state(true),
                        )
                        .child(TreeViewItem::new("index-2", "Tree Item #2"))
                        .child(TreeViewItem::new("index-3", "Tree Item #3"))
                        .into_any_element(),
                ),
                single_example(
                    "Focused Child",
                    container()
                        .child(
                            TreeViewItem::new("index-1", "Tree Item Root #1")
                                .root_item(true)
                                .toggle_state(true),
                        )
                        .child(TreeViewItem::new("index-2", "Tree Item #2").focused(true))
                        .child(TreeViewItem::new("index-3", "Tree Item #3"))
                        .into_any_element(),
                ),
            ])
            .into_any_element(),
        )
    }
}
