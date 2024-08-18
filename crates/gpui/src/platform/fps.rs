use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

const NANOS_PER_SEC: u64 = 1_000_000_000;
const WINDOW_SIZE: usize = 128;

/// Represents a rolling FPS (Frames Per Second) counter.
///
/// This struct provides a lock-free mechanism to measure and calculate FPS
/// continuously, updating with every frame. It uses atomic operations to
/// ensure thread-safety without the need for locks.
pub struct FpsCounter {
    frame_times: [AtomicU64; WINDOW_SIZE],
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl FpsCounter {
    /// Creates a new `Fps` counter.
    ///
    /// Returns an `Arc<Fps>` for safe sharing across threads.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            frame_times: std::array::from_fn(|_| AtomicU64::new(0)),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        })
    }

    /// Increments the FPS counter with a new frame timestamp.
    ///
    /// This method updates the internal state to maintain a rolling window
    /// of frame data for the last second. It uses atomic operations to
    /// ensure thread-safety.
    ///
    /// # Arguments
    ///
    /// * `timestamp_ns` - The timestamp of the new frame in nanoseconds.
    pub fn increment(&self, timestamp_ns: u64) {
        let mut head = self.head.load(Ordering::Relaxed);
        let mut tail = self.tail.load(Ordering::Relaxed);

        // Add new timestamp
        self.frame_times[head].store(timestamp_ns, Ordering::Relaxed);
        // Increment head and wrap around to 0 if it reaches WINDOW_SIZE
        head = (head + 1) % WINDOW_SIZE;
        self.head.store(head, Ordering::Relaxed);

        // Remove old timestamps (older than 1 second)
        while tail != head {
            let oldest = self.frame_times[tail].load(Ordering::Relaxed);
            if timestamp_ns.wrapping_sub(oldest) <= NANOS_PER_SEC {
                break;
            }
            // Increment tail and wrap around to 0 if it reaches WINDOW_SIZE
            tail = (tail + 1) % WINDOW_SIZE;
            self.tail.store(tail, Ordering::Relaxed);
        }
    }

    /// Calculates and returns the current FPS.
    ///
    /// This method computes the FPS based on the frames recorded in the last second.
    /// It uses atomic loads to ensure thread-safety.
    ///
    /// # Returns
    ///
    /// The calculated FPS as a `f32`, or 0.0 if no frames have been recorded.
    pub fn fps(&self) -> f32 {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);

        if head == tail {
            return 0.0;
        }

        let newest =
            self.frame_times[head.wrapping_sub(1) & (WINDOW_SIZE - 1)].load(Ordering::Relaxed);
        let oldest = self.frame_times[tail].load(Ordering::Relaxed);

        let time_diff = newest.wrapping_sub(oldest) as f32;
        if time_diff == 0.0 {
            return 0.0;
        }

        let frame_count = if head > tail {
            head - tail
        } else {
            WINDOW_SIZE - tail + head
        };

        (frame_count as f32 - 1.0) * NANOS_PER_SEC as f32 / time_diff
    }
}
