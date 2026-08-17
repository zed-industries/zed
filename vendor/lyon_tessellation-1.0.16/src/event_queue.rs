use crate::fill::{compare_positions, is_after};
use crate::geom::{CubicBezierSegment, LineSegment, QuadraticBezierSegment};
use crate::math::{point, Point};
use crate::path::private::DebugValidator;
use crate::path::{EndpointId, IdEvent, PathEvent, PositionStore};
use crate::Orientation;

use core::cmp::Ordering;
use core::mem::swap;
use core::ops::Range;
use alloc::vec::Vec;

#[inline]
fn reorient(p: Point) -> Point {
    point(-p.y, p.x)
}

pub(crate) type TessEventId = u32;

pub(crate) const INVALID_EVENT_ID: TessEventId = u32::MAX;

pub(crate) struct Event {
    pub next_sibling: TessEventId,
    pub next_event: TessEventId,
    pub position: Point,
}

#[derive(Clone, Debug)]
pub(crate) struct EdgeData {
    pub to: Point,
    pub range: core::ops::Range<f32>,
    pub winding: i16,
    pub is_edge: bool,
    pub from_id: EndpointId,
    pub to_id: EndpointId,
}

#[doc(hidden)]
/// A queue of sorted events for the fill tessellator's sweep-line algorithm.
pub struct EventQueue {
    pub(crate) events: Vec<Event>,
    pub(crate) edge_data: Vec<EdgeData>,
    first: TessEventId,
    sorted: bool,
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            events: Vec::new(),
            edge_data: Vec::new(),
            first: INVALID_EVENT_ID,
            sorted: false,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        EventQueue {
            events: Vec::with_capacity(cap),
            edge_data: Vec::with_capacity(cap),
            first: 0,
            sorted: false,
        }
    }

    pub fn reset(&mut self) {
        self.events.clear();
        self.edge_data.clear();
        self.first = INVALID_EVENT_ID;
        self.sorted = false;
    }

    /// Creates an `EventQueue` from an iterator of path event and a tolerance threshold.
    ///
    /// The tolerance threshold is used for curve flattening approximation. See the
    /// [Flattening and tolerance](index.html#flattening-and-tolerance) section of the
    /// crate documentation.
    pub fn from_path(tolerance: f32, path: impl IntoIterator<Item = PathEvent>) -> Self {
        let path = path.into_iter();
        let (min, max) = path.size_hint();
        let capacity = max.unwrap_or(min);
        let mut builder = EventQueueBuilder::with_capacity(capacity, tolerance);
        builder.set_path(tolerance, Orientation::Vertical, path);

        builder.build()
    }

    /// Creates an `EventQueue` from an an iterator over endpoint and control
    /// point ids, storage for the positions and, optionally, storage for
    /// custom endpoint attributes.
    ///
    /// The tolerance threshold is used for curve flattening approximation. See the
    /// [Flattening and tolerance](index.html#flattening-and-tolerance) section of the
    /// crate documentation.
    pub fn from_path_with_ids(
        tolerance: f32,
        sweep_orientation: Orientation,
        path: impl IntoIterator<Item = IdEvent>,
        positions: &impl PositionStore,
    ) -> Self {
        let path = path.into_iter();
        let (min, max) = path.size_hint();
        let capacity = max.unwrap_or(min);
        let mut builder = EventQueueBuilder::with_capacity(capacity, tolerance);
        builder.set_path_with_ids(tolerance, sweep_orientation, path, positions);

        builder.build()
    }

    pub fn into_builder(mut self, tolerance: f32) -> EventQueueBuilder {
        self.reset();
        EventQueueBuilder {
            queue: self,
            current: point(f32::NAN, f32::NAN),
            prev: point(f32::NAN, f32::NAN),
            second: point(f32::NAN, f32::NAN),
            nth: 0,
            tolerance,
            prev_endpoint_id: EndpointId(u32::MAX),
            validator: DebugValidator::new(),
        }
    }

    pub fn reserve(&mut self, n: usize) {
        self.events.reserve(n);
    }

    fn push_unsorted(&mut self, position: Point) {
        self.events.push(Event {
            position,
            next_sibling: INVALID_EVENT_ID,
            next_event: INVALID_EVENT_ID,
        });
    }

    fn push_sorted(&mut self, position: Point) {
        self.events.push(Event {
            position,
            next_sibling: u32::MAX,
            next_event: u32::MAX,
        });

        let id = (self.events.len() - 1) as u32;
        let mut current = self.first_id();
        let mut prev = current;
        while self.valid_id(current) {
            let evt_pos = self.events[current as usize].position;
            match compare_positions(position, evt_pos) {
                Ordering::Greater => {}
                Ordering::Equal => {
                    // Add to sibling list.
                    let mut current_sibling = current;
                    let mut next_sibling = self.next_sibling_id(current);
                    while self.valid_id(next_sibling) {
                        current_sibling = next_sibling;
                        next_sibling = self.next_sibling_id(current_sibling);
                    }
                    self.events[current_sibling as usize].next_sibling = id;
                    return;
                }
                Ordering::Less => {
                    if prev != current {
                        // Insert between `prev` and `current`.
                        self.events[prev as usize].next_event = id;
                    } else {
                        // It's the first element.
                        self.first = id;
                    }
                    self.events[id as usize].next_event = current;
                    return;
                }
            }

            prev = current;
            current = self.next_id(current);
        }

        // Append at the end.
        self.events[prev as usize].next_event = id;
    }

    pub(crate) fn insert_sorted(
        &mut self,
        position: Point,
        data: EdgeData,
        after: TessEventId,
    ) -> TessEventId {
        debug_assert!(self.sorted);
        debug_assert!(
            is_after(data.to, position),
            "{:?} should be after {:?}",
            data.to,
            position
        );

        let idx = self.events.len() as TessEventId;
        self.events.push(Event {
            position,
            next_sibling: INVALID_EVENT_ID,
            next_event: INVALID_EVENT_ID,
        });
        self.edge_data.push(data);

        self.insert_into_sorted_list(idx, position, after);

        idx
    }

    pub(crate) fn insert_sibling(&mut self, sibling: TessEventId, position: Point, data: EdgeData) {
        debug_assert!(is_after(data.to, position));
        let idx = self.events.len() as TessEventId;
        let next_sibling = self.events[sibling as usize].next_sibling;

        self.events.push(Event {
            position,
            next_event: INVALID_EVENT_ID,
            next_sibling,
        });

        self.edge_data.push(data);

        self.events[sibling as usize].next_sibling = idx;
    }

    pub(crate) fn vertex_event_sorted(
        &mut self,
        position: Point,
        endpoint_id: EndpointId,
        after: TessEventId,
    ) {
        let idx = self.events.len() as TessEventId;

        self.push_unsorted(position);
        self.edge_data.push(EdgeData {
            to: point(f32::NAN, f32::NAN),
            range: 0.0..0.0,
            winding: 0,
            is_edge: false,
            from_id: endpoint_id,
            to_id: endpoint_id,
        });

        self.insert_into_sorted_list(idx, position, after);
    }

    fn insert_into_sorted_list(&mut self, idx: TessEventId, position: Point, after: TessEventId) {
        let mut prev = after;
        let mut current = after;
        while self.valid_id(current) {
            let pos = self.events[current as usize].position;

            if pos == position {
                debug_assert!(prev != current);
                self.events[idx as usize].next_sibling = self.events[current as usize].next_sibling;
                self.events[current as usize].next_sibling = idx;
                return;
            } else if is_after(pos, position) {
                self.events[prev as usize].next_event = idx;
                self.events[idx as usize].next_event = current;
                return;
            }

            prev = current;
            current = self.next_id(current);
        }

        self.events[prev as usize].next_event = idx;
    }

    pub(crate) fn clear(&mut self) {
        self.events.clear();
        self.first = INVALID_EVENT_ID;
        self.sorted = false;
    }

    /// Returns the ID of the first event in the queue.
    pub(crate) fn first_id(&self) -> TessEventId {
        self.first
    }

    /// Returns the ID of the next (non-sibling) event after the provided one.
    pub(crate) fn next_id(&self, id: TessEventId) -> TessEventId {
        self.events[id as usize].next_event
    }

    /// Returns the ID of the next sibling event after the provided one.
    pub(crate) fn next_sibling_id(&self, id: TessEventId) -> TessEventId {
        self.events[id as usize].next_sibling
    }

    /// Returns whether or not the given event ID is valid.
    pub(crate) fn valid_id(&self, id: TessEventId) -> bool {
        id != INVALID_EVENT_ID
    }

    /// Returns the position of a given event in the queue.
    pub(crate) fn position(&self, id: TessEventId) -> Point {
        self.events[id as usize].position
    }

    fn sort(&mut self) {
        self.sorted = true;

        if self.events.is_empty() {
            return;
        }

        let range = 0..self.events.len();
        self.first = self.merge_sort(range);
    }

    /// Merge sort with two twists:
    /// - Events at the same position are grouped into a "sibling" list.
    /// - We take advantage of having events stored contiguously in a vector
    ///   by recursively splitting ranges of the array instead of traversing
    ///   the lists to find a split point.
    fn merge_sort(&mut self, range: Range<usize>) -> TessEventId {
        let split = (range.start + range.end) / 2;

        if split == range.start {
            return range.start as TessEventId;
        }

        let a_head = self.merge_sort(range.start..split);
        let b_head = self.merge_sort(split..range.end);

        self.merge(a_head, b_head)
    }

    fn merge(&mut self, mut a: TessEventId, mut b: TessEventId) -> TessEventId {
        if a == INVALID_EVENT_ID {
            return b;
        }
        if b == INVALID_EVENT_ID {
            return a;
        }

        debug_assert!(a != b);
        let mut first = true;
        let mut sorted_head = INVALID_EVENT_ID;
        let mut prev = INVALID_EVENT_ID;

        loop {
            if a == INVALID_EVENT_ID {
                if !first {
                    self.events[prev as usize].next_event = b;
                }
                break;
            }

            if b == INVALID_EVENT_ID {
                if !first {
                    self.events[prev as usize].next_event = a;
                }
                break;
            }

            let node;
            match compare_positions(
                self.events[a as usize].position,
                self.events[b as usize].position,
            ) {
                Ordering::Less => {
                    node = a;
                    a = self.events[a as usize].next_event;
                }
                Ordering::Greater => {
                    node = b;
                    b = self.events[b as usize].next_event;
                }
                Ordering::Equal => {
                    // Add b to a's sibling list.
                    let a_sib = self.find_last_sibling(a) as usize;
                    self.events[a_sib].next_sibling = b;

                    b = self.events[b as usize].next_event;

                    continue;
                }
            }

            if first {
                first = false;
                sorted_head = node;
            } else {
                self.events[prev as usize].next_event = node;
            }

            prev = node;
        }

        if sorted_head == INVALID_EVENT_ID {
            sorted_head = a;
        }

        sorted_head
    }

    fn find_last_sibling(&self, id: TessEventId) -> TessEventId {
        let mut current_sibling = id;
        let mut next_sibling = self.next_sibling_id(id);
        while self.valid_id(next_sibling) {
            current_sibling = next_sibling;
            next_sibling = self.next_sibling_id(current_sibling);
        }

        current_sibling
    }

    #[cfg(all(debug_assertions, feature = "std"))]
    fn log(&self) {
        let mut iter_count = self.events.len() * self.events.len();

        std::println!("--");
        let mut current = self.first;
        while (current as usize) < self.events.len() {
            assert!(iter_count > 0);
            iter_count -= 1;

            std::print!("[");
            let mut current_sibling = current;
            while (current_sibling as usize) < self.events.len() {
                std::print!("{:?},", self.events[current_sibling as usize].position);
                current_sibling = self.events[current_sibling as usize].next_sibling;
            }
            std::print!("]  ");
            current = self.events[current as usize].next_event;
        }
        std::println!("\n--");
    }

    fn assert_sorted(&self) {
        let mut current = self.first;
        let mut pos = point(f32::MIN, f32::MIN);
        let mut n = 0;
        while self.valid_id(current) {
            assert!(is_after(self.events[current as usize].position, pos));
            pos = self.events[current as usize].position;
            let mut current_sibling = current;
            while self.valid_id(current_sibling) {
                n += 1;
                assert_eq!(self.events[current_sibling as usize].position, pos);
                current_sibling = self.next_sibling_id(current_sibling);
            }
            current = self.next_id(current);
        }
        assert_eq!(n, self.events.len());
    }
}

