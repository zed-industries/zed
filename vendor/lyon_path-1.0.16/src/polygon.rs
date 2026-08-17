//! Specific path types for polygons.

use crate::math::Point;
use crate::{
    ControlPointId, EndpointId, Event, EventId, IdEvent, PathEvent, Position, PositionStore,
};

/// A view over a sequence of endpoints forming a polygon.
///
/// ## Example
///
/// ```
/// use lyon_path::polygon::Polygon;
/// use lyon_path::geom::point;
///
/// let path = Polygon {
///     points: &[
///         point(0.0, 0.0),
///         point(10.0, 10.0),
///         point(0.0, 10.0),
///     ],
///     closed: true,
/// };
///
/// for event in path.path_events() {
///     // same as iterating a regular `Path` object.
/// }
/// ```
#[derive(Clone)]
pub struct Polygon<'l, T> {
    pub points: &'l [T],
    pub closed: bool,
}

impl<'l, T> Polygon<'l, T> {
    /// Returns an iterator of `Event<&T>`.
    pub fn iter(&self) -> PolygonIter<'l, T> {
        PolygonIter {
            points: self.points.iter(),
            prev: None,
            first: None,
            closed: self.closed,
        }
    }

    /// Returns an iterator of `IdEvent`.
    pub fn id_iter(&self) -> PolygonIdIter {
        PolygonIdIter::new(0..(self.points.len() as u32), self.closed)
    }

    /// Returns an iterator of `PathEvent`.
    pub fn path_events(&self) -> PathEvents<T>
    where
        T: Position,
    {
        PathEvents {
            points: self.points.iter(),
            first: None,
            prev: None,
            closed: self.closed,
        }
    }

    /// Returns the event for a given event ID.
    pub fn event(&self, id: EventId) -> Event<&T, ()> {
        let idx = id.0 as usize;
        if idx == 0 {
            Event::Begin {
                at: &self.points[0],
            }
        } else if idx == self.points.len() - 1 {
            Event::End {
                last: &self.points[self.points.len() - 1],
                first: &self.points[0],
                close: self.closed,
            }
        } else {
            Event::Line {
                from: &self.points[idx - 1],
                to: &self.points[idx],
            }
        }
    }
}

impl<'l, T> core::ops::Index<EndpointId> for Polygon<'l, T> {
    type Output = T;
    fn index(&self, id: EndpointId) -> &T {
        &self.points[id.to_usize()]
    }
}

/// A view over a sequence of endpoint IDs forming a polygon.
#[derive(Clone)]
pub struct IdPolygon<'l> {
    pub points: &'l [EndpointId],
    pub closed: bool,
}

impl<'l> IdPolygon<'l> {
    // Returns an iterator over the endpoint IDs of the polygon.
    pub fn iter(&self) -> IdPolygonIter<'l> {
        IdPolygonIter {
            points: self.points.iter(),
            idx: 0,
            prev: None,
            first: EndpointId(0),
            closed: self.closed,
        }
    }

    /// Returns the event for a given event ID.
    pub fn event(&self, id: EventId) -> IdEvent {
        let idx = id.0 as usize;
        if idx == 0 {
            IdEvent::Begin { at: self.points[0] }
        } else if idx == self.points.len() {
            IdEvent::End {
                last: self.points[self.points.len() - 1],
                first: self.points[0],
                close: self.closed,
            }
        } else {
            IdEvent::Line {
                from: self.points[idx - 1],
                to: self.points[idx],
            }
        }
    }
}

/// An iterator of `Event<EndpointId, ()>`.
#[derive(Clone)]
pub struct IdPolygonIter<'l> {
    points: core::slice::Iter<'l, EndpointId>,
    idx: u32,
    prev: Option<EndpointId>,
    first: EndpointId,
    closed: bool,
}

impl<'l> Iterator for IdPolygonIter<'l> {
    type Item = IdEvent;
    fn next(&mut self) -> Option<IdEvent> {
        match (self.prev, self.points.next()) {
            (Some(from), Some(to)) => {
                self.prev = Some(*to);
                self.idx += 1;
                Some(IdEvent::Line { from, to: *to })
            }
            (None, Some(at)) => {
                self.prev = Some(*at);
                self.first = *at;
                self.idx += 1;
                Some(IdEvent::Begin { at: *at })
            }
            (Some(last), None) => {
                self.prev = None;
                Some(IdEvent::End {
                    last,
                    first: self.first,
                    close: self.closed,
                })
            }
            (None, None) => None,
        }
    }
}

/// An iterator of `Event<&Endpoint, ()>`.
#[derive(Clone)]
pub struct PolygonIter<'l, T> {
    points: core::slice::Iter<'l, T>,
    prev: Option<&'l T>,
    first: Option<&'l T>,
    closed: bool,
}

impl<'l, T> Iterator for PolygonIter<'l, T> {
    type Item = Event<&'l T, ()>;
    fn next(&mut self) -> Option<Event<&'l T, ()>> {
        match (self.prev, self.points.next()) {
            (Some(from), Some(to)) => {
                self.prev = Some(to);
                Some(Event::Line { from, to })
            }
            (None, Some(at)) => {
                self.prev = Some(at);
                self.first = Some(at);
                Some(Event::Begin { at })
            }
            (Some(last), None) => {
                self.prev = None;
                Some(Event::End {
                    last,
                    first: self.first.unwrap(),
                    close: self.closed,
                })
            }
            (None, None) => None,
        }
    }
}

