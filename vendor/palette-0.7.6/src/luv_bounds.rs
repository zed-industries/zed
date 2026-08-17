//! Utility functions for computing in-gamut regions for CIELuv color space.
use crate::{
    angle::RealAngle,
    num::{Abs, Powi, Real, Sqrt, Trigonometry},
    LuvHue,
};

/// Boundary line in the u-v plane of the Luv color space.
struct BoundaryLine {
    slope: f64,
    intercept: f64,
}

impl BoundaryLine {
    /// Given array starting at the origin at angle theta, determine
    /// the signed length at which the ray intersects with the
    /// boundary.
    fn intersect_length_at_angle(&self, theta: f64) -> Option<f64> {
        let (sin_theta, cos_theta) = Trigonometry::sin_cos(theta);
        let denom = sin_theta - self.slope * cos_theta;
        if denom.abs() > 1.0e-6 {
            Some(self.intercept / denom)
        } else {
            None
        }
    }

    /// Return the distance from this line to the origin.
    #[allow(unused)]
    fn distance_to_origin(&self) -> f64 {
        Abs::abs(self.intercept) / Sqrt::sqrt(self.slope * self.slope + 1.0)
    }
}

/// `LuvBounds` represents the convex polygon formed by the in-gamut
/// region in the uv plane at a given lightness.
pub(crate) struct LuvBounds {
    bounds: [BoundaryLine; 6],
}

const M: [[f64; 3]; 3] = [
    [3.240969941904521, -1.537383177570093, -0.498610760293],
    [-0.96924363628087, 1.87596750150772, 0.041555057407175],
    [0.055630079696993, -0.20397695888897, 1.056971514242878],
];
const KAPPA: f64 = 903.2962962;
const EPSILON: f64 = 0.0088564516;

impl LuvBounds {
    pub fn from_lightness<T>(l: T) -> Self
    where
        T: Into<f64> + Powi,
    {
        let l: f64 = l.into();

        let sub1 = (l + 16.0).powi(3) / 1560896.0;
        let sub2 = if sub1 > EPSILON { sub1 } else { l / KAPPA };

        let line = |c: usize, t: f64| {
            let m: &[f64; 3] = &M[c];
            let top1 = (284517.0 * m[0] - 94839.0 * m[2]) * sub2;
            let top2 =
                (838422.0 * m[2] + 769860.0 * m[1] + 731718.0 * m[0]) * l * sub2 - 769860.0 * t * l;
            let bottom = (632260.0 * m[2] - 126452.0 * m[1]) * sub2 + 126452.0 * t;

            BoundaryLine {
                slope: top1 / bottom,
                intercept: top2 / bottom,
            }
        };

        Self {
            bounds: [
                line(0, 0.0),
                line(0, 1.0),
                line(1, 0.0),
                line(1, 1.0),
                line(2, 0.0),
                line(2, 1.0),
            ],
        }
    }

    /// Given a particular hue, return the distance to the boundary at
    /// the angle determined by the hue.
    pub fn max_chroma_at_hue<T: Into<f64> + RealAngle>(&self, hue: LuvHue<T>) -> T {
        let mut min_chroma = f64::MAX;
        let h = hue.into_raw_radians().into();

        // minimize the distance across all individual boundaries
        for b in &self.bounds {
            if let Some(t) = b.intersect_length_at_angle(h) {
                if t >= 0.0 && min_chroma > t {
                    min_chroma = t;
                }
            }
        }
        T::from_f64(min_chroma)
    }

    /// Return the minimum chroma such that, at any hue, the chroma is
    /// in-gamut.
    ///
    /// This is equivalent to finding the minimum distance to the
    /// origin across all boundaries.
    ///
    /// # Remarks
    /// This is useful for a n HPLuv implementation.
    #[allow(unused)]
    pub fn max_safe_chroma<T>(&self) -> T
    where
        T: Real,
    {
        let mut min_dist = f64::MAX;

        // minimize the distance across all individual boundaries
        for b in &self.bounds {
            let d = b.distance_to_origin();
            if min_dist > d {
                min_dist = d;
            }
        }
        T::from_f64(min_dist)
    }
}

#[cfg(feature = "approx")]
#[cfg(test)]
mod tests {
    use super::BoundaryLine;

    #[test]
    fn boundary_intersect() {
        let line = BoundaryLine {
            slope: -1.0,
            intercept: 1.0,
        };
        assert_relative_eq!(line.intersect_length_at_angle(0.0).unwrap(), 1.0);
        assert_relative_eq!(
            line.intersect_length_at_angle(core::f64::consts::FRAC_PI_4)
                .unwrap(),
            core::f64::consts::FRAC_1_SQRT_2
        );
        assert_eq!(
            line.intersect_length_at_angle(-core::f64::consts::FRAC_PI_4),
            None
        );

        let line = BoundaryLine {
            slope: 0.0,
            intercept: 2.0,
        };
        assert_eq!(line.intersect_length_at_angle(0.0), None);
        assert_relative_eq!(
            line.intersect_length_at_angle(core::f64::consts::FRAC_PI_2)
                .unwrap(),
            2.0
        );
        assert_relative_eq!(
            line.intersect_length_at_angle(2.0 * core::f64::consts::FRAC_PI_3)
                .unwrap(),
            4.0 / 3.0f64.sqrt()
        );
    }

    #[test]
    fn line_distance() {
        let line = BoundaryLine {
            slope: 0.0,
            intercept: 2.0,
        };
        assert_relative_eq!(line.distance_to_origin(), 2.0);

        let line = BoundaryLine {
            slope: 1.0,
            intercept: 2.0,
        };
        assert_relative_eq!(line.distance_to_origin(), core::f64::consts::SQRT_2);
    }
}
