mod assistant_panel;
mod breadcrumb;
mod buffer;
mod chat_panel;
mod collab_panel;
mod command_palette;
mod context_menu;
mod editor_pane;
mod facepile;
mod icon_button;
mod keybinding;
mod language_selector;
mod list;
mod multi_buffer;
mod palette;
mod panel;
mod panes;
mod player_stack;
mod project_panel;
mod recent_projects;
mod status_bar;
mod tab;
mod tab_bar;
mod terminal;
mod theme_selector;
mod title_bar;
mod toast;
mod toolbar;
mod traffic_lights;
mod workspace;

pub use assistant_panel::*;
pub use breadcrumb::*;
pub use buffer::*;
pub use chat_panel::*;
pub use collab_panel::*;
pub use command_palette::*;
pub use context_menu::*;
pub use editor_pane::*;
pub use facepile::*;
pub use icon_button::*;
pub use keybinding::*;
pub use language_selector::*;
pub use list::*;
pub use multi_buffer::*;
pub use palette::*;
pub use panel::*;
pub use panes::*;
pub use player_stack::*;
pub use project_panel::*;
pub use recent_projects::*;
pub use status_bar::*;
pub use tab::*;
pub use tab_bar::*;
pub use terminal::*;
pub use theme_selector::*;
pub use title_bar::*;
pub use toast::*;
pub use toolbar::*;
pub use traffic_lights::*;
pub use workspace::*;

// Nate: Commenting this out for now, unsure if we need it.

// use std::marker::PhantomData;
// use std::rc::Rc;

// use gpui2::elements::div;
// use gpui2::interactive::Interactive;
// use gpui2::platform::MouseButton;
// use gpui2::{ArcCow, Element, EventContext, IntoElement, ParentElement, ViewContext};

// struct ButtonHandlers<V, D> {
//     click: Option<Rc<dyn Fn(&mut V, &D, &mut EventContext<V>)>>,
// }

// impl<V, D> Default for ButtonHandlers<V, D> {
//     fn default() -> Self {
//         Self { click: None }
//     }
// }

// #[derive(Element)]
// pub struct Button<V: 'static, D: 'static> {
//     handlers: ButtonHandlers<V, D>,
//     label: Option<ArcCow<'static, str>>,
//     icon: Option<ArcCow<'static, str>>,
//     data: Rc<D>,
//     view_type: PhantomData<V>,
// }

// // Impl block for buttons without data.
// // See below for an impl block for any button.
// impl<V: 'static> Button<V, ()> {
//     fn new() -> Self {
//         Self {
//             handlers: ButtonHandlers::default(),
//             label: None,
//             icon: None,
//             data: Rc::new(()),
//             view_type: PhantomData,
//         }
//     }

//     pub fn data<D: 'static>(self, data: D) -> Button<V, D> {
//         Button {
//             handlers: ButtonHandlers::default(),
//             label: self.label,
//             icon: self.icon,
//             data: Rc::new(data),
//             view_type: PhantomData,
//         }
//     }
// }

// // Impl block for button regardless of its data type.
// impl<V: 'static, D: 'static> Button<V, D> {
//     pub fn label(mut self, label: impl Into<ArcCow<'static, str>>) -> Self {
//         self.label = Some(label.into());
//         self
//     }

//     pub fn icon(mut self, icon: impl Into<ArcCow<'static, str>>) -> Self {
//         self.icon = Some(icon.into());
//         self
//     }

//     pub fn on_click(
//         mut self,
//         handler: impl Fn(&mut V, &D, &mut EventContext<V>) + 'static,
//     ) -> Self {
//         self.handlers.click = Some(Rc::new(handler));
//         self
//     }
// }

// pub fn button<V>() -> Button<V, ()> {
//     Button::new()
// }

// impl<V: 'static, D: 'static> Button<V, D> {
//     fn render(
//         &mut self,
//         view: &mut V,
//         cx: &mut ViewContext<V>,
//     ) -> impl IntoElement<V> + Interactive<V> {
//         // let colors = &cx.theme::<Theme>().colors;

//         let button = div()
//             // .fill(colors.error(0.5))
//             .h_4()
//             .children(self.label.clone());

//         if let Some(handler) = self.handlers.click.clone() {
//             let data = self.data.clone();
//             button.on_mouse_down(MouseButton::Left, move |view, event, cx| {
//                 handler(view, data.as_ref(), cx)
//             })
//         } else {
//             button
//         }
//     }
// }
