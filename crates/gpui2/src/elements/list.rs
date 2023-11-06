use std::ops::Range;

use smallvec::SmallVec;

use crate::{AnyElement, Component, Element, ElementId, StyleRefinement, ViewContext};

// We want to support uniform and non-uniform height
// We need to make the ID mandatory, to replace the 'state' field
// Previous implementation measured the first element as early as possible

fn list<'a, Id, V, Iter, C>(
    id: Id,
    f: impl 'static + FnOnce(&'a mut V, Range<usize>, &'a mut ViewContext<V>) -> Iter,
) -> List<V>
where
    Id: Into<ElementId>,
    V: 'static,
    Iter: 'a + Iterator<Item = C>,
    C: Component<V>,
{
    List {
        id: id.into(),
        render_items: Box::new(|view, visible_range, cx| {
            f(view, visible_range, cx)
                .map(|element| element.render())
                .collect()
        }),
    }
}

struct List<V> {
    id: ElementId,
    render_items: Box<
        dyn for<'a> FnOnce(
            &'a mut V,
            Range<usize>,
            &'a mut ViewContext<V>,
        ) -> SmallVec<[AnyElement<V>; 64]>,
    >,
}

impl<V> List<V> {}

// #[derive(Debug)]
// pub enum ScrollTarget {
//     Show(usize),
//     Center(usize),
// }

#[derive(Default)]
struct ListState {
    scroll_top: f32,
    style: StyleRefinement,
    // todo
    // scroll_to: Option<ScrollTarget>,
}
impl<V: 'static> Element<V> for List<V> {
    type ElementState = ListState;

    fn id(&self) -> Option<crate::ElementId> {
        Some(self.id)
    }

    fn initialize(
        &mut self,
        _: &mut V,
        element_state: Option<Self::ElementState>,
        _: &mut crate::ViewContext<V>,
    ) -> Self::ElementState {
        let element_state = element_state.unwrap_or_default();
        element_state
    }

    fn layout(
        &mut self,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<V>,
    ) -> crate::LayoutId {
        todo!()
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut crate::ViewContext<V>,
    ) {
    }
}
