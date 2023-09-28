pub use crate::ViewContext;
use anyhow::Result;
use gpui::geometry::vector::Vector2F;
pub use gpui::{Layout, LayoutId};
use smallvec::SmallVec;

pub trait Element<V: 'static>: 'static + IntoElement<V> {
    type PaintState;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Result<(LayoutId, Self::PaintState)>
    where
        Self: Sized;

    fn paint(
        &mut self,
        view: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        state: &mut Self::PaintState,
        cx: &mut ViewContext<V>,
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

    /// Applies a given function `then` to the current element if `condition` is true.
    /// This function is used to conditionally modify the element based on a given condition.
    /// If `condition` is false, it just returns the current element as it is.
    ///
    /// # Parameters
    /// - `self`: The current element
    /// - `condition`: The boolean condition based on which `then` is applied to the element.
    /// - `then`: A function that takes in the current element and returns a possibly modified element.
    ///
    /// # Return
    /// It returns the potentially modified element.
    fn when(mut self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if condition {
            self = then(self);
        }
        self
    }
}

/// Used to make ElementState<V, E> into a trait object, so we can wrap it in AnyElement<V>.
trait AnyStatefulElement<V> {
    fn layout(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> Result<LayoutId>;
    fn paint(&mut self, view: &mut V, parent_origin: Vector2F, cx: &mut ViewContext<V>);
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
    #[allow(dead_code)]
    PostPaint {
        layout: Layout,
        paint_state: E::PaintState,
    },
    Error(String),
}

impl<V: 'static, E: Element<V>> std::fmt::Debug for ElementPhase<V, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementPhase::Init => write!(f, "Init"),
            ElementPhase::PostLayout { layout_id, .. } => {
                write!(f, "PostLayout with layout id: {:?}", layout_id)
            }
            ElementPhase::PostPaint { layout, .. } => {
                write!(f, "PostPaint with layout: {:?}", layout)
            }
            ElementPhase::Error(err) => write!(f, "Error: {}", err),
        }
    }
}

impl<V: 'static, E: Element<V>> Default for ElementPhase<V, E> {
    fn default() -> Self {
        Self::Init
    }
}

/// We blanket-implement the object-safe ElementStateObject interface to make ElementStates into trait objects
impl<V, E: Element<V>> AnyStatefulElement<V> for StatefulElement<V, E> {
    fn layout(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> Result<LayoutId> {
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

    fn paint(&mut self, view: &mut V, parent_origin: Vector2F, cx: &mut ViewContext<V>) {
        self.phase = match std::mem::take(&mut self.phase) {
            ElementPhase::PostLayout {
                layout_id,
                mut paint_state,
            } => match cx.computed_layout(layout_id) {
                Ok(layout) => {
                    self.element
                        .paint(view, parent_origin, &layout, &mut paint_state, cx);
                    ElementPhase::PostPaint {
                        layout,
                        paint_state,
                    }
                }
                Err(error) => ElementPhase::Error(error.to_string()),
            },
            ElementPhase::PostPaint {
                layout,
                mut paint_state,
            } => {
                self.element
                    .paint(view, parent_origin, &layout, &mut paint_state, cx);
                ElementPhase::PostPaint {
                    layout,
                    paint_state,
                }
            }
            phase @ ElementPhase::Error(_) => phase,

            phase @ _ => {
                panic!("invalid element phase to call paint: {:?}", phase);
            }
        };
    }
}

/// A dynamic element.
pub struct AnyElement<V>(Box<dyn AnyStatefulElement<V>>);

impl<V> AnyElement<V> {
    pub fn layout(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> Result<LayoutId> {
        self.0.layout(view, cx)
    }

    pub fn paint(&mut self, view: &mut V, parent_origin: Vector2F, cx: &mut ViewContext<V>) {
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

    // HACK: This is a temporary hack to get children working for the purposes
    // of building UI on top of the current version of gpui2.
    //
    // We'll (hopefully) be moving away from this in the future.
    fn children_any<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = AnyElement<V>>,
        Self: Sized,
    {
        self.children_mut().extend(children.into_iter());
        self
    }

    // HACK: This is a temporary hack to get children working for the purposes
    // of building UI on top of the current version of gpui2.
    //
    // We'll (hopefully) be moving away from this in the future.
    fn child_any(mut self, children: AnyElement<V>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(children);
        self
    }
}

pub trait IntoElement<V: 'static> {
    type Element: Element<V>;

    fn into_element(self) -> Self::Element;
}
