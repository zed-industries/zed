use std::ops::Range;

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, Context, Entity, Pixels, Render, UniformListTopSlot,
    Window, point, size,
};
use smallvec::SmallVec;

pub trait StickyCandidate {
    fn depth(&self) -> usize;
}

pub struct StickyItems<T> {
    compute_fn: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<T>>,
    render_fn: Box<dyn Fn(T, &mut Window, &mut App) -> SmallVec<[AnyElement; 8]>>,
    last_item_is_drifting: bool,
    anchor_index: Option<usize>,
}

pub fn sticky_items<V, T>(
    entity: Entity<V>,
    compute_fn: impl Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> Vec<T> + 'static,
    render_fn: impl Fn(&mut V, T, &mut Window, &mut Context<V>) -> SmallVec<[AnyElement; 8]> + 'static,
) -> StickyItems<T>
where
    V: Render,
    T: StickyCandidate + Clone + 'static,
{
    let entity_compute = entity.clone();
    let entity_render = entity.clone();

    let compute_fn = Box::new(
        move |range: Range<usize>, window: &mut Window, cx: &mut App| -> Vec<T> {
            entity_compute.update(cx, |view, cx| compute_fn(view, range, window, cx))
        },
    );
    let render_fn = Box::new(
        move |entry: T, window: &mut Window, cx: &mut App| -> SmallVec<[AnyElement; 8]> {
            entity_render.update(cx, |view, cx| render_fn(view, entry, window, cx))
        },
    );
    StickyItems {
        compute_fn,
        render_fn,
        last_item_is_drifting: false,
        anchor_index: None,
    }
}

impl<T> UniformListTopSlot for StickyItems<T>
where
    T: StickyCandidate + Clone + 'static,
{
    fn compute(
        &mut self,
        visible_range: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) -> SmallVec<[AnyElement; 8]> {
        let entries = (self.compute_fn)(visible_range.clone(), window, cx);

        let mut anchor_entry = None;

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
                    self.last_item_is_drifting = true;
                    self.anchor_index = Some(visible_range.start + ix);
                    anchor_entry = Some(current_entry.clone());
                    break;
                }
            }
        }

        if let Some(anchor_entry) = anchor_entry {
            (self.render_fn)(anchor_entry, window, cx)
        } else {
            SmallVec::new()
        }
    }

    fn prepaint(
        &self,
        items: &mut SmallVec<[AnyElement; 8]>,
        bounds: Bounds<Pixels>,
        item_height: Pixels,
        scroll_offset: gpui::Point<Pixels>,
        padding: gpui::Edges<Pixels>,
        can_scroll_horizontally: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let items_count = items.len();

        for (ix, item) in items.iter_mut().enumerate() {
            let mut item_y_offset = None;
            if ix == items_count - 1 && self.last_item_is_drifting {
                if let Some(anchor_index) = self.anchor_index {
                    let scroll_top = -scroll_offset.y;
                    let anchor_top = item_height * anchor_index;
                    let sticky_area_height = item_height * items_count;
                    item_y_offset =
                        Some((anchor_top - scroll_top - sticky_area_height).min(Pixels::ZERO));
                };
            }

            let sticky_origin = bounds.origin
                + point(
                    if can_scroll_horizontally {
                        scroll_offset.x + padding.left
                    } else {
                        scroll_offset.x
                    },
                    item_height * ix + padding.top + item_y_offset.unwrap_or(Pixels::ZERO),
                );

            let available_width = if can_scroll_horizontally {
                bounds.size.width + scroll_offset.x.abs()
            } else {
                bounds.size.width
            };

            let available_space = size(
                AvailableSpace::Definite(available_width),
                AvailableSpace::Definite(item_height),
            );

            item.layout_as_root(available_space, window, cx);
            item.prepaint_at(sticky_origin, window, cx);
        }
    }

    fn paint(&self, items: &mut SmallVec<[AnyElement; 8]>, window: &mut Window, cx: &mut App) {
        // reverse so that last item is bottom most among sticky items
        for item in items.iter_mut().rev() {
            item.paint(window, cx);
        }
    }
}
