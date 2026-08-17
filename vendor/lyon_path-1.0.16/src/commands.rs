//! A generic representation for paths that allow more control over how
//! endpoints and control points are stored.
//!
//! # Motivation
//!
//! The default `Path` data structure in this crate is works well for the
//! most common use cases. Sometimes, however, it is useful to be able to
//! specify exactly how endpoints and control points are stored instead of
//! relying on implicitly following the order of the events.
//!
//! This module contains bricks to help with building custom path representations.
//! The central piece is the [`PathCommands`](struct.PathCommands.html) buffer and
//! its [`PathCommandsBuilder`](struct.PathCommandsBuilder.html), providing a compact
//! representation for path events with IDs instead of positions.
//!
//! # Examples
//!
//! The following example shows how `PathCommands` can be used together with an
//! external buffers for positions to implement features similar to the default
//! Path type with a different data structure.
//!
//! ```
//! use lyon_path::{EndpointId, Event, IdEvent, commands::PathCommands};
//! let points = &[
//!     [0.0, 0.0],
//!     [1.0, 1.0],
//!     [0.0, 2.0],
//! ];
//!
//! let mut cmds = PathCommands::builder();
//! cmds.begin(EndpointId(0));
//! cmds.line_to(EndpointId(1));
//! cmds.line_to(EndpointId(2));
//! cmds.end(true);
//!
//! let cmds = cmds.build();
//!
//! for event in &cmds {
//!     match event {
//!         IdEvent::Begin { at } => { println!("move to {:?}", points[at.to_usize()]); }
//!         IdEvent::Line { to, .. } => { println!("line to {:?}", points[to.to_usize()]); }
//!         IdEvent::End { close: true, .. } => { println!("close"); }
//!         _ => { panic!("unexpected event!") }
//!     }
//! }
//!
//! // Iterate over the points directly using CommandsPathSlice
//! for event in cmds.path_slice(points, points).events() {
//!     match event {
//!         Event::Begin { at } => { println!("move to {:?}", at); }
//!         Event::Line { to, .. } => { println!("line to {:?}", to); }
//!         Event::End { close: true, .. } => { println!("close"); }
//!         _ => { panic!("unexpected event!") }
//!     }
//! }
//!
//! ```

use crate::events::{Event, IdEvent, PathEvent};
use crate::math::Point;
use crate::{ControlPointId, EndpointId, EventId, Position, PositionStore};

use core::fmt;

use crate::private::DebugValidator;
use alloc::boxed::Box;
use alloc::vec::Vec;

// Note: Tried making the path generic over the integer type used to store
// the commands to allow u16 and u32, but the performance difference is very
// small and not worth the added complexity.

mod verb {
    pub const LINE: u32 = 0;
    pub const QUADRATIC: u32 = 1;
    pub const CUBIC: u32 = 2;
    pub const BEGIN: u32 = 3;
    pub const CLOSE: u32 = 4;
    pub const END: u32 = 5;
}

/// Sadly this is very close to core::slice::Iter but reimplementing
/// it manually to iterate over u32 makes a difference.
/// It would seem that having next return u32 with a special value
/// for the end of the iteration instead of Option<u32> should
/// improve performance (simpler code and a bunch of unwraps removed),
/// however a naive initial attempt led to worse performance.
#[derive(Copy, Clone)]
struct CmdIter<'l> {
    ptr: *const u32,
    end: *const u32,
    _marker: core::marker::PhantomData<&'l u32>,
}

