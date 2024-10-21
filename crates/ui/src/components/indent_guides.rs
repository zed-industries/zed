#![allow(missing_docs)]
use std::{cmp::Ordering, ops::Range, rc::Rc};

use gpui::{fill, point, size, AnyElement, Bounds, Hsla, Point, UniformListDecoration, View};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct IndentGuideColors {
    pub default: Hsla,
    pub hovered: Hsla,
    pub active: Hsla,
}

pub struct IndentGuides {
    colors: IndentGuideColors,
    indent_size: Pixels,
    compute_fn: Box<dyn Fn(Range<usize>, &mut WindowContext) -> SmallVec<[usize; 64]>>,
    render_fn: Option<
        Box<
            dyn Fn(
                RenderIndentGuideParams,
                &mut WindowContext,
            ) -> SmallVec<[RenderedIndentGuide; 16]>,
        >,
    >,
    on_click: Option<Rc<dyn Fn(&IndentGuideLayout, &mut WindowContext)>>,
}

pub fn indent_guides<V: Render>(
    view: View<V>,
    indent_size: Pixels,
    colors: IndentGuideColors,
    compute_fn: impl Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> SmallVec<[usize; 64]> + 'static,
) -> IndentGuides {
    let compute_indent_guides = move |range, cx: &mut WindowContext| {
        view.update(cx, |this, cx| compute_fn(this, range, cx))
    };
    IndentGuides {
        colors,
        indent_size,
        compute_fn: Box::new(compute_indent_guides),
        render_fn: None,
        on_click: None,
    }
}

impl IndentGuides {
    pub fn on_click(
        mut self,
        on_click: impl Fn(&IndentGuideLayout, &mut WindowContext) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    pub fn with_render_fn<V: Render>(
        mut self,
        view: View<V>,
        render_fn: impl Fn(
                &mut V,
                RenderIndentGuideParams,
                &mut WindowContext,
            ) -> SmallVec<[RenderedIndentGuide; 16]>
            + 'static,
    ) -> Self {
        let render_fn = move |params, cx: &mut WindowContext| {
            view.update(cx, |this, cx| render_fn(this, params, cx))
        };
        self.render_fn = Some(Box::new(render_fn));
        self
    }
}

pub struct RenderIndentGuideParams {
    pub indent_guides: SmallVec<[IndentGuideLayout; 16]>,
    pub indent_size: Pixels,
    pub item_height: Pixels,
}

pub struct RenderedIndentGuide {
    pub bounds: Bounds<Pixels>,
    pub layout: IndentGuideLayout,
    pub is_active: bool,
    pub hitbox: Option<Bounds<Pixels>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IndentGuideLayout {
    pub offset: Point<usize>,
    pub length: usize,
    pub continues_offscreen: bool,
}

mod uniform_list {
    use gpui::{DispatchPhase, Hitbox, MouseButton, MouseDownEvent};

    use super::*;

    impl UniformListDecoration for IndentGuides {
        fn compute(
            &self,
            visible_range: Range<usize>,
            bounds: Bounds<Pixels>,
            item_height: Pixels,
            cx: &mut WindowContext,
        ) -> AnyElement {
            let mut visible_range = visible_range.clone();
            visible_range.end += 1;
            let visible_entries = &(self.compute_fn)(visible_range.clone(), cx);
            // Check if we have an additional indent that is outside of the visible range
            let includes_trailing_indent = visible_entries.len() == visible_range.len();
            let indent_guides = compute_indent_guides(
                &visible_entries,
                visible_range.start,
                includes_trailing_indent,
            );
            let mut indent_guides = if let Some(ref custom_render) = self.render_fn {
                let params = RenderIndentGuideParams {
                    indent_guides,
                    indent_size: self.indent_size,
                    item_height,
                };
                custom_render(params, cx)
            } else {
                indent_guides
                    .into_iter()
                    .map(|layout| RenderedIndentGuide {
                        bounds: Bounds::new(
                            point(
                                px(layout.offset.x as f32) * self.indent_size,
                                px(layout.offset.y as f32) * item_height,
                            ),
                            size(px(1.), px(layout.length as f32) * -item_height),
                        ),
                        layout,
                        is_active: false,
                        hitbox: None,
                    })
                    .collect()
            };
            for guide in &mut indent_guides {
                guide.bounds.origin += bounds.origin;
                if let Some(hitbox) = guide.hitbox.as_mut() {
                    hitbox.origin += bounds.origin;
                }
            }

            let indent_guides = IndentGuidesElement {
                indent_guides: Rc::new(indent_guides),
                colors: self.colors.clone(),
                on_hovered_indent_guide_click: self.on_click.clone(),
            };
            indent_guides.into_any_element()
        }
    }

