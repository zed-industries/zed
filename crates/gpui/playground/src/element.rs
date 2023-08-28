use std::marker::PhantomData;

pub use crate::layout_context::LayoutContext;
pub use crate::paint_context::PaintContext;
use crate::themes::{Theme, Themed};
use anyhow::Result;
use gpui::geometry::vector::Vector2F;
pub use gpui::{Layout, LayoutId};
use smallvec::SmallVec;

pub trait Element<V: 'static>: 'static {
    type PaintState;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<(LayoutId, Self::PaintState)>
    where
        Self: Sized;

    fn paint(
        &mut self,
        view: &mut V,
        layout: &Layout,
        state: &mut Self::PaintState,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized;

    fn into_any(self) -> AnyElement<V>
    where
        Self: 'static + Sized,
    {
        AnyElement(Box::new(StatefulElement {
            element: self,
            phase: ElementPhase::Init,
        }))
    }

    fn themed(self, theme: Theme) -> Themed<V, Self>
    where
        Self: Sized,
    {
        crate::themes::Themed {
            child: self,
            theme,
            view_type: PhantomData,
        }
    }
}

/// Used to make ElementState<V, E> into a trait object, so we can wrap it in AnyElement<V>.
trait AnyStatefulElement<V> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId>;
    fn paint(&mut self, view: &mut V, parent_origin: Vector2F, cx: &mut PaintContext<V>);
}

/// A wrapper around an element that stores its layout state.
struct StatefulElement<V: 'static, E: Element<V>> {
    element: E,
    phase: ElementPhase<V, E>,
}

enum ElementPhase<V: 'static, E: Element<V>> {
    Init,
    PostLayout {
        layout_id: LayoutId,
        paint_state: E::PaintState,
    },
    PostPaint {
        layout: Layout,
        paint_state: E::PaintState,
    },
    Error(String),
}

impl<V: 'static, E: Element<V>> Default for ElementPhase<V, E> {
    fn default() -> Self {
        Self::Init
    }
}

/// We blanket-implement the object-safe ElementStateObject interface to make ElementStates into trait objects
impl<V, E: Element<V>> AnyStatefulElement<V> for StatefulElement<V, E> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId> {
        let result;
        self.phase = match self.element.layout(view, cx) {
            Ok((layout_id, paint_state)) => {
                result = Ok(layout_id);
                ElementPhase::PostLayout {
                    layout_id,
                    paint_state,
                }
            }
            Err(error) => {
                let message = error.to_string();
                result = Err(error);
                ElementPhase::Error(message)
            }
        };
        result
    }

    fn paint(&mut self, view: &mut V, parent_origin: Vector2F, cx: &mut PaintContext<V>) {
        self.phase = match std::mem::take(&mut self.phase) {
            ElementPhase::PostLayout {
                layout_id,
                mut paint_state,
            } => match cx.computed_layout(layout_id) {
                Ok(mut layout) => {
                    layout.bounds = layout.bounds + parent_origin;
                    self.element.paint(view, &layout, &mut paint_state, cx);
                    ElementPhase::PostPaint {
                        layout,
                        paint_state,
                    }
                }
                Err(error) => ElementPhase::Error(error.to_string()),
            },
            phase @ ElementPhase::Error(_) => phase,
            _ => panic!("invalid element phase to call paint"),
        };
    }
}

/// A dynamic element.
pub struct AnyElement<V>(Box<dyn AnyStatefulElement<V>>);

impl<V> AnyElement<V> {
    pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId> {
        self.0.layout(view, cx)
    }

    pub fn paint(&mut self, view: &mut V, parent_origin: Vector2F, cx: &mut PaintContext<V>) {
        self.0.paint(view, parent_origin, cx)
    }
}

pub trait ParentElement<V: 'static> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]>;

    fn child(mut self, child: impl IntoElement<V>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.into_element().into_any());
        self
    }

    fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement<V>,
        Self: Sized,
    {
        self.children_mut().extend(
            children
                .into_iter()
                .map(|child| child.into_element().into_any()),
        );
        self
    }
}

pub trait IntoElement<V: 'static> {
    type Element: Element<V>;

    fn into_element(self) -> Self::Element;
}
