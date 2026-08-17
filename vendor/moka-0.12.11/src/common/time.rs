mod atomic_time;
mod clock;
mod instant;

pub(crate) use atomic_time::AtomicInstant;
pub(crate) use clock::Clock;
pub(crate) use instant::Instant;

#[cfg(test)]
pub(crate) use clock::Mock;
