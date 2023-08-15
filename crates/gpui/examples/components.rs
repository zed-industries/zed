use button_component::Button;

use component::Component;
use gpui::{
    color::Color,
    elements::{ContainerStyle, Flex, Label, ParentElement},
    fonts::{self, TextStyle},
    platform::WindowOptions,
    AnyElement, App, Element, Entity, View, ViewContext,
};
use log::LevelFilter;
use pathfinder_geometry::vector::vec2f;
use simplelog::SimpleLogger;
use theme::Toggleable;
use toggleable_button::ToggleableButton;

// cargo run -p gpui --example components

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(WindowOptions::with_bounds(vec2f(300., 200.)), |_| {
            TestView {
                count: 0,
                is_doubling: false,
            }
        });
    });
}

pub struct TestView {
    count: usize,
    is_doubling: bool,
}

impl TestView {
    fn increase_count(&mut self) {
        if self.is_doubling {
            self.count *= 2;
        } else {
            self.count += 1;
        }
    }
}

impl Entity for TestView {
    type Event = ();
}

type ButtonStyle = ContainerStyle;

impl View for TestView {
    fn ui_name() -> &'static str {
        "TestView"
    }

    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self> {
        fonts::with_font_cache(cx.font_cache.to_owned(), || {
            Flex::column()
                .with_child(Label::new(
                    format!("Count: {}", self.count),
                    TextStyle::for_color(Color::red()),
                ))
                .with_child(
                    Button::new(move |_, v: &mut Self, cx| {
                        v.increase_count();
                        cx.notify();
                    })
                    .with_text(
                        "Hello from a counting BUTTON",
                        TextStyle::for_color(Color::blue()),
                    )
                    .with_style(ButtonStyle::fill(Color::yellow()))
                    .into_element(),
                )
                .with_child(
                    ToggleableButton::new(self.is_doubling, move |_, v: &mut Self, cx| {
                        v.is_doubling = !v.is_doubling;
                        cx.notify();
                    })
                    .with_text("Double the count?", TextStyle::for_color(Color::black()))
                    .with_style(Toggleable {
                        inactive: ButtonStyle::fill(Color::red()),
                        active: ButtonStyle::fill(Color::green()),
                    })
                    .into_element(),
                )
                .expanded()
                .contained()
                .with_background_color(Color::white())
                .into_any()
        })
    }
}

mod theme {
    pub struct Toggleable<T> {
        pub inactive: T,
        pub active: T,
    }

    impl<T> Toggleable<T> {
        pub fn style_for(&self, active: bool) -> &T {
            if active {
                &self.active
            } else {
                &self.inactive
            }
        }
    }
}

// Component creation:
mod toggleable_button {
    use gpui::{
        elements::{ContainerStyle, LabelStyle},
        scene::MouseClick,
        EventContext, View,
    };

    use crate::{button_component::Button, component::Component, theme::Toggleable};

    pub struct ToggleableButton<V: View> {
        active: bool,
        style: Option<Toggleable<ContainerStyle>>,
        button: Button<V>,
    }

    impl<V: View> ToggleableButton<V> {
        pub fn new<F>(active: bool, on_click: F) -> Self
        where
            F: Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
        {
            Self {
                active,
                button: Button::new(on_click),
                style: None,
            }
        }

        pub fn with_text(self, text: &str, style: impl Into<LabelStyle>) -> ToggleableButton<V> {
            ToggleableButton {
                active: self.active,
                style: self.style,
                button: self.button.with_text(text, style),
            }
        }

        pub fn with_style(self, style: Toggleable<ContainerStyle>) -> ToggleableButton<V> {
            ToggleableButton {
                active: self.active,
                style: Some(style),
                button: self.button,
            }
        }
    }

    impl<V: View> Component<V> for ToggleableButton<V> {
        fn render(self, v: &mut V, cx: &mut gpui::ViewContext<V>) -> gpui::AnyElement<V> {
            let button = if let Some(style) = self.style {
                self.button.with_style(*style.style_for(self.active))
            } else {
                self.button
            };
            button.render(v, cx)
        }
    }
}

mod button_component {