impl<'l> CmdIter<'l> {
    fn new(slice: &'l [u32]) -> Self {
        let ptr = slice.as_ptr();
        let end = unsafe { ptr.add(slice.len()) };
        CmdIter {
            ptr,
            end,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    fn next(&mut self) -> Option<u32> {
        unsafe {
            if self.ptr == self.end {
                return None;
            }

            let val = *self.ptr;
            self.ptr = self.ptr.offset(1);

            Some(val)
        }
    }
}

/// The commands of a path encoded in a single array using IDs to refer
/// to endpoints and control points externally.
///
/// `PathCommands` is a good fit when the a custom endpoint and control point
/// types are needed or when their the user needs full control over their storage.
///
/// # Representation
///
/// Path commands contains a single array of 32 bits integer values encoding path
/// commands, endpoint IDs or control point IDs.
///
/// ```ascii
///  __________________________________________________________________________
/// |       |          |      |          |         |              |          |
/// | Begin |EndpointID| Line |EndpointID|Quadratic|ControlPointId|EndpointID| ...
/// |_______|__________|______|__________|_________|______________|__________|_
///
/// ```
///
/// # Example
///
/// ```
/// use lyon_path::{EndpointId, PathCommands};
///
/// let mut cmds = PathCommands::builder();
///
/// cmds.begin(EndpointId(0));
/// cmds.line_to(EndpointId(1));
/// cmds.line_to(EndpointId(2));
/// cmds.end(true);
///
/// let cmds = cmds.build();
///
#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct PathCommands {
    cmds: Box<[u32]>,
}

impl PathCommands {
    /// Creates a [PathCommandsBuilder](struct.PathCommandsBuilder.html) to create path commands.
    pub fn builder() -> PathCommandsBuilder {
        PathCommandsBuilder::new()
    }

    /// Returns an iterator over the path commands.
    pub fn iter(&self) -> Iter {
        Iter::new(&self.cmds)
    }

    /// Returns a view on the path commands.
    pub fn as_slice(&self) -> PathCommandsSlice {
        PathCommandsSlice { cmds: &self.cmds }
    }

    /// Returns a view on a path made of these commands with endpoint and
    /// control point slices.
    pub fn path_slice<'l, Endpoint, ControlPoint>(
        &'l self,
        endpoints: &'l [Endpoint],
        control_points: &'l [ControlPoint],
    ) -> CommandsPathSlice<Endpoint, ControlPoint> {
        CommandsPathSlice {
            endpoints,
            control_points,
            cmds: self.as_slice(),
        }
    }

    /// Returns an iterator over the path, with endpoints and control points.
    pub fn events<'l, Endpoint, ControlPoint>(
        &'l self,
        endpoints: &'l [Endpoint],
        control_points: &'l [ControlPoint],
    ) -> Events<Endpoint, ControlPoint> {
        Events {
            cmds: CmdIter::new(&self.cmds),
            first_endpoint: 0,
            prev_endpoint: 0,
            endpoints,
            control_points,
        }
    }

    /// Returns the event for a given event ID.
    pub fn event(&self, id: EventId) -> IdEvent {
        self.as_slice().event(id)
    }

    /// Returns the next event id within the path.
    pub fn next_event_id_in_path(&self, id: EventId) -> Option<EventId> {
        self.as_slice().next_event_id_in_path(id)
    }

    /// Returns the next event id within the sub-path.
    ///
    /// Loops back to the first event after the end of the sub-path.
    pub fn next_event_id_in_sub_path(&self, id: EventId) -> EventId {
        self.as_slice().next_event_id_in_sub_path(id)
    }
}

impl fmt::Debug for PathCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<'l> IntoIterator for &'l PathCommands {
    type Item = IdEvent;
    type IntoIter = Iter<'l>;

    fn into_iter(self) -> Iter<'l> {
        self.iter()
    }
}

impl<'l> From<&'l PathCommands> for PathCommandsSlice<'l> {
    fn from(commands: &'l PathCommands) -> Self {
        commands.as_slice()
    }
}

/// A view over [`PathCommands`](struct.PathCommands.html).
#[derive(Copy, Clone)]
pub struct PathCommandsSlice<'l> {
    cmds: &'l [u32],
}

impl<'l> PathCommandsSlice<'l> {
    /// Returns an iterator over the path commands.
    pub fn iter(&self) -> Iter {
        Iter::new(self.cmds)
    }

