use std::ops::Range;

use gpui::{AnyElement, App, Context, Entity, Render, UniformListSticky, Window};
use smallvec::SmallVec;

pub trait StickyCandidate {
    fn depth(&self) -> usize;
}

pub struct StickyItems<T> {
    compute_fn: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<T>>,
    render_fn: Box<dyn Fn(T, &mut Window, &mut App) -> SmallVec<[AnyElement; 8]>>,
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
    }
}

impl<T> UniformListSticky for StickyItems<T>
where
    T: StickyCandidate + Clone + 'static,
{
    fn compute(
        &self,
        visible_range: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) -> (SmallVec<[AnyElement; 8]>, usize, Option<usize>, bool) {
        let entries = (self.compute_fn)(visible_range.clone(), window, cx);

        let mut iter = entries.iter().enumerate().peekable();

        let mut last_item_is_drifting = false;
        let mut marker_index = None;
        let mut marker_entry = None;

        while let Some((ix, current_entry)) = iter.next() {
            let current_depth = current_entry.depth();
            let index_in_range = ix;

            if current_depth < index_in_range {
                marker_entry = Some(current_entry.clone());
                break;
            }

            if let Some(&(_next_ix, next_entry)) = iter.peek() {
                let next_depth = next_entry.depth();

                if next_depth < current_depth && next_depth < index_in_range {
                    last_item_is_drifting = true;
                    marker_index = Some(visible_range.start + ix);
                    marker_entry = Some(current_entry.clone());
                    break;
                }
            }
        }

        let mut elements = SmallVec::new();
        if let Some(marker_entry) = marker_entry {
            elements = (self.render_fn)(marker_entry, window, cx);
        }

        let count = elements.len();
        (elements, count, marker_index, last_item_is_drifting)
    }
}
