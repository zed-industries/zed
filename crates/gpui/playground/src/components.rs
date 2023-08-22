use crate::{
    div::div,
    element::{Element, ParentElement},
    style::StyleHelpers,
    text::ArcCow,
    themes::rose_pine,
};
use gpui::ViewContext;
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

use crate as playground;
#[derive(Element)]
pub struct Button<V: 'static, D: 'static> {
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
            handlers: ButtonHandlers::default(),
            label: None,
            icon: None,
            data: Rc::new(()),
            view_type: PhantomData,
        }
    }

    pub fn data<D: 'static>(self, data: D) -> Button<V, D> {
        Button {
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

    // pub fn click(self, handler: impl Fn(&mut V, &D, &mut ViewContext<V>) + 'static) -> Self {
    //     let data = self.data.clone();
    //     Self::click(self, MouseButton::Left, move |view, _, cx| {
    //         handler(view, data.as_ref(), cx);
    //     })
    // }
}

pub fn button<V>() -> Button<V, ()> {
    Button::new()
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        // TODO: Drive theme from the context
        let button = div()
            .fill(rose_pine::dawn().error(0.5))
            .h_4()
            .children(self.label.clone());

        button

        // TODO: Event handling
        // if let Some(handler) = self.handlers.click.clone() {
        //     let data = self.data.clone();
        //     // button.mouse_down(MouseButton::Left, move |view, event, cx| {
        //     //     handler(view, data.as_ref(), cx)
        //     // })
        // } else {
        //     button
        // }
    }
}