    struct IndentGuidesElement {
        colors: IndentGuideColors,
        indent_guides: Rc<SmallVec<[RenderedIndentGuide; 16]>>,
        on_hovered_indent_guide_click: Option<Rc<dyn Fn(&IndentGuideLayout, &mut WindowContext)>>,
    }

    struct IndentGuidesElementPrepaintState {
        hitboxes: SmallVec<[Hitbox; 16]>,
    }

    impl Element for IndentGuidesElement {
        type RequestLayoutState = ();
        type PrepaintState = IndentGuidesElementPrepaintState;

        fn id(&self) -> Option<ElementId> {
            None
        }

        fn request_layout(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            cx: &mut WindowContext,
        ) -> (gpui::LayoutId, Self::RequestLayoutState) {
            (cx.request_layout(gpui::Style::default(), []), ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            cx: &mut WindowContext,
        ) -> Self::PrepaintState {
            let mut hitboxes = SmallVec::<[Hitbox; 16]>::new();
            for guide in self.indent_guides.as_ref().iter() {
                hitboxes.push(cx.insert_hitbox(guide.hitbox.unwrap_or(guide.bounds), true));
            }
            Self::PrepaintState { hitboxes }
        }

        fn paint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            prepaint: &mut Self::PrepaintState,
            cx: &mut WindowContext,
        ) {
            let callback = self.on_hovered_indent_guide_click.clone();
            if let Some(callback) = callback {
                cx.on_mouse_event({
                    let hitboxes = prepaint.hitboxes.clone();
                    let indent_guides = self.indent_guides.clone();
                    move |event: &MouseDownEvent, phase, cx| {
                        if phase == DispatchPhase::Bubble && event.button == MouseButton::Left {
                            let mut active_hitbox_ix = None;
                            for (i, hitbox) in hitboxes.iter().enumerate() {
                                if hitbox.is_hovered(cx) {
                                    active_hitbox_ix = Some(i);
                                    break;
                                }
                            }

                            let Some(active_hitbox_ix) = active_hitbox_ix else {
                                return;
                            };

                            let active_indent_guide = &indent_guides[active_hitbox_ix].layout;
                            callback(active_indent_guide, cx);

                            cx.stop_propagation();
                            cx.prevent_default();
                        }
                    }
                });
            }

            for (i, hitbox) in prepaint.hitboxes.iter().enumerate() {
                cx.set_cursor_style(gpui::CursorStyle::PointingHand, hitbox);
                let indent_guide = &self.indent_guides[i];
                let fill_color = if hitbox.is_hovered(cx) {
                    self.colors.hovered
                } else if indent_guide.is_active {
                    self.colors.active
                } else {
                    self.colors.default
                };

                cx.paint_quad(fill(indent_guide.bounds, fill_color));
            }
        }
    }

