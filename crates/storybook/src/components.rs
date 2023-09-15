use gpui2::{
    elements::div, interactive::Interactive, platform::MouseButton, style::StyleHelpers, ArcCow,
    Element, EventContext, IntoElement, ParentElement, ViewContext,
};
use std::{marker::PhantomData, rc::Rc};

mod avatar;
mod icon_button;
mod tab;
mod text_button;
mod tool_divider;

pub use avatar::avatar;
pub use avatar::Avatar;
pub use icon_button::icon_button;
pub use tab::tab;
pub use tab::Tab;
pub use text_button::text_button;
pub use text_button::TextButton;
pub use tool_divider::tool_divider;
pub use tool_divider::ToolDivider;

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