    /// Returns the event for a given event ID.
    pub fn event(&self, id: EventId) -> IdEvent {
        let idx = id.to_usize();
        match self.cmds[idx] {
            verb::LINE => IdEvent::Line {
                from: EndpointId(self.cmds[idx - 1]),
                to: EndpointId(self.cmds[idx + 1]),
            },
            verb::QUADRATIC => IdEvent::Quadratic {
                from: EndpointId(self.cmds[idx - 1]),
                ctrl: ControlPointId(self.cmds[idx + 1]),
                to: EndpointId(self.cmds[idx + 2]),
            },
            verb::CUBIC => IdEvent::Cubic {
                from: EndpointId(self.cmds[idx - 1]),
                ctrl1: ControlPointId(self.cmds[idx + 1]),
                ctrl2: ControlPointId(self.cmds[idx + 2]),
                to: EndpointId(self.cmds[idx + 3]),
            },
            verb::BEGIN => IdEvent::Begin {
                at: EndpointId(self.cmds[idx + 1]),
            },
            verb::END => {
                let first_event = self.cmds[idx + 1] as usize;
                IdEvent::End {
                    last: EndpointId(self.cmds[idx - 1]),
                    first: EndpointId(self.cmds[first_event + 1]),
                    close: false,
                }
            }
            _ => {
                // CLOSE
                let first_event = self.cmds[idx + 1] as usize;
                IdEvent::End {
                    last: EndpointId(self.cmds[idx - 1]),
                    first: EndpointId(self.cmds[first_event + 1]),
                    close: true,
                }
            }
        }
    }

    /// Returns the next event id within the path.
    pub fn next_event_id_in_sub_path(&self, id: EventId) -> EventId {
        let idx = id.to_usize();
        match self.cmds[idx] {
            verb::LINE | verb::BEGIN => EventId(id.0 + 2),
            verb::QUADRATIC => EventId(id.0 + 3),
            verb::CUBIC => EventId(id.0 + 4),
            //verb::END | verb::CLOSE
            _ => EventId(self.cmds[idx + 1]),
        }
    }

    /// Returns the next event id within the path.
    pub fn next_event_id_in_path(&self, id: EventId) -> Option<EventId> {
        let idx = id.to_usize();
        let next = match self.cmds[idx] {
            verb::QUADRATIC => EventId(id.0 + 3),
            verb::CUBIC => EventId(id.0 + 4),
            // verb::LINE | verb::BEGIN | verb::END | verb::CLOSE
            _ => EventId(id.0 + 2),
        };

        if next.0 < self.cmds.len() as u32 {
            return Some(next);
        }

        None
    }
}

impl<'l> fmt::Debug for PathCommandsSlice<'l> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"")?;
        for evt in self.iter() {
            match evt {
                IdEvent::Line { to, .. } => write!(f, "L {to:?}"),
                IdEvent::Quadratic { ctrl, to, .. } => write!(f, "Q {ctrl:?} {to:?} "),
                IdEvent::Cubic {
                    ctrl1, ctrl2, to, ..
                } => write!(f, "C {ctrl1:?} {ctrl2:?} {to:?} "),
                IdEvent::Begin { at, .. } => write!(f, "M {at:?} "),
                IdEvent::End { close: true, .. } => write!(f, "Z "),
                IdEvent::End { close: false, .. } => Ok(()),
            }?;
        }
        write!(f, "\"")
    }
}

/// A view on a [`PathCommands`](struct.PathCommands.html) buffer and
/// two slices for endpoints and control points, providing similar
/// functionalities as `PathSlice`.
#[derive(Copy, Clone)]
pub struct CommandsPathSlice<'l, Endpoint, ControlPoint> {
    endpoints: &'l [Endpoint],
    control_points: &'l [ControlPoint],
    cmds: PathCommandsSlice<'l>,
}