    impl IntoElement for IndentGuidesElement {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }
}

fn compute_indent_guides(
    indents: &[usize],
    offset: usize,
    includes_trailing_indent: bool,
) -> SmallVec<[IndentGuideLayout; 16]> {
    let mut indent_guides = SmallVec::<[IndentGuideLayout; 16]>::new();
    let mut indent_stack = SmallVec::<[IndentGuideLayout; 8]>::new();

    let mut min_depth = usize::MAX;
    for (row, &depth) in indents.iter().enumerate() {
        if includes_trailing_indent && row == indents.len() - 1 {
            continue;
        }

        let current_row = row + offset;
        let current_depth = indent_stack.len();
        if depth < min_depth {
            min_depth = depth;
        }

        match depth.cmp(&current_depth) {
            Ordering::Less => {
                for _ in 0..(current_depth - depth) {
                    if let Some(guide) = indent_stack.pop() {
                        indent_guides.push(guide);
                    }
                }
            }
            Ordering::Greater => {
                for new_depth in current_depth..depth {
                    indent_stack.push(IndentGuideLayout {
                        offset: Point::new(new_depth, current_row),
                        length: current_row,
                        continues_offscreen: false,
                    });
                }
            }
            _ => {}
        }

        for indent in indent_stack.iter_mut() {
            indent.length = current_row - indent.offset.y + 1;
        }
    }

    indent_guides.extend(indent_stack);

    for guide in indent_guides.iter_mut() {
        if includes_trailing_indent
            && guide.offset.y + guide.length == offset + indents.len().saturating_sub(1)
        {
            guide.continues_offscreen = indents
                .last()
                .map(|last_indent| guide.offset.x < *last_indent)
                .unwrap_or(false);
        }
    }

    indent_guides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_indent_guides() {
        fn assert_compute_indent_guides(
            input: &[usize],
            offset: usize,
            includes_trailing_indent: bool,
            expected: Vec<IndentGuideLayout>,
        ) {
            use std::collections::HashSet;
            assert_eq!(
                compute_indent_guides(input, offset, includes_trailing_indent)
                    .into_vec()
                    .into_iter()
                    .collect::<HashSet<_>>(),
                expected.into_iter().collect::<HashSet<_>>(),
            );
        }

        assert_compute_indent_guides(
            &[0, 1, 2, 2, 1, 0],
            0,
            false,
            vec![
                IndentGuideLayout {
                    offset: Point::new(0, 1),
                    length: 4,
                    continues_offscreen: false,
                },
                IndentGuideLayout {
                    offset: Point::new(1, 2),
                    length: 2,
                    continues_offscreen: false,
                },
            ],
        );

        assert_compute_indent_guides(
            &[2, 2, 2, 1, 1],
            0,
            false,
            vec![
                IndentGuideLayout {
                    offset: Point::new(0, 0),
                    length: 5,
                    continues_offscreen: false,
                },
                IndentGuideLayout {
                    offset: Point::new(1, 0),
                    length: 3,
                    continues_offscreen: false,
                },
            ],
        );

        assert_compute_indent_guides(
            &[1, 2, 3, 2, 1],
            0,
            false,
            vec![
                IndentGuideLayout {
                    offset: Point::new(0, 0),
                    length: 5,
                    continues_offscreen: false,
                },
                IndentGuideLayout {
                    offset: Point::new(1, 1),
                    length: 3,
                    continues_offscreen: false,
                },
                IndentGuideLayout {
                    offset: Point::new(2, 2),
                    length: 1,
                    continues_offscreen: false,
                },
            ],
        );

        assert_compute_indent_guides(
            &[0, 1, 0],
            0,
            true,
            vec![IndentGuideLayout {
                offset: Point::new(0, 1),
                length: 1,
                continues_offscreen: false,
            }],
        );

        assert_compute_indent_guides(
            &[0, 1, 1],
            0,
            true,
            vec![IndentGuideLayout {
                offset: Point::new(0, 1),
                length: 1,
                continues_offscreen: true,
            }],
        );
        assert_compute_indent_guides(
            &[0, 1, 2],
            0,
            true,
            vec![IndentGuideLayout {
                offset: Point::new(0, 1),
                length: 1,
                continues_offscreen: true,
            }],
        );
    }
}
