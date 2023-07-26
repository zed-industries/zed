use gpui::{
    elements::node::{column, length::auto, row, text},
    AnyElement, Element, View, ViewContext,
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
            .child(dialog(
                "This is a dialog",
                "You would see a description here.",
            ))
            .into_any()
    }
}

pub trait DialogDelegate<V: View>: 'static {
    fn handle_submit<B>(&mut self, view: &mut V, button: B);
}

impl<V: View> DialogDelegate<V> for () {
    fn handle_submit<B>(&mut self, _: &mut V, _: B) {}
}

#[derive(Element)]
pub struct Dialog<V: View, D: DialogDelegate<V>> {
    title: Cow<'static, str>,
    description: Cow<'static, str>,
    delegate: Option<D>,
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
        view_type: PhantomData,
    }
}

impl<V: View, D: DialogDelegate<V>> Dialog<V, D> {
    pub fn with_delegate(mut self, delegate: D) -> Dialog<V, D> {
        let old_delegate = self.delegate.replace(delegate);
        debug_assert!(old_delegate.is_none(), "delegate already set");
        self
    }
}

struct Button<V: View, F: FnOnce(&mut V, &mut ViewContext<V>)> {
    label: Cow<'static, str>,
    on_click: Option<F>,
    view_type: PhantomData<V>,
}

fn button<V: View, F: FnOnce(&mut V, &mut ViewContext<V>)>(
    label: impl Into<Cow<'static, str>>,
) -> Button<V, F> {
    Button {
        label: label.into(),
        on_click: None,
        view_type: PhantomData,
    }
}

impl<V: View, D: DialogDelegate<V>> Dialog<V, D> {
    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> AnyElement<V> {
        column()
            .child(text(self.title.clone()).text_size(lg()))
            .child(text(self.description.clone()).margins(m4(), auto()))
            .child(row().children([
                button("Cancel").margin_left(auto()),
                button("OK").margin_left(m4()),
            ]))
            .into_any()
    }
}
