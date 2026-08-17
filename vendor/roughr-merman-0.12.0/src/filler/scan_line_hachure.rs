use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::marker::PhantomData;

use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};

use super::traits::PatternFiller;
use crate::core::{_c, OpSet, Options};
use crate::geometry::{rotate_lines, rotate_points, Line};

#[derive(Clone)]
struct EdgeEntry<F: Float + FromPrimitive + Trig> {
    pub(crate) ymin: F,
    pub(crate) ymax: F,
    pub(crate) x: F,
    pub(crate) islope: F,
}

impl<F: Float + FromPrimitive + Trig> std::fmt::Display for EdgeEntry<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ymin={} ymax={} x={} islope={}",
            self.ymin.to_f64().unwrap(),
            self.ymax.to_f64().unwrap(),
            self.x.to_f64().unwrap(),
            self.islope.to_f64().unwrap()
        )
    }
}

struct ActiveEdgeEntry<F: Float + FromPrimitive + Trig> {
    pub(crate) edge: EdgeEntry<F>,
}

pub fn polygon_hachure_lines<F: Float + FromPrimitive + Trig>(
    polygon_list: &mut [Vec<Point2D<F>>],
    options: &Options,
) -> Vec<Line<F>> {
    let angle = options.hachure_angle.unwrap_or(0.0) + 90.0;
    let mut gap = options.hachure_gap.unwrap_or(0.0);
    if gap < 0.0 {
        gap = options.stroke_width.unwrap_or(0.0) * 4.0;
    }

    gap = f32::max(gap, 0.1);

    let center = Point2D::new(_c(0.0), _c(0.0));
    if angle != 0.0 {
        polygon_list
            .iter_mut()
            .for_each(|polygon| *polygon = rotate_points(polygon, &center, _c(angle)))
    }

    let mut lines = straight_hachure_lines(polygon_list, _c(gap));

    if angle != 0.0 {
        polygon_list
            .iter_mut()
            .for_each(|polygon| *polygon = rotate_points(polygon, &center, _c(-angle)));
        lines = rotate_lines(&lines, &center, _c(-angle));
    }

    lines
}