    use gpui::{
        elements::{ContainerStyle, Label, LabelStyle, MouseEventHandler},
        platform::MouseButton,
        scene::MouseClick,
        AnyElement, Element, EventContext, TypeTag, View, ViewContext,
    };

    use crate::component::Component;

    type ClickHandler<V> = Box<dyn Fn(MouseClick, &mut V, &mut EventContext<V>)>;

    pub struct Button<V: View> {
        click_handler: ClickHandler<V>,
        tag: TypeTag,
        contents: Option<AnyElement<V>>,
        style: Option<ContainerStyle>,
    }

    impl<V: View> Button<V> {
        pub fn new<F: Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static>(handler: F) -> Self {
            Self {
                click_handler: Box::new(handler),
                tag: TypeTag::new::<F>(),
                style: None,
                contents: None,
            }
        }

        pub fn with_text(mut self, text: &str, style: impl Into<LabelStyle>) -> Self {
            self.contents = Some(Label::new(text.to_string(), style).into_any());
            self
        }

        pub fn _with_contents<E: Element<V>>(mut self, contents: E) -> Self {
            self.contents = Some(contents.into_any());
            self
        }

        pub fn with_style(mut self, style: ContainerStyle) -> Self {
            self.style = Some(style);
            self
        }
    }

    impl<V: View> Component<V> for Button<V> {
        fn render(self, _: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V> {
            let click_handler = self.click_handler;

            let result = MouseEventHandler::new_dynamic(self.tag, 0, cx, |_, _| {
                self.contents
                    .unwrap_or_else(|| gpui::elements::Empty::new().into_any())
            })
            .on_click(MouseButton::Left, move |click, v, cx| {
                click_handler(click, v, cx);
            })
            .contained();

            let result = if let Some(style) = self.style {
                result.with_style(style)
            } else {
                result
            };

            result.into_any()
        }
    }
}

mod component {

    use gpui::{AnyElement, Element, View, ViewContext};
    use pathfinder_geometry::vector::Vector2F;

    pub trait Component<V: View> {
        fn render(self, v: &mut V, cx: &mut ViewContext<V>) -> AnyElement<V>;

        fn into_element(self) -> ComponentAdapter<V, Self>
        where
            Self: Sized,
        {
            ComponentAdapter::new(self)
        }
    }

    pub struct ComponentAdapter<V, E> {
        component: Option<E>,
        phantom: std::marker::PhantomData<V>,
    }

    impl<E, V> ComponentAdapter<V, E> {
        pub fn new(e: E) -> Self {
            Self {
                component: Some(e),
                phantom: std::marker::PhantomData,
            }
        }
    }

    impl<V: View, C: Component<V> + 'static> Element<V> for ComponentAdapter<V, C> {
        type LayoutState = AnyElement<V>;

        type PaintState = ();

        fn layout(
            &mut self,
            constraint: gpui::SizeConstraint,
            view: &mut V,
            cx: &mut gpui::LayoutContext<V>,
        ) -> (Vector2F, Self::LayoutState) {
            let component = self.component.take().unwrap();
            let mut element = component.render(view, cx.view_context());
            let constraint = element.layout(constraint, view, cx);
            (constraint, element)
        }

        fn paint(
            &mut self,
            scene: &mut gpui::SceneBuilder,
            bounds: gpui::geometry::rect::RectF,
            visible_bounds: gpui::geometry::rect::RectF,
            layout: &mut Self::LayoutState,
            view: &mut V,
            cx: &mut gpui::PaintContext<V>,
        ) -> Self::PaintState {
            layout.paint(scene, bounds.origin(), visible_bounds, view, cx)
        }

        fn rect_for_text_range(
            &self,
            _: std::ops::Range<usize>,
            _: gpui::geometry::rect::RectF,
            _: gpui::geometry::rect::RectF,
            _: &Self::LayoutState,
            _: &Self::PaintState,
            _: &V,
            _: &ViewContext<V>,
        ) -> Option<gpui::geometry::rect::RectF> {
            todo!()
        }

        fn debug(
            &self,
            _: gpui::geometry::rect::RectF,
            _: &Self::LayoutState,
            _: &Self::PaintState,
            _: &V,
            _: &ViewContext<V>,
        ) -> serde_json::Value {
            todo!()
        }
    }
}
