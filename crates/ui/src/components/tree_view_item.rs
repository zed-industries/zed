use std::sync::Arc;

use gpui::{AnyElement, AnyView, ClickEvent, MouseButton, MouseDownEvent};
use smallvec::SmallVec;

use crate::{Disclosure, prelude::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum TreeViewItemSpacing {
    #[default]
    Dense,
    Sparse,
}

#[derive(IntoElement, RegisterComponent)]
pub struct TreeViewItem {
    id: ElementId,
    group_name: Option<SharedString>,
    disabled: bool,
    selected: bool,
    spacing: TreeViewItemSpacing,
    label: SharedString,
    indent_level: usize,
    /// A slot for content that appears before the children, like an icon or avatar.
    start_slot: Option<AnyElement>,
    /// A slot for content that appears after the children, usually on the other side of the header.
    /// This might be a button, a disclosure arrow, a face pile, etc.
    end_slot: Option<AnyElement>,
    /// A slot for content that appears on hover after the children
    /// It will obscure the `end_slot` when visible.
    end_hover_slot: Option<AnyElement>,
    toggle: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_hover: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    on_secondary_mouse_down: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
    default_expanded: bool,
    root_item: bool,
    focused: Option<bool>,
}

impl TreeViewItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            group_name: None,
            disabled: false,
            selected: false,
            spacing: TreeViewItemSpacing::Dense,
            label: label.into(),
            indent_level: 0,
            start_slot: None,
            end_slot: None,
            end_hover_slot: None,
            toggle: false,
            on_click: None,
            on_secondary_mouse_down: None,
            on_toggle: None,
            on_hover: None,
            tooltip: None,
            children: SmallVec::new(),
            default_expanded: false,
            root_item: false,
            focused: None,
        }
    }

    pub fn group_name(mut self, group_name: impl Into<SharedString>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }

    pub fn spacing(mut self, spacing: TreeViewItemSpacing) -> Self {
        self.spacing = spacing;
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

    pub fn indent_level(mut self, indent_level: usize) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn toggle(mut self, toggle: bool) -> Self {
        self.toggle = toggle.into();
        self
    }

    pub fn default_expanded(mut self, default_expanded: bool) -> Self {
        self.default_expanded = default_expanded.into();
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Arc::new(on_toggle));
        self
    }

    pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self {
        self.start_slot = start_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_hover_slot<E: IntoElement>(mut self, end_hover_slot: impl Into<Option<E>>) -> Self {
        self.end_hover_slot = end_hover_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn root_item(mut self, root_item: bool) -> Self {
        self.root_item = root_item.into();
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = Some(focused);
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

impl ParentElement for TreeViewItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TreeViewItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let selected_bg = cx.theme().colors().element_active.opacity(0.2);
        let selected_border = cx.theme().colors().border.opacity(0.6);

        let indentation_line = h_flex().size_7().flex_none().justify_center().child(
            div()
                .w_px()
                .h_full()
                .bg(cx.theme().colors().border.opacity(0.5)),
        );

        h_flex()
            .id(self.id)
            .when_some(self.group_name, |this, group| this.group(group))
            .w_full()
            .rounded_md()
            .child(
                h_flex()
                    .id("inner_list_item")
                    .group("list_item")
                    .size_full()
                    .relative()
                    .map(|this| {
                        let label = self.label;
                        if self.root_item {
                            this.px_1()
                                .mb_1()
                                .gap_2()
                                .rounded_sm()
                                .border_1()
                                .map(|this| {
                                    if self.selected {
                                        this.border_color(selected_border).bg(selected_bg)
                                    } else {
                                        this.border_color(cx.theme().colors().border_transparent)
                                    }
                                })
                                .hover(|s| s.bg(cx.theme().colors().element_hover))
                                .child(
                                    Disclosure::new("toggle", true).when_some(
                                        self.on_toggle.clone(),
                                        |disclosure, on_toggle| disclosure.on_toggle(on_toggle),
                                    ),
                                )
                                .child(
                                    Label::new(label)
                                        .when(!self.selected, |this| this.color(Color::Muted)),
                                )
                        } else {
                            this.child(indentation_line).child(
                                h_flex()
                                    .w_full()
                                    .flex_grow()
                                    .px_0p5()
                                    .rounded_sm()
                                    .border_1()
                                    .map(|this| {
                                        if self.selected {
                                            this.border_color(selected_border).bg(selected_bg)
                                        } else {
                                            this.border_color(
                                                cx.theme().colors().border_transparent,
                                            )
                                        }
                                    })
                                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                                    .child(
                                        Label::new(label.clone())
                                            .when(!self.selected, |this| this.color(Color::Muted)),
                                    ),
                            )
                        }
                    })
                    .when_some(self.on_hover, |this, on_hover| this.on_hover(on_hover))
                    .when_some(
                        self.on_click.filter(|_| !self.disabled),
                        |this, on_click| this.cursor_pointer().on_click(on_click),
                    )
                    .when_some(self.on_secondary_mouse_down, |this, on_mouse_down| {
                        this.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                            (on_mouse_down)(event, window, cx)
                        })
                    })
                    .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip)), // .child(
                                                                                     //     h_flex()
                                                                                     //         .flex_grow()
                                                                                     //         .flex_shrink_0()
                                                                                     //         .flex_basis(relative(0.25))
                                                                                     //         .gap(DynamicSpacing::Base06.rems(cx))
                                                                                     //         .overflow_hidden()
                                                                                     //         .children(self.start_slot)
                                                                                     //         .children(self.children),
                                                                                     // ),
                                                                                     // .when_some(self.end_slot, |this, end_slot| {
                                                                                     //     this.justify_between().child(
                                                                                     //         h_flex()
                                                                                     //             .flex_shrink()
                                                                                     //             .overflow_hidden()
                                                                                     //             .when(self.end_hover_slot.is_some(), |this| {
                                                                                     //                 this.visible()
                                                                                     //                     .group_hover("list_item", |this| this.invisible())
                                                                                     //             })
                                                                                     //             .child(end_slot),
                                                                                     //     )
                                                                                     // })
                                                                                     // .when_some(self.end_hover_slot, |this, end_hover_slot| {
                                                                                     //     this.child(
                                                                                     //         h_flex()
                                                                                     //             .h_full()
                                                                                     //             .absolute()
                                                                                     //             .right(DynamicSpacing::Base06.rems(cx))
                                                                                     //             .top_0()
                                                                                     //             .visible_on_hover("list_item")
                                                                                     //             .child(end_hover_slot),
                                                                                     //     )
                                                                                     // }),
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
        Some(
            example_group(vec![single_example(
                "Basic Tree View",
                v_flex()
                    .p_2()
                    .w_64()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .bg(cx.theme().colors().panel_background)
                    .child(
                        TreeViewItem::new("index-1", "Tree Item Root #1")
                            .root_item(true)
                            .toggle_state(true),
                    )
                    .child(TreeViewItem::new("index-2", "Tree Item #2").indent_level(1))
                    .child(TreeViewItem::new("index-3", "Tree Item #3").indent_level(1))
                    .child(TreeViewItem::new("index-4", "Tree Item Root #2").root_item(true))
                    .child(TreeViewItem::new("index-5", "Tree Item #5").indent_level(1))
                    .child(TreeViewItem::new("index-6", "Tree Item #6").indent_level(1))
                    .child(TreeViewItem::new("index-7", "Tree Item #7").indent_level(1))
                    .child(TreeViewItem::new("index-8", "Tree Item #8").indent_level(1))
                    .child(TreeViewItem::new("index-9", "Tree Item Root #3").root_item(true))
                    .child(TreeViewItem::new("index-10", "Tree Item #10").indent_level(1))
                    .child(TreeViewItem::new("index-11", "Tree Item #11").indent_level(1))
                    .child(TreeViewItem::new("index-12", "Tree Item #12").indent_level(1))
                    .child(TreeViewItem::new("index-13", "Tree Item #13").indent_level(1))
                    .child(TreeViewItem::new("index-14", "Tree Item Root #4").root_item(true))
                    .child(TreeViewItem::new("index-15", "Tree Item #15").indent_level(1))
                    .child(TreeViewItem::new("index-16", "Tree Item #16").indent_level(1))
                    .into_any_element(),
            )])
            .into_any_element(),
        )
    }
}
