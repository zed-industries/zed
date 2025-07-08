use std::{ops::Range, rc::Rc};

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, Context, Element, ElementId, Entity, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Point, Render, Style, UniformListDecoration,
    Window, fill, point, size,
};
use smallvec::SmallVec;

use crate::{
    IndentGuideColors, RenderedIndentGuide, components::indent_guides::compute_indent_guides,
};

pub trait StickyCandidate {
    fn depth(&self) -> usize;
}

#[derive(Clone)]
pub struct StickyItems<T> {
    compute_fn: Rc<dyn Fn(Range<usize>, &mut Window, &mut App) -> SmallVec<[T; 8]>>,
    render_fn: Rc<dyn Fn(T, &mut Window, &mut App) -> SmallVec<[AnyElement; 8]>>,
    indent_guides: Option<StickyIndentGuides>,
}

#[derive(Clone)]
struct StickyIndentGuides {
    colors: IndentGuideColors,
    indent_size: Pixels,
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
        indent_guides: None,
    }
}

impl<T> StickyItems<T>
where
    T: StickyCandidate + Clone + 'static,
{
    pub fn with_indent_guides(mut self, colors: IndentGuideColors, indent_size: Pixels) -> Self {
        self.indent_guides = Some(StickyIndentGuides {
            colors,
            indent_size,
        });
        self
    }
}

struct StickyItemsElement {
    elements: SmallVec<[AnyElement; 8]>,
    indent_guides: SmallVec<[RenderedIndentGuide; 12]>,
    indent_guide_colors: Option<IndentGuideColors>,
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
        if let Some(colors) = &self.indent_guide_colors {
            for guide in &self.indent_guides {
                let fill_color = if guide.is_active {
                    colors.active
                } else {
                    colors.default
                };
                window.paint_quad(fill(guide.bounds, fill_color));
            }
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
        let mut indent_guides = SmallVec::new();

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

            if let Some(indent_guide_config) = &self.indent_guides {
                let sticky_depths: SmallVec<[usize; 8]> = elements
                    .iter()
                    .enumerate()
                    .map(|(ix, _)| {
                        anchor_depth
                            .saturating_sub(items_count.saturating_sub(1).saturating_sub(ix))
                    })
                    .collect();

                let indent_guide_layouts = compute_indent_guides(&sticky_depths, 0, false);

                for layout in indent_guide_layouts {
                    let bounds = gpui::Bounds::new(
                        bounds.origin
                            + point(
                                layout.offset.x * indent_guide_config.indent_size - scroll_offset.x,
                                layout.offset.y * item_height - scroll_offset.y,
                            ),
                        size(gpui::px(1.), layout.length * item_height),
                    );

                    indent_guides.push(RenderedIndentGuide {
                        bounds,
                        layout,
                        is_active: false,
                        hitbox: None,
                    });
                }
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
            indent_guides,
            indent_guide_colors: self.indent_guides.as_ref().map(|ig| ig.colors.clone()),
        }
        .into_any_element()
    }
}