fn straight_hachure_lines<F>(polygon_list: &mut [Vec<Point2D<F>>], gap: F) -> Vec<Line<F>>
where
    F: Float + FromPrimitive + Trig,
{
    let mut vertex_array: Vec<Vec<Point2D<F>>> = vec![];
    for polygon in polygon_list.iter_mut() {
        if polygon.first() != polygon.last() {
            polygon.push(
                *polygon
                    .first()
                    .expect("can not get first element of polygon"),
            );
        }
        if polygon.len() > 2 {
            vertex_array.push(polygon.clone());
        }
    }

    let mut lines: Vec<Line<F>> = vec![];
    let gap = F::max(gap, _c(0.1));

    // create sorted edges table
    let mut edges: Vec<EdgeEntry<F>> = vec![];

    for vertices in vertex_array.iter() {
        let mut edge_extension = vertices[..]
            .windows(2)
            .filter_map(|w| {
                let p1 = w[0];
                let p2 = w[1];
                if p1.y != p2.y {
                    let ymin = F::min(p1.y, p2.y);
                    Some(EdgeEntry {
                        ymin,
                        ymax: F::max(p1.y, p2.y),
                        x: if ymin == p1.y { p1.x } else { p2.x },
                        islope: (p2.x - p1.x) / (p2.y - p1.y),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<EdgeEntry<F>>>();

        edges.append(&mut edge_extension);
    }

    edges.sort_by(|e1, e2| {
        if e1.ymin < e2.ymin {
            Ordering::Less
        } else if e1.ymin > e2.ymin {
            Ordering::Greater
        } else if e1.x < e2.x {
            Ordering::Less
        } else if e1.x > e2.x {
            Ordering::Greater
        } else if e1.ymax == e2.ymax {
            Ordering::Equal
        } else {
            let ordering = (e1.ymax - e2.ymax) / F::abs(e1.ymax - e2.ymax);
            if ordering > _c(0.0) {
                Ordering::Greater
            } else if ordering < _c(0.0) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    });

    if edges.is_empty() {
        return lines;
    }

    let mut active_edges: Vec<ActiveEdgeEntry<F>> = Vec::new();
    let mut y = edges.first().unwrap().ymin;

    loop {
        if !edges.is_empty() {
            let ix = edges
                .iter()
                .enumerate()
                .find(|(_ind, v)| v.ymin > y)
                .map(|(ind, _v)| ind);

            if let Some(indx) = ix {
                let removed_elements = edges.splice(0..indx, vec![]);

                removed_elements
                    .into_iter()
                    .for_each(|ee| active_edges.push(ActiveEdgeEntry { edge: ee }));
            } else {
                let removed_elements = edges.splice(0..edges.len(), vec![]);

                removed_elements
                    .into_iter()
                    .for_each(|ee| active_edges.push(ActiveEdgeEntry { edge: ee }));
            }
        }

        active_edges.retain(|ae| ae.edge.ymax > y);

        active_edges.sort_by(|ae1, ae2| {
            if ae1.edge.x == ae2.edge.x {
                Ordering::Equal
            } else {
                let ratio = (ae1.edge.x - ae2.edge.x) / F::abs(ae1.edge.x - ae2.edge.x);
                if ratio > _c(0.0) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        });
        if active_edges.len() > 1 {
            active_edges[..].chunks(2).for_each(|ae| {
                let ce = &ae[0];
                let ne = &ae[1];
                lines.push(Line::from(&[
                    euclid::Point2D::new(ce.edge.x, y),
                    euclid::Point2D::new(ne.edge.x, y),
                ]));
            });
        }

        y = y + gap;
        active_edges.iter_mut().for_each(|ae| {
            ae.edge.x = ae.edge.x + (gap * ae.edge.islope);
        });
        if edges.is_empty() && active_edges.is_empty() {
            break;
        }
    }

    lines
}

pub struct ScanlineHachureFiller<F> {
    _phantom: PhantomData<F>,
}

impl<F, P> PatternFiller<F, P> for ScanlineHachureFiller<F>
where
    F: Float + Trig + FromPrimitive,
    P: BorrowMut<Vec<Vec<Point2D<F>>>>,
{
    fn fill_polygons(&self, mut polygon_list: P, o: &mut Options) -> crate::core::OpSet<F> {
        let lines = polygon_hachure_lines(polygon_list.borrow_mut().as_mut_slice(), o);
        let ops = ScanlineHachureFiller::render_lines(lines, o);
        OpSet {
            op_set_type: crate::core::OpSetType::FillSketch,
            ops,
            size: None,
            path: None,
        }
    }
}

impl<F: Float + Trig + FromPrimitive> ScanlineHachureFiller<F> {
    pub fn new() -> Self {
        ScanlineHachureFiller {
            _phantom: PhantomData,
        }
    }

    fn render_lines(lines: Vec<Line<F>>, o: &mut Options) -> Vec<crate::core::Op<F>> {
        let mut ops: Vec<crate::core::Op<F>> = vec![];
        lines.iter().for_each(|l| {
            ops.extend(crate::renderer::_double_line(
                l.start_point.x,
                l.start_point.y,
                l.end_point.x,
                l.end_point.y,
                o,
                true,
            ))
        });

        ops
    }
}

impl<F: Float + Trig + FromPrimitive> Default for ScanlineHachureFiller<F> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use euclid::point2;

    use crate::geometry::Line;

    #[test]
    fn straight_hachure_lines() {
        let mut input = vec![vec![
            point2(0.0, 0.0),
            point2(0.0, 1.0),
            point2(1.0, 1.0),
            point2(1.0, 0.0),
        ]];
        let expected = vec![
            Line::from(&[point2(0.0, 0.0), point2(1.0, 0.0)]),
            Line::from(&[
                point2(0.0, 0.10000000149011612),
                point2(1.0, 0.10000000149011612),
            ]),
            Line::from(&[
                point2(0.0, 0.20000000298023224),
                point2(1.0, 0.20000000298023224),
            ]),
            Line::from(&[
                point2(0.0, 0.30000000447034836),
                point2(1.0, 0.30000000447034836),
            ]),
            Line::from(&[
                point2(0.0, 0.4000000059604645),
                point2(1.0, 0.4000000059604645),
            ]),
            Line::from(&[
                point2(0.0, 0.5000000074505806),
                point2(1.0, 0.5000000074505806),
            ]),
            Line::from(&[
                point2(0.0, 0.6000000089406967),
                point2(1.0, 0.6000000089406967),
            ]),
            Line::from(&[
                point2(0.0, 0.7000000104308128),
                point2(1.0, 0.7000000104308128),
            ]),
            Line::from(&[
                point2(0.0, 0.800000011920929),
                point2(1.0, 0.800000011920929),
            ]),
            Line::from(&[
                point2(0.0, 0.9000000134110451),
                point2(1.0, 0.9000000134110451),
            ]),
        ];
        let result = super::straight_hachure_lines(&mut input, 0.1);
        assert_eq!(expected, result);
    }
}