impl<'l, Endpoint, ControlPoint> CommandsPathSlice<'l, Endpoint, ControlPoint> {
    /// Returns an iterator over the events of the path using IDs.
    pub fn iter(&self) -> Iter {
        self.cmds.iter()
    }

    /// Returns an iterator over the events of the path using endpoint
    /// and control point references.
    pub fn events(&self) -> Events<Endpoint, ControlPoint> {
        Events {
            cmds: CmdIter::new(self.cmds.cmds),
            first_endpoint: 0,
            prev_endpoint: 0,
            endpoints: self.endpoints,
            control_points: self.control_points,
        }
    }
}

impl<'l, Endpoint, ControlPoint> core::ops::Index<EndpointId>
    for CommandsPathSlice<'l, Endpoint, ControlPoint>
{
    type Output = Endpoint;
    fn index(&self, id: EndpointId) -> &Endpoint {
        &self.endpoints[id.to_usize()]
    }
}

impl<'l, Endpoint, ControlPoint> core::ops::Index<ControlPointId>
    for CommandsPathSlice<'l, Endpoint, ControlPoint>
{
    type Output = ControlPoint;
    fn index(&self, id: ControlPointId) -> &ControlPoint {
        &self.control_points[id.to_usize()]
    }
}

impl<'l, Endpoint, ControlPoint> fmt::Debug for CommandsPathSlice<'l, Endpoint, ControlPoint>
where
    Endpoint: fmt::Debug,
    ControlPoint: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ ")?;
        for evt in self.events() {
            match evt {
                Event::Line { to, .. } => write!(f, "L {to:?}"),
                Event::Quadratic { ctrl, to, .. } => write!(f, "Q {ctrl:?} {to:?} "),
                Event::Cubic {
                    ctrl1, ctrl2, to, ..
                } => write!(f, "C {ctrl1:?} {ctrl2:?} {to:?} "),
                Event::Begin { at, .. } => write!(f, "M {at:?} "),
                Event::End { close: true, .. } => write!(f, "Z "),
                Event::End { close: false, .. } => Ok(()),
            }?;
        }
        write!(f, "}}")
    }
}

/// Builds path commands.
///
/// See [`PathCommands`](struct.PathCommands.html).
#[derive(Debug, Default, Clone)]
pub struct PathCommandsBuilder {
    cmds: Vec<u32>,
    first_event_index: u32,
    validator: DebugValidator,
}

impl PathCommandsBuilder {
    /// Creates a builder without allocating memory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a pre-allocated builder.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            cmds: Vec::with_capacity(cap),
            ..Self::default()
        }
    }

    pub fn begin(&mut self, to: EndpointId) -> EventId {
        self.validator.begin();

        self.first_event_index = self.cmds.len() as u32;
        let id = EventId(self.cmds.len() as u32);
        self.cmds.push(verb::BEGIN);
        self.cmds.push(to.0);

        id
    }

    pub fn end(&mut self, close: bool) -> Option<EventId> {
        self.validator.end();

        let id = EventId(self.cmds.len() as u32);
        let cmd = if close { verb::CLOSE } else { verb::END };
        self.cmds.push(cmd);
        self.cmds.push(self.first_event_index);

        Some(id)
    }

    pub fn line_to(&mut self, to: EndpointId) -> EventId {
        self.validator.edge();

        let id = EventId(self.cmds.len() as u32);
        self.cmds.push(verb::LINE);
        self.cmds.push(to.0);

        id
    }

    pub fn quadratic_bezier_to(&mut self, ctrl: ControlPointId, to: EndpointId) -> EventId {
        self.validator.edge();

        let id = EventId(self.cmds.len() as u32);
        self.cmds.push(verb::QUADRATIC);
        self.cmds.push(ctrl.0);
        self.cmds.push(to.0);

        id
    }

    pub fn cubic_bezier_to(
        &mut self,
        ctrl1: ControlPointId,
        ctrl2: ControlPointId,
        to: EndpointId,
    ) -> EventId {
        self.validator.edge();

        let id = EventId(self.cmds.len() as u32);
        self.cmds.push(verb::CUBIC);
        self.cmds.push(ctrl1.0);
        self.cmds.push(ctrl2.0);
        self.cmds.push(to.0);

        id
    }

    /// Consumes the builder and returns path commands.
    pub fn build(self) -> PathCommands {
        self.validator.build();

        PathCommands {
            cmds: self.cmds.into_boxed_slice(),
        }
    }
}

