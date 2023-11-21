use crate::{
    point, px, size, AnyElement, AvailableSpace, Bounds, Element, ElementId, InteractiveElement,
    InteractiveElementState, Interactivity, LayoutId, Pixels, Point, Render, RenderOnce, Size,
    StyleRefinement, Styled, View, ViewContext, WindowContext,
};
use smallvec::SmallVec;
use std::{cell::RefCell, cmp, ops::Range, rc::Rc};
use taffy::style::Overflow;

/// uniform_list provides lazy rendering for a set of items that are of uniform height.
/// When rendered into a container with overflow-y: hidden and a fixed (or max) height,
/// uniform_list will only render the visibile subset of items.
pub fn uniform_list<I, R, V>(
    view: View<V>,
    id: I,
    item_count: usize,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> Vec<R>,
) -> UniformList
where
    I: Into<ElementId>,
    R: RenderOnce,
    V: Render,
{
    let id = id.into();
    let mut style = StyleRefinement::default();
    style.overflow.y = Some(Overflow::Hidden);

    let render_range = move |range, cx: &mut WindowContext| {
        view.update(cx, |this, cx| {
            f(this, range, cx)
                .into_iter()
                .map(|component| component.render_into_any())
                .collect()
        })
    };

    UniformList {
        id: id.clone(),
        style,
        item_count,
        item_to_measure_index: 0,
        render_items: Box::new(render_range),
        interactivity: Interactivity {
            element_id: Some(id.into()),
            ..Default::default()
        },
        scroll_handle: None,
    }
}

pub struct UniformList {
    id: ElementId,
    style: StyleRefinement,
    item_count: usize,
    item_to_measure_index: usize,
    render_items:
        Box<dyn for<'a> Fn(Range<usize>, &'a mut WindowContext) -> SmallVec<[AnyElement; 64]>>,
    interactivity: Interactivity,
    scroll_handle: Option<UniformListScrollHandle>,
}

#[derive(Clone, Default)]
pub struct UniformListScrollHandle(Rc<RefCell<Option<ScrollHandleState>>>);

#[derive(Clone, Debug)]
struct ScrollHandleState {
    item_height: Pixels,
    list_height: Pixels,
    scroll_offset: Rc<RefCell<Point<Pixels>>>,
}

impl UniformListScrollHandle {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }

    pub fn scroll_to_item(&self, ix: usize) {
        if let Some(state) = &*self.0.borrow() {
            let mut scroll_offset = state.scroll_offset.borrow_mut();
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

impl Styled for UniformList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[derive(Default)]
pub struct UniformListState {
    interactive: InteractiveElementState,
    item_size: Size<Pixels>,
}

impl Element for UniformList {
    type State = UniformListState;

    fn layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let max_items = self.item_count;
        let rem_size = cx.rem_size();
        let item_size = state
            .as_ref()
            .map(|s| s.item_size)
            .unwrap_or_else(|| self.measure_item(None, cx));

        let (layout_id, interactive) =
            self.interactivity
                .layout(state.map(|s| s.interactive), cx, |style, cx| {
                    cx.request_measured_layout(
                        style,
                        rem_size,
                        move |known_dimensions: Size<Option<Pixels>>,
                              available_space: Size<AvailableSpace>| {
                            let desired_height = item_size.height * max_items;
                            let width =
                                known_dimensions
                                    .width
                                    .unwrap_or(match available_space.width {
                                        AvailableSpace::Definite(x) => x,
                                        AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                            item_size.width
                                        }
                                    });
                            let height = match available_space.height {
                                AvailableSpace::Definite(x) => desired_height.min(x),
                                AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                    desired_height
                                }
                            };
                            size(width, height)
                        },
                    )
                });

        let element_state = UniformListState {
            interactive,
            item_size,
        };

        (layout_id, element_state)
    }

    fn paint(
        self,
        bounds: Bounds<crate::Pixels>,
        element_state: &mut Self::State,
        cx: &mut WindowContext,
    ) {
        let style =
            self.interactivity
                .compute_style(Some(bounds), &mut element_state.interactive, cx);
        let border = style.border_widths.to_pixels(cx.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.lower_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let item_size = element_state.item_size;
        let content_size = Size {
            width: padded_bounds.size.width,
            height: item_size.height * self.item_count,
        };

        let shared_scroll_offset = element_state
            .interactive
            .scroll_offset
            .get_or_insert_with(Rc::default)
            .clone();

        let item_height = self.measure_item(Some(padded_bounds.size.width), cx).height;

        self.interactivity.paint(
            bounds,
            content_size,
            &mut element_state.interactive,
            cx,
            |style, scroll_offset, cx| {
                let border = style.border_widths.to_pixels(cx.rem_size());
                let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

                let padded_bounds = Bounds::from_corners(
                    bounds.origin + point(border.left + padding.left, border.top + padding.top),
                    bounds.lower_right()
                        - point(border.right + padding.right, border.bottom + padding.bottom),
                );

                cx.with_z_index(style.z_index.unwrap_or(0), |cx| {
                    style.paint(bounds, cx);

                    if self.item_count > 0 {
                        if let Some(scroll_handle) = self.scroll_handle.clone() {
                            dbg!("update scroll handle", &shared_scroll_offset);
                            scroll_handle.0.borrow_mut().replace(ScrollHandleState {
                                item_height,
                                list_height: padded_bounds.size.height,
                                scroll_offset: shared_scroll_offset,
                            });
                        }
                        let visible_item_count = if item_height > px(0.) {
                            (padded_bounds.size.height / item_height).ceil() as usize + 1
                        } else {
                            0
                        };

                        let first_visible_element_ix =
                            (-scroll_offset.y / item_height).floor() as usize;
                        let visible_range = first_visible_element_ix
                            ..cmp::min(
                                first_visible_element_ix + visible_item_count,
                                self.item_count,
                            );

                        let items = (self.render_items)(visible_range.clone(), cx);
                        cx.with_z_index(1, |cx| {
                            for (item, ix) in items.into_iter().zip(visible_range) {
                                let item_origin = padded_bounds.origin
                                    + point(px(0.), item_height * ix + scroll_offset.y);
                                let available_space = size(
                                    AvailableSpace::Definite(padded_bounds.size.width),
                                    AvailableSpace::Definite(item_height),
                                );
                                item.draw(item_origin, available_space, cx);
                            }
                        });
                    }
                })
            },
        );
    }
}

impl RenderOnce for UniformList {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        Some(self.id.clone())
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

impl UniformList {
    pub fn with_width_from_item(mut self, item_index: Option<usize>) -> Self {
        self.item_to_measure_index = item_index.unwrap_or(0);
        self
    }

    fn measure_item(&self, list_width: Option<Pixels>, cx: &mut WindowContext) -> Size<Pixels> {
        if self.item_count == 0 {
            return Size::default();
        }

        let item_ix = cmp::min(self.item_to_measure_index, self.item_count - 1);
        let mut items = (self.render_items)(item_ix..item_ix + 1, cx);
        let mut item_to_measure = items.pop().unwrap();
        let available_space = size(
            list_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            AvailableSpace::MinContent,
        );
        item_to_measure.measure(available_space, cx)
    }

    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }
}

impl InteractiveElement for UniformList {
    fn interactivity(&mut self) -> &mut crate::Interactivity {
        &mut self.interactivity
    }
}
