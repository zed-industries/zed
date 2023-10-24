use crate::{Element, ViewContext, AnyElement, IntoAnyElement};

pub trait TemporaryRenderTraitDontWorryAboutIt<S> {
    fn render(self, cx: &mut ViewContext<S>) -> AnyElement<S>;
}


struct IntoAnyElementTrampolineName<S, E: TemporaryRenderTraitDontWorryAboutIt<S>> {
    contents: Option<E>,
    phantom: std::marker::PhantomData<S>,
}

impl<S, E: TemporaryRenderTraitDontWorryAboutIt<S>> IntoAnyElementTrampolineName<S, E> {
    fn new(contents: E) -> Self {
        IntoAnyElementTrampolineName {
            contents: Some(contents),
            phantom: std::marker::PhantomData,

        }
    }
}

impl<S, E: TemporaryRenderTraitDontWorryAboutIt<S>> Element for IntoAnyElementTrampolineName<S, E> {
    type ViewState = S;

    type ElementState = AnyElement<S>;

    fn id(&self) -> Option<crate::ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        self.contents.take().unwrap().render(cx)
    }

    fn layout(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> crate::LayoutId {
        element_state.layout(view_state, cx)
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        view_state: &mut Self::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) {
        element_state.paint(view_state, cx);
    }
}

impl<S, E: TemporaryRenderTraitDontWorryAboutIt<S>> IntoAnyElement<S> for IntoAnyElementTrampolineName<S, E> {
    fn into_any(self) -> AnyElement<S> {
        AnyElement::new(self)
    }
}