/// An iterator of `Event<&Endpoint, &ControlPoint>`.
#[derive(Clone)]
pub struct Events<'l, Endpoint, ControlPoint> {
    cmds: CmdIter<'l>,
    prev_endpoint: usize,
    first_endpoint: usize,
    endpoints: &'l [Endpoint],
    control_points: &'l [ControlPoint],
}

impl<'l, Endpoint, ControlPoint> Iterator for Events<'l, Endpoint, ControlPoint> {
    type Item = Event<&'l Endpoint, &'l ControlPoint>;

    #[inline]
    fn next(&mut self) -> Option<Event<&'l Endpoint, &'l ControlPoint>> {
        match self.cmds.next() {
            Some(verb::BEGIN) => {
                let to = self.cmds.next().unwrap() as usize;
                self.prev_endpoint = to;
                self.first_endpoint = to;
                Some(Event::Begin {
                    at: &self.endpoints[to],
                })
            }
            Some(verb::LINE) => {
                let to = self.cmds.next().unwrap() as usize;
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                Some(Event::Line {
                    from: &self.endpoints[from],
                    to: &self.endpoints[to],
                })
            }
            Some(verb::QUADRATIC) => {
                let ctrl = self.cmds.next().unwrap() as usize;
                let to = self.cmds.next().unwrap() as usize;
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                Some(Event::Quadratic {
                    from: &self.endpoints[from],
                    ctrl: &self.control_points[ctrl],
                    to: &self.endpoints[to],
                })
            }
            Some(verb::CUBIC) => {
                let ctrl1 = self.cmds.next().unwrap() as usize;
                let ctrl2 = self.cmds.next().unwrap() as usize;
                let to = self.cmds.next().unwrap() as usize;
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                Some(Event::Cubic {
                    from: &self.endpoints[from],
                    ctrl1: &self.control_points[ctrl1],
                    ctrl2: &self.control_points[ctrl2],
                    to: &self.endpoints[to],
                })
            }
            Some(verb::END) => {
                let _first_index = self.cmds.next();
                let last = self.prev_endpoint;
                let first = self.first_endpoint;
                self.prev_endpoint = first;
                Some(Event::End {
                    last: &self.endpoints[last],
                    first: &self.endpoints[first],
                    close: false,
                })
            }
            Some(_) => {
                // CLOSE
                let _first_index = self.cmds.next();
                let last = self.prev_endpoint;
                let first = self.first_endpoint;
                self.prev_endpoint = first;
                Some(Event::End {
                    last: &self.endpoints[last],
                    first: &self.endpoints[first],
                    close: true,
                })
            }
            None => None,
        }
    }
}

impl<'l, Ep, Cp> Events<'l, Ep, Cp>
where
    Ep: Position,
    Cp: Position,
{
    pub fn points(self) -> PointEvents<'l, Ep, Cp> {
        PointEvents {
            cmds: self.cmds,
            prev_endpoint: self.prev_endpoint,
            first_endpoint: self.first_endpoint,
            endpoints: self.endpoints,
            control_points: self.control_points,
        }
    }
}
/// An iterator of `Event<&Endpoint, &ControlPoint>`.
#[derive(Clone)]
pub struct Iter<'l> {
    cmds: CmdIter<'l>,
    idx: u32,
    prev_endpoint: EndpointId,
    first_endpoint: EndpointId,
}

