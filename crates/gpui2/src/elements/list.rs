use std::{cmp, ops::Range};

use smallvec::SmallVec;

use crate::{
    point, px, AnyElement, AvailableSpace, BorrowWindow, Bounds, Component, Element, ElementId,
    LayoutId, Pixels, Size, StyleRefinement, Styled, ViewContext,
};

// We want to support uniform and non-uniform height
// We need to make the ID mandatory, to replace the 'state' field
// Previous implementation measured the first element as early as possible

pub fn list<Id, V, C>(
    id: Id,
    item_count: usize,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> SmallVec<[C; 64]>,
) -> List<V>
where
    Id: Into<ElementId>,
    V: 'static,
    C: Component<V>,
{
    List {
        id: id.into(),
        style: Default::default(),
        item_count,
        render_items: Box::new(move |view, visible_range, cx| {
            f(view, visible_range, cx)
                .into_iter()
                .map(|component| component.render())
                .collect()
        }),
    }
}

pub struct List<V> {
    id: ElementId,
    style: StyleRefinement,
    item_count: usize,
    render_items: Box<
        dyn for<'a> Fn(
            &'a mut V,
            Range<usize>,
            &'a mut ViewContext<V>,
        ) -> SmallVec<[AnyElement<V>; 64]>,
    >,
}

// #[derive(Debug)]
// pub enum ScrollTarget {
//     Show(usize),
//     Center(usize),
// }

#[derive(Default)]
pub struct ListState {
    scroll_top: f32,
    // todo
    // scroll_to: Option<ScrollTarget>,
}

impl<V: 'static> Styled for List<V> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<V: 'static> Element<V> for List<V> {
    type ElementState = ListState;

    fn id(&self) -> Option<crate::ElementId> {
        Some(self.id.clone())
    }

    fn initialize(
        &mut self,
        _: &mut V,
        element_state: Option<Self::ElementState>,
        _: &mut ViewContext<V>,
    ) -> Self::ElementState {
        let element_state = element_state.unwrap_or_default();
        element_state
    }

    fn layout(
        &mut self,
        _view_state: &mut V,
        _element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        cx.request_layout(&self.computed_style(), None)
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        let style = self.computed_style();
        style.paint(bounds, cx);

        let border = style.border_widths.to_pixels(cx.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.lower_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        if self.item_count > 0 {
            let item_height = self.measure_item_height(view_state, padded_bounds, cx);
            let visible_item_count = (padded_bounds.size.height / item_height).ceil() as usize;
            let visible_range = 0..cmp::min(visible_item_count, self.item_count);

            let mut items = (self.render_items)(view_state, visible_range, cx);

            dbg!(items.len(), self.item_count, visible_item_count);

            for (ix, item) in items.iter_mut().enumerate() {
                item.initialize(view_state, cx);

                let layout_id = item.layout(view_state, cx);
                cx.compute_layout(
                    layout_id,
                    Size {
                        width: AvailableSpace::Definite(bounds.size.width),
                        height: AvailableSpace::Definite(item_height),
                    },
                );
                let offset = padded_bounds.origin + point(px(0.), item_height * ix);
                cx.with_element_offset(Some(offset), |cx| item.paint(view_state, cx))
            }
        }
    }
}

impl<V> List<V> {
    fn measure_item_height(
        &self,
        view_state: &mut V,
        list_bounds: Bounds<Pixels>,
        cx: &mut ViewContext<V>,
    ) -> Pixels {
        let mut items = (self.render_items)(view_state, 0..1, cx);
        debug_assert!(items.len() == 1);
        let mut item_to_measure = items.pop().unwrap();
        item_to_measure.initialize(view_state, cx);
        let layout_id = item_to_measure.layout(view_state, cx);
        cx.compute_layout(
            layout_id,
            Size {
                width: AvailableSpace::Definite(list_bounds.size.width),
                height: AvailableSpace::MinContent,
            },
        );
        cx.layout_bounds(layout_id).size.height
    }
}

impl<V: 'static> Component<V> for List<V> {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}