pub struct EventQueueBuilder {
    current: Point,
    prev: Point,
    second: Point,
    nth: u32,
    queue: EventQueue,
    tolerance: f32,
    prev_endpoint_id: EndpointId,
    validator: DebugValidator,
}

impl EventQueueBuilder {
    pub fn new(tolerance: f32) -> Self {
        EventQueue::new().into_builder(tolerance)
    }

    pub fn with_capacity(cap: usize, tolerance: f32) -> Self {
        EventQueue::with_capacity(cap).into_builder(tolerance)
    }

    pub fn set_tolerance(&mut self, tolerance: f32) {
        self.tolerance = tolerance;
    }

    pub fn build(mut self) -> EventQueue {
        self.validator.build();

        self.queue.sort();

        self.queue
    }

    pub fn set_path(
        &mut self,
        tolerance: f32,
        sweep_orientation: Orientation,
        path: impl IntoIterator<Item = PathEvent>,
    ) {
        self.reset();

        self.tolerance = tolerance;
        let endpoint_id = EndpointId(u32::MAX);
        match sweep_orientation {
            Orientation::Vertical => {
                for evt in path {
                    match evt {
                        PathEvent::Begin { at } => {
                            self.begin(at, endpoint_id);
                        }
                        PathEvent::Line { to, .. } => {
                            self.line_segment(to, endpoint_id, 0.0, 1.0);
                        }
                        PathEvent::Quadratic { ctrl, to, .. } => {
                            self.quadratic_bezier_segment(ctrl, to, endpoint_id);
                        }
                        PathEvent::Cubic {
                            ctrl1, ctrl2, to, ..
                        } => {
                            self.cubic_bezier_segment(ctrl1, ctrl2, to, endpoint_id);
                        }
                        PathEvent::End { first, .. } => {
                            self.end(first, endpoint_id);
                        }
                    }
                }
            }

            Orientation::Horizontal => {
                for evt in path {
                    match evt {
                        PathEvent::Begin { at } => {
                            self.begin(reorient(at), endpoint_id);
                        }
                        PathEvent::Line { to, .. } => {
                            self.line_segment(reorient(to), endpoint_id, 0.0, 1.0);
                        }
                        PathEvent::Quadratic { ctrl, to, .. } => {
                            self.quadratic_bezier_segment(
                                reorient(ctrl),
                                reorient(to),
                                endpoint_id,
                            );
                        }
                        PathEvent::Cubic {
                            ctrl1, ctrl2, to, ..
                        } => {
                            self.cubic_bezier_segment(
                                reorient(ctrl1),
                                reorient(ctrl2),
                                reorient(to),
                                endpoint_id,
                            );
                        }
                        PathEvent::End { first, .. } => {
                            self.end(reorient(first), endpoint_id);
                        }
                    }
                }
            }
        }
    }

