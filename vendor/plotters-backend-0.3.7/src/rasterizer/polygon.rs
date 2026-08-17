use crate::{BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};

use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Clone, Debug)]
struct Edge {
    epoch: u32,
    total_epoch: u32,
    slave_begin: i32,
    slave_end: i32,
}

impl Edge {
    fn horizontal_sweep(mut from: BackendCoord, mut to: BackendCoord) -> Option<Edge> {
        if from.0 == to.0 {
            return None;
        }

        if from.0 > to.0 {
            std::mem::swap(&mut from, &mut to);
        }

        Some(Edge {
            epoch: 0,
            total_epoch: (to.0 - from.0) as u32,
            slave_begin: from.1,
            slave_end: to.1,
        })
    }

    fn vertical_sweep(from: BackendCoord, to: BackendCoord) -> Option<Edge> {
        Edge::horizontal_sweep((from.1, from.0), (to.1, to.0))
    }

    fn get_master_pos(&self) -> i32 {
        (self.total_epoch - self.epoch) as i32
    }

    fn inc_epoch(&mut self) {
        self.epoch += 1;
    }

    fn get_slave_pos(&self) -> f64 {
        f64::from(self.slave_begin)
            + (i64::from(self.slave_end - self.slave_begin) * i64::from(self.epoch)) as f64
                / f64::from(self.total_epoch)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.get_slave_pos() == other.get_slave_pos()
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_slave_pos()
            .partial_cmp(&other.get_slave_pos())
            .unwrap()
    }
}

pub fn fill_polygon<DB: DrawingBackend, S: BackendStyle>(
    back: &mut DB,
    vertices: &[BackendCoord],
    style: &S,
) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
    if let Some((x_span, y_span)) =
        vertices
            .iter()
            .fold(None, |res: Option<((i32, i32), (i32, i32))>, (x, y)| {
                Some(
                    res.map(|((min_x, max_x), (min_y, max_y))| {
                        (
                            (min_x.min(*x), max_x.max(*x)),
                            (min_y.min(*y), max_y.max(*y)),
                        )
                    })
                    .unwrap_or(((*x, *x), (*y, *y))),
                )
            })
    {
        // First of all, let's handle the case that all the points is in a same vertical or
        // horizontal line
        if x_span.0 == x_span.1 || y_span.0 == y_span.1 {
            return back.draw_line((x_span.0, y_span.0), (x_span.1, y_span.1), style);
        }

        let horizontal_sweep = x_span.1 - x_span.0 > y_span.1 - y_span.0;

        let mut edges: Vec<_> = vertices
            .iter()
            .zip(vertices.iter().skip(1))
            .map(|(a, b)| (*a, *b))
            .collect();
        edges.push((vertices[vertices.len() - 1], vertices[0]));
        edges.sort_by_key(|((x1, y1), (x2, y2))| {
            if horizontal_sweep {
                *x1.min(x2)
            } else {
                *y1.min(y2)
            }
        });

        for edge in &mut edges.iter_mut() {
            if horizontal_sweep {
                if (edge.0).0 > (edge.1).0 {
                    std::mem::swap(&mut edge.0, &mut edge.1);
                }
            } else if (edge.0).1 > (edge.1).1 {
                std::mem::swap(&mut edge.0, &mut edge.1);
            }
        }

        let (low, high) = if horizontal_sweep { x_span } else { y_span };

        let mut idx = 0;

        let mut active_edge: Vec<Edge> = vec![];

        for sweep_line in low..=high {
            let mut new_vec = vec![];

            for mut e in active_edge {
                if e.get_master_pos() > 0 {
                    e.inc_epoch();
                    new_vec.push(e);
                }
            }

            active_edge = new_vec;

            loop {
                if idx >= edges.len() {
                    break;
                }
                let line = if horizontal_sweep {
                    (edges[idx].0).0
                } else {
                    (edges[idx].0).1
                };
                if line > sweep_line {
                    break;
                }

                let edge_obj = if horizontal_sweep {
                    Edge::horizontal_sweep(edges[idx].0, edges[idx].1)
                } else {
                    Edge::vertical_sweep(edges[idx].0, edges[idx].1)
                };

                if let Some(edge_obj) = edge_obj {
                    active_edge.push(edge_obj);
                }

                idx += 1;
            }

            active_edge.sort();

            let mut first = None;
            let mut second = None;

            for edge in active_edge.iter() {
                if first.is_none() {
                    first = Some(edge.clone())
                } else if second.is_none() {
                    second = Some(edge.clone())
                }

                if let Some(a) = first.clone() {
                    if let Some(b) = second.clone() {
                        if a.get_master_pos() == 0 && b.get_master_pos() != 0 {
                            first = Some(b);
                            second = None;
                            continue;
                        }

                        if a.get_master_pos() != 0 && b.get_master_pos() == 0 {
                            first = Some(a);
                            second = None;
                            continue;
                        }

                        let from = a.get_slave_pos();
                        let to = b.get_slave_pos();

                        if a.get_master_pos() == 0 && b.get_master_pos() == 0 && to - from > 1.0 {
                            first = None;
                            second = None;
                            continue;
                        }

                        if horizontal_sweep {
                            check_result!(back.draw_line(
                                (sweep_line, from.ceil() as i32),
                                (sweep_line, to.floor() as i32),
                                &style.color(),
                            ));
                            check_result!(back.draw_pixel(
                                (sweep_line, from.floor() as i32),
                                style.color().mix(from.ceil() - from),
                            ));
                            check_result!(back.draw_pixel(
                                (sweep_line, to.ceil() as i32),
                                style.color().mix(to - to.floor()),
                            ));
                        } else {
                            check_result!(back.draw_line(
                                (from.ceil() as i32, sweep_line),
                                (to.floor() as i32, sweep_line),
                                &style.color(),
                            ));
                            check_result!(back.draw_pixel(
                                (from.floor() as i32, sweep_line),
                                style.color().mix(from.ceil() - from),
                            ));
                            check_result!(back.draw_pixel(
                                (to.ceil() as i32, sweep_line),
                                style.color().mix(to.floor() - to),
                            ));
                        }

                        first = None;
                        second = None;
                    }
                }
            }
        }
    }

    Ok(())
}
