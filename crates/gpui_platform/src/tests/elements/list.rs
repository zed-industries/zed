use gpui::elements::list::ListItem;
use gpui::elements::List;
use gpui::elements::ListOffset;
use gpui::elements::ListState;
use gpui::elements::Orientation;
use gpui::geometry::rect::RectF;
use gpui::geometry::vector::Vector2F;
use gpui::serde_json;
use gpui::AnyElement;
use gpui::AppContext;
use gpui::Element;
use gpui::LayoutContext;
use gpui::SceneBuilder;
use gpui::SizeConstraint;
use gpui::View;
use gpui::ViewContext;
use gpui::{elements::Empty, geometry::vector::vec2f, Entity, PaintContext};
use rand::prelude::*;
use std::cell::RefCell;
use std::env;
use std::ops::Range;
use std::rc::Rc;
#[crate::test(self)]
fn test_layout(cx: &mut AppContext) {
    cx.add_window(Default::default(), |cx| {
        let mut view = TestView;
        let constraint = SizeConstraint::new(vec2f(0., 0.), vec2f(100., 40.));
        let elements = Rc::new(RefCell::new(vec![(0, 20.), (1, 30.), (2, 100.)]));
        let state = ListState::new(elements.borrow().len(), Orientation::Top, 1000.0, {
            let elements = elements.clone();
            move |_, ix, _| {
                let (id, height) = elements.borrow()[ix];
                TestElement::new(id, height).into_any()
            }
        });

        let mut list = List::new(state.clone());
        let mut new_parents = Default::default();
        let mut notify_views_if_parents_change = Default::default();
        let mut layout_cx = LayoutContext::new(
            cx,
            &mut new_parents,
            &mut notify_views_if_parents_change,
            false,
        );
        let (size, _) = list.layout(constraint, &mut view, &mut layout_cx);
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().items.summary().clone(),
            ListItemSummary {
                count: 3,
                rendered_count: 3,
                unrendered_count: 0,
                height: 150.
            }
        );

        state.0.borrow_mut().scroll(
            &ListOffset {
                item_ix: 0,
                offset_in_item: 0.,
            },
            40.,
            vec2f(0., -54.),
            true,
            &mut view,
            cx,
        );

        let mut layout_cx = LayoutContext::new(
            cx,
            &mut new_parents,
            &mut notify_views_if_parents_change,
            false,
        );
        let (_, logical_scroll_top) = list.layout(constraint, &mut view, &mut layout_cx);
        assert_eq!(
            logical_scroll_top,
            ListOffset {
                item_ix: 2,
                offset_in_item: 4.
            }
        );
        assert_eq!(state.0.borrow().scroll_top(&logical_scroll_top), 54.);

        elements.borrow_mut().splice(1..2, vec![(3, 40.), (4, 50.)]);
        elements.borrow_mut().push((5, 60.));
        state.splice(1..2, 2);
        state.splice(4..4, 1);
        assert_eq!(
            state.0.borrow().items.summary().clone(),
            ListItemSummary {
                count: 5,
                rendered_count: 2,
                unrendered_count: 3,
                height: 120.
            }
        );

        let mut layout_cx = LayoutContext::new(
            cx,
            &mut new_parents,
            &mut notify_views_if_parents_change,
            false,
        );
        let (size, logical_scroll_top) = list.layout(constraint, &mut view, &mut layout_cx);
        assert_eq!(size, vec2f(100., 40.));
        assert_eq!(
            state.0.borrow().items.summary().clone(),
            ListItemSummary {
                count: 5,
                rendered_count: 5,
                unrendered_count: 0,
                height: 270.
            }
        );
        assert_eq!(
            logical_scroll_top,
            ListOffset {
                item_ix: 3,
                offset_in_item: 4.
            }
        );
        assert_eq!(state.0.borrow().scroll_top(&logical_scroll_top), 114.);

        view
    });
}

