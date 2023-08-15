use crate::{element::Element, frame, themes::rose_pine};
use gpui::ViewContext;
use std::{any::Any, borrow::Cow, marker::PhantomData, rc::Rc};

pub struct Button<V: 'static, D: 'static> {
    label: Cow<'static, str>,
    data: Rc<D>,
    click_handler: Option<Rc<dyn Fn(&mut V, &dyn Any)>>,
    view_type: PhantomData<V>,
}

impl<V: 'static> Button<V, ()> {
    fn new(label: impl Into<Cow<'static, str>>) -> Self {
        Self {
            label: label.into(),
            data: Rc::new(()),
            click_handler: None,
            view_type: PhantomData,
        }
    }

    pub fn data<D: 'static>(self, data: D) -> Button<V, D> {
        Button {
            label: self.label,
            data: Rc::new(data),
            click_handler: None,
            view_type: PhantomData,
        }
    }
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn click(mut self, handler: impl Fn(&mut V, &D) + 'static) -> Self {
        self.click_handler = Some(Rc::new(move |view, data| {
            let data = data.downcast_ref::<D>().unwrap();
            handler(view, data);
        }));
        self
    }
}

pub fn button<V>(label: impl Into<Cow<'static, str>>) -> Button<V, ()> {
    Button::new(label)
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        // TODO: Drive from the context
        let button = frame().fill(rose_pine::dawn().error(0.5)).h_5().w_9();

        if let Some(handler) = self.click_handler.clone() {
            let data = self.data.clone();
            button.click(move |view, event| handler(view, data.as_ref()))
        } else {
            button
        }
    }
}
