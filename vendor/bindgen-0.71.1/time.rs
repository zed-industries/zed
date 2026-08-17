use std::io::{self, Write};
use std::time::{Duration, Instant};

/// RAII timer to measure how long phases take.
#[derive(Debug)]
pub struct Timer<'a> {
    output: bool,
    name: &'a str,
    start: Instant,
}

impl<'a> Timer<'a> {
    /// Creates a Timer with the given name, and starts it. By default,
    /// will print to stderr when it is `drop`'d
    pub fn new(name: &'a str) -> Self {
        Timer {
            output: true,
            name,
            start: Instant::now(),
        }
    }

    /// Sets whether or not the Timer will print a message
    /// when it is dropped.
    pub fn with_output(mut self, output: bool) -> Self {
        self.output = output;
        self
    }

    /// Returns the time elapsed since the timer's creation
    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    fn print_elapsed(&mut self) {
        if self.output {
            let elapsed = self.elapsed();
            let time = (elapsed.as_secs() as f64) * 1e3 +
                f64::from(elapsed.subsec_nanos()) / 1e6;
            let stderr = io::stderr();
            // Arbitrary output format, subject to change.
            writeln!(stderr.lock(), "  time: {time:>9.3} ms.\t{}", self.name)
                .expect("timer write should not fail");
        }
    }
}

impl Drop for Timer<'_> {
    fn drop(&mut self) {
        self.print_elapsed();
    }
}
