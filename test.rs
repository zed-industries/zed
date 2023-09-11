#![feature(prelude_import)]
#![allow(dead_code, unused_variables)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use color::black;
use components::button;
use element::Element;
use frame::frame;
use gpui::{
    geometry::{rect::RectF, vector::vec2f},
    platform::WindowOptions,
};
use log::LevelFilter;
use simplelog::SimpleLogger;
use themes::{rose_pine, ThemeColors};
use view::view;
mod adapter {
    use crate::element::AnyElement;
    use crate::element::{LayoutContext, PaintContext};
    use gpui::{geometry::rect::RectF, LayoutEngine};
    use util::ResultExt;
    pub struct Adapter<V>(pub(crate) AnyElement<V>);
    impl<V: 'static> gpui::Element<V> for Adapter<V> {
        type LayoutState = Option<LayoutEngine>;
        type PaintState = ();
        fn layout(
            &mut self,
            constraint: gpui::SizeConstraint,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
            cx.push_layout_engine(LayoutEngine::new());
            let node = self.0.layout(view, cx).log_err();
            if let Some(node) = node {
                let layout_engine = cx.layout_engine().unwrap();
                layout_engine.compute_layout(node, constraint.max).log_err();
            }
            let layout_engine = cx.pop_layout_engine();
            if true {
                if !layout_engine.is_some() {
                    ::core::panicking::panic("assertion failed: layout_engine.is_some()")
                }
            }
            (constraint.max, layout_engine)
        }
        fn paint(
            &mut self,
            scene: &mut gpui::SceneBuilder,
            bounds: RectF,
            visible_bounds: RectF,
            layout_engine: &mut Option<LayoutEngine>,
            view: &mut V,
            legacy_cx: &mut gpui::PaintContext<V>,
        ) -> Self::PaintState {
            legacy_cx.push_layout_engine(layout_engine.take().unwrap());
            let mut cx = PaintContext::new(legacy_cx, scene);
            self.0.paint(view, &mut cx).log_err();
            *layout_engine = legacy_cx.pop_layout_engine();
            if true {
                if !layout_engine.is_some() {
                    ::core::panicking::panic("assertion failed: layout_engine.is_some()")
                }
            }
        }
        fn rect_for_text_range(
            &self,
            range_utf16: std::ops::Range<usize>,
            bounds: RectF,
            visible_bounds: RectF,
            layout: &Self::LayoutState,
            paint: &Self::PaintState,
            view: &V,
            cx: &gpui::ViewContext<V>,
        ) -> Option<RectF> {
            ::core::panicking::panic("not yet implemented")
        }
        fn debug(
            &self,
            bounds: RectF,
            layout: &Self::LayoutState,
            paint: &Self::PaintState,
            view: &V,
            cx: &gpui::ViewContext<V>,
        ) -> gpui::serde_json::Value {
            ::core::panicking::panic("not yet implemented")
        }
    }
}
mod color {
    #![allow(dead_code)]
    use smallvec::SmallVec;
    use std::{num::ParseIntError, ops::Range};
    pub fn rgb<C: From<Rgba>>(hex: u32) -> C {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Rgba { r, g, b, a: 1.0 }.into()
    }
    pub struct Rgba {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Rgba {
        #[inline]
        fn clone(&self) -> Rgba {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Rgba {}
    #[automatically_derived]
    impl ::core::default::Default for Rgba {
        #[inline]
        fn default() -> Rgba {
            Rgba {
                r: ::core::default::Default::default(),
                g: ::core::default::Default::default(),
                b: ::core::default::Default::default(),
                a: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Rgba {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f, "Rgba", "r", &self.r, "g", &self.g, "b", &self.b, "a", &&self.a,
            )
        }
    }
    pub trait Lerp {
        fn lerp(&self, level: f32) -> Hsla;
    }
    impl Lerp for Range<Hsla> {
        fn lerp(&self, level: f32) -> Hsla {
            let level = level.clamp(0., 1.);
            Hsla {
                h: self.start.h + (level * (self.end.h - self.start.h)),
                s: self.start.s + (level * (self.end.s - self.start.s)),
                l: self.start.l + (level * (self.end.l - self.start.l)),
                a: self.start.a + (level * (self.end.a - self.start.a)),
            }
        }
    }
    impl From<gpui::color::Color> for Rgba {
        fn from(value: gpui::color::Color) -> Self {
            Self {
                r: value.0.r as f32 / 255.0,
                g: value.0.g as f32 / 255.0,
                b: value.0.b as f32 / 255.0,
                a: value.0.a as f32 / 255.0,
            }
        }
    }
    impl From<Hsla> for Rgba {
        fn from(color: Hsla) -> Self {
            let h = color.h;
            let s = color.s;
            let l = color.l;
            let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
            let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
            let m = l - c / 2.0;
            let cm = c + m;
            let xm = x + m;
            let (r, g, b) = match (h * 6.0).floor() as i32 {
                0 | 6 => (cm, xm, m),
                1 => (xm, cm, m),
                2 => (m, cm, xm),
                3 => (m, xm, cm),
                4 => (xm, m, cm),
                _ => (cm, m, xm),
            };
            Rgba {
                r,
                g,
                b,
                a: color.a,
            }
        }
    }
    impl TryFrom<&'_ str> for Rgba {
        type Error = ParseIntError;
        fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
            let r = u8::from_str_radix(&value[1..3], 16)? as f32 / 255.0;
            let g = u8::from_str_radix(&value[3..5], 16)? as f32 / 255.0;
            let b = u8::from_str_radix(&value[5..7], 16)? as f32 / 255.0;
            let a = if value.len() > 7 {
                u8::from_str_radix(&value[7..9], 16)? as f32 / 255.0
            } else {
                1.0
            };
            Ok(Rgba { r, g, b, a })
        }
    }
    impl Into<gpui::color::Color> for Rgba {
        fn into(self) -> gpui::color::Color {
            gpui::color::rgba(self.r, self.g, self.b, self.a)
        }
    }
    pub struct Hsla {
        pub h: f32,
        pub s: f32,
        pub l: f32,
        pub a: f32,
    }
    #[automatically_derived]
    impl ::core::default::Default for Hsla {
        #[inline]
        fn default() -> Hsla {
            Hsla {
                h: ::core::default::Default::default(),
                s: ::core::default::Default::default(),
                l: ::core::default::Default::default(),
                a: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Hsla {}
    #[automatically_derived]
    impl ::core::clone::Clone for Hsla {
        #[inline]
        fn clone(&self) -> Hsla {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Hsla {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f, "Hsla", "h", &self.h, "s", &self.s, "l", &self.l, "a", &&self.a,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Hsla {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Hsla {
        #[inline]
        fn eq(&self, other: &Hsla) -> bool {
            self.h == other.h && self.s == other.s && self.l == other.l && self.a == other.a
        }
    }
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
        Hsla {
            h: h.clamp(0., 1.),
            s: s.clamp(0., 1.),
            l: l.clamp(0., 1.),
            a: a.clamp(0., 1.),
        }
    }
    pub fn black() -> Hsla {
        Hsla {
            h: 0.,
            s: 0.,
            l: 0.,
            a: 1.,
        }
    }
    impl From<Rgba> for Hsla {
        fn from(color: Rgba) -> Self {
            let r = color.r;
            let g = color.g;
            let b = color.b;
            let max = r.max(g.max(b));
            let min = r.min(g.min(b));
            let delta = max - min;
            let l = (max + min) / 2.0;
            let s = if l == 0.0 || l == 1.0 {
                0.0
            } else if l < 0.5 {
                delta / (2.0 * l)
            } else {
                delta / (2.0 - 2.0 * l)
            };
            let h = if delta == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / delta).rem_euclid(6.0) / 6.0
            } else if max == g {
                ((b - r) / delta + 2.0) / 6.0
            } else {
                ((r - g) / delta + 4.0) / 6.0
            };
            Hsla {
                h,
                s,
                l,
                a: color.a,
            }
        }
    }
    impl Hsla {
        /// Scales the saturation and lightness by the given values, clamping at 1.0.
        pub fn scale_sl(mut self, s: f32, l: f32) -> Self {
            self.s = (self.s * s).clamp(0., 1.);
            self.l = (self.l * l).clamp(0., 1.);
            self
        }
        /// Increases the saturation of the color by a certain amount, with a max
        /// value of 1.0.
        pub fn saturate(mut self, amount: f32) -> Self {
            self.s += amount;
            self.s = self.s.clamp(0.0, 1.0);
            self
        }
        /// Decreases the saturation of the color by a certain amount, with a min
        /// value of 0.0.
        pub fn desaturate(mut self, amount: f32) -> Self {
            self.s -= amount;
            self.s = self.s.max(0.0);
            if self.s < 0.0 {
                self.s = 0.0;
            }
            self
        }
        /// Brightens the color by increasing the lightness by a certain amount,
        /// with a max value of 1.0.
        pub fn brighten(mut self, amount: f32) -> Self {
            self.l += amount;
            self.l = self.l.clamp(0.0, 1.0);
            self
        }
        /// Darkens the color by decreasing the lightness by a certain amount,
        /// with a max value of 0.0.
        pub fn darken(mut self, amount: f32) -> Self {
            self.l -= amount;
            self.l = self.l.clamp(0.0, 1.0);
            self
        }
    }
    impl From<gpui::color::Color> for Hsla {
        fn from(value: gpui::color::Color) -> Self {
            Rgba::from(value).into()
        }
    }
    impl Into<gpui::color::Color> for Hsla {
        fn into(self) -> gpui::color::Color {
            Rgba::from(self).into()
        }
    }
    pub struct ColorScale {
        colors: SmallVec<[Hsla; 2]>,
        positions: SmallVec<[f32; 2]>,
    }
    pub fn scale<I, C>(colors: I) -> ColorScale
    where
        I: IntoIterator<Item = C>,
        C: Into<Hsla>,
    {
        let mut scale = ColorScale {
            colors: colors.into_iter().map(Into::into).collect(),
            positions: SmallVec::new(),
        };
        let num_colors: f32 = scale.colors.len() as f32 - 1.0;
        scale.positions = (0..scale.colors.len())
            .map(|i| i as f32 / num_colors)
            .collect();
        scale
    }
    impl ColorScale {
        fn at(&self, t: f32) -> Hsla {
            if true {
                if !(0.0 <= t && t <= 1.0) {
                    {
                        ::core::panicking::panic_fmt(format_args!(
                            "t value {0} is out of range. Expected value in range 0.0 to 1.0",
                            t,
                        ));
                    }
                }
            }
            let position = match self
                .positions
                .binary_search_by(|a| a.partial_cmp(&t).unwrap())
            {
                Ok(index) | Err(index) => index,
            };
            let lower_bound = position.saturating_sub(1);
            let upper_bound = position.min(self.colors.len() - 1);
            let lower_color = &self.colors[lower_bound];
            let upper_color = &self.colors[upper_bound];
            match upper_bound.checked_sub(lower_bound) {
                Some(0) | None => *lower_color,
                Some(_) => {
                    let interval_t = (t - self.positions[lower_bound])
                        / (self.positions[upper_bound] - self.positions[lower_bound]);
                    let h = lower_color.h + interval_t * (upper_color.h - lower_color.h);
                    let s = lower_color.s + interval_t * (upper_color.s - lower_color.s);
                    let l = lower_color.l + interval_t * (upper_color.l - lower_color.l);
                    let a = lower_color.a + interval_t * (upper_color.a - lower_color.a);
                    Hsla { h, s, l, a }
                }
            }
        }
    }
}
mod components {
    use crate::{
        element::{Element, ElementMetadata},
        frame,
        text::ArcCow,
        themes::rose_pine,
    };
    use gpui::{platform::MouseButton, ViewContext};
    use gpui2_macros::Element;
    use std::{marker::PhantomData, rc::Rc};
    struct ButtonHandlers<V, D> {
        click: Option<Rc<dyn Fn(&mut V, &D, &mut ViewContext<V>)>>,
    }
    impl<V, D> Default for ButtonHandlers<V, D> {
        fn default() -> Self {
            Self { click: None }
        }
    }
    #[element_crate = "crate"]
    pub struct Button<V: 'static, D: 'static> {
        metadata: ElementMetadata<V>,
        handlers: ButtonHandlers<V, D>,
        label: Option<ArcCow<'static, str>>,
        icon: Option<ArcCow<'static, str>>,
        data: Rc<D>,
        view_type: PhantomData<V>,
    }
    impl<V: 'static, D: 'static> crate::element::Element<V> for Button<V, D> {
        type Layout = crate::element::AnyElement<V>;
        fn declared_style(&mut self) -> &mut crate::style::OptionalStyle {
            &mut self.metadata.style
        }
        fn handlers_mut(&mut self) -> &mut Vec<crate::element::EventHandler<V>> {
            &mut self.metadata.handlers
        }
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut crate::element::LayoutContext<V>,
        ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
            let mut element = self.render(view, cx).into_any();
            let node_id = element.layout(view, cx)?;
            Ok((node_id, element))
        }
        fn paint<'a>(
            &mut self,
            layout: crate::element::Layout<'a, Self::Layout>,
            view: &mut V,
            cx: &mut crate::element::PaintContext<V>,
        ) -> anyhow::Result<()> {
            layout.from_element.paint(view, cx)?;
            Ok(())
        }
    }
    impl<V: 'static, D: 'static> crate::element::IntoElement<V> for Button<V, D> {
        type Element = Self;
        fn into_element(self) -> Self {
            self
        }
    }
    impl<V: 'static> Button<V, ()> {
        fn new() -> Self {
            Self {
                metadata: Default::default(),
                handlers: ButtonHandlers::default(),
                label: None,
                icon: None,
                data: Rc::new(()),
                view_type: PhantomData,
            }
        }
        pub fn data<D: 'static>(self, data: D) -> Button<V, D> {
            Button {
                metadata: Default::default(),
                handlers: ButtonHandlers::default(),
                label: self.label,
                icon: self.icon,
                data: Rc::new(data),
                view_type: PhantomData,
            }
        }
    }
    impl<V: 'static, D: 'static> Button<V, D> {
        pub fn label(mut self, label: impl Into<ArcCow<'static, str>>) -> Self {
            self.label = Some(label.into());
            self
        }
        pub fn icon(mut self, icon: impl Into<ArcCow<'static, str>>) -> Self {
            self.icon = Some(icon.into());
            self
        }
        pub fn click(self, handler: impl Fn(&mut V, &D, &mut ViewContext<V>) + 'static) -> Self {
            let data = self.data.clone();
            Element::click(self, MouseButton::Left, move |view, _, cx| {
                handler(view, data.as_ref(), cx);
            })
        }
    }
    pub fn button<V>() -> Button<V, ()> {
        Button::new()
    }
    impl<V: 'static, D: 'static> Button<V, D> {
        fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
            let button = frame()
                .fill(rose_pine::dawn().error(0.5))
                .h_4()
                .children(self.label.clone());
            if let Some(handler) = self.handlers.click.clone() {
                let data = self.data.clone();
                button.mouse_down(MouseButton::Left, move |view, event, cx| {
                    handler(view, data.as_ref(), cx)
                })
            } else {
                button
            }
        }
    }
}
mod element {
    pub use crate::paint_context::PaintContext;
    use crate::{
        adapter::Adapter,
        color::Hsla,
        hoverable::Hoverable,
        style::{Display, Fill, OptionalStyle, Overflow, Position},
    };
    use anyhow::Result;
    pub use gpui::LayoutContext;
    use gpui::{
        geometry::{DefinedLength, Length, OptionalPoint},
        platform::{MouseButton, MouseButtonEvent},
        EngineLayout, EventContext, RenderContext, ViewContext,
    };
    use gpui2_macros::tailwind_lengths;
    use std::{
        any::{Any, TypeId},
        cell::Cell,
        rc::Rc,
    };
    pub use taffy::tree::NodeId;
    pub struct Layout<'a, E: ?Sized> {
        pub from_engine: EngineLayout,
        pub from_element: &'a mut E,
    }
    pub struct ElementMetadata<V> {
        pub style: OptionalStyle,
        pub handlers: Vec<EventHandler<V>>,
    }
    pub struct EventHandler<V> {
        handler: Rc<dyn Fn(&mut V, &dyn Any, &mut EventContext<V>)>,
        event_type: TypeId,
        outside_bounds: bool,
    }
    impl<V> Clone for EventHandler<V> {
        fn clone(&self) -> Self {
            Self {
                handler: self.handler.clone(),
                event_type: self.event_type,
                outside_bounds: self.outside_bounds,
            }
        }
    }
    impl<V> Default for ElementMetadata<V> {
        fn default() -> Self {
            Self {
                style: OptionalStyle::default(),
                handlers: Vec::new(),
            }
        }
    }
    pub trait Element<V: 'static>: 'static {
        type Layout: 'static;
        fn declared_style(&mut self) -> &mut OptionalStyle;
        fn computed_style(&mut self) -> &OptionalStyle {
            self.declared_style()
        }
        fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>>;
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> Result<(NodeId, Self::Layout)>;
        fn paint<'a>(
            &mut self,
            layout: Layout<Self::Layout>,
            view: &mut V,
            cx: &mut PaintContext<V>,
        ) -> Result<()>;
        /// Convert to a dynamically-typed element suitable for layout and paint.
        fn into_any(self) -> AnyElement<V>
        where
            Self: 'static + Sized,
        {
            AnyElement {
                element: Box::new(self) as Box<dyn ElementObject<V>>,
                layout: None,
            }
        }
        fn adapt(self) -> Adapter<V>
        where
            Self: Sized,
            Self: Element<V>,
        {
            Adapter(self.into_any())
        }
        fn click(
            self,
            button: MouseButton,
            handler: impl Fn(&mut V, &MouseButtonEvent, &mut ViewContext<V>) + 'static,
        ) -> Self
        where
            Self: Sized,
        {
            let pressed: Rc<Cell<bool>> = Default::default();
            self.mouse_down(button, {
                let pressed = pressed.clone();
                move |_, _, _| {
                    pressed.set(true);
                }
            })
            .mouse_up_outside(button, {
                let pressed = pressed.clone();
                move |_, _, _| {
                    pressed.set(false);
                }
            })
            .mouse_up(button, move |view, event, event_cx| {
                if pressed.get() {
                    pressed.set(false);
                    handler(view, event, event_cx);
                }
            })
        }
        fn mouse_down(
            mut self,
            button: MouseButton,
            handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
        ) -> Self
        where
            Self: Sized,
        {
            self.handlers_mut().push(EventHandler {
                handler: Rc::new(move |view, event, event_cx| {
                    let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                    if event.button == button && event.is_down {
                        handler(view, event, event_cx);
                    }
                }),
                event_type: TypeId::of::<MouseButtonEvent>(),
                outside_bounds: false,
            });
            self
        }
        fn mouse_down_outside(
            mut self,
            button: MouseButton,
            handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
        ) -> Self
        where
            Self: Sized,
        {
            self.handlers_mut().push(EventHandler {
                handler: Rc::new(move |view, event, event_cx| {
                    let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                    if event.button == button && event.is_down {
                        handler(view, event, event_cx);
                    }
                }),
                event_type: TypeId::of::<MouseButtonEvent>(),
                outside_bounds: true,
            });
            self
        }
        fn mouse_up(
            mut self,
            button: MouseButton,
            handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
        ) -> Self
        where
            Self: Sized,
        {
            self.handlers_mut().push(EventHandler {
                handler: Rc::new(move |view, event, event_cx| {
                    let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                    if event.button == button && !event.is_down {
                        handler(view, event, event_cx);
                    }
                }),
                event_type: TypeId::of::<MouseButtonEvent>(),
                outside_bounds: false,
            });
            self
        }
        fn mouse_up_outside(
            mut self,
            button: MouseButton,
            handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
        ) -> Self
        where
            Self: Sized,
        {
            self.handlers_mut().push(EventHandler {
                handler: Rc::new(move |view, event, event_cx| {
                    let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                    if event.button == button && !event.is_down {
                        handler(view, event, event_cx);
                    }
                }),
                event_type: TypeId::of::<MouseButtonEvent>(),
                outside_bounds: true,
            });
            self
        }
        fn block(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().display = Some(Display::Block);
            self
        }
        fn flex(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().display = Some(Display::Flex);
            self
        }
        fn grid(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().display = Some(Display::Grid);
            self
        }
        fn overflow_visible(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow = OptionalPoint {
                x: Some(Overflow::Visible),
                y: Some(Overflow::Visible),
            };
            self
        }
        fn overflow_hidden(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow = OptionalPoint {
                x: Some(Overflow::Hidden),
                y: Some(Overflow::Hidden),
            };
            self
        }
        fn overflow_scroll(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow = OptionalPoint {
                x: Some(Overflow::Scroll),
                y: Some(Overflow::Scroll),
            };
            self
        }
        fn overflow_x_visible(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow.x = Some(Overflow::Visible);
            self
        }
        fn overflow_x_hidden(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow.x = Some(Overflow::Hidden);
            self
        }
        fn overflow_x_scroll(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow.x = Some(Overflow::Scroll);
            self
        }
        fn overflow_y_visible(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow.y = Some(Overflow::Visible);
            self
        }
        fn overflow_y_hidden(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow.y = Some(Overflow::Hidden);
            self
        }
        fn overflow_y_scroll(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().overflow.y = Some(Overflow::Scroll);
            self
        }
        fn relative(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().position = Some(Position::Relative);
            self
        }
        fn absolute(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().position = Some(Position::Absolute);
            self
        }
        fn inset_0(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(0.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_px(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(1.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_0_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.125).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.25).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.375).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.5).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.625).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_3(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.75).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_3_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.875).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_4(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.25).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_6(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.5).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_7(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.75).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_8(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_9(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.25).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_10(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.5).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_11(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.75).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_12(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_14(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.5).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_16(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(4.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_20(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(5.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_24(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(6.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_28(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(7.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_32(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(8.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_36(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(9.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_40(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(10.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_44(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(11.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_48(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(12.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_52(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(13.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_56(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(14.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_60(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(15.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_64(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(16.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_72(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(18.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_80(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(20.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_96(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(24.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_half(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_3_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(20.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(40.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_3_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(60.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_4_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(80.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_3_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_4_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_5_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_1_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(8.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_2_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_3_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_4_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_5_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(41.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_6_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_7_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(58.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_8_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_9_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_10_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_11_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(91.666667).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn inset_full(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(100.).into();
            {
                let inset = self
                    .computed_style()
                    .inset
                    .get_or_insert_with(Default::default);
                inset.top = length;
                inset.right = length;
                inset.bottom = length;
                inset.left = length;
                self
            }
        }
        fn w(mut self, width: impl Into<Length>) -> Self
        where
            Self: Sized,
        {
            self.declared_style().size.width = Some(width.into());
            self
        }
        fn w_auto(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().size.width = Some(Length::Auto);
            self
        }
        fn w_0(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(0.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_px(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(1.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_0_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.125).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.25).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.375).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.5).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.625).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_3(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.75).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_3_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.875).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_4(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.25).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_6(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.5).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_7(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.75).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_8(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_9(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.25).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_10(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.5).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_11(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.75).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_12(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_14(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.5).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_16(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(4.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_20(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(5.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_24(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(6.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_28(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(7.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_32(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(8.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_36(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(9.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_40(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(10.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_44(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(11.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_48(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(12.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_52(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(13.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_56(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(14.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_60(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(15.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_64(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(16.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_72(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(18.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_80(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(20.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_96(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(24.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_half(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_3_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(20.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(40.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_3_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(60.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_4_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(80.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_3_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_4_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_5_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_1_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(8.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_2_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_3_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_4_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_5_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(41.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_6_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_7_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(58.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_8_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_9_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_10_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_11_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(91.666667).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn w_full(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(100.).into();
            {
                self.declared_style().size.width = Some(length);
                self
            }
        }
        fn min_w_0(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(0.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_px(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(1.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_0_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.125).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.25).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.375).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.5).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.625).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_3(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.75).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_3_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.875).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_4(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.25).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_6(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.5).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_7(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.75).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_8(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_9(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.25).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_10(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.5).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_11(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.75).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_12(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_14(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.5).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_16(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(4.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_20(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(5.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_24(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(6.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_28(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(7.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_32(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(8.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_36(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(9.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_40(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(10.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_44(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(11.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_48(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(12.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_52(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(13.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_56(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(14.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_60(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(15.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_64(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(16.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_72(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(18.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_80(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(20.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_96(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(24.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_half(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_3_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(20.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(40.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_3_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(60.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_4_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(80.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_3_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_4_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_5_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_1_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(8.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_2_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_3_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_4_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_5_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(41.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_6_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_7_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(58.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_8_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_9_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_10_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_11_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(91.666667).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn min_w_full(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(100.).into();
            {
                self.declared_style().min_size.width = Some(length);
                self
            }
        }
        fn h(mut self, height: impl Into<Length>) -> Self
        where
            Self: Sized,
        {
            self.declared_style().size.height = Some(height.into());
            self
        }
        fn h_auto(mut self) -> Self
        where
            Self: Sized,
        {
            self.declared_style().size.height = Some(Length::Auto);
            self
        }
        fn h_0(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Pixels(0.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_px(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Pixels(1.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_0_5(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.125).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.25).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1_5(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.375).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.5).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2_5(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.625).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_3(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.75).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_3_5(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(0.875).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_4(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(1.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_5(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(1.25).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_6(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(1.5).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_7(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(1.75).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_8(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(2.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_9(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(2.25).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_10(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(2.5).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_11(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(2.75).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_12(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(3.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_14(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(3.5).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_16(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(4.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_20(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(5.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_24(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(6.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_28(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(7.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_32(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(8.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_36(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(9.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_40(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(10.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_44(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(11.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_48(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(12.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_52(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(13.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_56(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(14.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_60(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(15.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_64(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(16.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_72(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(18.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_80(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(20.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_96(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Rems(24.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_half(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(25.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_3_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(75.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(20.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(40.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_3_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(60.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_4_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(80.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_3_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_4_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_5_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_1_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(8.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_2_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_3_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(25.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_4_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_5_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(41.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_6_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(50.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_7_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(58.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_8_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_9_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(75.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_10_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_11_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(91.666667).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn h_full(mut self) -> Self
        where
            Self: Sized,
        {
            let height = DefinedLength::Percent(100.).into();
            {
                self.declared_style().size.height = Some(height);
                self
            }
        }
        fn min_h_0(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(0.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_px(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Pixels(1.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_0_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.125).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.25).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.375).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.5).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.625).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_3(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.75).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_3_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(0.875).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_4(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_5(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.25).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_6(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.5).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_7(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(1.75).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_8(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_9(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.25).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_10(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.5).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_11(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(2.75).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_12(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_14(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(3.5).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_16(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(4.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_20(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(5.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_24(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(6.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_28(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(7.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_32(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(8.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_36(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(9.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_40(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(10.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_44(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(11.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_48(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(12.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_52(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(13.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_56(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(14.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_60(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(15.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_64(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(16.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_72(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(18.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_80(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(20.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_96(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Rems(24.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_half(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2_3rd(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_3_4th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(20.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(40.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_3_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(60.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_4_5th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(80.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_3_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_4_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_5_6th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_1_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(8.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_2_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(16.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_3_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(25.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_4_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(33.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_5_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(41.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_6_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(50.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_7_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(58.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_8_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(66.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_9_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(75.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_10_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(83.333333).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_11_12th(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(91.666667).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn min_h_full(mut self) -> Self
        where
            Self: Sized,
        {
            let length = DefinedLength::Percent(100.).into();
            {
                self.declared_style().min_size.height = Some(length);
                self
            }
        }
        fn hoverable(self) -> Hoverable<V, Self>
        where
            Self: Sized,
        {
            Hoverable::new(self)
        }
        fn fill(mut self, fill: impl Into<Fill>) -> Self
        where
            Self: Sized,
        {
            self.declared_style().fill = Some(Some(fill.into()));
            self
        }
        fn text_color(mut self, color: impl Into<Hsla>) -> Self
        where
            Self: Sized,
        {
            self.declared_style().text_color = Some(Some(color.into()));
            self
        }
    }
    trait ElementObject<V> {
        fn style(&mut self) -> &mut OptionalStyle;
        fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>>;
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> Result<(NodeId, Box<dyn Any>)>;
        fn paint(
            &mut self,
            layout: Layout<dyn Any>,
            view: &mut V,
            cx: &mut PaintContext<V>,
        ) -> Result<()>;
    }
    impl<V: 'static, E: Element<V>> ElementObject<V> for E {
        fn style(&mut self) -> &mut OptionalStyle {
            Element::declared_style(self)
        }
        fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
            Element::handlers_mut(self)
        }
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> Result<(NodeId, Box<dyn Any>)> {
            let (node_id, layout) = self.layout(view, cx)?;
            let layout = Box::new(layout) as Box<dyn Any>;
            Ok((node_id, layout))
        }
        fn paint(
            &mut self,
            layout: Layout<dyn Any>,
            view: &mut V,
            cx: &mut PaintContext<V>,
        ) -> Result<()> {
            let layout = Layout {
                from_engine: layout.from_engine,
                from_element: layout.from_element.downcast_mut::<E::Layout>().unwrap(),
            };
            self.paint(layout, view, cx)
        }
    }
    /// A dynamically typed element.
    pub struct AnyElement<V> {
        element: Box<dyn ElementObject<V>>,
        layout: Option<(NodeId, Box<dyn Any>)>,
    }
    impl<V: 'static> AnyElement<V> {
        pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId> {
            let pushed_text_style = self.push_text_style(cx);
            let (node_id, layout) = self.element.layout(view, cx)?;
            self.layout = Some((node_id, layout));
            if pushed_text_style {
                cx.pop_text_style();
            }
            Ok(node_id)
        }
        pub fn push_text_style(&mut self, cx: &mut impl RenderContext) -> bool {
            let text_style = self.element.style().text_style();
            if let Some(text_style) = text_style {
                let mut current_text_style = cx.text_style();
                text_style.apply(&mut current_text_style);
                cx.push_text_style(current_text_style);
                true
            } else {
                false
            }
        }
        pub fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
            let pushed_text_style = self.push_text_style(cx);
            let (layout_node_id, element_layout) =
                self.layout.as_mut().expect("paint called before layout");
            let layout = Layout {
                from_engine: cx
                    .layout_engine()
                    .unwrap()
                    .computed_layout(*layout_node_id)
                    .expect("make sure you're using this within a gpui2 adapter element"),
                from_element: element_layout.as_mut(),
            };
            let style = self.element.style();
            let fill_color = style.fill.flatten().and_then(|fill| fill.color());
            if let Some(fill_color) = fill_color {
                cx.scene.push_quad(gpui::scene::Quad {
                    bounds: layout.from_engine.bounds,
                    background: Some(fill_color.into()),
                    border: Default::default(),
                    corner_radii: Default::default(),
                });
            }
            for event_handler in self.element.handlers_mut().iter().cloned() {
                let EngineLayout { order, bounds } = layout.from_engine;
                let view_id = cx.view_id();
                let view_event_handler = event_handler.handler.clone();
                cx.scene
                    .interactive_regions
                    .push(gpui::scene::InteractiveRegion {
                        order,
                        bounds,
                        outside_bounds: event_handler.outside_bounds,
                        event_handler: Rc::new(move |view, event, window_cx, view_id| {
                            let mut view_context = ViewContext::mutable(window_cx, view_id);
                            let mut event_context = EventContext::new(&mut view_context);
                            view_event_handler(
                                view.downcast_mut().unwrap(),
                                event,
                                &mut event_context,
                            );
                        }),
                        event_type: event_handler.event_type,
                        view_id,
                    });
            }
            self.element.paint(layout, view, cx)?;
            if pushed_text_style {
                cx.pop_text_style();
            }
            Ok(())
        }
    }
    impl<V: 'static> Element<V> for AnyElement<V> {
        type Layout = ();
        fn declared_style(&mut self) -> &mut OptionalStyle {
            self.element.style()
        }
        fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
            self.element.handlers_mut()
        }
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> Result<(NodeId, Self::Layout)> {
            Ok((self.layout(view, cx)?, ()))
        }
        fn paint(
            &mut self,
            layout: Layout<()>,
            view: &mut V,
            cx: &mut PaintContext<V>,
        ) -> Result<()> {
            self.paint(view, cx)
        }
    }
    pub trait IntoElement<V: 'static> {
        type Element: Element<V>;
        fn into_element(self) -> Self::Element;
        fn into_any_element(self) -> AnyElement<V>
        where
            Self: Sized,
        {
            self.into_element().into_any()
        }
    }
}
mod frame {
    use crate::{
        element::{
            AnyElement, Element, EventHandler, IntoElement, Layout, LayoutContext, NodeId,
            PaintContext,
        },
        style::{OptionalStyle, Style},
    };
    use anyhow::{anyhow, Result};
    use gpui::LayoutNodeId;
    use gpui2_macros::IntoElement;
    #[element_crate = "crate"]
    pub struct Frame<V: 'static> {
        style: OptionalStyle,
        handlers: Vec<EventHandler<V>>,
        children: Vec<AnyElement<V>>,
    }
    impl<V: 'static> crate::element::IntoElement<V> for Frame<V> {
        type Element = Self;
        fn into_element(self) -> Self {
            self
        }
    }
    pub fn frame<V>() -> Frame<V> {
        Frame {
            style: OptionalStyle::default(),
            handlers: Vec::new(),
            children: Vec::new(),
        }
    }
    impl<V: 'static> Element<V> for Frame<V> {
        type Layout = ();
        fn declared_style(&mut self) -> &mut OptionalStyle {
            &mut self.style
        }
        fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
            &mut self.handlers
        }
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> Result<(NodeId, Self::Layout)> {
            let child_layout_node_ids = self
                .children
                .iter_mut()
                .map(|child| child.layout(view, cx))
                .collect::<Result<Vec<LayoutNodeId>>>()?;
            let rem_size = cx.rem_pixels();
            let style: Style = self.style.into();
            let node_id = cx
                .layout_engine()
                .ok_or_else(|| {
                    ::anyhow::__private::must_use({
                        let error =
                            ::anyhow::__private::format_err(format_args!("no layout engine"));
                        error
                    })
                })?
                .add_node(style.to_taffy(rem_size), child_layout_node_ids)?;
            Ok((node_id, ()))
        }
        fn paint(
            &mut self,
            layout: Layout<()>,
            view: &mut V,
            cx: &mut PaintContext<V>,
        ) -> Result<()> {
            for child in &mut self.children {
                child.paint(view, cx)?;
            }
            Ok(())
        }
    }
    impl<V: 'static> Frame<V> {
        pub fn child(mut self, child: impl IntoElement<V>) -> Self {
            self.children.push(child.into_any_element());
            self
        }
        pub fn children<I, E>(mut self, children: I) -> Self
        where
            I: IntoIterator<Item = E>,
            E: IntoElement<V>,
        {
            self.children
                .extend(children.into_iter().map(|e| e.into_any_element()));
            self
        }
    }
}
mod hoverable {
    use crate::{
        element::Element,
        style::{OptionalStyle, Style},
    };
    use gpui::{
        geometry::{rect::RectF, vector::Vector2F},
        scene::MouseMove,
        EngineLayout,
    };
    use std::{cell::Cell, marker::PhantomData, rc::Rc};
    pub struct Hoverable<V, E> {
        hover_style: OptionalStyle,
        computed_style: Option<Style>,
        view_type: PhantomData<V>,
        child: E,
    }
    impl<V, E> Hoverable<V, E> {
        pub fn new(child: E) -> Self {
            Self {
                hover_style: OptionalStyle::default(),
                computed_style: None,
                view_type: PhantomData,
                child,
            }
        }
    }
    impl<V: 'static, E: Element<V>> Element<V> for Hoverable<V, E> {
        type Layout = E::Layout;
        fn declared_style(&mut self) -> &mut OptionalStyle {
            &mut self.hover_style
        }
        fn computed_style(&mut self) -> &OptionalStyle {
            ::core::panicking::panic("not yet implemented")
        }
        fn handlers_mut(&mut self) -> &mut Vec<crate::element::EventHandler<V>> {
            self.child.handlers_mut()
        }
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut gpui::LayoutContext<V>,
        ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
            self.child.layout(view, cx)
        }
        fn paint<'a>(
            &mut self,
            layout: crate::element::Layout<Self::Layout>,
            view: &mut V,
            cx: &mut crate::element::PaintContext<V>,
        ) -> anyhow::Result<()> {
            let EngineLayout { bounds, order } = layout.from_engine;
            let window_bounds = RectF::new(Vector2F::zero(), cx.window_size());
            let was_hovered = Rc::new(Cell::new(false));
            self.child.paint(layout, view, cx)?;
            cx.draw_interactive_region(
                order,
                window_bounds,
                false,
                move |view, event: &MouseMove, cx| {
                    let is_hovered = bounds.contains_point(cx.mouse_position());
                    if is_hovered != was_hovered.get() {
                        was_hovered.set(is_hovered);
                        cx.repaint();
                    }
                },
            );
            Ok(())
        }
    }
}
mod paint_context {
    use derive_more::{Deref, DerefMut};
    use gpui::{geometry::rect::RectF, EventContext, RenderContext, ViewContext};
    pub use gpui::{LayoutContext, PaintContext as LegacyPaintContext};
    use std::{any::TypeId, rc::Rc};
    pub use taffy::tree::NodeId;
    pub struct PaintContext<'a, 'b, 'c, 'd, V> {
        #[deref]
        #[deref_mut]
        pub(crate) legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
        pub(crate) scene: &'d mut gpui::SceneBuilder,
    }
    impl<'a, 'b, 'c, 'd, V> ::core::ops::Deref for PaintContext<'a, 'b, 'c, 'd, V> {
        type Target = &'d mut LegacyPaintContext<'a, 'b, 'c, V>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.legacy_cx
        }
    }
    impl<'a, 'b, 'c, 'd, V> ::core::ops::DerefMut for PaintContext<'a, 'b, 'c, 'd, V> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.legacy_cx
        }
    }
    impl<V> RenderContext for PaintContext<'_, '_, '_, '_, V> {
        fn text_style(&self) -> gpui::fonts::TextStyle {
            self.legacy_cx.text_style()
        }
        fn push_text_style(&mut self, style: gpui::fonts::TextStyle) {
            self.legacy_cx.push_text_style(style)
        }
        fn pop_text_style(&mut self) {
            self.legacy_cx.pop_text_style()
        }
    }
    impl<'a, 'b, 'c, 'd, V: 'static> PaintContext<'a, 'b, 'c, 'd, V> {
        pub fn new(
            legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
            scene: &'d mut gpui::SceneBuilder,
        ) -> Self {
            Self { legacy_cx, scene }
        }
        pub fn draw_interactive_region<E: 'static>(
            &mut self,
            order: u32,
            bounds: RectF,
            outside_bounds: bool,
            event_handler: impl Fn(&mut V, &E, &mut EventContext<V>) + 'static,
        ) {
            self.scene
                .interactive_regions
                .push(gpui::scene::InteractiveRegion {
                    order,
                    bounds,
                    outside_bounds,
                    event_handler: Rc::new(move |view, event, window_cx, view_id| {
                        let mut view_context = ViewContext::mutable(window_cx, view_id);
                        let mut event_context = EventContext::new(&mut view_context);
                        event_handler(
                            view.downcast_mut().unwrap(),
                            event.downcast_ref().unwrap(),
                            &mut event_context,
                        );
                    }),
                    event_type: TypeId::of::<E>(),
                    view_id: self.view_id(),
                });
        }
    }
}
mod style {
    use crate::color::Hsla;
    use gpui::geometry::{
        DefinedLength, Edges, Length, OptionalEdges, OptionalPoint, OptionalSize, Point, Size,
    };
    use optional::Optional;
    pub use taffy::style::{
        AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
        Overflow, Position,
    };
    pub struct Style {
        /// What layout strategy should be used?
        pub display: Display,
        /// How children overflowing their container should affect layout
        #[optional]
        pub overflow: Point<Overflow>,
        /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
        pub scrollbar_width: f32,
        /// What should the `position` value of this struct use as a base offset?
        pub position: Position,
        /// How should the position of this element be tweaked relative to the layout defined?
        pub inset: Edges<Length>,
        /// Sets the initial size of the item
        #[optional]
        pub size: Size<Length>,
        /// Controls the minimum size of the item
        #[optional]
        pub min_size: Size<Length>,
        /// Controls the maximum size of the item
        #[optional]
        pub max_size: Size<Length>,
        /// Sets the preferred aspect ratio for the item. The ratio is calculated as width divided by height.
        pub aspect_ratio: Option<f32>,
        /// How large should the margin be on each side?
        #[optional]
        pub margin: Edges<Length>,
        /// How large should the padding be on each side?
        pub padding: Edges<DefinedLength>,
        /// How large should the border be on each side?
        pub border: Edges<DefinedLength>,
        /// How this node's children aligned in the cross/block axis?
        pub align_items: Option<AlignItems>,
        /// How this node should be aligned in the cross/block axis. Falls back to the parents [`AlignItems`] if not set
        pub align_self: Option<AlignSelf>,
        /// How should content contained within this item be aligned in the cross/block axis
        pub align_content: Option<AlignContent>,
        /// How should contained within this item be aligned in the main/inline axis
        pub justify_content: Option<JustifyContent>,
        /// How large should the gaps between items in a flex container be?
        pub gap: Size<DefinedLength>,
        /// Which direction does the main axis flow in?
        pub flex_direction: FlexDirection,
        /// Should elements wrap, or stay in a single line?
        pub flex_wrap: FlexWrap,
        /// Sets the initial main axis size of the item
        pub flex_basis: Length,
        /// The relative rate at which this item grows when it is expanding to fill space, 0.0 is the default value, and this value must be positive.
        pub flex_grow: f32,
        /// The relative rate at which this item shrinks when it is contracting to fit into space, 1.0 is the default value, and this value must be positive.
        pub flex_shrink: f32,
        /// The fill color of this element
        pub fill: Option<Fill>,
        /// The color of text within this element. Cascades to children unless overridden.
        pub text_color: Option<Hsla>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Style {
        #[inline]
        fn clone(&self) -> Style {
            Style {
                display: ::core::clone::Clone::clone(&self.display),
                overflow: ::core::clone::Clone::clone(&self.overflow),
                scrollbar_width: ::core::clone::Clone::clone(&self.scrollbar_width),
                position: ::core::clone::Clone::clone(&self.position),
                inset: ::core::clone::Clone::clone(&self.inset),
                size: ::core::clone::Clone::clone(&self.size),
                min_size: ::core::clone::Clone::clone(&self.min_size),
                max_size: ::core::clone::Clone::clone(&self.max_size),
                aspect_ratio: ::core::clone::Clone::clone(&self.aspect_ratio),
                margin: ::core::clone::Clone::clone(&self.margin),
                padding: ::core::clone::Clone::clone(&self.padding),
                border: ::core::clone::Clone::clone(&self.border),
                align_items: ::core::clone::Clone::clone(&self.align_items),
                align_self: ::core::clone::Clone::clone(&self.align_self),
                align_content: ::core::clone::Clone::clone(&self.align_content),
                justify_content: ::core::clone::Clone::clone(&self.justify_content),
                gap: ::core::clone::Clone::clone(&self.gap),
                flex_direction: ::core::clone::Clone::clone(&self.flex_direction),
                flex_wrap: ::core::clone::Clone::clone(&self.flex_wrap),
                flex_basis: ::core::clone::Clone::clone(&self.flex_basis),
                flex_grow: ::core::clone::Clone::clone(&self.flex_grow),
                flex_shrink: ::core::clone::Clone::clone(&self.flex_shrink),
                fill: ::core::clone::Clone::clone(&self.fill),
                text_color: ::core::clone::Clone::clone(&self.text_color),
            }
        }
    }
    pub struct OptionalStyle {
        pub display: Option<Display>,
        pub overflow: OptionalPoint<Overflow>,
        pub scrollbar_width: Option<f32>,
        pub position: Option<Position>,
        pub inset: Option<Edges<Length>>,
        pub size: OptionalSize<Length>,
        pub min_size: OptionalSize<Length>,
        pub max_size: OptionalSize<Length>,
        pub aspect_ratio: Option<Option<f32>>,
        pub margin: OptionalEdges<Length>,
        pub padding: Option<Edges<DefinedLength>>,
        pub border: Option<Edges<DefinedLength>>,
        pub align_items: Option<Option<AlignItems>>,
        pub align_self: Option<Option<AlignSelf>>,
        pub align_content: Option<Option<AlignContent>>,
        pub justify_content: Option<Option<JustifyContent>>,
        pub gap: Option<Size<DefinedLength>>,
        pub flex_direction: Option<FlexDirection>,
        pub flex_wrap: Option<FlexWrap>,
        pub flex_basis: Option<Length>,
        pub flex_grow: Option<f32>,
        pub flex_shrink: Option<f32>,
        pub fill: Option<Option<Fill>>,
        pub text_color: Option<Option<Hsla>>,
    }
    #[automatically_derived]
    impl ::core::default::Default for OptionalStyle {
        #[inline]
        fn default() -> OptionalStyle {
            OptionalStyle {
                display: ::core::default::Default::default(),
                overflow: ::core::default::Default::default(),
                scrollbar_width: ::core::default::Default::default(),
                position: ::core::default::Default::default(),
                inset: ::core::default::Default::default(),
                size: ::core::default::Default::default(),
                min_size: ::core::default::Default::default(),
                max_size: ::core::default::Default::default(),
                aspect_ratio: ::core::default::Default::default(),
                margin: ::core::default::Default::default(),
                padding: ::core::default::Default::default(),
                border: ::core::default::Default::default(),
                align_items: ::core::default::Default::default(),
                align_self: ::core::default::Default::default(),
                align_content: ::core::default::Default::default(),
                justify_content: ::core::default::Default::default(),
                gap: ::core::default::Default::default(),
                flex_direction: ::core::default::Default::default(),
                flex_wrap: ::core::default::Default::default(),
                flex_basis: ::core::default::Default::default(),
                flex_grow: ::core::default::Default::default(),
                flex_shrink: ::core::default::Default::default(),
                fill: ::core::default::Default::default(),
                text_color: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OptionalStyle {
        #[inline]
        fn clone(&self) -> OptionalStyle {
            OptionalStyle {
                display: ::core::clone::Clone::clone(&self.display),
                overflow: ::core::clone::Clone::clone(&self.overflow),
                scrollbar_width: ::core::clone::Clone::clone(&self.scrollbar_width),
                position: ::core::clone::Clone::clone(&self.position),
                inset: ::core::clone::Clone::clone(&self.inset),
                size: ::core::clone::Clone::clone(&self.size),
                min_size: ::core::clone::Clone::clone(&self.min_size),
                max_size: ::core::clone::Clone::clone(&self.max_size),
                aspect_ratio: ::core::clone::Clone::clone(&self.aspect_ratio),
                margin: ::core::clone::Clone::clone(&self.margin),
                padding: ::core::clone::Clone::clone(&self.padding),
                border: ::core::clone::Clone::clone(&self.border),
                align_items: ::core::clone::Clone::clone(&self.align_items),
                align_self: ::core::clone::Clone::clone(&self.align_self),
                align_content: ::core::clone::Clone::clone(&self.align_content),
                justify_content: ::core::clone::Clone::clone(&self.justify_content),
                gap: ::core::clone::Clone::clone(&self.gap),
                flex_direction: ::core::clone::Clone::clone(&self.flex_direction),
                flex_wrap: ::core::clone::Clone::clone(&self.flex_wrap),
                flex_basis: ::core::clone::Clone::clone(&self.flex_basis),
                flex_grow: ::core::clone::Clone::clone(&self.flex_grow),
                flex_shrink: ::core::clone::Clone::clone(&self.flex_shrink),
                fill: ::core::clone::Clone::clone(&self.fill),
                text_color: ::core::clone::Clone::clone(&self.text_color),
            }
        }
    }
    impl Optional for OptionalStyle {
        type Base = Style;
        fn assign(&self, base: &mut Self::Base) {
            if let Some(value) = self.display.clone() {
                base.display = value;
            }
            if let Some(value) = self.overflow.clone() {
                base.overflow = value;
            }
            if let Some(value) = self.scrollbar_width.clone() {
                base.scrollbar_width = value;
            }
            if let Some(value) = self.position.clone() {
                base.position = value;
            }
            if let Some(value) = self.inset.clone() {
                base.inset = value;
            }
            if let Some(value) = self.size.clone() {
                base.size = value;
            }
            if let Some(value) = self.min_size.clone() {
                base.min_size = value;
            }
            if let Some(value) = self.max_size.clone() {
                base.max_size = value;
            }
            if let Some(value) = self.aspect_ratio.clone() {
                base.aspect_ratio = value;
            }
            if let Some(value) = self.margin.clone() {
                base.margin = value;
            }
            if let Some(value) = self.padding.clone() {
                base.padding = value;
            }
            if let Some(value) = self.border.clone() {
                base.border = value;
            }
            if let Some(value) = self.align_items.clone() {
                base.align_items = value;
            }
            if let Some(value) = self.align_self.clone() {
                base.align_self = value;
            }
            if let Some(value) = self.align_content.clone() {
                base.align_content = value;
            }
            if let Some(value) = self.justify_content.clone() {
                base.justify_content = value;
            }
            if let Some(value) = self.gap.clone() {
                base.gap = value;
            }
            if let Some(value) = self.flex_direction.clone() {
                base.flex_direction = value;
            }
            if let Some(value) = self.flex_wrap.clone() {
                base.flex_wrap = value;
            }
            if let Some(value) = self.flex_basis.clone() {
                base.flex_basis = value;
            }
            if let Some(value) = self.flex_grow.clone() {
                base.flex_grow = value;
            }
            if let Some(value) = self.flex_shrink.clone() {
                base.flex_shrink = value;
            }
            if let Some(value) = self.fill.clone() {
                base.fill = value;
            }
            if let Some(value) = self.text_color.clone() {
                base.text_color = value;
            }
        }
    }
    impl From<OptionalStyle> for Style
    where
        Style: Default,
    {
        fn from(wrapper: OptionalStyle) -> Self {
            let mut base = Self::default();
            wrapper.assign(&mut base);
            base
        }
    }
    impl Style {
        pub const DEFAULT: Style = Style {
            display: Display::DEFAULT,
            overflow: Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
            scrollbar_width: 0.0,
            position: Position::Relative,
            inset: Edges::auto(),
            margin: Edges::<Length>::zero(),
            padding: Edges::<DefinedLength>::zero(),
            border: Edges::<DefinedLength>::zero(),
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: None,
            gap: Size::zero(),
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            fill: None,
            text_color: None,
        };
        pub fn new() -> Self {
            Self::DEFAULT.clone()
        }
        pub fn to_taffy(&self, rem_size: f32) -> taffy::style::Style {
            taffy::style::Style {
                display: self.display,
                overflow: self.overflow.clone().into(),
                scrollbar_width: self.scrollbar_width,
                position: self.position,
                inset: self.inset.to_taffy(rem_size),
                size: self.size.to_taffy(rem_size),
                min_size: self.min_size.to_taffy(rem_size),
                max_size: self.max_size.to_taffy(rem_size),
                aspect_ratio: self.aspect_ratio,
                margin: self.margin.to_taffy(rem_size),
                padding: self.padding.to_taffy(rem_size),
                border: self.border.to_taffy(rem_size),
                align_items: self.align_items,
                align_self: self.align_self,
                align_content: self.align_content,
                justify_content: self.justify_content,
                gap: self.gap.to_taffy(rem_size),
                flex_direction: self.flex_direction,
                flex_wrap: self.flex_wrap,
                flex_basis: self.flex_basis.to_taffy(rem_size).into(),
                flex_grow: self.flex_grow,
                flex_shrink: self.flex_shrink,
                ..Default::default()
            }
        }
    }
    impl Default for Style {
        fn default() -> Self {
            Self::DEFAULT.clone()
        }
    }
    impl OptionalStyle {
        pub fn text_style(&self) -> Option<OptionalTextStyle> {
            self.text_color.map(|color| OptionalTextStyle { color })
        }
    }
    pub struct OptionalTextStyle {
        color: Option<Hsla>,
    }
    impl OptionalTextStyle {
        pub fn apply(&self, style: &mut gpui::fonts::TextStyle) {
            if let Some(color) = self.color {
                style.color = color.into();
            }
        }
    }
    pub enum Fill {
        Color(Hsla),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Fill {
        #[inline]
        fn clone(&self) -> Fill {
            match self {
                Fill::Color(__self_0) => Fill::Color(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl Fill {
        pub fn color(&self) -> Option<Hsla> {
            match self {
                Fill::Color(color) => Some(*color),
            }
        }
    }
    impl Default for Fill {
        fn default() -> Self {
            Self::Color(Hsla::default())
        }
    }
    impl From<Hsla> for Fill {
        fn from(color: Hsla) -> Self {
            Self::Color(color)
        }
    }
}
mod text {
    use crate::{
        element::{Element, ElementMetadata, EventHandler, IntoElement},
        style::Style,
    };
    use gpui::{geometry::Size, text_layout::LineLayout, RenderContext};
    use parking_lot::Mutex;
    use std::sync::Arc;
    impl<V: 'static, S: Into<ArcCow<'static, str>>> IntoElement<V> for S {
        type Element = Text<V>;
        fn into_element(self) -> Self::Element {
            Text {
                text: self.into(),
                metadata: Default::default(),
            }
        }
    }
    pub struct Text<V> {
        text: ArcCow<'static, str>,
        metadata: ElementMetadata<V>,
    }
    impl<V: 'static> Element<V> for Text<V> {
        type Layout = Arc<Mutex<Option<TextLayout>>>;
        fn declared_style(&mut self) -> &mut crate::style::OptionalStyle {
            &mut self.metadata.style
        }
        fn layout(
            &mut self,
            view: &mut V,
            cx: &mut gpui::LayoutContext<V>,
        ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
            let rem_size = cx.rem_pixels();
            let fonts = cx.platform().fonts();
            let text_style = cx.text_style();
            let line_height = cx.font_cache().line_height(text_style.font_size);
            let layout_engine = cx.layout_engine().expect("no layout engine present");
            let text = self.text.clone();
            let layout = Arc::new(Mutex::new(None));
            let style: Style = self.metadata.style.into();
            let node_id = layout_engine.add_measured_node(style.to_taffy(rem_size), {
                let layout = layout.clone();
                move |params| {
                    let line_layout = fonts.layout_line(
                        text.as_ref(),
                        text_style.font_size,
                        &[(text.len(), text_style.to_run())],
                    );
                    let size = Size {
                        width: line_layout.width,
                        height: line_height,
                    };
                    layout.lock().replace(TextLayout {
                        line_layout: Arc::new(line_layout),
                        line_height,
                    });
                    size
                }
            })?;
            Ok((node_id, layout))
        }
        fn paint<'a>(
            &mut self,
            layout: crate::element::Layout<Arc<Mutex<Option<TextLayout>>>>,
            view: &mut V,
            cx: &mut crate::element::PaintContext<V>,
        ) -> anyhow::Result<()> {
            let element_layout_lock = layout.from_element.lock();
            let element_layout = element_layout_lock
                .as_ref()
                .expect("layout has not been performed");
            let line_layout = element_layout.line_layout.clone();
            let line_height = element_layout.line_height;
            drop(element_layout_lock);
            let text_style = cx.text_style();
            let line = gpui::text_layout::Line::new(
                line_layout,
                &[(self.text.len(), text_style.to_run())],
            );
            line.paint(
                cx.scene,
                layout.from_engine.bounds.origin(),
                layout.from_engine.bounds,
                line_height,
                cx.legacy_cx,
            );
            Ok(())
        }
        fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
            &mut self.metadata.handlers
        }
    }
    pub struct TextLayout {
        line_layout: Arc<LineLayout>,
        line_height: f32,
    }
    pub enum ArcCow<'a, T: ?Sized> {
        Borrowed(&'a T),
        Owned(Arc<T>),
    }
    impl<'a, T: ?Sized> Clone for ArcCow<'a, T> {
        fn clone(&self) -> Self {
            match self {
                Self::Borrowed(borrowed) => Self::Borrowed(borrowed),
                Self::Owned(owned) => Self::Owned(owned.clone()),
            }
        }
    }
    impl<'a, T: ?Sized> From<&'a T> for ArcCow<'a, T> {
        fn from(s: &'a T) -> Self {
            Self::Borrowed(s)
        }
    }
    impl<T> From<Arc<T>> for ArcCow<'_, T> {
        fn from(s: Arc<T>) -> Self {
            Self::Owned(s)
        }
    }
    impl From<String> for ArcCow<'_, str> {
        fn from(value: String) -> Self {
            Self::Owned(value.into())
        }
    }
    impl<T: ?Sized> std::ops::Deref for ArcCow<'_, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            match self {
                ArcCow::Borrowed(s) => s,
                ArcCow::Owned(s) => s.as_ref(),
            }
        }
    }
    impl<T: ?Sized> AsRef<T> for ArcCow<'_, T> {
        fn as_ref(&self) -> &T {
            match self {
                ArcCow::Borrowed(borrowed) => borrowed,
                ArcCow::Owned(owned) => owned.as_ref(),
            }
        }
    }
}
mod themes {
    use crate::color::{Hsla, Lerp};
    use std::ops::Range;
    pub mod rose_pine {
        use crate::{
            color::{hsla, rgb, Hsla},
            ThemeColors,
        };
        use std::ops::Range;
        pub struct RosePineThemes {
            pub default: RosePinePalette,
            pub dawn: RosePinePalette,
            pub moon: RosePinePalette,
        }
        pub struct RosePinePalette {
            pub base: Hsla,
            pub surface: Hsla,
            pub overlay: Hsla,
            pub muted: Hsla,
            pub subtle: Hsla,
            pub text: Hsla,
            pub love: Hsla,
            pub gold: Hsla,
            pub rose: Hsla,
            pub pine: Hsla,
            pub foam: Hsla,
            pub iris: Hsla,
            pub highlight_low: Hsla,
            pub highlight_med: Hsla,
            pub highlight_high: Hsla,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RosePinePalette {
            #[inline]
            fn clone(&self) -> RosePinePalette {
                let _: ::core::clone::AssertParamIsClone<Hsla>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RosePinePalette {}
        #[automatically_derived]
        impl ::core::fmt::Debug for RosePinePalette {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "base",
                    "surface",
                    "overlay",
                    "muted",
                    "subtle",
                    "text",
                    "love",
                    "gold",
                    "rose",
                    "pine",
                    "foam",
                    "iris",
                    "highlight_low",
                    "highlight_med",
                    "highlight_high",
                ];
                let values: &[&dyn::core::fmt::Debug] = &[
                    &self.base,
                    &self.surface,
                    &self.overlay,
                    &self.muted,
                    &self.subtle,
                    &self.text,
                    &self.love,
                    &self.gold,
                    &self.rose,
                    &self.pine,
                    &self.foam,
                    &self.iris,
                    &self.highlight_low,
                    &self.highlight_med,
                    &&self.highlight_high,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "RosePinePalette",
                    names,
                    values,
                )
            }
        }
        impl RosePinePalette {
            pub fn default() -> RosePinePalette {
                RosePinePalette {
                    base: rgb(0x191724),
                    surface: rgb(0x1f1d2e),
                    overlay: rgb(0x26233a),
                    muted: rgb(0x6e6a86),
                    subtle: rgb(0x908caa),
                    text: rgb(0xe0def4),
                    love: rgb(0xeb6f92),
                    gold: rgb(0xf6c177),
                    rose: rgb(0xebbcba),
                    pine: rgb(0x31748f),
                    foam: rgb(0x9ccfd8),
                    iris: rgb(0xc4a7e7),
                    highlight_low: rgb(0x21202e),
                    highlight_med: rgb(0x403d52),
                    highlight_high: rgb(0x524f67),
                }
            }
            pub fn moon() -> RosePinePalette {
                RosePinePalette {
                    base: rgb(0x232136),
                    surface: rgb(0x2a273f),
                    overlay: rgb(0x393552),
                    muted: rgb(0x6e6a86),
                    subtle: rgb(0x908caa),
                    text: rgb(0xe0def4),
                    love: rgb(0xeb6f92),
                    gold: rgb(0xf6c177),
                    rose: rgb(0xea9a97),
                    pine: rgb(0x3e8fb0),
                    foam: rgb(0x9ccfd8),
                    iris: rgb(0xc4a7e7),
                    highlight_low: rgb(0x2a283e),
                    highlight_med: rgb(0x44415a),
                    highlight_high: rgb(0x56526e),
                }
            }
            pub fn dawn() -> RosePinePalette {
                RosePinePalette {
                    base: rgb(0xfaf4ed),
                    surface: rgb(0xfffaf3),
                    overlay: rgb(0xf2e9e1),
                    muted: rgb(0x9893a5),
                    subtle: rgb(0x797593),
                    text: rgb(0x575279),
                    love: rgb(0xb4637a),
                    gold: rgb(0xea9d34),
                    rose: rgb(0xd7827e),
                    pine: rgb(0x286983),
                    foam: rgb(0x56949f),
                    iris: rgb(0x907aa9),
                    highlight_low: rgb(0xf4ede8),
                    highlight_med: rgb(0xdfdad9),
                    highlight_high: rgb(0xcecacd),
                }
            }
        }
        pub fn default() -> ThemeColors {
            theme_colors(&RosePinePalette::default())
        }
        pub fn moon() -> ThemeColors {
            theme_colors(&RosePinePalette::moon())
        }
        pub fn dawn() -> ThemeColors {
            theme_colors(&RosePinePalette::dawn())
        }
        fn theme_colors(p: &RosePinePalette) -> ThemeColors {
            ThemeColors {
                base: scale_sl(p.base, (0.8, 0.8), (1.2, 1.2)),
                surface: scale_sl(p.surface, (0.8, 0.8), (1.2, 1.2)),
                overlay: scale_sl(p.overlay, (0.8, 0.8), (1.2, 1.2)),
                muted: scale_sl(p.muted, (0.8, 0.8), (1.2, 1.2)),
                subtle: scale_sl(p.subtle, (0.8, 0.8), (1.2, 1.2)),
                text: scale_sl(p.text, (0.8, 0.8), (1.2, 1.2)),
                highlight_low: scale_sl(p.highlight_low, (0.8, 0.8), (1.2, 1.2)),
                highlight_med: scale_sl(p.highlight_med, (0.8, 0.8), (1.2, 1.2)),
                highlight_high: scale_sl(p.highlight_high, (0.8, 0.8), (1.2, 1.2)),
                success: scale_sl(p.foam, (0.8, 0.8), (1.2, 1.2)),
                warning: scale_sl(p.gold, (0.8, 0.8), (1.2, 1.2)),
                error: scale_sl(p.love, (0.8, 0.8), (1.2, 1.2)),
                inserted: scale_sl(p.foam, (0.8, 0.8), (1.2, 1.2)),
                deleted: scale_sl(p.love, (0.8, 0.8), (1.2, 1.2)),
                modified: scale_sl(p.rose, (0.8, 0.8), (1.2, 1.2)),
            }
        }
        /// Produces a range by multiplying the saturation and lightness of the base color by the given
        /// start and end factors.
        fn scale_sl(
            base: Hsla,
            (start_s, start_l): (f32, f32),
            (end_s, end_l): (f32, f32),
        ) -> Range<Hsla> {
            let start = hsla(base.h, base.s * start_s, base.l * start_l, base.a);
            let end = hsla(base.h, base.s * end_s, base.l * end_l, base.a);
            Range { start, end }
        }
    }
    pub struct ThemeColors {
        pub base: Range<Hsla>,
        pub surface: Range<Hsla>,
        pub overlay: Range<Hsla>,
        pub muted: Range<Hsla>,
        pub subtle: Range<Hsla>,
        pub text: Range<Hsla>,
        pub highlight_low: Range<Hsla>,
        pub highlight_med: Range<Hsla>,
        pub highlight_high: Range<Hsla>,
        pub success: Range<Hsla>,
        pub warning: Range<Hsla>,
        pub error: Range<Hsla>,
        pub inserted: Range<Hsla>,
        pub deleted: Range<Hsla>,
        pub modified: Range<Hsla>,
    }
    impl ThemeColors {
        pub fn base(&self, level: f32) -> Hsla {
            self.base.lerp(level)
        }
        pub fn surface(&self, level: f32) -> Hsla {
            self.surface.lerp(level)
        }
        pub fn overlay(&self, level: f32) -> Hsla {
            self.overlay.lerp(level)
        }
        pub fn muted(&self, level: f32) -> Hsla {
            self.muted.lerp(level)
        }
        pub fn subtle(&self, level: f32) -> Hsla {
            self.subtle.lerp(level)
        }
        pub fn text(&self, level: f32) -> Hsla {
            self.text.lerp(level)
        }
        pub fn highlight_low(&self, level: f32) -> Hsla {
            self.highlight_low.lerp(level)
        }
        pub fn highlight_med(&self, level: f32) -> Hsla {
            self.highlight_med.lerp(level)
        }
        pub fn highlight_high(&self, level: f32) -> Hsla {
            self.highlight_high.lerp(level)
        }
        pub fn success(&self, level: f32) -> Hsla {
            self.success.lerp(level)
        }
        pub fn warning(&self, level: f32) -> Hsla {
            self.warning.lerp(level)
        }
        pub fn error(&self, level: f32) -> Hsla {
            self.error.lerp(level)
        }
        pub fn inserted(&self, level: f32) -> Hsla {
            self.inserted.lerp(level)
        }
        pub fn deleted(&self, level: f32) -> Hsla {
            self.deleted.lerp(level)
        }
        pub fn modified(&self, level: f32) -> Hsla {
            self.modified.lerp(level)
        }
    }
}
mod view {
    use crate::element::{AnyElement, Element};
    use gpui::{Element as _, ViewContext};
    pub fn view<F, E>(mut render: F) -> ViewFn
    where
        F: 'static + FnMut(&mut ViewContext<ViewFn>) -> E,
        E: Element<ViewFn>,
    {
        ViewFn(Box::new(move |cx| (render)(cx).into_any()))
    }
    pub struct ViewFn(Box<dyn FnMut(&mut ViewContext<ViewFn>) -> AnyElement<ViewFn>>);
    impl gpui::Entity for ViewFn {
        type Event = ();
    }
    impl gpui::View for ViewFn {
        fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
            (self.0)(cx).adapt().into_any()
        }
    }
}
fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");
    gpui::App::new(()).unwrap().run(|cx| {
        cx.add_window(
            WindowOptions {
                bounds: gpui::platform::WindowBounds::Fixed(RectF::new(
                    vec2f(0., 0.),
                    vec2f(400., 300.),
                )),
                center: true,
                ..Default::default()
            },
            |_| view(|_| storybook(&rose_pine::moon())),
        );
        cx.platform().activate(true);
    });
}
fn storybook<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    frame()
        .text_color(black())
        .h_full()
        .w_half()
        .fill(theme.success(0.5))
        .child(button().label("Hello").click(|_, _, _| {
            ::std::io::_print(format_args!("click!\n"));
        }))
}
