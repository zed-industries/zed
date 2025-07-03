use crate::{
    Clickable, Color, DynamicSpacing, Headline, HeadlineSize, IconButton, IconButtonShape,
    IconName, Label, LabelCommon, LabelSize, h_flex, v_flex,
};
use gpui::{prelude::FluentBuilder, *};
use smallvec::SmallVec;
use theme::ActiveTheme;

#[derive(IntoElement)]
pub struct Modal {
    id: ElementId,
    header: ModalHeader,
    children: SmallVec<[AnyElement; 2]>,
    footer: Option<ModalFooter>,
    container_id: ElementId,
    container_scroll_handler: Option<ScrollHandle>,
}

impl Modal {
    pub fn new(id: impl Into<SharedString>, scroll_handle: Option<ScrollHandle>) -> Self {
        let id = id.into();

        let container_id = ElementId::Name(format!("{}_container", id).into());
        Self {
            id: ElementId::Name(id),
            header: ModalHeader::new(),
            children: SmallVec::new(),
            footer: None,
            container_id,
            container_scroll_handler: scroll_handle,
        }
    }

    pub fn header(mut self, header: ModalHeader) -> Self {
        self.header = header;
        self
    }

    pub fn section(mut self, section: Section) -> Self {
        self.children.push(section.into_any_element());
        self
    }

    pub fn footer(mut self, footer: ModalFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn show_dismiss(mut self, show: bool) -> Self {
        self.header.show_dismiss_button = show;
        self
    }

    pub fn show_back(mut self, show: bool) -> Self {
        self.header.show_back_button = show;
        self
    }
}

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Modal {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .id(self.id.clone())
            .size_full()
            .flex_1()
            .overflow_hidden()
            .child(self.header)
            .child(
                v_flex()
                    .id(self.container_id.clone())
                    .w_full()
                    .flex_1()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .when_some(
                        self.container_scroll_handler,
                        |this, container_scroll_handle| {
                            this.overflow_y_scroll()
                                .track_scroll(&container_scroll_handle)
                        },
                    )
                    .children(self.children),
            )
            .children(self.footer)
    }
}

#[derive(IntoElement)]
pub struct ModalHeader {
    headline: Option<SharedString>,
    children: SmallVec<[AnyElement; 2]>,
    show_dismiss_button: bool,
    show_back_button: bool,
}

impl Default for ModalHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalHeader {
    pub fn new() -> Self {
        Self {
            headline: None,
            children: SmallVec::new(),
            show_dismiss_button: false,
            show_back_button: false,
        }
    }

    /// Set the headline of the modal.
    ///
    /// This will insert the headline as the first item
    /// of `children` if it is not already present.
    pub fn headline(mut self, headline: impl Into<SharedString>) -> Self {
        self.headline = Some(headline.into());
        self
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut children = self.children;

        if self.headline.is_some() {
            children.insert(
                0,
                Headline::new(self.headline.unwrap())
                    .size(HeadlineSize::XSmall)
                    .color(Color::Muted)
                    .into_any_element(),
            );
        }

        h_flex()
            .flex_none()
            .justify_between()
            .w_full()
            .px(DynamicSpacing::Base12.rems(cx))
            .pt(DynamicSpacing::Base08.rems(cx))
            .pb(DynamicSpacing::Base04.rems(cx))
            .gap(DynamicSpacing::Base08.rems(cx))
            .when(self.show_back_button, |this| {
                this.child(
                    IconButton::new("back", IconName::ArrowLeft)
                        .shape(IconButtonShape::Square)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Cancel.boxed_clone(), cx);
                        }),
                )
            })
            .child(div().flex_1().children(children))
            .when(self.show_dismiss_button, |this| {
                this.child(
                    IconButton::new("dismiss", IconName::Close)
                        .shape(IconButtonShape::Square)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Cancel.boxed_clone(), cx);
                        }),
                )
            })
    }
}

