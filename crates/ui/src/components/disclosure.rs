use std::sync::Arc;

use gpui::{AnyElement, ClickEvent};
use smallvec::SmallVec;

use crate::{prelude::*, Color, IconButton, IconName, IconSize};

#[derive(IntoElement)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            on_toggle: None,
        }
    }

    pub fn on_toggle(
        mut self,
        handler: impl Into<Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>>,
    ) -> Self {
        self.on_toggle = handler.into();
        self
    }
}

impl RenderOnce for Disclosure {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        IconButton::new(
            self.id,
            match self.is_open {
                true => IconName::ChevronDown,
                false => IconName::ChevronRight,
            },
        )
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(move |event, cx| on_toggle(event, cx))
        })
    }
}

#[derive(IntoElement)]
pub struct DisclosableContainerHeader {
    /// The label of the header.
    label: SharedString,
    /// A slot for content that appears before the label, like an icon or avatar.
    start_slot: Option<AnyElement>,
    /// A slot for content that appears after the label, usually on the other side of the header.
    /// This might be a button, a disclosure arrow, a face pile, etc.
    end_slot: Option<AnyElement>,
    /// A slot for content that appears in the `end_slot` on hover
    /// It will obscure the `end_slot` when visible.
    end_hover_slot: Option<AnyElement>,
    toggle: Option<bool>,
    on_toggle: Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
}

impl DisclosableContainerHeader {
    pub fn new(
        label: impl Into<
            SharedString,
            on_toggle: Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
        >,
    ) -> Self {
        Self {
            label: label.into(),
            start_slot: None,
            end_slot: None,
            end_hover_slot: None,
            toggle: None,
            on_toggle,
        }
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }

    pub fn on_toggle(
        mut self,
        on_toggle: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
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
}

impl RenderOnce for DisclosableContainerHeader {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .id(self.label.clone())
            .w_full()
            .relative()
            .group("disclosable_container_header")
            .child(
                div()
                    .h_7()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .children(self.toggle.map(|is_open| {
                                Disclosure::new("toggle", is_open).on_toggle(self.on_toggle.clone())
                            }))
                            .child(
                                div()
                                    .id("label_container")
                                    .flex()
                                    .gap_1()
                                    .items_center()
                                    .children(self.start_slot)
                                    .child(Label::new(self.label.clone()).color(Color::Muted))
                                    .when_some(self.on_toggle, |this, on_toggle| {
                                        this.on_click(move |event, cx| on_toggle(event, cx))
                                    }),
                            ),
                    )
                    .child(h_flex().children(self.end_slot))
                    .when_some(self.end_hover_slot, |this, end_hover_slot| {
                        this.child(
                            div()
                                .absolute()
                                .right_0()
                                .visible_on_hover("disclosable_container_header")
                                .child(end_hover_slot),
                        )
                    }),
            )
    }
}

#[derive(IntoElement)]
pub struct DisclosableContainer {
    id: ElementId,
    is_open: bool,
    on_toggle: Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    children: SmallVec<[AnyElement; 2]>,
}

impl DisclosableContainer {
    fn new(
        id: impl Into<ElementId>,
        is_open: bool,
        on_toggle: Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    ) -> Self {
        Self {
            id: id.into(),
            is_open,
            on_toggle,
            children: SmallVec::new(),
        }
    }

    /// Creates a new DisclosableContainer that is closed by default
    pub fn new_closed(
        id: impl Into<ElementId>,
        on_toggle: Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    ) -> Self {
        Self::new(id, false, on_toggle)
    }

    /// Creates a new DisclosableContainer that is open by default
    pub fn new_open(
        id: impl Into<ElementId>,
        on_toggle: Arc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
    ) -> Self {
        Self::new(id, true, on_toggle)
    }

    pub fn toggle(mut self, toggle: bool) -> Self {
        self.is_open = toggle;
        self
    }
}

impl ParentElement for DisclosableContainer {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for DisclosableContainer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .id(self.id)
            .child(
                DisclosableContainerHeader::new("header")
                    .toggle(self.is_open)
                    .on_toggle(self.on_toggle),
            )
            .when(self.is_open, |this| this.children(self.children))
    }
}

use gpui::Render;
use story::Story;

pub struct DisclosableContainerStory;

impl Render for DisclosableContainerStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        Story::container()
            .child(Story::title_for::<Disclosure>())
            .child(Story::label("Open by default"))
            .child(
                DisclosableContainer::new_open(
                    "default_open",
                    cx.listener(move |this, _, cx| {
                        this.is_open(!this.is_open);
                    }),
                )
                .child("This is some content"),
            )
    }
}
