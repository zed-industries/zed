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
    button_handlers: ButtonHandlers<V, D>,
    label: Cow<'static, str>,
    data: Rc<D>,
    view_type: PhantomData<V>,
}

impl<V: 'static> Button<V, ()> {
    fn new(label: impl Into<Cow<'static, str>>) -> Self {
        Self {
            metadata: Default::default(),
            button_handlers: ButtonHandlers::default(),
            label: label.into(),
            data: Rc::new(()),
            view_type: PhantomData,
        }
    }

    pub fn data<D: 'static>(self, data: D) -> Button<V, D> {
        Button {
            metadata: Default::default(),
            button_handlers: ButtonHandlers::default(),
            label: self.label,
            data: Rc::new(data),
            view_type: PhantomData,
        }
    }
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn click(self, handler: impl Fn(&mut V, &D) + 'static) -> Self {
        let data = self.data.clone();
        Element::click(self, move |view, _| {
            handler(view, data.as_ref());
        })
    }
}

pub fn button<V>(label: impl Into<Cow<'static, str>>) -> Button<V, ()> {
    Button::new(label)
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        // TODO: Drive from the context
        let button = frame().fill(rose_pine::dawn().error(0.5)).h_5().w_9();

        if let Some(handler) = self.button_handlers.click.clone() {
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
