use std::ops::Range;

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
    }
}

pub struct IndentGuides {
    line_color: Color,
    indent_size: Pixels,
    compute_fn: Box<dyn Fn(Range<usize>, &mut WindowContext) -> SmallVec<[usize; 64]>>,
}

impl IndentGuides {
    pub fn with_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }
}

pub struct IndentGuidesLayoutState {
    guides: SmallVec<[(Bounds<Pixels>, Color); 8]>,
}

impl Into<UniformListDecoration<IndentGuidesLayoutState>> for IndentGuides {
    fn into(self) -> UniformListDecoration<IndentGuidesLayoutState> {
        let line_color = self.line_color;
        let indent_size = self.indent_size;
        let compute_fn = self.compute_fn;

        UniformListDecoration {
            prepaint_fn: Box::new(move |visible_range, bounds, item_height, cx| {
                let visible_entries = &(compute_fn)(visible_range.clone(), cx);
                let indent_guides = compute_indent_guides(&visible_entries, visible_range.start);

                let guides: SmallVec<[(Bounds<Pixels>, Color); 8]> = indent_guides
                    .into_iter()
                    .map(|layout| {
                        (
                            Bounds::new(
                                bounds.origin
                                    + point(
                                        px(layout.offset.x as f32) * indent_size,
                                        px(layout.offset.y as f32) * item_height,
                                    ),
                                size(px(1.), px(layout.length as f32) * item_height),
                            ),
                            line_color,
                        )
                    })
                    .collect();

                IndentGuidesLayoutState { guides }
            }),
            paint_fn: Box::new(|state, cx| {
                for (bounds, color) in &state.guides {
                    cx.paint_quad(fill(*bounds, color.color(cx)));
                }
            }),
        }
    }
}

#[derive(Debug)]
struct IndentGuideLayout {
    offset: Point<usize>,
    length: usize,
}

fn compute_indent_guides(items: &[usize], offset: usize) -> SmallVec<[IndentGuideLayout; 8]> {
    let mut guides = SmallVec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();

    for (mut y, &depth) in items.iter().enumerate() {
        y += offset;
        while let Some(&(last_depth, start)) = stack.last() {
            if depth <= last_depth {
                if y > start + 1 {
                    guides.push(IndentGuideLayout {
                        offset: Point::new(last_depth, start + 1),
                        length: y - start - 1,
                    });
                }
                stack.pop();
            } else {
                break;
            }
        }

        if depth > 0
            && stack
                .last()
                .map(|&(last_depth, _)| depth > last_depth)
                .unwrap_or(true)
        {
            stack.push((depth, y));
        }
    }

    let total_lines = items.len();
    for (depth, start) in stack {
        if total_lines > start + 1 {
            guides.push(IndentGuideLayout {
                offset: Point::new(depth, start + 1),
                length: total_lines - start - 1,
            });
        }
    }

    guides
}
