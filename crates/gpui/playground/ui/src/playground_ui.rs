use gpui::{AnyElement, Element, LayoutContext, View, ViewContext};
use node::{length::auto, *};
use std::{borrow::Cow, cell::RefCell, marker::PhantomData, rc::Rc};
use tokens::{margin::m4, text::lg};

mod color;
mod node;
mod themes;
mod tokens;

#[derive(Element, Clone, Default)]
pub struct Playground<V: View>(PhantomData<V>);

impl<V: View> Node<V> {}

impl<V: View> Playground<V> {
    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> impl Element<V> {
        column()
    }
}

pub trait DialogDelegate<V: View>: 'static {}

impl<V: View> DialogDelegate<V> for () {}

#[derive(Element)]
pub struct Dialog<V: View, D: DialogDelegate<V>> {
    title: Cow<'static, str>,
    description: Cow<'static, str>,
    delegate: Option<Rc<RefCell<D>>>,
    buttons: Vec<Box<dyn FnOnce() -> AnyElement<V>>>,
    view_type: PhantomData<V>,
}

pub fn dialog<V: View>(
    title: impl Into<Cow<'static, str>>,
    description: impl Into<Cow<'static, str>>,
) -> Dialog<V, ()> {
    Dialog {
        title: title.into(),
        description: description.into(),
        delegate: None,
        buttons: Vec::new(),
        view_type: PhantomData,
    }
}

impl<V: View, D: DialogDelegate<V>> Dialog<V, D> {
    pub fn delegate(mut self, delegate: D) -> Dialog<V, D> {
        let old_delegate = self.delegate.replace(Rc::new(RefCell::new(delegate)));
        debug_assert!(old_delegate.is_none(), "delegate already set");
        self
    }

    pub fn button<L, Data, H>(mut self, label: L, data: Data, handler: H) -> Self
    where
        L: 'static + Into<Cow<'static, str>>,
        Data: 'static + Clone,
        H: ClickHandler<V, Data>,
    {
        let label = label.into();
        self.buttons.push(Box::new(move || {
            button(label).data(data).click(handler).into_any()
        }));
        self
    }
}

#[derive(Element)]
struct Button<V: View, D: 'static, H: ClickHandler<V, D>> {
    label: Cow<'static, str>,
    click_handler: Option<H>,
    data: Option<D>,
    view_type: PhantomData<V>,
}

pub trait ClickHandler<V, D>: 'static {
    fn handle(&self, view: &mut V, data: &D, cx: &mut ViewContext<V>);
}

impl<V: View, M, F: 'static + Fn(&mut V, &M, &mut ViewContext<V>)> ClickHandler<V, M> for F {
    fn handle(&self, view: &mut V, data: &M, cx: &mut ViewContext<V>) {
        self(view, data, cx)
    }
}

impl<V, D> ClickHandler<V, D> for () {
    fn handle(&self, view: &mut V, data: &D, cx: &mut ViewContext<V>) {}
}

fn button<V>(label: impl Into<Cow<'static, str>>) -> Button<V, (), ()>
where
    V: View,
{
    Button {
        label: label.into(),
        click_handler: None,
        data: None,
        view_type: PhantomData,
    }
}

impl<V, D, F> Button<V, D, F>
where
    V: View,
    F: ClickHandler<V, D>,
{
    fn render(&mut self, _: &mut V, _: &mut LayoutContext<V>) -> AnyElement<V> {
        // TODO! Handle click etc
        row().child(text(self.label.clone())).into_any()
    }
}

impl<V: View> Button<V, (), ()> {
    fn data<D>(self, data: D) -> Button<V, D, ()>
    where
        D: 'static,
    {
        Button {
            label: self.label,
            click_handler: self.click_handler,
            data: Some(data),
            view_type: self.view_type,
        }
    }
}

impl<V: View, D> Button<V, D, ()> {
    fn click<H>(self, handler: H) -> Button<V, D, H>
    where
        H: 'static + ClickHandler<V, D>,
    {
        Button {
            label: self.label,
            click_handler: Some(handler),
            data: self.data,
            view_type: self.view_type,
        }
    }
}

impl<V: View, D: DialogDelegate<V>> Dialog<V, D> {
    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> AnyElement<V> {
        column()
            .child(text(self.title.clone()).text_size(lg()))
            .child(text(self.description.clone()).margins((m4(), auto())))
            .child(row().children(self.buttons.drain(..).map(|button| (button)())))
            .into_any()
    }
}
