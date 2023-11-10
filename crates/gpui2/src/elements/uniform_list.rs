use crate::{
    point, px, size, AnyElement, AvailableSpace, BorrowWindow, Bounds, Component, Element,
    ElementId, ElementInteractivity, InteractiveElementState, LayoutId, Pixels, Point, Size,
    StatefulInteractive, StatefulInteractivity, StatelessInteractive, StatelessInteractivity,
    StyleRefinement, Styled, ViewContext,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::{cmp, ops::Range, sync::Arc};
use taffy::style::Overflow;

/// uniform_list provides lazy rendering for a set of items that are of uniform height.
/// When rendered into a container with overflow-y: hidden and a fixed (or max) height,
/// uniform_list will only render the visibile subset of items.
pub fn uniform_list<Id, V, C>(
    id: Id,
    item_count: usize,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> SmallVec<[C; 64]>,
) -> UniformList<V>
where
    Id: Into<ElementId>,
    V: 'static,
    C: Component<V>,
{
    let id = id.into();
    let mut style = StyleRefinement::default();
    style.overflow.y = Some(Overflow::Hidden);

    UniformList {
        id: id.clone(),
        style,
        item_count,
        render_items: Box::new(move |view, visible_range, cx| {
            f(view, visible_range, cx)
                .into_iter()
                .map(|component| component.render())
                .collect()
        }),
        interactivity: StatefulInteractivity::new(id, StatelessInteractivity::default()),
        scroll_handle: None,
    }
}

pub struct UniformList<V: 'static> {
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
    interactivity: StatefulInteractivity<V>,
    scroll_handle: Option<UniformListScrollHandle>,
}

#[derive(Clone, Default)]
pub struct UniformListScrollHandle(Arc<Mutex<Option<ScrollHandleState>>>);

#[derive(Clone, Debug)]
struct ScrollHandleState {
    item_height: Pixels,
    list_height: Pixels,
    scroll_offset: Arc<Mutex<Point<Pixels>>>,
}

impl UniformListScrollHandle {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    pub fn scroll_to_item(&self, ix: usize) {
        if let Some(state) = &*self.0.lock() {
            let mut scroll_offset = state.scroll_offset.lock();
            let item_top = state.item_height * ix;
            let item_bottom = item_top + state.item_height;
            let scroll_top = -scroll_offset.y;
            if item_top < scroll_top {
                scroll_offset.y = -item_top;
            } else if item_bottom > scroll_top + state.list_height {
                scroll_offset.y = -(item_bottom - state.list_height);
            }
        }
    }
}

impl<V: 'static> Styled for UniformList<V> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[derive(Default)]
pub struct UniformListState {
    interactive: InteractiveElementState,
    item_size: Size<Pixels>,
}

impl<V: 'static> Element<V> for UniformList<V> {
    type ElementState = UniformListState;

    fn id(&self) -> Option<crate::ElementId> {
        Some(self.id.clone())
    }

    fn initialize(
        &mut self,
        view_state: &mut V,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<V>,
    ) -> Self::ElementState {
        element_state.unwrap_or_else(|| {
            let item_size = self.measure_first_item(view_state, None, cx);
            UniformListState {
                interactive: InteractiveElementState::default(),
                item_size,
            }
        })
    }

    fn layout(
        &mut self,
        _view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) -> LayoutId {
        let max_items = self.item_count;
        let item_size = element_state.item_size;
        let rem_size = cx.rem_size();

        cx.request_measured_layout(
            self.computed_style(),
            rem_size,
            move |known_dimensions: Size<Option<Pixels>>, available_space: Size<AvailableSpace>| {
                let desired_height = item_size.height * max_items;
                let width = known_dimensions
                    .width
                    .unwrap_or(match available_space.width {
                        AvailableSpace::Definite(x) => x,
                        AvailableSpace::MinContent | AvailableSpace::MaxContent => item_size.width,
                    });
                let height = match available_space.height {
                    AvailableSpace::Definite(x) => desired_height.min(x),
                    AvailableSpace::MinContent | AvailableSpace::MaxContent => desired_height,
                };
                size(width, height)
            },
        )
    }

    fn paint(
        &mut self,
        bounds: crate::Bounds<crate::Pixels>,
        view_state: &mut V,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<V>,
    ) {
        let style = self.computed_style();

        let border = style.border_widths.to_pixels(cx.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.lower_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        cx.with_z_index(style.z_index.unwrap_or(0), |cx| {
            style.paint(bounds, cx);

            let content_size;
            if self.item_count > 0 {
                let item_height = self
                    .measure_first_item(view_state, Some(padded_bounds.size.width), cx)
                    .height;
                if let Some(scroll_handle) = self.scroll_handle.clone() {
                    scroll_handle.0.lock().replace(ScrollHandleState {
                        item_height,
                        list_height: padded_bounds.size.height,
                        scroll_offset: element_state.interactive.track_scroll_offset(),
                    });
                }
                let visible_item_count = if item_height > px(0.) {
                    (padded_bounds.size.height / item_height).ceil() as usize + 1
                } else {
                    0
                };
                let scroll_offset = element_state
                    .interactive
                    .scroll_offset()
                    .map_or((0.0).into(), |offset| offset.y);
                let first_visible_element_ix = (-scroll_offset / item_height).floor() as usize;
                let visible_range = first_visible_element_ix
                    ..cmp::min(
                        first_visible_element_ix + visible_item_count,
                        self.item_count,
                    );

                let mut items = (self.render_items)(view_state, visible_range.clone(), cx);

                content_size = Size {
                    width: padded_bounds.size.width,
                    height: item_height * self.item_count,
                };

                cx.with_z_index(1, |cx| {
                    for (item, ix) in items.iter_mut().zip(visible_range) {
                        let item_origin =
                            padded_bounds.origin + point(px(0.), item_height * ix + scroll_offset);
                        let available_space = size(
                            AvailableSpace::Definite(bounds.size.width),
                            AvailableSpace::Definite(item_height),
                        );
                        item.draw(item_origin, available_space, view_state, cx);
                    }
                });
            } else {
                content_size = Size {
                    width: bounds.size.width,
                    height: px(0.),
                };
            }

            let overflow = point(style.overflow.x, Overflow::Scroll);

            cx.with_z_index(0, |cx| {
                self.interactivity.paint(
                    bounds,
                    content_size,
                    overflow,
                    &mut element_state.interactive,
                    cx,
                );
            });
        })
    }
}

impl<V> UniformList<V> {
    fn measure_first_item(
        &self,
        view_state: &mut V,
        list_width: Option<Pixels>,
        cx: &mut ViewContext<V>,
    ) -> Size<Pixels> {
        let mut items = (self.render_items)(view_state, 0..1, cx);
        debug_assert_eq!(items.len(), 1);
        let mut item_to_measure = items.pop().unwrap();
        let available_space = size(
            list_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            AvailableSpace::MinContent,
        );
        item_to_measure.measure(available_space, view_state, cx)
    }

    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }
}

impl<V: 'static> StatelessInteractive<V> for UniformList<V> {
    fn stateless_interactivity(&mut self) -> &mut StatelessInteractivity<V> {
        self.interactivity.as_stateless_mut()
    }
}

impl<V: 'static> StatefulInteractive<V> for UniformList<V> {
    fn stateful_interactivity(&mut self) -> &mut StatefulInteractivity<V> {
        &mut self.interactivity
    }
}

impl<V: 'static> Component<V> for UniformList<V> {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}