#[crate::test(self, iterations = 10)]
fn test_random(cx: &mut AppContext, mut rng: StdRng) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    cx.add_window(Default::default(), |cx| {
        let mut view = TestView;

        let mut next_id = 0;
        let elements = Rc::new(RefCell::new(
            (0..rng.gen_range(0..=20))
                .map(|_| {
                    let id = next_id;
                    next_id += 1;
                    (id, rng.gen_range(0..=200) as f32 / 2.0)
                })
                .collect::<Vec<_>>(),
        ));
        let orientation = *[Orientation::Top, Orientation::Bottom]
            .choose(&mut rng)
            .unwrap();
        let overdraw = rng.gen_range(1..=100) as f32;

        let state = ListState::new(elements.borrow().len(), orientation, overdraw, {
            let elements = elements.clone();
            move |_, ix, _| {
                let (id, height) = elements.borrow()[ix];
                TestElement::new(id, height).into_any()
            }
        });

        let mut width = rng.gen_range(0..=2000) as f32 / 2.;
        let mut height = rng.gen_range(0..=2000) as f32 / 2.;
        log::info!("orientation: {:?}", orientation);
        log::info!("overdraw: {}", overdraw);
        log::info!("elements: {:?}", elements.borrow());
        log::info!("size: ({:?}, {:?})", width, height);
        log::info!("==================");

        let mut last_logical_scroll_top = None;
        for _ in 0..operations {
            match rng.gen_range(0..=100) {
                0..=29 if last_logical_scroll_top.is_some() => {
                    let delta = vec2f(0., rng.gen_range(-overdraw..=overdraw));
                    log::info!(
                        "Scrolling by {:?}, previous scroll top: {:?}",
                        delta,
                        last_logical_scroll_top.unwrap()
                    );
                    state.0.borrow_mut().scroll(
                        last_logical_scroll_top.as_ref().unwrap(),
                        height,
                        delta,
                        true,
                        &mut view,
                        cx,
                    );
                }
                30..=34 => {
                    width = rng.gen_range(0..=2000) as f32 / 2.;
                    log::info!("changing width: {:?}", width);
                }
                35..=54 => {
                    height = rng.gen_range(0..=1000) as f32 / 2.;
                    log::info!("changing height: {:?}", height);
                }
                _ => {
                    let mut elements = elements.borrow_mut();
                    let end_ix = rng.gen_range(0..=elements.len());
                    let start_ix = rng.gen_range(0..=end_ix);
                    let new_elements = (0..rng.gen_range(0..10))
                        .map(|_| {
                            let id = next_id;
                            next_id += 1;
                            (id, rng.gen_range(0..=200) as f32 / 2.)
                        })
                        .collect::<Vec<_>>();
                    log::info!("splice({:?}, {:?})", start_ix..end_ix, new_elements);
                    state.splice(start_ix..end_ix, new_elements.len());
                    elements.splice(start_ix..end_ix, new_elements);
                    for (ix, item) in state.0.borrow().items.cursor::<()>().enumerate() {
                        if let ListItem::Rendered(element) = item {
                            let (expected_id, _) = elements[ix];
                            element.borrow().with_metadata(|metadata: Option<&usize>| {
                                assert_eq!(*metadata.unwrap(), expected_id);
                            });
                        }
                    }
                }
            }

            let mut list = List::new(state.clone());
            let window_size = vec2f(width, height);
            let mut new_parents = Default::default();
            let mut notify_views_if_parents_change = Default::default();
            let mut layout_cx = LayoutContext::new(
                cx,
                &mut new_parents,
                &mut notify_views_if_parents_change,
                false,
            );
            let (size, logical_scroll_top) = list.layout(
                SizeConstraint::new(vec2f(0., 0.), window_size),
                &mut view,
                &mut layout_cx,
            );
            assert_eq!(size, window_size);
            last_logical_scroll_top = Some(logical_scroll_top);

            let state = state.0.borrow();
            log::info!("items {:?}", state.items.items(&()));

            let scroll_top = state.scroll_top(&logical_scroll_top);
            let rendered_top = (scroll_top - overdraw).max(0.);
            let rendered_bottom = scroll_top + height + overdraw;
            let mut item_top = 0.;

            log::info!(
                "rendered top {:?}, rendered bottom {:?}, scroll top {:?}",
                rendered_top,
                rendered_bottom,
                scroll_top,
            );

            let mut first_rendered_element_top = None;
            let mut last_rendered_element_bottom = None;
            assert_eq!(state.items.summary().count, elements.borrow().len());
            for (ix, item) in state.items.cursor::<()>().enumerate() {
                match item {
                    ListItem::Unrendered => {
                        let item_bottom = item_top;
                        assert!(item_bottom <= rendered_top || item_top >= rendered_bottom);
                        item_top = item_bottom;
                    }
                    ListItem::Removed(height) => {
                        let (id, expected_height) = elements.borrow()[ix];
                        assert_eq!(
                            *height, expected_height,
                            "element {} height didn't match",
                            id
                        );
                        let item_bottom = item_top + height;
                        assert!(item_bottom <= rendered_top || item_top >= rendered_bottom);
                        item_top = item_bottom;
                    }
                    ListItem::Rendered(element) => {
                        let (expected_id, expected_height) = elements.borrow()[ix];
                        let element = element.borrow();
                        element.with_metadata(|metadata: Option<&usize>| {
                            assert_eq!(*metadata.unwrap(), expected_id);
                        });
                        assert_eq!(element.size().y(), expected_height);
                        let item_bottom = item_top + element.size().y();
                        first_rendered_element_top.get_or_insert(item_top);
                        last_rendered_element_bottom = Some(item_bottom);
                        assert!(item_bottom > rendered_top || item_top < rendered_bottom);
                        item_top = item_bottom;
                    }
                }
            }

            match orientation {
                Orientation::Top => {
                    if let Some(first_rendered_element_top) = first_rendered_element_top {
                        assert!(first_rendered_element_top <= scroll_top);
                    }
                }
                Orientation::Bottom => {
                    if let Some(last_rendered_element_bottom) = last_rendered_element_bottom {
                        assert!(last_rendered_element_bottom >= scroll_top + height);
                    }
                }
            }
        }

        view
    });
}

struct TestView;

impl Entity for TestView {
    type Event = ();
}

impl View for TestView {
    fn ui_name() -> &'static str {
        "TestView"
    }

    fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
        Empty::new().into_any()
    }
}

struct TestElement {
    id: usize,
    size: Vector2F,
}

impl TestElement {
    fn new(id: usize, height: f32) -> Self {
        Self {
            id,
            size: vec2f(100., height),
        }
    }
}

impl<V: 'static> Element<V> for TestElement {
    type LayoutState = ();
    type PaintState = ();

    fn layout(&mut self, _: SizeConstraint, _: &mut V, _: &mut LayoutContext<V>) -> (Vector2F, ()) {
        (self.size, ())
    }

    fn paint(
        &mut self,
        _: &mut SceneBuilder,
        _: RectF,
        _: RectF,
        _: &mut (),
        _: &mut V,
        _: &mut PaintContext<V>,
    ) {
        unimplemented!()
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        unimplemented!()
    }

    fn debug(&self, _: RectF, _: &(), _: &(), _: &V, _: &ViewContext<V>) -> serde_json::Value {
        self.id.into()
    }

    fn metadata(&self) -> Option<&dyn std::any::Any> {
        Some(&self.id)
    }
}