impl<'l> Iter<'l> {
    fn new(cmds: &'l [u32]) -> Self {
        Iter {
            cmds: CmdIter::new(cmds),
            idx: 0,
            prev_endpoint: EndpointId(0),
            first_endpoint: EndpointId(0),
        }
    }
}

impl<'l> Iterator for Iter<'l> {
    type Item = IdEvent;

    #[inline]
    fn next(&mut self) -> Option<IdEvent> {
        match self.cmds.next() {
            Some(verb::BEGIN) => {
                let to = EndpointId(self.cmds.next().unwrap());
                self.prev_endpoint = to;
                self.first_endpoint = to;
                self.idx += 2;
                Some(IdEvent::Begin { at: to })
            }
            Some(verb::LINE) => {
                let to = EndpointId(self.cmds.next().unwrap());
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                self.idx += 2;
                Some(IdEvent::Line { from, to })
            }
            Some(verb::QUADRATIC) => {
                let ctrl = ControlPointId(self.cmds.next().unwrap());
                let to = EndpointId(self.cmds.next().unwrap());
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                self.idx += 3;
                Some(IdEvent::Quadratic { from, ctrl, to })
            }
            Some(verb::CUBIC) => {
                let ctrl1 = ControlPointId(self.cmds.next().unwrap());
                let ctrl2 = ControlPointId(self.cmds.next().unwrap());
                let to = EndpointId(self.cmds.next().unwrap());
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                self.idx += 4;
                Some(IdEvent::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                })
            }
            Some(verb::END) => {
                let _first_index = self.cmds.next();
                let last = self.prev_endpoint;
                let first = self.first_endpoint;
                self.prev_endpoint = first;
                self.idx += 2;
                Some(IdEvent::End {
                    last,
                    first,
                    close: false,
                })
            }
            Some(_) => {
                let _first_index = self.cmds.next();
                let last = self.prev_endpoint;
                let first = self.first_endpoint;
                self.prev_endpoint = first;
                self.idx += 2;
                Some(IdEvent::End {
                    last,
                    first,
                    close: true,
                })
            }
            None => None,
        }
    }
}

/// An iterator of `PathEvent`.
#[derive(Clone)]
pub struct PointEvents<'l, Endpoint, ControlPoint> {
    cmds: CmdIter<'l>,
    prev_endpoint: usize,
    first_endpoint: usize,
    endpoints: &'l [Endpoint],
    control_points: &'l [ControlPoint],
}

impl<'l, Endpoint, ControlPoint> Iterator for PointEvents<'l, Endpoint, ControlPoint>
where
    Endpoint: Position,
    ControlPoint: Position,
{
    type Item = PathEvent;

    #[inline]
    fn next(&mut self) -> Option<PathEvent> {
        match self.cmds.next() {
            Some(verb::BEGIN) => {
                let to = self.cmds.next().unwrap() as usize;
                self.prev_endpoint = to;
                self.first_endpoint = to;
                Some(Event::Begin {
                    at: self.endpoints[to].position(),
                })
            }
            Some(verb::LINE) => {
                let to = self.cmds.next().unwrap() as usize;
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                Some(Event::Line {
                    from: self.endpoints[from].position(),
                    to: self.endpoints[to].position(),
                })
            }
            Some(verb::QUADRATIC) => {
                let ctrl = self.cmds.next().unwrap() as usize;
                let to = self.cmds.next().unwrap() as usize;
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                Some(Event::Quadratic {
                    from: self.endpoints[from].position(),
                    ctrl: self.control_points[ctrl].position(),
                    to: self.endpoints[to].position(),
                })
            }
            Some(verb::CUBIC) => {
                let ctrl1 = self.cmds.next().unwrap() as usize;
                let ctrl2 = self.cmds.next().unwrap() as usize;
                let to = self.cmds.next().unwrap() as usize;
                let from = self.prev_endpoint;
                self.prev_endpoint = to;
                Some(Event::Cubic {
                    from: self.endpoints[from].position(),
                    ctrl1: self.control_points[ctrl1].position(),
                    ctrl2: self.control_points[ctrl2].position(),
                    to: self.endpoints[to].position(),
                })
            }
            Some(verb::END) => {
                let _first_index = self.cmds.next();
                let last = self.prev_endpoint;
                let first = self.first_endpoint;
                self.prev_endpoint = first;
                Some(Event::End {
                    last: self.endpoints[last].position(),
                    first: self.endpoints[first].position(),
                    close: false,
                })
            }
            Some(_) => {
                let _first_index = self.cmds.next();
                let last = self.prev_endpoint;
                let first = self.first_endpoint;
                self.prev_endpoint = first;
                Some(Event::End {
                    last: self.endpoints[last].position(),
                    first: self.endpoints[first].position(),
                    close: true,
                })
            }
            None => None,
        }
    }
}

