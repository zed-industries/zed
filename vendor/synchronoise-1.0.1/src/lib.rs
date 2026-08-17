//! A collection of synchronization primitives that build on the primitives available in the
//! standard library.
//!
//! This library contains the following special-purpose synchronization primitives:
//!
//! * [`CountdownEvent`], a primitive that keeps a counter and allows a thread to wait until the
//!   counter reaches zero.
//! * [`SignalEvent`], a primitive that allows one or more threads to wait on a signal from another
//!   thread.
//! * [`WriterReaderPhaser`], a primitive that allows multiple wait-free "writer critical sections"
//!   against a "reader phase flip" that waits for currently-active writers to finish.
//!
//! [`CountdownEvent`]: struct.CountdownEvent.html
//! [`SignalEvent`]: struct.SignalEvent.html
//! [`WriterReaderPhaser`]: struct.WriterReaderPhaser.html

#![deny(warnings, missing_docs)]

// Name source: http://bulbapedia.bulbagarden.net/wiki/Synchronoise_(move)

extern crate crossbeam_queue;

pub mod event;
pub mod phaser;

pub use event::{CountdownEvent, SignalEvent};
pub use phaser::WriterReaderPhaser;
