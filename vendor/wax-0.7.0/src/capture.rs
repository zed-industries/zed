use regex::Captures as BorrowedText;
use std::str;

use crate::CandidatePath;

#[derive(Clone, Debug)]
struct OwnedText {
    matched: String,
    ranges: Vec<Option<(usize, usize)>>,
}

impl OwnedText {
    pub fn get(&self, index: usize) -> Option<&str> {
        if index == 0 {
            Some(self.matched.as_ref())
        }
        else {
            self.ranges
                .get(index - 1)
                .and_then(|range| range.map(|range| &self.matched[range.0..range.1]))
        }
    }
}

impl<'t> From<BorrowedText<'t>> for OwnedText {
    fn from(captures: BorrowedText<'t>) -> Self {
        From::from(&captures)
    }
}

impl<'m, 't> From<&'m BorrowedText<'t>> for OwnedText {
    fn from(captures: &'m BorrowedText<'t>) -> Self {
        let matched = captures.get(0).unwrap().as_str().into();
        let ranges = captures
            .iter()
            .skip(1)
            .map(|capture| capture.map(|capture| (capture.start(), capture.end())))
            .collect();
        OwnedText { matched, ranges }
    }
}

#[derive(Debug)]
enum MaybeOwnedText<'t> {
    Borrowed(BorrowedText<'t>),
    Owned(OwnedText),
}

impl<'t> MaybeOwnedText<'t> {
    fn into_owned(self) -> MaybeOwnedText<'static> {
        match self {
            MaybeOwnedText::Borrowed(borrowed) => OwnedText::from(borrowed).into(),
            MaybeOwnedText::Owned(owned) => owned.into(),
        }
    }

    // This conversion may appear to operate in place.
    #[must_use]
    fn to_owned(&self) -> MaybeOwnedText<'static> {
        match self {
            MaybeOwnedText::Borrowed(borrowed) => OwnedText::from(borrowed).into(),
            MaybeOwnedText::Owned(owned) => owned.clone().into(),
        }
    }
}

impl<'t> From<BorrowedText<'t>> for MaybeOwnedText<'t> {
    fn from(captures: BorrowedText<'t>) -> Self {
        MaybeOwnedText::Borrowed(captures)
    }
}

impl From<OwnedText> for MaybeOwnedText<'static> {
    fn from(captures: OwnedText) -> Self {
        MaybeOwnedText::Owned(captures)
    }
}

/// Text that has been matched by a [`Program`] and its captures.
///
/// To match a [`Glob`] or other [`Program`] against a [`CandidatePath`] and get the matched text,
/// use the [`Program::matched`] function.
///
/// All [`Program`]s provide an implicit capture of the complete text of a match. This implicit
/// capture has index zero, and is exposed via the [`complete`] function as well as the [`get`]
/// function using index zero. Capturing tokens are indexed starting at one, and can be used to
/// isolate more specific sub-text.
///
/// # Examples
///
/// Capturing tokens and matched text can be used to isolate sub-text in a match. For example, the
/// file name of a match can be extracted using an alternation to group patterns.
///
/// ```rust
/// use wax::{CandidatePath, Glob, Program};
///
/// let glob = Glob::new("src/**/{*.{go,rs}}").unwrap();
/// let candidate = CandidatePath::from("src/graph/link.rs");
/// let matched = glob.matched(&candidate).unwrap();
///
/// assert_eq!("link.rs", matched.get(2).unwrap());
/// ```
///
/// [`CandidatePath`]: crate::CandidatePath
/// [`complete`]: crate::MatchedText::complete
/// [`get`]: crate::MatchedText::get
/// [`Glob`]: crate::Glob
/// [`Program`]: crate::Program
/// [`Program::matched`]: crate::Program::matched
#[derive(Debug)]
pub struct MatchedText<'t> {
    inner: MaybeOwnedText<'t>,
}

impl<'t> MatchedText<'t> {
    /// Clones any borrowed data into an owning instance.
    pub fn into_owned(self) -> MatchedText<'static> {
        let MatchedText { inner } = self;
        MatchedText {
            inner: inner.into_owned(),
        }
    }

    /// Clones any borrowed data to an owning instance.
    ///
    /// This function is similar to [`into_owned`], but does not consume its receiver. Due to a
    /// technical limitation, `MatchedText` cannot properly implement [`Clone`], so this function
    /// is provided as a stop gap that allows a distinct instance to be created that owns its data.
    ///
    /// [`Clone`]: std::clone::Clone
    /// [`into_owned`]: crate::MatchedText::into_owned
    // This conversion may appear to operate in place.
    #[must_use]
    pub fn to_owned(&self) -> MatchedText<'static> {
        MatchedText {
            inner: self.inner.to_owned(),
        }
    }

    /// Gets the complete text of a match.
    ///
    /// All [`Program`]s have an implicit capture of the complete text at index zero. This function
    /// is therefore equivalent to unwrapping the output of the [`get`] function with index zero.
    ///
    /// [`get`]: crate::MatchedText::get
    /// [`Program`]: crate::Program
    pub fn complete(&self) -> &str {
        self.get(0).expect("match has no complete text")
    }

    /// Gets the matched text of a capture at the given index.
    ///
    /// All [`Program`]s have an implicit capture of the complete text at index zero. Capturing
    /// tokens are indexed from one, so any capturing sub-expression will be indexed after the
    /// implicit complete text. For example, the sub-expression `*` in the glob expression `*.txt`
    /// is at index one and will exclude the suffix `.txt` in its matched text.
    ///
    /// Alternation and repetition patterns group their sub-globs into a single capture, so it is
    /// not possible to isolate matched text from their sub-globs. This can be used to explicitly
    /// group matched text, such as isolating an entire matched file name using an expression like
    /// `{*.{go,rs}}`.
    ///
    /// [`Program`]: crate::Program
    pub fn get(&self, index: usize) -> Option<&str> {
        match &self.inner {
            MaybeOwnedText::Borrowed(captures) => {
                captures.get(index).map(|capture| capture.as_str())
            },
            MaybeOwnedText::Owned(captures) => captures.get(index),
        }
    }

    pub fn to_candidate_path(&self) -> CandidatePath<'_> {
        CandidatePath::from(self.complete())
    }
}

// TODO: This probably shouldn't be part of the public API.
impl<'t> From<BorrowedText<'t>> for MatchedText<'t> {
    fn from(captures: BorrowedText<'t>) -> Self {
        MatchedText {
            inner: captures.into(),
        }
    }
}

impl From<OwnedText> for MatchedText<'static> {
    fn from(captures: OwnedText) -> Self {
        MatchedText {
            inner: captures.into(),
        }
    }
}
