mod breadcrumb;
mod chat_panel;
mod collab_panel;
mod command_palette;
mod facepile;
mod follow_group;
mod icon_button;
mod list;
mod list_item;
mod list_section_header;
mod palette;
mod palette_item;
mod project_panel;
mod status_bar;
mod tab;
mod tab_bar;
mod title_bar;
mod toolbar;
mod traffic_lights;
mod workspace;

pub use breadcrumb::*;
pub use chat_panel::*;
pub use collab_panel::*;
pub use command_palette::*;
pub use facepile::*;
pub use follow_group::*;
pub use icon_button::*;
pub use list::*;
pub use list_item::*;
pub use list_section_header::*;
pub use palette::*;
pub use palette_item::*;
pub use project_panel::*;
pub use status_bar::*;
pub use tab::*;
pub use tab_bar::*;
pub use title_bar::*;
pub use toolbar::*;
pub use traffic_lights::*;
pub use workspace::*;

use std::marker::PhantomData;
use std::rc::Rc;

use gpui2::elements::div;
use gpui2::interactive::Interactive;
use gpui2::platform::MouseButton;
use gpui2::style::StyleHelpers;
use gpui2::{ArcCow, Element, EventContext, IntoElement, ParentElement, ViewContext};

struct ButtonHandlers<V, D> {
    click: Option<Rc<dyn Fn(&mut V, &D, &mut EventContext<V>)>>,
}

impl<V, D> Default for ButtonHandlers<V, D> {
    fn default() -> Self {
        Self { click: None }
    }
}

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

// Impl block for button regardless of its data type.
impl<V: 'static, D: 'static> Button<V, D> {
    pub fn label(mut self, label: impl Into<ArcCow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<ArcCow<'static, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&mut V, &D, &mut EventContext<V>) + 'static,
    ) -> Self {
        self.handlers.click = Some(Rc::new(handler));
        self
    }
}

pub fn button<V>() -> Button<V, ()> {
    Button::new()
}

impl<V: 'static, D: 'static> Button<V, D> {
    fn render(
        &mut self,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> impl IntoElement<V> + Interactive<V> {
        // let colors = &cx.theme::<Theme>().colors;

        let button = div()
            // .fill(colors.error(0.5))
            .h_4()
            .children(self.label.clone());

        if let Some(handler) = self.handlers.click.clone() {
            let data = self.data.clone();
            button.on_mouse_down(MouseButton::Left, move |view, event, cx| {
                handler(view, data.as_ref(), cx)
            })
        } else {
            button
        }
    }
}
