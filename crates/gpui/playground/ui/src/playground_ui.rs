use gpui::{color::Color, AnyElement, Element, LayoutContext, View, ViewContext};
use node::{
    length::{auto, rems},
    *,
};
use std::{borrow::Cow, cell::RefCell, marker::PhantomData, rc::Rc};
use tokens::{margin::m4, text::lg};

mod node;
mod tokens;

#[derive(Element, Clone, Default)]
pub struct Playground<V: View>(PhantomData<V>);

impl<V: View> Playground<V> {
    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> AnyElement<V> {
        column()
            .id("red column")
            .width(auto())
            .height(auto())
            .fill(Color::red())
            // .child(
            //     row()
            //         .id("green row")
            //         .width(auto())
            //         .height(rems(20.))
            //         .margin_left(auto())
            //         .fill(Color::green()), // .child(
            //                                //     row()
            //                                //         .id("blue child")
            //                                //         .height(auto())
            //                                //         .width(rems(20.))
            //                                //         .fill(Color::blue())
            //                                //         .margin_left(auto()),
            //                                // ),
            // )
            .into_any()

        // .child(
        //     dialog("This is a dialog", "You would see a description here.")
        //         .button("Button 1", 1, Self::action_1)
        //         .button("Button 2", 2, Self::action_2),
        // )
    }

    fn action_1(_: &mut V, data: &usize, _: &mut ViewContext<V>) {
        println!("action 1: data is {}", *data);
    }

    fn action_2(_: &mut V, data: &usize, _: &mut ViewContext<V>) {
        println!("action 1: data is {}", *data);
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
            .child(text(self.description.clone()).margins(m4(), auto()))
            .child(row().children(self.buttons.drain(..).map(|button| (button)())))
            .into_any()
    }
}
