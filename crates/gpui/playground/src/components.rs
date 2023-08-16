use crate::{
    element::{AnyElement, Element, ElementMetadata},
    frame,
    themes::rose_pine,
};
use gpui::ViewContext;
use std::{borrow::Cow, marker::PhantomData, rc::Rc};

struct ButtonHandlers<V, D> {
    click: Option<Rc<dyn Fn(&mut V, &D)>>,
}

impl<V, D> Default for ButtonHandlers<V, D> {
    fn default() -> Self {
        Self { click: None }
    }
}

pub struct Button<V: 'static, D: 'static> {
    metadata: ElementMetadata<V>,
    handlers: ButtonHandlers<V, D>,
    label: Option<Cow<'static, str>>,
    icon: Option<Cow<'static, str>>,
    data: Rc<D>,
    view_type: PhantomData<V>,
}

// Impl block for buttons without data.
// See below for an impl block for any button.
impl<V: 'static> Button<V, ()> {
    fn new() -> Self {
        Self {
            metadata: Default::default(),
            handlers: ButtonHandlers::default(),
            label: None,
            icon: None,
            data: Rc::new(()),
            view_type: PhantomData,
        }
    }

    pub fn data<D: 'static>(self, data: D) -> Button<V, D> {
        Button {
            metadata: Default::default(),
            handlers: ButtonHandlers::default(),
            label: self.label,
            icon: self.icon,
            data: Rc::new(data),
            view_type: PhantomData,
        }
    }
}

// Impl block for *any* button.
impl<V: 'static, D: 'static> Button<V, D> {
    fn label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn icon(mut self, icon: impl Into<Cow<'static, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    fn click(self, handler: impl Fn(&mut V, &D) + 'static) -> Self {
        let data = self.data.clone();
        Element::click(self, move |view, _| {
            handler(view, data.as_ref());
        })
    }
}

pub fn button<V>() -> Button<V, ()> {
    Button::new()
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        // TODO: Drive from the context
        let button = frame().fill(rose_pine::dawn().error(0.5)).h_5().w_9();

        if let Some(handler) = self.handlers.click.clone() {
            let data = self.data.clone();
            button.click(move |view, event| handler(view, data.as_ref()))
        } else {
            button
        }
    }
}

impl<V: 'static, D> Element<V> for Button<V, D> {
    type Layout = AnyElement<V>;

    fn style_mut(&mut self) -> &mut crate::style::ElementStyle {
        &mut self.metadata.style
    }

    fn handlers_mut(&mut self) -> &mut crate::element::ElementHandlers<V> {
        &mut self.metadata.handlers
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut crate::element::LayoutContext<V>,
    ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
        let mut element = self.render(view, cx).into_any();
        let node_id = element.layout(view, cx)?;
        Ok((node_id, element))
    }

    fn paint<'a>(
        &mut self,
        layout: crate::element::Layout<'a, Self::Layout>,
        view: &mut V,
        cx: &mut crate::element::PaintContext<V>,
    ) -> anyhow::Result<()> {
        layout.from_element.paint(view, cx)?;
        Ok(())
    }
}
