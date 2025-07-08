use std::{ops::Range, rc::Rc};

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, Context, Element, ElementId, Entity, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Point, Render, Style, UniformListDecoration,
    Window, point, size,
};
use smallvec::SmallVec;

pub trait StickyCandidate {
    fn depth(&self) -> usize;
}

pub struct StickyItems<T> {
    compute_fn: Rc<dyn Fn(Range<usize>, &mut Window, &mut App) -> SmallVec<[T; 8]>>,
    render_fn: Rc<dyn Fn(T, &mut Window, &mut App) -> SmallVec<[AnyElement; 8]>>,
    decorations: Vec<Box<dyn StickyItemsDecoration>>,
}

pub fn sticky_items<V, T>(
    entity: Entity<V>,
    compute_fn: impl Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> SmallVec<[T; 8]>
    + 'static,
    render_fn: impl Fn(&mut V, T, &mut Window, &mut Context<V>) -> SmallVec<[AnyElement; 8]> + 'static,
) -> StickyItems<T>
where
    V: Render,
    T: StickyCandidate + Clone + 'static,
{
    let entity_compute = entity.clone();
    let entity_render = entity.clone();

    let compute_fn = Rc::new(
        move |range: Range<usize>, window: &mut Window, cx: &mut App| -> SmallVec<[T; 8]> {
            entity_compute.update(cx, |view, cx| compute_fn(view, range, window, cx))
        },
    );
    let render_fn = Rc::new(
        move |entry: T, window: &mut Window, cx: &mut App| -> SmallVec<[AnyElement; 8]> {
            entity_render.update(cx, |view, cx| render_fn(view, entry, window, cx))
        },
    );

    StickyItems {
        compute_fn,
        render_fn,
        decorations: Vec::new(),
    }
}

impl<T> StickyItems<T>
where
    T: StickyCandidate + Clone + 'static,
{
    /// Adds a decoration element to the sticky items.
    pub fn with_decoration(mut self, decoration: impl StickyItemsDecoration + 'static) -> Self {
        self.decorations.push(Box::new(decoration));
        self
    }
}

struct StickyItemsElement {
    elements: SmallVec<[AnyElement; 8]>,
    decorations: SmallVec<[AnyElement; 1]>,
}

impl IntoElement for StickyItemsElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for StickyItemsElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        // reverse so that last item is bottom most among sticky items
        for item in self.elements.iter_mut().rev() {
            item.paint(window, cx);
        }
        for item in self.decorations.iter_mut().rev() {
            item.paint(window, cx);
        }
    }
}

impl<T> UniformListDecoration for StickyItems<T>
where
    T: StickyCandidate + Clone + 'static,
{
    fn compute(
        &self,
        visible_range: Range<usize>,
        bounds: Bounds<Pixels>,
        scroll_offset: Point<Pixels>,
        item_height: Pixels,
        _item_count: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let entries = (self.compute_fn)(visible_range.clone(), window, cx);
        let mut elements = SmallVec::new();
        let mut decorations = SmallVec::new();

        let mut anchor_entry = None;
        let mut last_item_is_drifting = false;
        let mut anchor_index = None;

        let mut iter = entries.iter().enumerate().peekable();
        while let Some((ix, current_entry)) = iter.next() {
            let current_depth = current_entry.depth();
            let index_in_range = ix;

            if current_depth < index_in_range {
                anchor_entry = Some(current_entry.clone());
                break;
            }

            if let Some(&(_next_ix, next_entry)) = iter.peek() {
                let next_depth = next_entry.depth();

                if next_depth < current_depth && next_depth < index_in_range {
                    last_item_is_drifting = true;
                    anchor_index = Some(visible_range.start + ix);
                    anchor_entry = Some(current_entry.clone());
                    break;
                }
            }
        }

        if let Some(anchor_entry) = anchor_entry {
            let anchor_depth = anchor_entry.depth();
            elements = (self.render_fn)(anchor_entry, window, cx);
            let items_count = elements.len();

            let sticky_depths: SmallVec<[usize; 8]> = elements
                .iter()
                .enumerate()
                .map(|(ix, _)| {
                    anchor_depth.saturating_sub(items_count.saturating_sub(1).saturating_sub(ix))
                })
                .collect();

            for decoration in &self.decorations {
                let mut decoration = decoration.as_ref().compute(
                    sticky_depths,
                    bounds,
                    scroll_offset,
                    item_height,
                    window,
                    cx,
                );
                let available_space = size(
                    AvailableSpace::Definite(bounds.size.width),
                    AvailableSpace::Definite(bounds.size.height),
                );
                decoration.layout_as_root(available_space, window, cx);
                decoration.prepaint_at(bounds.origin, window, cx);
                decorations.push(decoration);
            }

            for (ix, element) in elements.iter_mut().enumerate() {
                let mut item_y_offset = None;
                if ix == items_count - 1 && last_item_is_drifting {
                    if let Some(anchor_index) = anchor_index {
                        let scroll_top = -scroll_offset.y;
                        let anchor_top = item_height * anchor_index;
                        let sticky_area_height = item_height * items_count;
                        item_y_offset =
                            Some((anchor_top - scroll_top - sticky_area_height).min(Pixels::ZERO));
                    };
                }

                let sticky_origin = bounds.origin
                    + point(
                        -scroll_offset.x,
                        -scroll_offset.y + item_height * ix + item_y_offset.unwrap_or(Pixels::ZERO),
                    );

                let available_space = size(
                    AvailableSpace::Definite(bounds.size.width),
                    AvailableSpace::Definite(item_height),
                );
                element.layout_as_root(available_space, window, cx);
                element.prepaint_at(sticky_origin, window, cx);
            }
        }

        StickyItemsElement {
            elements,
            decorations,
        }
        .into_any_element()
    }
}

/// A decoration for a [`StickyItems`]. This can be used for various things,
/// such as rendering indent guides, or other visual effects.
pub trait StickyItemsDecoration {
    /// Compute the decoration element, given the visible range of list items,
    /// the bounds of the list, and the height of each item.
    fn compute(
        &self,
        visible_range: Range<usize>,
        bounds: Bounds<Pixels>,
        scroll_offset: Point<Pixels>,
        item_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}
