//! Regression analysis

use crate::stats::bivariate::Data;
use crate::stats::float::Float;

/// A straight line that passes through the origin `y = m * x`
#[derive(Clone, Copy)]
pub struct Slope<A>(pub A)
where
    A: Float;

impl<A> Slope<A>
where
    A: Float,
{
    /// Fits the data to a straight line that passes through the origin using ordinary least
    /// squares
    ///
    /// - Time: `O(length)`
    pub fn fit(data: &Data<'_, A, A>) -> Slope<A> {
        let xs = data.0;
        let ys = data.1;

        let xy = crate::stats::dot(xs, ys);
        let x2 = crate::stats::dot(xs, xs);

        Slope(xy / x2)
    }

    /// Computes the goodness of fit (coefficient of determination) for this data set
    ///
    /// - Time: `O(length)`
    pub fn r_squared(&self, data: &Data<'_, A, A>) -> A {
        let _0 = A::cast(0);
        let _1 = A::cast(1);
        let m = self.0;
        let xs = data.0;
        let ys = data.1;

        let n = A::cast(xs.len());
        let y_bar = crate::stats::sum(ys) / n;

        let mut ss_res = _0;
        let mut ss_tot = _0;

        for (&x, &y) in data.iter() {
            ss_res = ss_res + (y - m * x).powi(2);
            ss_tot = ss_res + (y - y_bar).powi(2);
        }

        _1 - ss_res / ss_tot
    }
}
