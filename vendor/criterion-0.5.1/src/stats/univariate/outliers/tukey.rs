//! Tukey's method
//!
//! The original method uses two "fences" to classify the data. All the observations "inside" the
//! fences are considered "normal", and the rest are considered outliers.
//!
//! The fences are computed from the quartiles of the sample, according to the following formula:
//!
//! ``` ignore
//! // q1, q3 are the first and third quartiles
//! let iqr = q3 - q1;  // The interquartile range
//! let (f1, f2) = (q1 - 1.5 * iqr, q3 + 1.5 * iqr);  // the "fences"
//!
//! let is_outlier = |x| if x > f1 && x < f2 { true } else { false };
//! ```
//!
//! The classifier provided here adds two extra outer fences:
//!
//! ``` ignore
//! let (f3, f4) = (q1 - 3 * iqr, q3 + 3 * iqr);  // the outer "fences"
//! ```
//!
//! The extra fences add a sense of "severity" to the classification. Data points outside of the
//! outer fences are considered "severe" outliers, whereas points outside the inner fences are just
//! "mild" outliers, and, as the original method, everything inside the inner fences is considered
//! "normal" data.
//!
//! Some ASCII art for the visually oriented people:
//!
//! ``` ignore
//!          LOW-ish                NORMAL-ish                 HIGH-ish
//!         x   |       +    |  o o  o    o   o o  o  |        +   |   x
//!             f3           f1                       f2           f4
//!
//! Legend:
//! o: "normal" data (not an outlier)
//! +: "mild" outlier
//! x: "severe" outlier
//! ```

use std::iter::IntoIterator;
use std::ops::{Deref, Index};
use std::slice;

use crate::stats::float::Float;
use crate::stats::univariate::Sample;

use self::Label::*;

/// A classified/labeled sample.
///
/// The labeled data can be accessed using the indexing operator. The order of the data points is
/// retained.
///
/// NOTE: Due to limitations in the indexing traits, only the label is returned. Once the
/// `IndexGet` trait lands in stdlib, the indexing operation will return a `(data_point, label)`
/// pair.
#[derive(Clone, Copy)]
pub struct LabeledSample<'a, A>
where
    A: Float,
{
    fences: (A, A, A, A),
    sample: &'a Sample<A>,
}

impl<'a, A> LabeledSample<'a, A>
where
    A: Float,
{
    /// Returns the number of data points per label
    ///
    /// - Time: `O(length)`
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::similar_names))]
    pub fn count(&self) -> (usize, usize, usize, usize, usize) {
        let (mut los, mut lom, mut noa, mut him, mut his) = (0, 0, 0, 0, 0);

        for (_, label) in self {
            match label {
                LowSevere => {
                    los += 1;
                }
                LowMild => {
                    lom += 1;
                }
                NotAnOutlier => {
                    noa += 1;
                }
                HighMild => {
                    him += 1;
                }
                HighSevere => {
                    his += 1;
                }
            }
        }

        (los, lom, noa, him, his)
    }

    /// Returns the fences used to classify the outliers
    pub fn fences(&self) -> (A, A, A, A) {
        self.fences
    }

    /// Returns an iterator over the labeled data
    pub fn iter(&self) -> Iter<'a, A> {
        Iter {
            fences: self.fences,
            iter: self.sample.iter(),
        }
    }
}

impl<'a, A> Deref for LabeledSample<'a, A>
where
    A: Float,
{
    type Target = Sample<A>;

    fn deref(&self) -> &Sample<A> {
        self.sample
    }
}

// FIXME Use the `IndexGet` trait
impl<'a, A> Index<usize> for LabeledSample<'a, A>
where
    A: Float,
{
    type Output = Label;

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::similar_names))]
    fn index(&self, i: usize) -> &Label {
        static LOW_SEVERE: Label = LowSevere;
        static LOW_MILD: Label = LowMild;
        static HIGH_MILD: Label = HighMild;
        static HIGH_SEVERE: Label = HighSevere;
        static NOT_AN_OUTLIER: Label = NotAnOutlier;

        let x = self.sample[i];
        let (lost, lomt, himt, hist) = self.fences;

        if x < lost {
            &LOW_SEVERE
        } else if x > hist {
            &HIGH_SEVERE
        } else if x < lomt {
            &LOW_MILD
        } else if x > himt {
            &HIGH_MILD
        } else {
            &NOT_AN_OUTLIER
        }
    }
}

impl<'a, 'b, A> IntoIterator for &'b LabeledSample<'a, A>
where
    A: Float,
{
    type Item = (A, Label);
    type IntoIter = Iter<'a, A>;

    fn into_iter(self) -> Iter<'a, A> {
        self.iter()
    }
}

/// Iterator over the labeled data
pub struct Iter<'a, A>
where
    A: Float,
{
    fences: (A, A, A, A),
    iter: slice::Iter<'a, A>,
}

impl<'a, A> Iterator for Iter<'a, A>
where
    A: Float,
{
    type Item = (A, Label);

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::similar_names))]
    fn next(&mut self) -> Option<(A, Label)> {
        self.iter.next().map(|&x| {
            let (lost, lomt, himt, hist) = self.fences;

            let label = if x < lost {
                LowSevere
            } else if x > hist {
                HighSevere
            } else if x < lomt {
                LowMild
            } else if x > himt {
                HighMild
            } else {
                NotAnOutlier
            };

            (x, label)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Labels used to classify outliers
pub enum Label {
    /// A "mild" outlier in the "high" spectrum
    HighMild,
    /// A "severe" outlier in the "high" spectrum
    HighSevere,
    /// A "mild" outlier in the "low" spectrum
    LowMild,
    /// A "severe" outlier in the "low" spectrum
    LowSevere,
    /// A normal data point
    NotAnOutlier,
}

impl Label {
    /// Checks if the data point has an "unusually" high value
    pub fn is_high(&self) -> bool {
        matches!(*self, HighMild | HighSevere)
    }

    /// Checks if the data point is labeled as a "mild" outlier
    pub fn is_mild(&self) -> bool {
        matches!(*self, HighMild | LowMild)
    }

    /// Checks if the data point has an "unusually" low value
    pub fn is_low(&self) -> bool {
        matches!(*self, LowMild | LowSevere)
    }

    /// Checks if the data point is labeled as an outlier
    pub fn is_outlier(&self) -> bool {
        matches!(*self, NotAnOutlier)
    }

    /// Checks if the data point is labeled as a "severe" outlier
    pub fn is_severe(&self) -> bool {
        matches!(*self, HighSevere | LowSevere)
    }
}

/// Classifies the sample, and returns a labeled sample.
///
/// - Time: `O(N log N) where N = length`
pub fn classify<A>(sample: &Sample<A>) -> LabeledSample<'_, A>
where
    A: Float,
    usize: cast::From<A, Output = Result<usize, cast::Error>>,
{
    let (q1, _, q3) = sample.percentiles().quartiles();
    let iqr = q3 - q1;

    // Mild
    let k_m = A::cast(1.5_f32);
    // Severe
    let k_s = A::cast(3);

    LabeledSample {
        fences: (
            q1 - k_s * iqr,
            q1 - k_m * iqr,
            q3 + k_m * iqr,
            q3 + k_s * iqr,
        ),
        sample,
    }
}
