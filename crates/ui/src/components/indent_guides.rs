use std::{cmp::Ordering, ops::Range};

use gpui::{fill, point, size, Bounds, Point, UniformListDecoration, View};
use smallvec::SmallVec;

use crate::prelude::*;

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

pub struct IndentGuides {
    line_color: Color,
    indent_size: Pixels,
    compute_fn: Box<dyn Fn(Range<usize>, &mut WindowContext) -> SmallVec<[usize; 64]>>,
    render_fn: Option<
        Box<
            dyn Fn(
                SmallVec<[IndentGuideLayout; 16]>,
                Pixels,
                Pixels,
                &mut WindowContext,
            ) -> SmallVec<[RenderedIndentGuide; 16]>,
        >,
    >,
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
                SmallVec<[IndentGuideLayout; 16]>,
                Pixels,
                Pixels,
                &mut WindowContext,
            ) -> SmallVec<[RenderedIndentGuide; 16]>
            + 'static,
    ) -> Self {
        let render_fn = move |layout, indent_size, item_height, cx: &mut WindowContext| {
            view.update(cx, |this, cx| {
                render_fn(this, layout, indent_size, item_height, cx)
            })
        };
        self.render_fn = Some(Box::new(render_fn));
        self
    }
}

pub struct RenderedIndentGuide {
    pub bounds: Bounds<Pixels>,
    pub color: Color,
}

pub struct IndentGuidesLayoutState {
    indent_guides: SmallVec<[RenderedIndentGuide; 16]>,
}

impl Into<UniformListDecoration<IndentGuidesLayoutState>> for IndentGuides {
    fn into(self) -> UniformListDecoration<IndentGuidesLayoutState> {
        let line_color = self.line_color;
        let indent_size = self.indent_size;
        let compute_fn = self.compute_fn;
        let render_fn = self.render_fn;

        UniformListDecoration {
            prepaint_fn: Box::new(move |visible_range, bounds, item_height, cx| {
                let visible_entries = &(compute_fn)(visible_range.clone(), cx);
                let indent_guides = compute_indent_guides(&visible_entries, visible_range.start);

                let mut indent_guides = if let Some(ref custom_render) = render_fn {
                    custom_render(indent_guides, indent_size, item_height, cx)
                } else {
                    indent_guides
                        .into_iter()
                        .map(|layout| RenderedIndentGuide {
                            bounds: Bounds::new(
                                point(
                                    px(layout.offset.x as f32) * indent_size,
                                    px(layout.offset.y as f32) * item_height,
                                ),
                                size(px(1.), px(layout.length as f32) * item_height),
                            ),
                            color: line_color,
                        })
                        .collect()
                };

                for guide in &mut indent_guides {
                    guide.bounds.origin += bounds.origin;
                }
                IndentGuidesLayoutState { indent_guides }
            }),
            paint_fn: Box::new(|state, cx| {
                for guide in &state.indent_guides {
                    cx.paint_quad(fill(guide.bounds, guide.color.color(cx)));
                }
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct IndentGuideLayout {
    pub offset: Point<usize>,
    pub length: usize,
    pub overflows: bool,
}

fn compute_indent_guides(indents: &[usize], offset: usize) -> SmallVec<[IndentGuideLayout; 16]> {
    let mut indent_guides = SmallVec::<[IndentGuideLayout; 16]>::new();
    let mut indent_stack = SmallVec::<[IndentGuideLayout; 8]>::new();

    let mut min_depth = usize::MAX;
    for (row, &depth) in indents.iter().enumerate() {
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
                        overflows: false,
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
        guide.overflows = guide.offset.x < min_depth;
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
            expected: Vec<IndentGuideLayout>,
        ) {
            use std::collections::HashSet;
            assert_eq!(
                compute_indent_guides(input, offset)
                    .into_vec()
                    .into_iter()
                    .collect::<HashSet<_>>(),
                expected.into_iter().collect::<HashSet<_>>(),
            );
        }

        assert_compute_indent_guides(
            &[0, 1, 2, 2, 1, 0],
            0,
            vec![
                IndentGuideLayout {
                    offset: Point::new(0, 1),
                    length: 4,
                },
                IndentGuideLayout {
                    offset: Point::new(1, 2),
                    length: 2,
                },
            ],
        );

        assert_compute_indent_guides(
            &[2, 2, 2, 1, 1],
            0,
            vec![
                IndentGuideLayout {
                    offset: Point::new(0, 0),
                    length: 5,
                },
                IndentGuideLayout {
                    offset: Point::new(1, 0),
                    length: 3,
                },
            ],
        );

        assert_compute_indent_guides(
            &[1, 2, 3, 2, 1],
            0,
            vec![
                IndentGuideLayout {
                    offset: Point::new(0, 0),
                    length: 5,
                },
                IndentGuideLayout {
                    offset: Point::new(1, 1),
                    length: 3,
                },
                IndentGuideLayout {
                    offset: Point::new(2, 2),
                    length: 1,
                },
            ],
        );
    }
}
