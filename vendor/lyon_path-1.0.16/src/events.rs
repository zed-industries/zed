use crate::geom::traits::Transformation;
use crate::math::Point;
use crate::{ControlPointId, EndpointId, Position};

/// Represents an event or edge of path.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Event<Endpoint, ControlPoint> {
    Begin {
        at: Endpoint,
    },
    Line {
        from: Endpoint,
        to: Endpoint,
    },
    Quadratic {
        from: Endpoint,
        ctrl: ControlPoint,
        to: Endpoint,
    },
    Cubic {
        from: Endpoint,
        ctrl1: ControlPoint,
        ctrl2: ControlPoint,
        to: Endpoint,
    },
    End {
        last: Endpoint,
        first: Endpoint,
        close: bool,
    },
}

/// A path event representing endpoints and control points as positions.
pub type PathEvent = Event<Point, Point>;

/// A path event representing endpoints and control points as IDs.
pub type IdEvent = Event<EndpointId, ControlPointId>;

impl<Ep, Cp> Event<Ep, Cp> {
    pub fn is_edge(&self) -> bool {
        match self {
            &Event::Line { .. }
            | &Event::Quadratic { .. }
            | &Event::Cubic { .. }
            | &Event::End { close: true, .. } => true,
            _ => false,
        }
    }

    pub fn from(&self) -> Ep
    where
        Ep: Clone,
    {
        match &self {
            &Event::Line { from, .. }
            | &Event::Quadratic { from, .. }
            | &Event::Cubic { from, .. }
            | &Event::Begin { at: from }
            | &Event::End { last: from, .. } => from.clone(),
        }
    }

    pub fn to(&self) -> Ep
    where
        Ep: Clone,
    {
        match &self {
            &Event::Line { to, .. }
            | &Event::Quadratic { to, .. }
            | &Event::Cubic { to, .. }
            | &Event::Begin { at: to }
            | &Event::End { first: to, .. } => to.clone(),
        }
    }

    pub fn with_points(&self) -> PathEvent
    where
        Ep: Position,
        Cp: Position,
    {
        match self {
            Event::Line { from, to } => Event::Line {
                from: from.position(),
                to: to.position(),
            },
            Event::Quadratic { from, ctrl, to } => Event::Quadratic {
                from: from.position(),
                ctrl: ctrl.position(),
                to: to.position(),
            },
            Event::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => Event::Cubic {
                from: from.position(),
                ctrl1: ctrl1.position(),
                ctrl2: ctrl2.position(),
                to: to.position(),
            },
            Event::Begin { at } => Event::Begin { at: at.position() },
            Event::End { last, first, close } => Event::End {
                last: last.position(),
                first: first.position(),
                close: *close,
            },
        }
    }
}

impl PathEvent {
    pub fn transformed<T: Transformation<f32>>(&self, mat: &T) -> Self {
        match self {
            Event::Line { from, to } => Event::Line {
                from: mat.transform_point(*from),
                to: mat.transform_point(*to),
            },
            Event::Quadratic { from, ctrl, to } => Event::Quadratic {
                from: mat.transform_point(*from),
                ctrl: mat.transform_point(*ctrl),
                to: mat.transform_point(*to),
            },
            Event::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => Event::Cubic {
                from: mat.transform_point(*from),
                ctrl1: mat.transform_point(*ctrl1),
                ctrl2: mat.transform_point(*ctrl2),
                to: mat.transform_point(*to),
            },
            Event::Begin { at } => Event::Begin {
                at: mat.transform_point(*at),
            },
            Event::End { first, last, close } => Event::End {
                last: mat.transform_point(*last),
                first: mat.transform_point(*first),
                close: *close,
            },
        }
    }
}
