pub(crate) use cfg::Location;

macro_rules! location {
    () => {{
        let enabled = crate::rt::execution(|execution| execution.location);

        if enabled {
            let location = crate::rt::Location::from(std::panic::Location::caller());

            location
        } else {
            crate::rt::Location::disabled()
        }
    }};
}

use crate::rt::{thread, MAX_THREADS};

use std::ops;

#[derive(Debug)]
pub(super) struct LocationSet {
    locations: [Location; MAX_THREADS],
}

pub(super) struct PanicBuilder {
    msg: String,
    locations: Vec<(String, Option<usize>, Location)>,
}

// ===== impl LocationSet ======

impl LocationSet {
    pub(super) fn new() -> LocationSet {
        LocationSet {
            locations: Default::default(),
        }
    }

    pub(super) fn track(&mut self, location: Location, threads: &thread::Set) {
        let active_id = threads.active_id();
        self.locations[active_id.as_usize()] = location;
    }
}

impl ops::Index<usize> for LocationSet {
    type Output = Location;

    fn index(&self, index: usize) -> &Location {
        self.locations.index(index)
    }
}

impl ops::Index<&thread::Set> for LocationSet {
    type Output = Location;

    fn index(&self, threads: &thread::Set) -> &Location {
        let active_id = threads.active_id();
        self.locations.index(active_id.as_usize())
    }
}

// ===== impl PanicBuilder =====

pub(super) fn panic(msg: impl ToString) -> PanicBuilder {
    PanicBuilder {
        msg: msg.to_string(),
        locations: Vec::new(),
    }
}

impl PanicBuilder {
    pub(super) fn location(&mut self, key: &str, location: Location) -> &mut Self {
        self.locations.push((key.to_string(), None, location));
        self
    }

    pub(super) fn thread(
        &mut self,
        key: &str,
        thread: impl Into<usize>,
        location: Location,
    ) -> &mut Self {
        self.locations
            .push((key.to_string(), Some(thread.into()), location));
        self
    }

    pub(super) fn fire(&self) {
        let mut msg = self.msg.clone();

        let width = self
            .locations
            .iter()
            .filter(|(_, _, location)| location.is_captured())
            .map(|(key, ..)| key.len())
            .max();

        if let Some(width) = width {
            msg = format!("\n{}", msg);
            for (key, thread, location) in &self.locations {
                if !location.is_captured() {
                    continue;
                }
                let spaces: String = (0..width - key.len()).map(|_| " ").collect();

                let th = thread
                    .map(|th| format!("thread #{} @ ", th))
                    .unwrap_or_else(String::new);

                msg.push_str(&format!("\n    {}{}: {}{}", spaces, key, th, location));
            }
        }

        panic!("{}\n", msg);
    }
}

// ===== impl Location cfg =====

mod cfg {
    use std::fmt;

    #[derive(Debug, Default, Clone, Copy)]
    pub(crate) struct Location(Option<&'static std::panic::Location<'static>>);

    impl Location {
        pub(crate) fn from(location: &'static std::panic::Location<'static>) -> Location {
            Location(Some(location))
        }

        pub(crate) fn disabled() -> Location {
            Location(None)
        }

        pub(crate) fn is_captured(&self) -> bool {
            self.0.is_some()
        }
    }

    impl fmt::Display for Location {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(location) = &self.0 {
                location.fmt(fmt)
            } else {
                write!(fmt, "")
            }
        }
    }
}
