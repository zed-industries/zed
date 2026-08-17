/// The quartiles
#[derive(Clone, Debug)]
pub struct Quartiles {
    lower_fence: f64,
    lower: f64,
    median: f64,
    upper: f64,
    upper_fence: f64,
}

impl Quartiles {
    // Extract a value representing the `pct` percentile of a
    // sorted `s`, using linear interpolation.
    fn percentile_of_sorted<T: Into<f64> + Copy>(s: &[T], pct: f64) -> f64 {
        assert!(!s.is_empty());
        if s.len() == 1 {
            return s[0].into();
        }
        assert!(0_f64 <= pct);
        let hundred = 100_f64;
        assert!(pct <= hundred);
        if (pct - hundred).abs() < f64::EPSILON {
            return s[s.len() - 1].into();
        }
        let length = (s.len() - 1) as f64;
        let rank = (pct / hundred) * length;
        let lower_rank = rank.floor();
        let d = rank - lower_rank;
        let n = lower_rank as usize;
        let lo = s[n].into();
        let hi = s[n + 1].into();
        lo + (hi - lo) * d
    }

    /// Create a new quartiles struct with the values calculated from the argument.
    ///
    /// - `s`: The array of the original values
    /// - **returns** The newly created quartiles
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// assert_eq!(quartiles.median(), 37.5);
    /// ```
    pub fn new<T: Into<f64> + Copy + PartialOrd>(s: &[T]) -> Self {
        let mut s = s.to_owned();
        s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let lower = Quartiles::percentile_of_sorted(&s, 25_f64);
        let median = Quartiles::percentile_of_sorted(&s, 50_f64);
        let upper = Quartiles::percentile_of_sorted(&s, 75_f64);
        let iqr = upper - lower;
        let lower_fence = lower - 1.5 * iqr;
        let upper_fence = upper + 1.5 * iqr;
        Self {
            lower_fence,
            lower,
            median,
            upper,
            upper_fence,
        }
    }

    /// Get the quartiles values.
    ///
    /// - **returns** The array [lower fence, lower quartile, median, upper quartile, upper fence]
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// let values = quartiles.values();
    /// assert_eq!(values, [-9.0, 20.25, 37.5, 39.75, 69.0]);
    /// ```
    pub fn values(&self) -> [f32; 5] {
        [
            self.lower_fence as f32,
            self.lower as f32,
            self.median as f32,
            self.upper as f32,
            self.upper_fence as f32,
        ]
    }

    /// Get the quartiles median.
    ///
    /// - **returns** The median
    ///
    /// ```rust
    /// use plotters::prelude::*;
    ///
    /// let quartiles = Quartiles::new(&[7, 15, 36, 39, 40, 41]);
    /// assert_eq!(quartiles.median(), 37.5);
    /// ```
    pub fn median(&self) -> f64 {
        self.median
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_empty_input() {
        let empty_array: [i32; 0] = [];
        Quartiles::new(&empty_array);
    }

    #[test]
    fn test_low_inputs() {
        assert_eq!(
            Quartiles::new(&[15.0]).values(),
            [15.0, 15.0, 15.0, 15.0, 15.0]
        );
        assert_eq!(
            Quartiles::new(&[10, 20]).values(),
            [5.0, 12.5, 15.0, 17.5, 25.0]
        );
        assert_eq!(
            Quartiles::new(&[10, 20, 30]).values(),
            [0.0, 15.0, 20.0, 25.0, 40.0]
        );
    }
}