impl<'l, Endpoint, ControlPoint> PositionStore for CommandsPathSlice<'l, Endpoint, ControlPoint>
where
    Endpoint: Position,
    ControlPoint: Position,
{
    fn get_endpoint(&self, id: EndpointId) -> Point {
        self[id].position()
    }

    fn get_control_point(&self, id: ControlPointId) -> Point {
        self[id].position()
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn missing_begin_1() {
    let mut builder = PathCommands::builder();
    builder.line_to(EndpointId(1));
    builder.end(true);

    builder.build();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn missing_begin_2() {
    let mut builder = PathCommands::builder();
    builder.begin(EndpointId(0));
    builder.line_to(EndpointId(1));
    builder.end(true);

    builder.line_to(EndpointId(1));
    builder.end(true);

    builder.build();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn missing_end() {
    let mut builder = PathCommands::builder();
    builder.begin(EndpointId(0));
    builder.line_to(EndpointId(1));

    builder.build();
}

#[test]
fn simple_path() {
    let mut builder = PathCommands::builder();
    builder.begin(EndpointId(0));
    builder.line_to(EndpointId(1));
    builder.quadratic_bezier_to(ControlPointId(2), EndpointId(3));
    builder.cubic_bezier_to(ControlPointId(4), ControlPointId(5), EndpointId(6));
    builder.end(false);

    builder.begin(EndpointId(10));
    builder.line_to(EndpointId(11));
    builder.quadratic_bezier_to(ControlPointId(12), EndpointId(13));
    builder.cubic_bezier_to(ControlPointId(14), ControlPointId(15), EndpointId(16));
    builder.end(true);

    builder.begin(EndpointId(20));
    builder.line_to(EndpointId(21));
    builder.quadratic_bezier_to(ControlPointId(22), EndpointId(23));
    builder.cubic_bezier_to(ControlPointId(24), ControlPointId(25), EndpointId(26));
    builder.end(false);

    let path = builder.build();
    let mut iter = path.iter();
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
        Some(IdEvent::Quadratic {
            from: EndpointId(1),
            ctrl: ControlPointId(2),
            to: EndpointId(3)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Cubic {
            from: EndpointId(3),
            ctrl1: ControlPointId(4),
            ctrl2: ControlPointId(5),
            to: EndpointId(6)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::End {
            last: EndpointId(6),
            first: EndpointId(0),
            close: false
        })
    );

    assert_eq!(iter.next(), Some(IdEvent::Begin { at: EndpointId(10) }));
    assert_eq!(
        iter.next(),
        Some(IdEvent::Line {
            from: EndpointId(10),
            to: EndpointId(11)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Quadratic {
            from: EndpointId(11),
            ctrl: ControlPointId(12),
            to: EndpointId(13)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Cubic {
            from: EndpointId(13),
            ctrl1: ControlPointId(14),
            ctrl2: ControlPointId(15),
            to: EndpointId(16)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::End {
            last: EndpointId(16),
            first: EndpointId(10),
            close: true
        })
    );

    assert_eq!(iter.next(), Some(IdEvent::Begin { at: EndpointId(20) }));
    assert_eq!(
        iter.next(),
        Some(IdEvent::Line {
            from: EndpointId(20),
            to: EndpointId(21)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Quadratic {
            from: EndpointId(21),
            ctrl: ControlPointId(22),
            to: EndpointId(23)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::Cubic {
            from: EndpointId(23),
            ctrl1: ControlPointId(24),
            ctrl2: ControlPointId(25),
            to: EndpointId(26)
        })
    );
    assert_eq!(
        iter.next(),
        Some(IdEvent::End {
            last: EndpointId(26),
            first: EndpointId(20),
            close: false
        })
    );

    assert_eq!(iter.next(), None);
}

#[test]
fn next_event() {
    let mut builder = PathCommands::builder();
    builder.begin(EndpointId(0));
    builder.line_to(EndpointId(1));
    builder.quadratic_bezier_to(ControlPointId(2), EndpointId(3));
    builder.cubic_bezier_to(ControlPointId(4), ControlPointId(5), EndpointId(6));
    builder.end(false);

    builder.begin(EndpointId(10));
    builder.line_to(EndpointId(11));
    builder.quadratic_bezier_to(ControlPointId(12), EndpointId(13));
    builder.cubic_bezier_to(ControlPointId(14), ControlPointId(15), EndpointId(16));
    builder.end(true);

    builder.begin(EndpointId(20));
    builder.line_to(EndpointId(21));
    builder.quadratic_bezier_to(ControlPointId(22), EndpointId(23));
    builder.cubic_bezier_to(ControlPointId(24), ControlPointId(25), EndpointId(26));
    builder.end(false);

    let path = builder.build();

    let mut id = EventId(0);
    let first = id;
    assert_eq!(path.event(id), IdEvent::Begin { at: EndpointId(0) });
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Line {
            from: EndpointId(0),
            to: EndpointId(1)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Quadratic {
            from: EndpointId(1),
            ctrl: ControlPointId(2),
            to: EndpointId(3)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Cubic {
            from: EndpointId(3),
            ctrl1: ControlPointId(4),
            ctrl2: ControlPointId(5),
            to: EndpointId(6)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::End {
            last: EndpointId(6),
            first: EndpointId(0),
            close: false
        }
    );

    assert_eq!(path.next_event_id_in_sub_path(id), first);

    id = path.next_event_id_in_path(id).unwrap();
    let first = id;
    assert_eq!(path.event(id), IdEvent::Begin { at: EndpointId(10) });
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Line {
            from: EndpointId(10),
            to: EndpointId(11)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Quadratic {
            from: EndpointId(11),
            ctrl: ControlPointId(12),
            to: EndpointId(13)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Cubic {
            from: EndpointId(13),
            ctrl1: ControlPointId(14),
            ctrl2: ControlPointId(15),
            to: EndpointId(16)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::End {
            last: EndpointId(16),
            first: EndpointId(10),
            close: true
        }
    );

    assert_eq!(path.next_event_id_in_sub_path(id), first);

    id = path.next_event_id_in_path(id).unwrap();
    let first = id;
    assert_eq!(path.event(id), IdEvent::Begin { at: EndpointId(20) });
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Line {
            from: EndpointId(20),
            to: EndpointId(21)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Quadratic {
            from: EndpointId(21),
            ctrl: ControlPointId(22),
            to: EndpointId(23)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::Cubic {
            from: EndpointId(23),
            ctrl1: ControlPointId(24),
            ctrl2: ControlPointId(25),
            to: EndpointId(26)
        }
    );
    id = path.next_event_id_in_path(id).unwrap();
    assert_eq!(
        path.event(id),
        IdEvent::End {
            last: EndpointId(26),
            first: EndpointId(20),
            close: false
        }
    );

    assert_eq!(path.next_event_id_in_path(id), None);
    assert_eq!(path.next_event_id_in_sub_path(id), first);
}
