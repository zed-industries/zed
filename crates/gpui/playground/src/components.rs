use crate::{
    element::{Element, ElementMetadata, ParentElement},
    frame,
    text::ArcCow,
    themes::rose_pine,
};
use gpui::{platform::MouseButton, ViewContext};
use playground_macros::Element;
use std::{marker::PhantomData, rc::Rc};

struct ButtonHandlers<V, D> {
    click: Option<Rc<dyn Fn(&mut V, &D, &mut ViewContext<V>)>>,
}

impl<V, D> Default for ButtonHandlers<V, D> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Element)]
#[element_crate = "crate"]
pub struct Button<V: 'static, D: 'static> {
    metadata: ElementMetadata<V>,
    handlers: ButtonHandlers<V, D>,
    label: Option<ArcCow<'static, str>>,
    icon: Option<ArcCow<'static, str>>,
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
    pub fn label(mut self, label: impl Into<ArcCow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<ArcCow<'static, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn click(self, handler: impl Fn(&mut V, &D, &mut ViewContext<V>) + 'static) -> Self {
        let data = self.data.clone();
        Element::click(self, MouseButton::Left, move |view, _, cx| {
            handler(view, data.as_ref(), cx);
        })
    }
}

pub fn button<V>() -> Button<V, ()> {
    Button::new()
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        // TODO: Drive theme from the context
        let button = frame()
            .fill(rose_pine::dawn().error(0.5))
            .h_4()
            .children(self.label.clone());

        if let Some(handler) = self.handlers.click.clone() {
            let data = self.data.clone();
            button.mouse_down(MouseButton::Left, move |view, event, cx| {
                handler(view, data.as_ref(), cx)
            })
        } else {
            button
        }
    }
}

// impl<V: 'static, D> Element<V> for Button<V, D> {
//     type Layout = AnyElement<V>;

//     fn style_mut(&mut self) -> &mut crate::style::ElementStyle {
//         &mut self.metadata.style
//     }

//     fn handlers_mut(&mut self) -> &mut crate::element::ElementHandlers<V> {
//         &mut self.metadata.handlers
//     }

//     fn layout(
//         &mut self,
//         view: &mut V,
//         cx: &mut crate::element::LayoutContext<V>,
//     ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
//         let mut element = self.render(view, cx).into_any();
//         let node_id = element.layout(view, cx)?;
//         Ok((node_id, element))
//     }

//     fn paint<'a>(
//         &mut self,
//         layout: crate::element::Layout<'a, Self::Layout>,
//         view: &mut V,
//         cx: &mut crate::element::PaintContext<V>,
//     ) -> anyhow::Result<()> {
//         layout.from_element.paint(view, cx)?;
//         Ok(())
//     }
// }
