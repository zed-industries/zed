use std::{cmp::Ordering, ops::Range};

use gpui::{fill, point, size, AnyElement, Bounds, Point, UniformListDecoration, View};
use smallvec::SmallVec;

use crate::prelude::*;

pub struct IndentGuides {
    line_color: Color,
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
}

pub fn indent_guides<V: Render>(
    view: View<V>,
    indent_size: Pixels,
    compute_fn: impl Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> SmallVec<[usize; 64]> + 'static,
    cx: &WindowContext,
) -> IndentGuides {
    let compute_indent_guides = move |range, cx: &mut WindowContext| {
        view.update(cx, |this, cx| compute_fn(this, range, cx))
    };
    IndentGuides {
        line_color: Color::Custom(cx.theme().colors().editor_indent_guide),
        indent_size,
        compute_fn: Box::new(compute_indent_guides),
        render_fn: None,
    }
}

impl IndentGuides {
    pub fn with_color(mut self, color: Color) -> Self {
        self.line_color = color;
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
    pub color: Color,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IndentGuideLayout {
    pub offset: Point<usize>,
    pub length: usize,
    pub continues_offscreen: bool,
}

mod uniform_list {
    use super::*;

    struct IndentGuidesElement {
        indent_guides: SmallVec<[RenderedIndentGuide; 16]>,
    }

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
                        color: self.line_color,
                    })
                    .collect()
            };
            for guide in &mut indent_guides {
                guide.bounds.origin += bounds.origin;
            }

            let indent_guides = IndentGuidesElement { indent_guides };
            indent_guides.into_any_element()
        }
    }

    impl Element for IndentGuidesElement {
        type RequestLayoutState = ();
        type PrepaintState = ();

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
            _cx: &mut WindowContext,
        ) -> Self::PrepaintState {
            ()
        }

        fn paint(
            &mut self,
            _id: Option<&gpui::GlobalElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            cx: &mut WindowContext,
        ) {
            for guide in &self.indent_guides {
                cx.paint_quad(fill(guide.bounds, guide.color.color(cx)));
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
