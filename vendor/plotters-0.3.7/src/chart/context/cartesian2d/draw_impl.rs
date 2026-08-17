use std::ops::Range;

use plotters_backend::DrawingBackend;

use crate::chart::ChartContext;
use crate::coord::{
    cartesian::{Cartesian2d, MeshLine},
    ranged1d::{KeyPointHint, Ranged},
    Shift,
};
use crate::drawing::{DrawingArea, DrawingAreaErrorKind};
use crate::element::PathElement;
use crate::style::{
    text_anchor::{HPos, Pos, VPos},
    FontTransform, ShapeStyle, TextStyle,
};

impl<'a, DB: DrawingBackend, X: Ranged, Y: Ranged> ChartContext<'a, DB, Cartesian2d<X, Y>> {
    /// The actual function that draws the mesh lines.
    /// It also returns the label that suppose to be there.
    #[allow(clippy::type_complexity)]
    fn draw_mesh_lines<FmtLabel, YH: KeyPointHint, XH: KeyPointHint>(
        &mut self,
        (r, c): (YH, XH),
        (x_mesh, y_mesh): (bool, bool),
        mesh_line_style: &ShapeStyle,
        mut fmt_label: FmtLabel,
    ) -> Result<(Vec<(i32, String)>, Vec<(i32, String)>), DrawingAreaErrorKind<DB::ErrorType>>
    where
        FmtLabel: FnMut(&X, &Y, &MeshLine<X, Y>) -> Option<String>,
    {
        let mut x_labels = vec![];
        let mut y_labels = vec![];
        let xr = self.drawing_area.as_coord_spec().x_spec();
        let yr = self.drawing_area.as_coord_spec().y_spec();
        self.drawing_area.draw_mesh(
            |b, l| {
                let draw = match l {
                    MeshLine::XMesh((x, _), _, _) => {
                        if let Some(label_text) = fmt_label(xr, yr, &l) {
                            x_labels.push((x, label_text));
                        }
                        x_mesh
                    }
                    MeshLine::YMesh((_, y), _, _) => {
                        if let Some(label_text) = fmt_label(xr, yr, &l) {
                            y_labels.push((y, label_text));
                        }
                        y_mesh
                    }
                };
                if draw {
                    l.draw(b, mesh_line_style)
                } else {
                    Ok(())
                }
            },
            r,
            c,
        )?;
        Ok((x_labels, y_labels))
    }

    fn draw_axis(
        &self,
        area: &DrawingArea<DB, Shift>,
        axis_style: Option<&ShapeStyle>,
        orientation: (i16, i16),
        inward_labels: bool,
    ) -> Result<Range<i32>, DrawingAreaErrorKind<DB::ErrorType>> {
        let (x0, y0) = self.drawing_area.get_base_pixel();
        let (tw, th) = area.dim_in_pixel();

        let mut axis_range = if orientation.0 == 0 {
            self.drawing_area.get_x_axis_pixel_range()
        } else {
            self.drawing_area.get_y_axis_pixel_range()
        };

        // At this point, the coordinate system tells us the pixel range after the translation.
        // However, we need to use the logic coordinate system for drawing.
        if orientation.0 == 0 {
            axis_range.start -= x0;
            axis_range.end -= x0;
        } else {
            axis_range.start -= y0;
            axis_range.end -= y0;
        }

        if let Some(axis_style) = axis_style {
            let mut x0 = if orientation.0 > 0 { 0 } else { tw as i32 - 1 };
            let mut y0 = if orientation.1 > 0 { 0 } else { th as i32 - 1 };
            let mut x1 = if orientation.0 >= 0 { 0 } else { tw as i32 - 1 };
            let mut y1 = if orientation.1 >= 0 { 0 } else { th as i32 - 1 };

            if inward_labels {
                if orientation.0 == 0 {
                    if y0 == 0 {
                        y0 = th as i32 - 1;
                        y1 = th as i32 - 1;
                    } else {
                        y0 = 0;
                        y1 = 0;
                    }
                } else if x0 == 0 {
                    x0 = tw as i32 - 1;
                    x1 = tw as i32 - 1;
                } else {
                    x0 = 0;
                    x1 = 0;
                }
            }

            if orientation.0 == 0 {
                x0 = axis_range.start;
                x1 = axis_range.end;
            } else {
                y0 = axis_range.start;
                y1 = axis_range.end;
            }

            area.draw(&PathElement::new(vec![(x0, y0), (x1, y1)], *axis_style))?;
        }

        Ok(axis_range)
    }

