/*!
Script aware cluster segmentation.

This module provides support for breaking text into clusters that are
appropriate for shaping with a given script. For most scripts, clusters are
equivalent to Unicode grapheme clusters. More complex scripts, however,
may produce shaping clusters that contain multiple graphemes.
*/

mod char;
#[allow(clippy::module_inception)]
mod cluster;
mod complex;
mod info;
mod myanmar;
mod parse;
mod simple;
mod token;

pub use self::{
    char::{Char, ShapeClass},
    cluster::{CharCluster, SourceRange, Status, MAX_CLUSTER_SIZE},
    info::{CharInfo, ClusterInfo, Emoji, Whitespace},
    parse::Parser,
    token::Token,
};

use super::unicode_data;

/// Boundary type of a character or cluster.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Boundary {
    /// Not a boundary.
    None = 0,
    /// Start of a word.
    Word = 1,
    /// Potential line break.
    Line = 2,
    /// Mandatory line break.
    Mandatory = 3,
}

impl Boundary {
    pub(super) fn from_raw(raw: u16) -> Self {
        match raw & 0b11 {
            0 => Self::None,
            1 => Self::Word,
            2 => Self::Line,
            3 => Self::Mandatory,
            _ => Self::None,
        }
    }
}

/// Arbitrary user data that can be associated with a character throughout
/// the shaping pipeline.
pub type UserData = u32;
