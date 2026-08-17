//! Point projection.

use super::graphics::{CoordAxis, GraphicsState};
use raw::types::{F26Dot6, Point};

impl GraphicsState<'_> {
    /// Updates cached state that is derived from projection vectors.
    pub fn update_projection_state(&mut self) {
        // 1.0 in 2.14 fixed point.
        const ONE: i32 = 0x4000;
        // Based on Compute_Funcs() at
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2482>.
        // FreeType uses function pointers to select between various "modes"
        // but we use the CoordAxis type instead.
        if self.freedom_vector.x == ONE {
            self.fdotp = self.proj_vector.x;
        } else if self.freedom_vector.y == ONE {
            self.fdotp = self.proj_vector.y;
        } else {
            let px = self.proj_vector.x;
            let py = self.proj_vector.y;
            let fx = self.freedom_vector.x;
            let fy = self.freedom_vector.y;
            self.fdotp = (px * fx + py * fy) >> 14;
        }
        self.proj_axis = CoordAxis::Both;
        if self.proj_vector.x == ONE {
            self.proj_axis = CoordAxis::X;
        } else if self.proj_vector.y == ONE {
            self.proj_axis = CoordAxis::Y;
        }
        self.dual_proj_axis = CoordAxis::Both;
        if self.dual_proj_vector.x == ONE {
            self.dual_proj_axis = CoordAxis::X;
        } else if self.dual_proj_vector.y == ONE {
            self.dual_proj_axis = CoordAxis::Y;
        }
        self.freedom_axis = CoordAxis::Both;
        if self.fdotp == ONE {
            if self.freedom_vector.x == ONE {
                self.freedom_axis = CoordAxis::X;
            } else if self.freedom_vector.y == ONE {
                self.freedom_axis = CoordAxis::Y;
            }
        }
        // At small sizes, fdotp can become too small resulting in overflows
        // and spikes.
        if self.fdotp.abs() < 0x400 {
            self.fdotp = ONE;
        }
    }

    /// Computes the projection of vector given by (v1 - v2) along the
    /// current projection vector.
    #[inline(always)]
    pub fn project(&self, v1: Point<F26Dot6>, v2: Point<F26Dot6>) -> F26Dot6 {
        match self.proj_axis {
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2431>
            CoordAxis::X => v1.x - v2.x,
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2461>
            CoordAxis::Y => v1.y - v2.y,
            CoordAxis::Both => {
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2373>
                let dx = v1.x - v2.x;
                let dy = v1.y - v2.y;
                F26Dot6::from_bits(dot14(
                    dx.to_bits(),
                    dy.to_bits(),
                    self.proj_vector.x,
                    self.proj_vector.y,
                ))
            }
        }
    }

    /// Computes the projection of vector given by (v1 - v2) along the
    /// current dual projection vector.
    #[inline(always)]
    pub fn dual_project(&self, v1: Point<F26Dot6>, v2: Point<F26Dot6>) -> F26Dot6 {
        match self.dual_proj_axis {
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2431>
            CoordAxis::X => v1.x - v2.x,
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2461>
            CoordAxis::Y => v1.y - v2.y,
            CoordAxis::Both => {
                // https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2402
                let dx = v1.x - v2.x;
                let dy = v1.y - v2.y;
                F26Dot6::from_bits(dot14(
                    dx.to_bits(),
                    dy.to_bits(),
                    self.dual_proj_vector.x,
                    self.dual_proj_vector.y,
                ))
            }
        }
    }

    /// Computes the projection of vector given by (v1 - v2) along the
    /// current dual projection vector for unscaled points.
    #[inline(always)]
    pub fn dual_project_unscaled(&self, v1: Point<i32>, v2: Point<i32>) -> i32 {
        match self.dual_proj_axis {
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2431>
            CoordAxis::X => v1.x - v2.x,
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2461>
            CoordAxis::Y => v1.y - v2.y,
            CoordAxis::Both => {
                // https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2402
                let dx = v1.x - v2.x;
                let dy = v1.y - v2.y;
                dot14(dx, dy, self.dual_proj_vector.x, self.dual_proj_vector.y)
            }
        }
    }
}

/// Dot product for vectors in 2.14 fixed point.
fn dot14(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
    let mut v1 = ax as i64 * bx as i64;
    let v2 = ay as i64 * by as i64;
    v1 += v2;
    v1 += 0x2000 + (v1 >> 63);
    (v1 >> 14) as i32
}

#[cfg(test)]
mod tests {
    use super::{super::math, CoordAxis, F26Dot6, GraphicsState, Point};

    #[test]
    fn project_one_axis() {
        let mut state = GraphicsState {
            proj_vector: math::normalize14(1, 0),
            ..Default::default()
        };
        state.update_projection_state();
        assert_eq!(state.proj_axis, CoordAxis::X);
        assert_eq!(state.proj_vector, Point::new(0x4000, 0));
        let cases = &[
            (Point::new(0, 0), Point::new(0, 0), 0),
            (Point::new(100, 100), Point::new(0, 0), 100),
            (Point::new(42, 100), Point::new(100, 0), -58),
            (Point::new(0, 0), Point::new(100, 100), -100),
        ];
        test_project_cases(&state, cases);
    }

    #[test]
    fn project_both_axes() {
        let mut state = GraphicsState {
            proj_vector: math::normalize14(0x4000, 0x4000),
            ..Default::default()
        };
        state.update_projection_state();
        assert_eq!(state.proj_axis, CoordAxis::Both);
        let cases = &[
            (Point::new(0, 0), Point::new(0, 0), 0),
            (Point::new(100, 100), Point::new(0, 0), 141),
            (Point::new(42, 100), Point::new(100, 0), 30),
            (Point::new(0, 0), Point::new(100, 100), -141),
        ];
        test_project_cases(&state, cases);
    }

    fn test_project_cases(state: &GraphicsState, cases: &[(Point<i32>, Point<i32>, i32)]) {
        for (v1, v2, expected) in cases.iter().copied() {
            let v1 = v1.map(F26Dot6::from_bits);
            let v2 = v2.map(F26Dot6::from_bits);
            let result = state.project(v1, v2).to_bits();
            assert_eq!(
                result, expected,
                "project({v1:?}, {v2:?}) = {result} (expected {expected})"
            );
        }
    }
}
