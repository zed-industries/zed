use gpui::{
    elements::node::{column, length::auto, row, text},
    AnyElement, Element, LayoutContext, View, ViewContext,
};
use std::{borrow::Cow, marker::PhantomData};
use tokens::{margin::m4, text::lg};

mod tokens;

#[derive(Element, Clone, Default)]
pub struct Playground;

impl Playground {
    pub fn render<V: View>(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> AnyElement<V> {
        column()
            .width(auto())
            .child(
                dialog("This is a dialog", "You would see a description here.")
                    .button("Button 1", 1, Self::action_1)
                    .button("Button 2", 2, Self::action_2),
            )
            .into_any()
    }

    fn action_1(&mut self, data: &usize, _: &mut ViewContext<Self>) {
        println!("action 1: data is {}", *data);
    }

    fn action_2(&mut self, data: &usize, _: &mut ViewContext<Self>) {
        println!("action 1: data is {}", *data);
    }
}

pub trait DialogDelegate<V: View>: 'static {
    fn handle_confirm<B>(&mut self, view: &mut V, button: B);
}

impl<V: View> DialogDelegate<V> for () {
    fn handle_cancel<B>(&mut self, view: &mut V, button: B) {}
    fn handle_confirm<B>(&mut self, _: &mut V, _: B) {}
}

#[derive(Element)]
pub struct Dialog<V: View, D: DialogDelegate<V>> {
    title: Cow<'static, str>,
    description: Cow<'static, str>,
    delegate: Option<Rc<RefCell<D>>>,
    buttons: Vec<Box<dyn Fn() -> Button>>,
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
        let old_delegate = self.delegate.replace(delegate);
        debug_assert!(old_delegate.is_none(), "delegate already set");
        self
    }

    pub fn button<L, D, H>(mut self, label: L, data: D, handler: H) -> Self
    where
        L: Into<Cow<'static, str>>,
        D: 'static,
        H: ClickHandler<V, D>,
    {
        self.buttons
            .push(|| button(label).data(data).click(handler));
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
        todo!()
    }
}

impl<V: View> Button<V, (), ()> {
    fn data<D>(self, data: D) -> Button<V, D, ()>
    where
        D: 'static + FnOnce(&mut V, &D, &mut ViewContext<V>),
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
        H: 'static + Fn(&mut V, &D, &mut ViewContext<V>),
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
        let delegate = self.delegate.clone();

        column()
            .child(text(self.title.clone()).text_size(lg()))
            .child(text(self.description.clone()).margins(m4(), auto()))
            .child(row().children(self.buttons.iter().map(|button| (button)())))
            .into_any()
    }
}
