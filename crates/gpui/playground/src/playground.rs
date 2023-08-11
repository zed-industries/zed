#![allow(dead_code, unused_variables)]

use gpui::{
    platform::{TitlebarOptions, WindowOptions},
    AnyElement, Element,
};
use log::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_| view(|_| Playground::new()),
        );
    });
}

use frame::{length::auto, *};
use gpui::{LayoutContext, ViewContext};
use std::{borrow::Cow, cell::RefCell, marker::PhantomData, rc::Rc};
use themes::{rose_pine, ThemeColors};
use tokens::{margin::m4, text::lg};

mod color;
mod frame;
mod themes;
mod tokens;

#[derive(Element, Clone)]
pub struct Playground<V: 'static>(PhantomData<V>);

impl<V> Playground<V> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> impl Element<V> {
        workspace(&rose_pine::dawn())
    }
}

fn workspace<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    column()
        .size(auto())
        .fill(theme.base(0.5))
        .text_color(theme.text(0.5))
        .child(title_bar(theme))
        .child(stage(theme))
        .child(status_bar(theme))
}

fn title_bar<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    row()
        .fill(theme.base(0.2))
        .justify(0.)
        .width(auto())
        .child(text("Zed Playground"))
}

fn stage<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    row().fill(theme.surface(0.9))
}

fn status_bar<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    row().fill(theme.surface(0.1))
}

pub trait DialogDelegate<V>: 'static {}

impl<V> DialogDelegate<V> for () {}

#[derive(Element)]
pub struct Dialog<V: 'static, D: DialogDelegate<V>> {
    title: Cow<'static, str>,
    description: Cow<'static, str>,
    delegate: Option<Rc<RefCell<D>>>,
    buttons: Vec<Box<dyn FnOnce() -> AnyElement<V>>>,
    view_type: PhantomData<V>,
}

pub fn dialog<V>(
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

impl<V, D: DialogDelegate<V>> Dialog<V, D> {
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
struct Button<V: 'static, D: 'static, H: ClickHandler<V, D>> {
    label: Cow<'static, str>,
    click_handler: Option<H>,
    data: Option<D>,
    view_type: PhantomData<V>,
}

pub trait ClickHandler<V, D>: 'static {
    fn handle(&self, view: &mut V, data: &D, cx: &mut ViewContext<V>);
}

impl<V, M, F: 'static + Fn(&mut V, &M, &mut ViewContext<V>)> ClickHandler<V, M> for F {
    fn handle(&self, view: &mut V, data: &M, cx: &mut ViewContext<V>) {
        self(view, data, cx)
    }
}

impl<V, D> ClickHandler<V, D> for () {
    fn handle(&self, view: &mut V, data: &D, cx: &mut ViewContext<V>) {}
}

fn button<V>(label: impl Into<Cow<'static, str>>) -> Button<V, (), ()> {
    Button {
        label: label.into(),
        click_handler: None,
        data: None,
        view_type: PhantomData,
    }
}

impl<V, D, F> Button<V, D, F>
where
    F: ClickHandler<V, D>,
{
    fn render(&mut self, _: &mut V, _: &mut LayoutContext<V>) -> AnyElement<V> {
        // TODO! Handle click etc
        row().child(text(self.label.clone())).into_any()
    }
}

// impl<V, D, F> Button<V, D, F>
// where
//     V,
//     F: ClickHandler<V, D>,
// {
//     fn render(&mut self, _: &mut V, _: &mut LayoutContext<V>) -> impl Element<V> {
//         // TODO! Handle click etc
//         row()
//             .fill(theme.colors.primary(5))
//             .child(text(self.label.clone()).text_color(theme.colors.on_primary()))
//     }
// }

// struct Tab<V> {
//     active: bool,
// }

// impl<V> Tab<V>
// where
//     V,
// {
//     fn tab(&mut self, _: &mut V, _: &mut LayoutContext<V>) -> impl Element<V> {
//         let theme = todo!();
//         // TODO! Handle click etc
//         row()
//             .fill(theme.colors.neutral(6))
//             .child(text(self.label.clone()).text_color(theme.colors.on_neutral()))
//     }
// }

impl<V> Button<V, (), ()> {
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

impl<V, D> Button<V, D, ()> {
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

impl<V, D: DialogDelegate<V>> Dialog<V, D> {
    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> AnyElement<V> {
        column()
            .child(text(self.title.clone()).text_size(lg()))
            .child(text(self.description.clone()).margins((m4(), auto())))
            .child(row().children(self.buttons.drain(..).map(|button| (button)())))
            .into_any()
    }
}
