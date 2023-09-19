use super::{
    Element, IntoAnyElement, Layout, LayoutId, ParentElement, PhantomData, Result, ViewContext,
};

pub struct Div<S>(PhantomData<S>);

impl<S: 'static> Element for Div<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        todo!()
    }

    fn paint(
        &mut self,
        _layout: Layout,
        _state: &mut Self::State,
        _frame_state: &mut Self::FrameState,
        _cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        todo!()
    }
}

impl<S> ParentElement<S> for Div<S> {
    fn child(self, _child: impl IntoAnyElement<S>) -> Self {
        todo!()
    }
}

pub fn div<S>() -> Div<S> {
    todo!()
}
