use std::{ops::Range, rc::Rc};

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, Context, Element, ElementId, Entity, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Point, Render, Style, UniformListDecoration,
    Window, point, px, size,
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
    let entity_render = entity;

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
    drifting_element: Option<AnyElement>,
    drifting_decoration: Option<AnyElement>,
    rest_elements: SmallVec<[AnyElement; 8]>,
    rest_decorations: SmallVec<[AnyElement; 1]>,
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
        if let Some(ref mut drifting_element) = self.drifting_element {
            drifting_element.paint(window, cx);
        }
        if let Some(ref mut drifting_decoration) = self.drifting_decoration {
            drifting_decoration.paint(window, cx);
        }
        for item in self.rest_elements.iter_mut().rev() {
            item.paint(window, cx);
        }
        for item in self.rest_decorations.iter_mut() {
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

        let Some(sticky_anchor) = find_sticky_anchor(&entries, visible_range.start) else {
            return StickyItemsElement {
                drifting_element: None,
                drifting_decoration: None,
                rest_elements: SmallVec::new(),
                rest_decorations: SmallVec::new(),
            }
            .into_any_element();
        };

        let anchor_depth = sticky_anchor.entry.depth();
        let mut elements = (self.render_fn)(sticky_anchor.entry, window, cx);
        let items_count = elements.len();

        let indents: SmallVec<[usize; 8]> = (0..items_count)
            .map(|ix| anchor_depth.saturating_sub(items_count.saturating_sub(ix)))
            .collect();

        let mut last_decoration_element = None;
        let mut rest_decoration_elements = SmallVec::new();

        let expanded_width = bounds.size.width + scroll_offset.x.abs();

        let decor_available_space = size(
            AvailableSpace::Definite(expanded_width),
            AvailableSpace::Definite(bounds.size.height),
        );

        let drifting_y_offset = if sticky_anchor.drifting {
            let scroll_top = -scroll_offset.y;
            let anchor_top = item_height * (sticky_anchor.index + 1);
            let sticky_area_height = item_height * items_count;
            (anchor_top - scroll_top - sticky_area_height).min(Pixels::ZERO)
        } else {
            Pixels::ZERO
        };

        let (drifting_indent, rest_indents) = if sticky_anchor.drifting && !indents.is_empty() {
            let last = indents[indents.len() - 1];
            let rest: SmallVec<[usize; 8]> = indents[..indents.len() - 1].iter().copied().collect();
            (Some(last), rest)
        } else {
            (None, indents)
        };

        let base_origin = bounds.origin - point(px(0.), scroll_offset.y);

        for decoration in &self.decorations {
            if let Some(drifting_indent) = drifting_indent {
                let drifting_indent_vec: SmallVec<[usize; 8]> =
                    [drifting_indent].into_iter().collect();

                let sticky_origin = base_origin
                    + point(px(0.), item_height * rest_indents.len() + drifting_y_offset);
                let decoration_bounds = Bounds::new(sticky_origin, bounds.size);

                let mut drifting_dec = decoration.as_ref().compute(
                    &drifting_indent_vec,
                    decoration_bounds,
                    scroll_offset,
                    item_height,
                    window,
                    cx,
                );
                drifting_dec.layout_as_root(decor_available_space, window, cx);
                drifting_dec.prepaint_at(sticky_origin, window, cx);
                last_decoration_element = Some(drifting_dec);
            }

            if !rest_indents.is_empty() {
                let decoration_bounds = Bounds::new(base_origin, bounds.size);
                let mut rest_dec = decoration.as_ref().compute(
                    &rest_indents,
                    decoration_bounds,
                    scroll_offset,
                    item_height,
                    window,
                    cx,
                );
                rest_dec.layout_as_root(decor_available_space, window, cx);
                rest_dec.prepaint_at(bounds.origin, window, cx);
                rest_decoration_elements.push(rest_dec);
            }
        }

        let (mut drifting_element, mut rest_elements) =
            if sticky_anchor.drifting && !elements.is_empty() {
                let last = elements.pop().unwrap();
                (Some(last), elements)
            } else {
                (None, elements)
            };

        let element_available_space = size(
            AvailableSpace::Definite(expanded_width),
            AvailableSpace::Definite(item_height),
        );

        // order of prepaint is important here
        // mouse events checks hitboxes in reverse insertion order
        if let Some(ref mut drifting_element) = drifting_element {
            let sticky_origin = base_origin
                + point(
                    px(0.),
                    item_height * rest_elements.len() + drifting_y_offset,
                );

            drifting_element.layout_as_root(element_available_space, window, cx);
            drifting_element.prepaint_at(sticky_origin, window, cx);
        }

        for (ix, element) in rest_elements.iter_mut().enumerate() {
            let sticky_origin = base_origin + point(px(0.), item_height * ix);

            element.layout_as_root(element_available_space, window, cx);
            element.prepaint_at(sticky_origin, window, cx);
        }

        StickyItemsElement {
            drifting_element,
            drifting_decoration: last_decoration_element,
            rest_elements,
            rest_decorations: rest_decoration_elements,
        }
        .into_any_element()
    }
}

struct StickyAnchor<T> {
    entry: T,
    index: usize,
    drifting: bool,
}

fn find_sticky_anchor<T: StickyCandidate + Clone>(
    entries: &SmallVec<[T; 8]>,
    visible_range_start: usize,
) -> Option<StickyAnchor<T>> {
    let mut iter = entries.iter().enumerate().peekable();
    while let Some((ix, current_entry)) = iter.next() {
        let depth = current_entry.depth();

        if depth < ix {
            return Some(StickyAnchor {
                entry: current_entry.clone(),
                index: visible_range_start + ix,
                drifting: false,
            });
        }

        if let Some(&(_next_ix, next_entry)) = iter.peek() {
            let next_depth = next_entry.depth();
            let next_item_outdented = next_depth + 1 == depth;

            let depth_same_as_index = depth == ix;
            let depth_greater_than_index = depth == ix + 1;

            if next_item_outdented && (depth_same_as_index || depth_greater_than_index) {
                return Some(StickyAnchor {
                    entry: current_entry.clone(),
                    index: visible_range_start + ix,
                    drifting: depth_greater_than_index,
                });
            }
        }
    }

    None
}

/// A decoration for a [`StickyItems`]. This can be used for various things,
/// such as rendering indent guides, or other visual effects.
pub trait StickyItemsDecoration {
    /// Compute the decoration element, given the visible range of list items,
    /// the bounds of the list, and the height of each item.
    fn compute(
        &self,
        indents: &SmallVec<[usize; 8]>,
        bounds: Bounds<Pixels>,
        scroll_offset: Point<Pixels>,
        item_height: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}
