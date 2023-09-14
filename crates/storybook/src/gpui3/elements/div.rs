use super::{
    Element, IntoAnyElement, Layout, LayoutId, ParentElement, PhantomData, Result, ViewContext,
};

pub struct Div<S>(PhantomData<S>);

impl<S: 'static> Element for Div<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        todo!()
    }

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        todo!()
    }
}

impl<S> ParentElement<S> for Div<S> {
    fn child(self, child: impl IntoAnyElement<S>) -> Self {
        todo!()
    }
}

pub fn div<S>() -> Div<S> {
    todo!()
}