    // TODO: consider make this function less complicated
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cognitive_complexity)]
    fn draw_axis_and_labels(
        &self,
        area: Option<&DrawingArea<DB, Shift>>,
        axis_style: Option<&ShapeStyle>,
        labels: &[(i32, String)],
        label_style: &TextStyle,
        label_offset: i32,
        orientation: (i16, i16),
        axis_desc: Option<(&str, &TextStyle)>,
        tick_size: i32,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        let area = if let Some(target) = area {
            target
        } else {
            return Ok(());
        };

        let (x0, y0) = self.drawing_area.get_base_pixel();
        let (tw, th) = area.dim_in_pixel();

        /* This is the minimal distance from the axis to the box of the labels */
        let label_dist = tick_size.abs() * 2;

        /* Draw the axis and get the axis range so that we can do further label
         * and tick mark drawing */
        let axis_range = self.draw_axis(area, axis_style, orientation, tick_size < 0)?;

        /* To make the right label area looks nice, it's a little bit tricky, since for a that is
         * very long, we actually prefer left alignment instead of right alignment.
         * Otherwise, the right alignment looks better. So we estimate the max and min label width
         * So that we are able decide if we should apply right alignment for the text. */
        let label_width: Vec<_> = labels
            .iter()
            .map(|(_, text)| {
                if orientation.0 > 0 && orientation.1 == 0 && tick_size >= 0 {
                    self.drawing_area
                        .estimate_text_size(text, label_style)
                        .map(|(w, _)| w)
                        .unwrap_or(0) as i32
                } else {
                    // Don't ever do the layout estimationfor the drawing area that is either not
                    // the right one or the tick mark is inward.
                    0
                }
            })
            .collect();

        let min_width = *label_width.iter().min().unwrap_or(&1).max(&1);
        let max_width = *label_width
            .iter()
            .filter(|&&x| x < min_width * 2)
            .max()
            .unwrap_or(&min_width);
        let right_align_width = (min_width * 2).min(max_width);

        /* Then we need to draw the tick mark and the label */
        for ((p, t), w) in labels.iter().zip(label_width.into_iter()) {
            /* Make sure we are actually in the visible range */
            let rp = if orientation.0 == 0 { *p - x0 } else { *p - y0 };

            if rp < axis_range.start.min(axis_range.end)
                || axis_range.end.max(axis_range.start) < rp
            {
                continue;
            }

            let (cx, cy, h_pos, v_pos) = if tick_size >= 0 {
                match orientation {
                    // Right
                    (dx, dy) if dx > 0 && dy == 0 => {
                        if w >= right_align_width {
                            (label_dist, *p - y0, HPos::Left, VPos::Center)
                        } else {
                            (
                                label_dist + right_align_width,
                                *p - y0,
                                HPos::Right,
                                VPos::Center,
                            )
                        }
                    }
                    // Left
                    (dx, dy) if dx < 0 && dy == 0 => {
                        (tw as i32 - label_dist, *p - y0, HPos::Right, VPos::Center)
                    }
                    // Bottom
                    (dx, dy) if dx == 0 && dy > 0 => (*p - x0, label_dist, HPos::Center, VPos::Top),
                    // Top
                    (dx, dy) if dx == 0 && dy < 0 => {
                        (*p - x0, th as i32 - label_dist, HPos::Center, VPos::Bottom)
                    }
                    _ => panic!("Bug: Invalid orientation specification"),
                }
            } else {
                match orientation {
                    // Right
                    (dx, dy) if dx > 0 && dy == 0 => {
                        (tw as i32 - label_dist, *p - y0, HPos::Right, VPos::Center)
                    }
                    // Left
                    (dx, dy) if dx < 0 && dy == 0 => {
                        (label_dist, *p - y0, HPos::Left, VPos::Center)
                    }
                    // Bottom
                    (dx, dy) if dx == 0 && dy > 0 => {
                        (*p - x0, th as i32 - label_dist, HPos::Center, VPos::Bottom)
                    }
                    // Top
                    (dx, dy) if dx == 0 && dy < 0 => (*p - x0, label_dist, HPos::Center, VPos::Top),
                    _ => panic!("Bug: Invalid orientation specification"),
                }
            };

            let (text_x, text_y) = if orientation.0 == 0 {
                (cx + label_offset, cy)
            } else {
                (cx, cy + label_offset)
            };

            let label_style = &label_style.pos(Pos::new(h_pos, v_pos));
            area.draw_text(t, label_style, (text_x, text_y))?;

            if tick_size != 0 {
                if let Some(style) = axis_style {
                    let xmax = tw as i32 - 1;
                    let ymax = th as i32 - 1;
                    let (kx0, ky0, kx1, ky1) = if tick_size > 0 {
                        match orientation {
                            (dx, dy) if dx > 0 && dy == 0 => (0, *p - y0, tick_size, *p - y0),
                            (dx, dy) if dx < 0 && dy == 0 => {
                                (xmax - tick_size, *p - y0, xmax, *p - y0)
                            }
                            (dx, dy) if dx == 0 && dy > 0 => (*p - x0, 0, *p - x0, tick_size),
                            (dx, dy) if dx == 0 && dy < 0 => {
                                (*p - x0, ymax - tick_size, *p - x0, ymax)
                            }
                            _ => panic!("Bug: Invalid orientation specification"),
                        }
                    } else {
                        match orientation {
                            (dx, dy) if dx > 0 && dy == 0 => {
                                (xmax, *p - y0, xmax + tick_size, *p - y0)
                            }
                            (dx, dy) if dx < 0 && dy == 0 => (0, *p - y0, -tick_size, *p - y0),
                            (dx, dy) if dx == 0 && dy > 0 => {
                                (*p - x0, ymax, *p - x0, ymax + tick_size)
                            }
                            (dx, dy) if dx == 0 && dy < 0 => (*p - x0, 0, *p - x0, -tick_size),
                            _ => panic!("Bug: Invalid orientation specification"),
                        }
                    };
                    let line = PathElement::new(vec![(kx0, ky0), (kx1, ky1)], *style);
                    area.draw(&line)?;
                }
            }
        }

        if let Some((text, style)) = axis_desc {
            let actual_style = if orientation.0 == 0 {
                style.clone()
            } else if orientation.0 == -1 {
                style.transform(FontTransform::Rotate270)
            } else {
                style.transform(FontTransform::Rotate90)
            };

            let (x0, y0, h_pos, v_pos) = match orientation {
                // Right
                (dx, dy) if dx > 0 && dy == 0 => (tw, th / 2, HPos::Center, VPos::Top),
                // Left
                (dx, dy) if dx < 0 && dy == 0 => (0, th / 2, HPos::Center, VPos::Top),
                // Bottom
                (dx, dy) if dx == 0 && dy > 0 => (tw / 2, th, HPos::Center, VPos::Bottom),
                // Top
                (dx, dy) if dx == 0 && dy < 0 => (tw / 2, 0, HPos::Center, VPos::Top),
                _ => panic!("Bug: Invalid orientation specification"),
            };

            let actual_style = &actual_style.pos(Pos::new(h_pos, v_pos));
            area.draw_text(text, actual_style, (x0 as i32, y0 as i32))?;
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_mesh<FmtLabel, YH: KeyPointHint, XH: KeyPointHint>(
        &mut self,
        (r, c): (YH, XH),
        mesh_line_style: &ShapeStyle,
        x_label_style: &TextStyle,
        y_label_style: &TextStyle,
        fmt_label: FmtLabel,
        x_mesh: bool,
        y_mesh: bool,
        x_label_offset: i32,
        y_label_offset: i32,
        x_axis: bool,
        y_axis: bool,
        axis_style: &ShapeStyle,
        axis_desc_style: &TextStyle,
        x_desc: Option<String>,
        y_desc: Option<String>,
        x_tick_size: [i32; 2],
        y_tick_size: [i32; 2],
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        FmtLabel: FnMut(&X, &Y, &MeshLine<X, Y>) -> Option<String>,
    {
        let (x_labels, y_labels) =
            self.draw_mesh_lines((r, c), (x_mesh, y_mesh), mesh_line_style, fmt_label)?;

        for idx in 0..2 {
            self.draw_axis_and_labels(
                self.x_label_area[idx].as_ref(),
                if x_axis { Some(axis_style) } else { None },
                &x_labels[..],
                x_label_style,
                x_label_offset,
                (0, -1 + idx as i16 * 2),
                x_desc.as_ref().map(|desc| (&desc[..], axis_desc_style)),
                x_tick_size[idx],
            )?;

            self.draw_axis_and_labels(
                self.y_label_area[idx].as_ref(),
                if y_axis { Some(axis_style) } else { None },
                &y_labels[..],
                y_label_style,
                y_label_offset,
                (-1 + idx as i16 * 2, 0),
                y_desc.as_ref().map(|desc| (&desc[..], axis_desc_style)),
                y_tick_size[idx],
            )?;
        }

        Ok(())
    }
}
