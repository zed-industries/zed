use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;

use crate::{
    h_flex, Clickable, IconButton, IconButtonShape, IconName, Label, LabelCommon, LabelSize,
    Spacing,
};

#[derive(IntoElement)]
pub struct ModalHeader {
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    show_dismiss_button: bool,
    show_back_button: bool,
}

impl ModalHeader {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: SmallVec::new(),
            show_dismiss_button: false,
            show_back_button: false,
        }
    }

    pub fn show_dismiss_button(mut self, show: bool) -> Self {
        self.show_dismiss_button = show;
        self
    }

    pub fn show_back_button(mut self, show: bool) -> Self {
        self.show_back_button = show;
        self
    }
}

impl ParentElement for ModalHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ModalHeader {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .w_full()
            .px(Spacing::Large.rems(cx))
            .py_1p5()
            .when(self.show_back_button, |this| {
                this.child(
                    div().pr_1().child(
                        IconButton::new("back", IconName::ArrowLeft)
                            .shape(IconButtonShape::Square)
                            .on_click(|_, cx| {
                                cx.dispatch_action(menu::Cancel.boxed_clone());
                            }),
                    ),
                )
            })
            .child(div().flex_1().children(self.children))
            .justify_between()
            .when(self.show_dismiss_button, |this| {
                this.child(
                    IconButton::new("dismiss", IconName::Close)
                        .shape(IconButtonShape::Square)
                        .on_click(|_, cx| {
                            cx.dispatch_action(menu::Cancel.boxed_clone());
                        }),
                )
            })
    }
}

#[derive(IntoElement)]
pub struct ModalContent {
    children: SmallVec<[AnyElement; 2]>,
}

impl ModalContent {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for ModalContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ModalContent {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex().w_full().px_2().py_1p5().children(self.children)
    }
}

#[derive(IntoElement)]
pub struct ModalRow {
    children: SmallVec<[AnyElement; 2]>,
}

impl ModalRow {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for ModalRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ModalRow {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex().w_full().px_2().py_1().children(self.children)
    }
}

#[derive(IntoElement)]
pub struct SectionHeader {
    /// The label of the header.
    label: SharedString,
    /// A slot for content that appears after the label, usually on the other side of the header.
    /// This might be a button, a disclosure arrow, a face pile, etc.
    end_slot: Option<AnyElement>,
}

impl SectionHeader {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            end_slot: None,
        }
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }
}

impl RenderOnce for SectionHeader {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        h_flex().id(self.label.clone()).w_full().child(
            div()
                .h_7()
                .flex()
                .items_center()
                .justify_between()
                .w_full()
                .gap_1()
                .child(
                    div().flex_1().child(
                        Label::new(self.label.clone())
                            .size(LabelSize::Large)
                            .into_element(),
                    ),
                )
                .child(h_flex().children(self.end_slot)),
        )
    }
}