/// An iterator of `PathEvent`.
#[derive(Clone)]
pub struct PathEvents<'l, T> {
    points: core::slice::Iter<'l, T>,
    prev: Option<Point>,
    first: Option<Point>,
    closed: bool,
}

impl<'l, T: Position> Iterator for PathEvents<'l, T> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        match (self.prev, self.points.next()) {
            (Some(from), Some(to)) => {
                let to = to.position();
                self.prev = Some(to);
                Some(Event::Line { from, to })
            }
            (None, Some(at)) => {
                let at = at.position();
                self.prev = Some(at);
                self.first = Some(at);
                Some(Event::Begin { at })
            }
            (Some(last), None) => {
                self.prev = None;
                Some(Event::End {
                    last,
                    first: self.first.unwrap(),
                    close: self.closed,
                })
            }
            (None, None) => None,
        }
    }
}

/// An iterator of `IdEvent` for `Polygon`.
#[derive(Clone)]
pub struct PolygonIdIter {
    idx: u32,
    start: u32,
    end: u32,
    closed: bool,
}

impl PolygonIdIter {
    #[inline]
    pub fn new(range: core::ops::Range<u32>, closed: bool) -> Self {
        PolygonIdIter {
            idx: range.start,
            start: range.start,
            end: range.end,
            closed,
        }
    }
}

impl Iterator for PolygonIdIter {
    type Item = IdEvent;
    fn next(&mut self) -> Option<IdEvent> {
        let idx = self.idx;
        self.idx += 1;

        if idx == self.start {
            Some(IdEvent::Begin {
                at: EndpointId(self.start),
            })
        } else if idx < self.end {
            Some(IdEvent::Line {
                from: EndpointId(idx - 1),
                to: EndpointId(idx),
            })
        } else if idx == self.end {
            Some(IdEvent::End {
                last: EndpointId(self.end - 1),
                first: EndpointId(self.start),
                close: self.closed,
            })
        } else {
            None
        }
    }
}

impl<'l, Endpoint> PositionStore for Polygon<'l, Endpoint>
where
    Endpoint: Position,
{
    fn get_endpoint(&self, id: EndpointId) -> Point {
        self.points[id.to_usize()].position()
    }

    fn get_control_point(&self, _: ControlPointId) -> Point {
        panic!("Polygons do not have control points.");
    }
}

#[test]
fn event_ids() {
    let poly = IdPolygon {
        points: &[EndpointId(0), EndpointId(1), EndpointId(2), EndpointId(3)],
        closed: true,
    };

    assert_eq!(poly.event(EventId(0)), IdEvent::Begin { at: EndpointId(0) });
    assert_eq!(
        poly.event(EventId(1)),
        IdEvent::Line {
            from: EndpointId(0),
            to: EndpointId(1)
        }
    );
    assert_eq!(
        poly.event(EventId(2)),
        IdEvent::Line {
            from: EndpointId(1),
            to: EndpointId(2)
        }
    );
    assert_eq!(
        poly.event(EventId(3)),
        IdEvent::Line {
            from: EndpointId(2),
            to: EndpointId(3)
        }
    );
    assert_eq!(
        poly.event(EventId(4)),
        IdEvent::End {
            last: EndpointId(3),
            first: EndpointId(0),
            close: true
        }
    );

    let mut iter = poly.iter();
    assert_eq!(iter.next(), Some(IdEvent::Begin { at: EndpointId(0) }));
    assert_eq!(
        iter.next(),
        Some(IdEvent::Line {
            from: EndpointId(0),
            to: EndpointId(1)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Line {
            from: EndpointId(1),
            to: EndpointId(2)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Line {
            from: EndpointId(2),
            to: EndpointId(3)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::End {
            last: EndpointId(3),
            first: EndpointId(0),
            close: true
        })
    );
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn polygon_slice_id_ite() {
    let points: &[u32] = &[0, 1, 2, 3, 4, 5, 6];
    let polygon = Polygon {
        points,
        closed: true,
    };

    let mut it = polygon.id_iter();
    assert_eq!(it.next(), Some(IdEvent::Begin { at: EndpointId(0) }));
    assert_eq!(
        it.next(),
        Some(IdEvent::Line {
            from: EndpointId(0),
            to: EndpointId(1)
        })
    );
    assert_eq!(
        it.next(),
        Some(IdEvent::Line {
            from: EndpointId(1),
            to: EndpointId(2)
        })
    );
    assert_eq!(
        it.next(),
        Some(IdEvent::Line {
            from: EndpointId(2),
            to: EndpointId(3)
        })
    );
    assert_eq!(
        it.next(),
        Some(IdEvent::Line {
            from: EndpointId(3),
            to: EndpointId(4)
        })
    );
    assert_eq!(
        it.next(),
        Some(IdEvent::Line {
            from: EndpointId(4),
            to: EndpointId(5)
        })
    );
    assert_eq!(
        it.next(),
        Some(IdEvent::Line {
            from: EndpointId(5),
            to: EndpointId(6)
        })
    );
    assert_eq!(
        it.next(),
        Some(IdEvent::End {
            last: EndpointId(6),
            first: EndpointId(0),
            close: true
        })
    );
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}
