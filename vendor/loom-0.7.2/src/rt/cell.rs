use crate::rt::location::{self, Location, LocationSet};
use crate::rt::{self, object, thread, VersionVec};

/// Tracks immutable and mutable access to a single memory cell.
#[derive(Debug)]
pub(crate) struct Cell {
    state: object::Ref<State>,
}

#[derive(Debug)]
pub(super) struct State {
    /// Where the cell was created
    created_location: Location,

    /// Number of threads currently reading the cell
    is_reading: usize,

    /// `true` if in a `with_mut` closure.
    is_writing: bool,

    /// The transitive closure of all immutable accesses of `data`.
    read_access: VersionVec,

    /// Location for the *last* time a thread read from the cell.
    read_locations: LocationSet,

    /// The last mutable access of `data`.
    write_access: VersionVec,

    /// Location for the *last* time a thread wrote to the cell
    write_locations: LocationSet,
}

#[derive(Debug)]
pub(crate) struct Reading {
    state: object::Ref<State>,
}

#[derive(Debug)]
pub(crate) struct Writing {
    state: object::Ref<State>,
}

impl Cell {
    pub(crate) fn new(location: Location) -> Cell {
        rt::execution(|execution| {
            let state = State::new(&execution.threads, location);

            Cell {
                state: execution.objects.insert(state),
            }
        })
    }

    pub(crate) fn start_read(&self, location: Location) -> Reading {
        // Enter the read closure
        rt::synchronize(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            assert!(!state.is_writing, "currently writing to cell");

            state.is_reading += 1;
            state.read_locations.track(location, &execution.threads);
            state.track_read(&execution.threads);

            Reading { state: self.state }
        })
    }

    pub(crate) fn start_write(&self, location: Location) -> Writing {
        // Enter the write closure
        rt::synchronize(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            assert!(state.is_reading == 0, "currently reading from cell");
            assert!(!state.is_writing, "currently writing to cell");

            state.is_writing = true;
            state.write_locations.track(location, &execution.threads);
            state.track_write(&execution.threads);

            Writing { state: self.state }
        })
    }
}

impl State {
    fn new(threads: &thread::Set, location: Location) -> State {
        let version = threads.active().causality;

        State {
            created_location: location,
            is_reading: 0,
            is_writing: false,
            read_access: version,
            read_locations: LocationSet::new(),
            write_access: version,
            write_locations: LocationSet::new(),
        }
    }

    /// Perform a read access
    fn track_read(&mut self, threads: &thread::Set) {
        let current = &threads.active().causality;

        // Check that there is no concurrent mutable access, i.e., the last
        // mutable access must happen-before this immutable access.
        if let Some(writer) = current.ahead(&self.write_access) {
            location::panic("Causality violation: Concurrent read and write accesses.")
                .location("created", self.created_location)
                .thread("read", threads.active_id(), self.read_locations[threads])
                .thread("write", writer, self.write_locations[writer])
                .fire();
        }

        self.read_access.join(current);
    }

    fn track_write(&mut self, threads: &thread::Set) {
        let current = &threads.active().causality;

        // Check that there is no concurrent mutable access, i.e., the last
        // mutable access must happen-before this mutable access.
        if let Some(other) = current.ahead(&self.write_access) {
            location::panic("Causality violation: Concurrent write accesses to `UnsafeCell`.")
                .location("created", self.created_location)
                .thread("write one", other, self.write_locations[other])
                .thread(
                    "write two",
                    threads.active_id(),
                    self.write_locations[threads],
                )
                .fire();
        }

        // Check that there are no concurrent immutable accesses, i.e., every
        // immutable access must happen-before this mutable access.
        if let Some(reader) = current.ahead(&self.read_access) {
            location::panic(
                "Causality violation: Concurrent read and write accesses to `UnsafeCell`.",
            )
            .location("created", self.created_location)
            .thread("read", reader, self.read_locations[reader])
            .thread("write", threads.active_id(), self.write_locations[threads])
            .fire();
        }

        self.write_access.join(current);
    }
}

// === impl Reading ===

impl Drop for Reading {
    fn drop(&mut self) {
        rt::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            assert!(state.is_reading > 0);
            assert!(!state.is_writing);

            state.is_reading -= 1;

            if !std::thread::panicking() {
                state.track_read(&execution.threads);
            }
        })
    }
}

// === impl Writing ===

impl Drop for Writing {
    fn drop(&mut self) {
        rt::execution(|execution| {
            let state = self.state.get_mut(&mut execution.objects);

            assert!(state.is_writing);
            assert!(state.is_reading == 0);

            state.is_writing = false;

            if !std::thread::panicking() {
                state.track_write(&execution.threads);
            }
        })
    }
}