#[derive(IntoElement)]
pub struct ModalRow {
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for ModalRow {
    fn default() -> Self {
        Self::new()
    }
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex().w_full().py_1().children(self.children)
    }
}

#[derive(IntoElement)]
pub struct ModalFooter {
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
}

impl Default for ModalFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalFooter {
    pub fn new() -> Self {
        Self {
            start_slot: None,
            end_slot: None,
        }
    }

    pub fn start_slot<E: IntoElement>(mut self, start_slot: impl Into<Option<E>>) -> Self {
        self.start_slot = start_slot.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, end_slot: impl Into<Option<E>>) -> Self {
        self.end_slot = end_slot.into().map(IntoElement::into_any_element);
        self
    }
}

impl RenderOnce for ModalFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .w_full()
            .mt_4()
            .p(DynamicSpacing::Base08.rems(cx))
            .flex_none()
            .justify_between()
            .gap_1()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .child(div().when_some(self.start_slot, |this, start_slot| this.child(start_slot)))
            .child(div().when_some(self.end_slot, |this, end_slot| this.child(end_slot)))
    }
}

#[derive(IntoElement)]
pub struct Section {
    contained: bool,
    padded: bool,
    header: Option<SectionHeader>,
    meta: Option<SharedString>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl Section {
    pub fn new() -> Self {
        Self {
            contained: false,
            padded: true,
            header: None,
            meta: None,
            children: SmallVec::new(),
        }
    }

    pub fn new_contained() -> Self {
        Self {
            contained: true,
            padded: true,
            header: None,
            meta: None,
            children: SmallVec::new(),
        }
    }

    pub fn contained(mut self, contained: bool) -> Self {
        self.contained = contained;
        self
    }

    pub fn header(mut self, header: SectionHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }
    pub fn padded(mut self, padded: bool) -> Self {
        self.padded = padded;
        self
    }
}

impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Section {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut section_bg = cx.theme().colors().text;
        section_bg.fade_out(0.96);

        let children = if self.contained {
            v_flex()
                .flex_1()
                .when(self.padded, |this| this.px(DynamicSpacing::Base12.rems(cx)))
                .child(
                    v_flex()
                        .w_full()
                        .rounded_sm()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .bg(section_bg)
                        .py(DynamicSpacing::Base06.rems(cx))
                        .gap_y(DynamicSpacing::Base04.rems(cx))
                        .child(div().flex().flex_1().size_full().children(self.children)),
                )
        } else {
            v_flex()
                .w_full()
                .flex_1()
                .gap_y(DynamicSpacing::Base04.rems(cx))
                .when(self.padded, |this| {
                    this.px(DynamicSpacing::Base06.rems(cx) + DynamicSpacing::Base06.rems(cx))
                })
                .children(self.children)
        };

        v_flex()
            .size_full()
            .flex_1()
            .child(
                v_flex()
                    .flex_none()
                    .px(DynamicSpacing::Base12.rems(cx))
                    .children(self.header)
                    .when_some(self.meta, |this, meta| {
                        this.child(Label::new(meta).size(LabelSize::Small).color(Color::Muted))
                    }),
            )
            .child(children)
            // fill any leftover space
            .child(div().flex().flex_1())
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.label.clone())
            .w_full()
            .px(DynamicSpacing::Base08.rems(cx))
            .child(
                div()
                    .h_7()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap(DynamicSpacing::Base04.rems(cx))
                    .child(
                        div().flex_1().child(
                            Label::new(self.label.clone())
                                .size(LabelSize::Small)
                                .into_element(),
                        ),
                    )
                    .child(h_flex().children(self.end_slot)),
            )
    }
}

impl From<SharedString> for SectionHeader {
    fn from(val: SharedString) -> Self {
        SectionHeader::new(val)
    }
}

impl From<&'static str> for SectionHeader {
    fn from(val: &'static str) -> Self {
        let label: SharedString = val.into();
        SectionHeader::new(label)
    }
}