    pub fn set_path_with_ids(
        &mut self,
        tolerance: f32,
        sweep_orientation: Orientation,
        path_events: impl IntoIterator<Item = IdEvent>,
        points: &impl PositionStore,
    ) {
        self.reset();

        self.tolerance = tolerance;
        match sweep_orientation {
            Orientation::Vertical => {
                for evt in path_events {
                    match evt {
                        IdEvent::Begin { at } => {
                            self.begin(points.get_endpoint(at), at);
                        }
                        IdEvent::Line { to, .. } => {
                            self.line_segment(points.get_endpoint(to), to, 0.0, 1.0);
                        }
                        IdEvent::Quadratic { ctrl, to, .. } => {
                            self.quadratic_bezier_segment(
                                points.get_control_point(ctrl),
                                points.get_endpoint(to),
                                to,
                            );
                        }
                        IdEvent::Cubic {
                            ctrl1, ctrl2, to, ..
                        } => {
                            self.cubic_bezier_segment(
                                points.get_control_point(ctrl1),
                                points.get_control_point(ctrl2),
                                points.get_endpoint(to),
                                to,
                            );
                        }
                        IdEvent::End { first, .. } => {
                            self.end(points.get_endpoint(first), first);
                        }
                    }
                }
            }

            Orientation::Horizontal => {
                for evt in path_events {
                    match evt {
                        IdEvent::Begin { at } => {
                            self.begin(reorient(points.get_endpoint(at)), at);
                        }
                        IdEvent::Line { to, .. } => {
                            self.line_segment(reorient(points.get_endpoint(to)), to, 0.0, 1.0);
                        }
                        IdEvent::Quadratic { ctrl, to, .. } => {
                            self.quadratic_bezier_segment(
                                reorient(points.get_control_point(ctrl)),
                                reorient(points.get_endpoint(to)),
                                to,
                            );
                        }
                        IdEvent::Cubic {
                            ctrl1, ctrl2, to, ..
                        } => {
                            self.cubic_bezier_segment(
                                reorient(points.get_control_point(ctrl1)),
                                reorient(points.get_control_point(ctrl2)),
                                reorient(points.get_endpoint(to)),
                                to,
                            );
                        }
                        IdEvent::End { first, .. } => {
                            self.end(reorient(points.get_endpoint(first)), first);
                        }
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        self.queue.reset();
        self.nth = 0;
    }

    fn vertex_event(&mut self, at: Point, endpoint_id: EndpointId) {
        self.queue.push_unsorted(at);
        self.queue.edge_data.push(EdgeData {
            to: point(f32::NAN, f32::NAN),
            range: 0.0..0.0,
            winding: 0,
            is_edge: false,
            from_id: endpoint_id,
            to_id: endpoint_id,
        });
    }

    fn vertex_event_on_curve(&mut self, at: Point, t: f32, from_id: EndpointId, to_id: EndpointId) {
        self.queue.push_unsorted(at);
        self.queue.edge_data.push(EdgeData {
            to: point(f32::NAN, f32::NAN),
            range: t..t,
            winding: 0,
            is_edge: false,
            from_id,
            to_id,
        });
    }

    pub fn end(&mut self, first: Point, first_endpoint_id: EndpointId) {
        if self.nth == 0 {
            self.validator.end();
            return;
        }

        // Unless we are already back to the first point, we need to
        // to insert an edge.
        self.line_segment(first, first_endpoint_id, 0.0, 1.0);

        // Since we can only check for the need of a vertex event when
        // we have a previous edge, we skipped it for the first edge
        // and have to do it now.
        if is_after(first, self.prev) && is_after(first, self.second) {
            self.vertex_event(first, first_endpoint_id);
        }

        self.validator.end();

        self.prev_endpoint_id = first_endpoint_id;
        self.nth = 0;
    }

    pub fn begin(&mut self, to: Point, to_id: EndpointId) {
        self.validator.begin();

        self.nth = 0;
        self.current = to;
        self.prev_endpoint_id = to_id;
    }

    #[allow(clippy::too_many_arguments)]
    fn add_edge(
        &mut self,
        edge: &LineSegment<f32>,
        mut winding: i16,
        from_id: EndpointId,
        to_id: EndpointId,
        mut t0: f32,
        mut t1: f32,
    ) {
        if edge.from == edge.to {
            return;
        }

        let mut evt_pos = edge.from;
        let mut evt_to = edge.to;
        if is_after(evt_pos, edge.to) {
            evt_to = evt_pos;
            evt_pos = edge.to;
            swap(&mut t0, &mut t1);
            winding *= -1;
        }

        self.queue.push_unsorted(evt_pos);
        self.queue.edge_data.push(EdgeData {
            to: evt_to,
            range: t0..t1,
            winding,
            is_edge: true,
            from_id,
            to_id,
        });

        self.nth += 1;
    }

    pub fn line_segment(&mut self, to: Point, to_id: EndpointId, t0: f32, t1: f32) {
        self.validator.edge();

        let from = self.current;
        if from == to {
            return;
        }

        if is_after(from, to) && self.nth > 0 && is_after(from, self.prev) {
            self.vertex_event(from, self.prev_endpoint_id);
        }

        if self.nth == 0 {
            self.second = to;
        }

        self.add_edge(
            &LineSegment { from, to },
            1,
            self.prev_endpoint_id,
            to_id,
            t0,
            t1,
        );

        self.prev = self.current;
        self.prev_endpoint_id = to_id;
        self.current = to;
    }

    pub fn quadratic_bezier_segment(&mut self, ctrl: Point, to: Point, to_id: EndpointId) {
        self.validator.edge();
        // Swap the curve so that it always goes downwards. This way if two
        // paths share the same edge with different windings, the flattening will
        // play out the same way, which avoid cracks.

        // We have to put special care into properly tracking the previous and second
        // points as if we hadn't swapped.

        let original = QuadraticBezierSegment {
            from: self.current,
            ctrl,
            to,
        };

        let needs_swap = is_after(original.from, original.to);

        let mut segment = original;
        let mut winding = 1;
        if needs_swap {
            swap(&mut segment.from, &mut segment.to);
            winding = -1;
        }

        let mut prev = segment.from;
        let mut first = None;
        let is_first_edge = self.nth == 0;
        segment.for_each_flattened_with_t(self.tolerance, &mut |line, t| {
            if line.from == line.to {
                return;
            }

            if first.is_none() {
                first = Some(line.to)
            // We can't call vertex(prev, from, to) in the first iteration
            // because if we flipped the curve, we don't have a proper value for
            // the previous vertex yet.
            // We'll handle it after the loop.
            } else if is_after(line.from, line.to) && is_after(line.from, prev) {
                self.vertex_event_on_curve(line.from, t.start, self.prev_endpoint_id, to_id);
            }

            self.add_edge(line, winding, self.prev_endpoint_id, to_id, t.start, t.end);

            prev = line.from;
        });

        if let Some(first) = first {
            let (second, previous) = if needs_swap {
                (prev, first)
            } else {
                (first, prev)
            };

            if is_first_edge {
                self.second = second;
            } else if is_after(original.from, self.prev) && is_after(original.from, second) {
                // Handle the first vertex we took out of the loop above.
                // The missing vertex is always the origin of the edge (before the flip).
                self.vertex_event(original.from, self.prev_endpoint_id);
            }

            self.prev = previous;
            self.current = original.to;
            self.prev_endpoint_id = to_id;
        }
    }

    pub fn cubic_bezier_segment(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        to_id: EndpointId,
    ) {
        self.validator.edge();
        // Swap the curve so that it always goes downwards. This way if two
        // paths share the same edge with different windings, the flattening will
        // play out the same way, which avoid cracks.

        // We have to put special care into properly tracking the previous and second
        // points as if we hadn't swapped.

        let original = CubicBezierSegment {
            from: self.current,
            ctrl1,
            ctrl2,
            to,
        };

        let needs_swap = is_after(original.from, original.to);

        let mut segment = original;
        let mut winding = 1;
        if needs_swap {
            swap(&mut segment.from, &mut segment.to);
            swap(&mut segment.ctrl1, &mut segment.ctrl2);
            winding = -1;
        }

        let mut prev = segment.from;
        let mut first = None;
        let is_first_edge = self.nth == 0;
        segment.for_each_flattened_with_t(self.tolerance, &mut |line, t| {
            if line.from == line.to {
                return;
            }

            if first.is_none() {
                first = Some(line.to)
            // We can't call vertex(prev, from, to) in the first iteration
            // because if we flipped the curve, we don't have a proper value for
            // the previous vertex yet.
            // We'll handle it after the loop.
            } else if is_after(line.from, line.to) && is_after(line.from, prev) {
                self.vertex_event_on_curve(line.from, t.start, self.prev_endpoint_id, to_id);
            }

            self.add_edge(line, winding, self.prev_endpoint_id, to_id, t.start, t.end);

            prev = line.from;
        });

        if let Some(first) = first {
            let (second, previous) = if needs_swap {
                (prev, first)
            } else {
                (first, prev)
            };

            if is_first_edge {
                self.second = second;
            } else if is_after(original.from, self.prev) && is_after(original.from, second) {
                // Handle the first vertex we took out of the loop above.
                // The missing vertex is always the origin of the edge (before the flip).
                self.vertex_event(original.from, self.prev_endpoint_id);
            }

            self.prev = previous;
            self.current = original.to;
            self.prev_endpoint_id = to_id;
        }
    }

    pub fn reserve(&mut self, n: usize) {
        self.queue.reserve(n);
    }
}

#[test]
fn test_event_queue_sort_1() {
    let mut queue = EventQueue::new();
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(2.0, 0.0));
    queue.push_unsorted(point(3.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(6.0, 0.0));

    queue.sort();
    queue.assert_sorted();
}

#[test]
fn test_event_queue_sort_2() {
    let mut queue = EventQueue::new();
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));

    queue.sort();
    queue.assert_sorted();
}

#[test]
fn test_event_queue_sort_3() {
    let mut queue = EventQueue::new();
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(1.0, 0.0));
    queue.push_unsorted(point(2.0, 0.0));
    queue.push_unsorted(point(3.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(5.0, 0.0));

    queue.sort();
    queue.assert_sorted();
}

#[test]
fn test_event_queue_sort_4() {
    let mut queue = EventQueue::new();
    queue.push_unsorted(point(5.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(3.0, 0.0));
    queue.push_unsorted(point(2.0, 0.0));
    queue.push_unsorted(point(1.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));

    queue.sort();
    queue.assert_sorted();
}

#[test]
fn test_event_queue_sort_5() {
    let mut queue = EventQueue::new();
    queue.push_unsorted(point(5.0, 0.0));
    queue.push_unsorted(point(5.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(3.0, 0.0));
    queue.push_unsorted(point(3.0, 0.0));
    queue.push_unsorted(point(2.0, 0.0));
    queue.push_unsorted(point(2.0, 0.0));
    queue.push_unsorted(point(1.0, 0.0));
    queue.push_unsorted(point(1.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));

    queue.sort();
    queue.assert_sorted();
}

#[test]
fn test_event_queue_push_sorted() {
    let mut queue = EventQueue::new();
    queue.push_unsorted(point(5.0, 0.0));
    queue.push_unsorted(point(4.0, 0.0));
    queue.push_unsorted(point(3.0, 0.0));
    queue.push_unsorted(point(2.0, 0.0));
    queue.push_unsorted(point(1.0, 0.0));
    queue.push_unsorted(point(0.0, 0.0));

    queue.sort();
    queue.push_sorted(point(1.5, 0.0));
    queue.assert_sorted();

    queue.push_sorted(point(2.5, 0.0));
    queue.assert_sorted();

    queue.push_sorted(point(2.5, 0.0));
    queue.assert_sorted();

    queue.push_sorted(point(6.5, 0.0));
    queue.assert_sorted();
}

#[test]
fn test_logo() {
    use crate::path::Path;

    let mut path = Path::builder().with_svg();

    crate::extra::rust_logo::build_logo_path(&mut path);
    let path = path.build();

    crate::extra::debugging::find_reduced_test_case(path.as_slice(), &|path: Path| {
        let _ = EventQueue::from_path(0.05, path.iter());
        true
    });
}
