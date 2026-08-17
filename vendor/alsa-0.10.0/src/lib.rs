//! Thin but safe wrappers for [ALSA](https://alsa-project.org).
//!
//! [GitHub repo](https://github.com/diwic/alsa-rs)
//!
//! [Crates.io](https://crates.io/crates/alsa)
//!
//! This ALSA API wrapper/binding is WIP - the ALSA API is huge, and new
//! functions and structs might be added as requested.
//!
//! Most functions map 1-to-1 to alsa-lib functions, e g, `ctl::CardInfo::get_id()` is a wrapper around
//! `snd_ctl_card_info_get_id` and the [alsa-lib documentation](https://www.alsa-project.org/alsa-doc/alsa-lib/)
//! can be consulted for additional information.
//!
//! Enjoy!

#![allow(clippy::all)]
#![warn(clippy::correctness, clippy::suspicious, clippy::perf)]

extern crate alsa_sys as alsa;
extern crate libc;
#[macro_use]
extern crate bitflags;

macro_rules! alsa_enum {
 ($(#[$attr:meta])+ $name:ident, $static_name:ident [$count:expr], $( $a:ident = $b:ident),* ,) =>
{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
$(#[$attr])*
pub enum $name {
$(
    $a = alsa::$b as isize,
)*
}

static $static_name: [$name; $count] =
  [ $( $name::$a, )* ];

impl $name {
    /// Returns a slice of all possible values; useful for iteration
    pub fn all() -> &'static [$name] { &$static_name[..] }

    #[allow(dead_code)]
    fn from_c_int(c: ::libc::c_int, s: &'static str) -> Result<$name> {
        Self::all().iter().find(|&&x| c == x as ::libc::c_int).map(|&x| x)
            .ok_or_else(|| Error::unsupported(s))
    }

    #[allow(dead_code)]
    fn to_c_int(&self) -> ::libc::c_int {
        return *self as ::libc::c_int;
    }
}

}
}

/// Replaces constants ending with PLAYBACK/CAPTURE as well as
/// INPUT/OUTPUT
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Playback,
    Capture
}
impl Direction {
    #[inline]
    pub fn input() -> Direction { Direction::Capture }
    #[inline]
    pub fn output() -> Direction { Direction::Playback }
}

/// Used to restrict hw parameters. In case the submitted
/// value is unavailable, in which direction should one search
/// for available values?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueOr {
    /// The value set is the submitted value, or less
    Less = -1,
    /// The value set is the submitted value, or the nearest
    Nearest = 0,
    /// The value set is the submitted value, or greater
    Greater = 1,
}

/// Rounding mode (used in some mixer related calls)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Round {
    /// Round down (towards negative infinity)
    Floor = 0,
    /// Round up (towards positive infinity)
    Ceil = 1,
}

mod error;
pub use crate::error::{Error, Result};

pub mod card;
pub use crate::card::Card as Card;

mod ctl_int;
pub mod ctl {
    //! Control device API
    pub use super::ctl_int::{Ctl, CardInfo, DeviceIter, ElemIface, ElemId, ElemList, ElemType, ElemValue, ElemInfo};
}

pub use crate::ctl::Ctl as Ctl;

pub mod hctl;
pub use crate::hctl::HCtl as HCtl;

pub mod pcm;
pub use crate::pcm::PCM as PCM;

pub mod config;

pub mod rawmidi;
pub use crate::rawmidi::Rawmidi as Rawmidi;

pub mod device_name;

pub mod poll;
pub use crate::poll::Descriptors as PollDescriptors;

pub mod mixer;
pub use crate::mixer::Mixer as Mixer;

pub mod seq;
pub use crate::seq::Seq as Seq;

mod io;
pub use crate::io::Output;

// Reexported inside PCM module
mod chmap;

pub mod direct;
