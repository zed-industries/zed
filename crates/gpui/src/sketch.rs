use crate::{AnyElement, AnyModel, IntoElement, ViewContext, WindowContext};
use std::{any::Any, marker::PhantomData, sync::Arc};

pub struct Model<M> {
    state_type: PhantomData<M>,
}

pub trait Context {
    type ModelContext<'a, M>;

    fn update<M, F, R>(&mut self, model: &Model<M>, update: F) -> R
    where
        M: 'static,
        F: for<'a> FnOnce(&mut M, &mut Self::ModelContext<'a, M>) -> R;
}

impl Context for WindowContext<'_> {
    type ModelContext<'a, M> = ViewContext<'a, M>;

    fn update<M, F, R>(&mut self, model: &Model<M>, update: F) -> R
    where
        M: 'static,
        F: for<'a> FnOnce(&mut M, &mut Self::ModelContext<'a, M>) -> R,
    {
        todo!()
    }
}

impl<M> Model<M> {
    pub fn update<C, F, R>(&self, cx: &mut C, update: F) -> R
    where
        C: Context,
        F: for<'a> FnOnce(&mut M, &mut C::ModelContext<'a, M>) -> R,
    {
        todo!()
    }
}

/// A view is the combination of a model with a compatible render function for that model.
pub struct View<M, P> {
    /// A handle to the state we will render
    pub model: Model<M>,
    /// A recipe for displaying the state based on properties
    pub component: Arc<dyn StatefulComponent<M, P>>,
}

impl<M: 'static, P: 'static> View<M, P> {
    /// Creates a new `View` with the specified model and render function.
    pub fn new<F, E>(model: Model<M>, render: F) -> Self
    where
        F: 'static + Fn(&mut M, P, &mut ViewContext<M>) -> E,
        E: IntoElement,
    {
        View {
            model,
            component: Arc::new(
                move |model: &mut M, props: P, cx: &mut ViewContext<'_, M>| {
                    render(model, props, cx).into_any_element()
                },
            ),
        }
    }

    pub fn render(&self, props: P, cx: &mut WindowContext) -> AnyElement {
        self.model
            .update(cx, |model, cx| self.component.render(model, props, cx))
    }
}

/// A mapping from properties P to an element tree.
pub trait Component<P>: 'static {
    /// Render the properties
    fn render(&self, props: P, cx: &mut WindowContext) -> AnyElement;
}

/// A mapping from a stateful model M and properties P to an element tree.
pub trait StatefulComponent<M, P>: 'static {
    /// Render the model with the given properties
    fn render(&self, model: &mut M, props: P, cx: &mut ViewContext<M>) -> AnyElement;
}

impl<P, F> Component<P> for F
where
    F: Fn(P, &mut WindowContext) -> AnyElement + 'static,
{
    fn render(&self, props: P, cx: &mut WindowContext) -> AnyElement {
        (self)(props, cx)
    }
}

impl<M, P, F> StatefulComponent<M, P> for F
where
    F: for<'a, 'b, 'c> Fn(&'a mut M, P, &'b mut ViewContext<'c, M>) -> AnyElement + 'static,
{
    fn render(&self, model: &mut M, props: P, cx: &mut ViewContext<M>) -> AnyElement {
        (self)(model, props, cx)
    }
}

/// A dynamically typed view. It can be rendered with props P or downcast back to a typed view.
pub struct AnyView<P> {
    model: AnyModel,
    /// An upcasted render function that takes the dynamic reference.
    render: Arc<dyn Fn(&AnyModel, P, &mut WindowContext) -> AnyElement>,
    /// The original render function to enable downcasting to a View.
    typed_render: Arc<dyn Any>,
}
