//! Helpers for unit testing

use super::OutlinePen;
use core::str::FromStr;
use raw::{
    tables::glyf::PointFlags,
    types::{F26Dot6, F2Dot14, GlyphId, Point},
};

#[derive(Copy, Clone, PartialEq, Debug)]
// clippy doesn't like the common To suffix
#[allow(clippy::enum_variant_names)]
pub enum PathElement {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
    QuadTo([f32; 4]),
    CurveTo([f32; 6]),
}

#[derive(Default)]
pub struct Path {
    pub elements: Vec<PathElement>,
    last_end: Option<[f32; 2]>,
}

impl OutlinePen for Path {
    fn move_to(&mut self, x: f32, y: f32) {
        self.elements.push(PathElement::MoveTo([x, y]));
        self.last_end = Some([x, y]);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.elements.push(PathElement::LineTo([x, y]));
        self.last_end = Some([x, y]);
    }

    fn quad_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        self.elements.push(PathElement::QuadTo([x0, y0, x1, y1]));
        self.last_end = Some([x1, y1]);
    }

    fn curve_to(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.elements
            .push(PathElement::CurveTo([x0, y0, x1, y1, x2, y2]));
        self.last_end = Some([x2, y2]);
    }

    fn close(&mut self) {
        // FT_Outline_Decompose does not generate close commands, so for
        // testing purposes, we insert a line to same point as the most
        // recent move_to (if the last command didn't end at the same point)
        // which copies FreeType's behavior.
        let last_move = self
            .elements
            .iter()
            .rev()
            .find(|element| matches!(*element, PathElement::MoveTo(_)))
            .copied();
        if let Some(PathElement::MoveTo(point)) = last_move {
            if Some(point) != self.last_end {
                self.elements.push(PathElement::LineTo(point));
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GlyphOutline {
    pub glyph_id: GlyphId,
    pub size: f32,
    pub coords: Vec<F2Dot14>,
    pub points: Vec<Point<F26Dot6>>,
    pub contours: Vec<u16>,
    pub flags: Vec<PointFlags>,
    pub path: Vec<PathElement>,
}

pub fn parse_glyph_outlines(source: &str) -> Vec<GlyphOutline> {
    let mut outlines = vec![];
    let mut cur_outline = GlyphOutline::default();
    for line in source.lines() {
        let line = line.trim();
        if line == "-" {
            outlines.push(cur_outline.clone());
        } else if line.starts_with("glyph") {
            cur_outline = GlyphOutline::default();
            let parts = line.split(' ').collect::<Vec<_>>();
            cur_outline.glyph_id = GlyphId::new(parts[1].parse().unwrap());
            cur_outline.size = parts[2].parse().unwrap();
        } else if line.starts_with("coords") {
            for coord in line.split(' ').skip(1) {
                cur_outline
                    .coords
                    .push(F2Dot14::from_f32(coord.parse().unwrap()));
            }
        } else if line.starts_with("contours") {
            for contour in line.split(' ').skip(1) {
                cur_outline.contours.push(contour.parse().unwrap());
            }
        } else if line.starts_with("points") {
            let is_scaled = cur_outline.size != 0.0;
            for mut point in parse_points(line.strip_prefix("points").unwrap().trim()) {
                if !is_scaled {
                    point[0] <<= 6;
                    point[1] <<= 6;
                }
                cur_outline.points.push(Point {
                    x: F26Dot6::from_bits(point[0]),
                    y: F26Dot6::from_bits(point[1]),
                });
            }
        } else if line.starts_with("tags") {
            for tag in line.split(' ').skip(1) {
                cur_outline
                    .flags
                    .push(PointFlags::from_bits(tag.parse().unwrap()));
            }
        } else {
            match line.as_bytes()[0] {
                b'm' => {
                    let points = parse_points(line.strip_prefix("m ").unwrap().trim());
                    cur_outline.path.push(PathElement::MoveTo(points[0]));
                }
                b'l' => {
                    let points = parse_points(line.strip_prefix("l ").unwrap().trim());
                    cur_outline.path.push(PathElement::LineTo(points[0]));
                }
                b'q' => {
                    let points = parse_points(line.strip_prefix("q ").unwrap().trim());
                    cur_outline.path.push(PathElement::QuadTo([
                        points[0][0],
                        points[0][1],
                        points[1][0],
                        points[1][1],
                    ]));
                }
                b'c' => {
                    let points = parse_points(line.strip_prefix("c ").unwrap().trim());
                    cur_outline.path.push(PathElement::CurveTo([
                        points[0][0],
                        points[0][1],
                        points[1][0],
                        points[1][1],
                        points[2][0],
                        points[2][1],
                    ]));
                }
                _ => panic!("unexpected path element"),
            }
        }
    }
    outlines
}

fn parse_points<F>(source: &str) -> Vec<[F; 2]>
where
    F: FromStr + Copy + Default,
    <F as FromStr>::Err: core::fmt::Debug,
{
    let mut points = vec![];
    for point in source.split(' ') {
        let point = point.trim();
        if point.is_empty() {
            continue;
        }
        let mut components = [F::default(); 2];
        for (i, component) in point.trim().split(',').take(2).enumerate() {
            components[i] = F::from_str(component).unwrap();
        }
        points.push(components);
    }
    points
}
