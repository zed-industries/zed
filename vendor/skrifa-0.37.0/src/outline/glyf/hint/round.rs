//! Rounding state.

use super::{super::F26Dot6, graphics::GraphicsState};

/// Rounding strategies supported by the interpreter.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum RoundMode {
    /// Distances are rounded to the closest grid line.
    ///
    /// Set by `RTG` instruction.
    #[default]
    Grid,
    /// Distances are rounded to the nearest half grid line.
    ///
    /// Set by `RTHG` instruction.
    HalfGrid,
    /// Distances are rounded to the closest half or integer pixel.
    ///
    /// Set by `RTDG` instruction.
    DoubleGrid,
    /// Distances are rounded down to the closest integer grid line.
    ///
    /// Set by `RDTG` instruction.
    DownToGrid,
    /// Distances are rounded up to the closest integer pixel boundary.
    ///
    /// Set by `RUTG` instruction.
    UpToGrid,
    /// Rounding is turned off.
    ///
    /// Set by `ROFF` instruction.
    Off,
    /// Allows fine control over the effects of the round state variable by
    /// allowing you to set the values of three components of the round_state:
    /// period, phase, and threshold.
    ///
    /// More formally, maps the domain of 26.6 fixed point numbers into a set
    /// of discrete values that are separated by equal distances.
    ///
    /// Set by `SROUND` instruction.
    Super,
    /// Analogous to `Super`. The grid period is sqrt(2)/2 pixels rather than 1
    /// pixel. It is useful for measuring at a 45 degree angle with the
    /// coordinate axes.
    ///
    /// Set by `S45ROUND` instruction.
    Super45,
}

/// Graphics state that controls rounding.
///
/// See <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM04/Chap4.html#round%20state>
#[derive(Copy, Clone, Debug)]
pub struct RoundState {
    pub mode: RoundMode,
    pub threshold: i32,
    pub phase: i32,
    pub period: i32,
}

impl Default for RoundState {
    fn default() -> Self {
        Self {
            mode: RoundMode::Grid,
            threshold: 0,
            phase: 0,
            period: 64,
        }
    }
}

impl RoundState {
    pub fn round(&self, distance: F26Dot6) -> F26Dot6 {
        use super::math;
        use RoundMode::*;
        let distance = distance.to_bits();
        let result = match self.mode {
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1958>
            HalfGrid => {
                if distance >= 0 {
                    (math::floor(distance) + 32).max(0)
                } else {
                    (-(math::floor(-distance) + 32)).min(0)
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1913>
            Grid => {
                if distance >= 0 {
                    math::round(distance).max(0)
                } else {
                    (-math::round(-distance)).min(0)
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2094>
            DoubleGrid => {
                if distance >= 0 {
                    math::round_pad(distance, 32).max(0)
                } else {
                    (-math::round_pad(-distance, 32)).min(0)
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2005>
            DownToGrid => {
                if distance >= 0 {
                    math::floor(distance).max(0)
                } else {
                    (-math::floor(-distance)).min(0)
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2049>
            UpToGrid => {
                if distance >= 0 {
                    math::ceil(distance).max(0)
                } else {
                    (-math::ceil(-distance)).min(0)
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2145>
            Super => {
                if distance >= 0 {
                    let val =
                        ((distance + (self.threshold - self.phase)) & -self.period) + self.phase;
                    if val < 0 {
                        self.phase
                    } else {
                        val
                    }
                } else {
                    let val =
                        -(((self.threshold - self.phase) - distance) & -self.period) - self.phase;
                    if val > 0 {
                        -self.phase
                    } else {
                        val
                    }
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2199>
            Super45 => {
                if distance >= 0 {
                    let val = (((distance + (self.threshold - self.phase)) / self.period)
                        * self.period)
                        + self.phase;
                    if val < 0 {
                        self.phase
                    } else {
                        val
                    }
                } else {
                    let val = -((((self.threshold - self.phase) - distance) / self.period)
                        * self.period)
                        - self.phase;
                    if val > 0 {
                        -self.phase
                    } else {
                        val
                    }
                }
            }
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1870>
            Off => distance,
        };
        F26Dot6::from_bits(result)
    }
}

impl GraphicsState<'_> {
    pub fn round(&self, distance: F26Dot6) -> F26Dot6 {
        self.round_state.round(distance)
    }
}

#[cfg(test)]
mod tests {
    use super::{F26Dot6, RoundMode, RoundState};

    #[test]
    fn round_to_grid() {
        round_cases(
            RoundMode::Grid,
            &[(0, 0), (32, 64), (-32, -64), (64, 64), (50, 64)],
        );
    }

    #[test]
    fn round_to_half_grid() {
        round_cases(
            RoundMode::HalfGrid,
            &[(0, 32), (32, 32), (-32, -32), (64, 96), (50, 32)],
        );
    }

    #[test]
    fn round_to_double_grid() {
        round_cases(
            RoundMode::DoubleGrid,
            &[(0, 0), (32, 32), (-32, -32), (64, 64), (50, 64)],
        );
    }

    #[test]
    fn round_down_to_grid() {
        round_cases(
            RoundMode::DownToGrid,
            &[(0, 0), (32, 0), (-32, 0), (64, 64), (50, 0)],
        );
    }

    #[test]
    fn round_up_to_grid() {
        round_cases(
            RoundMode::UpToGrid,
            &[(0, 0), (32, 64), (-32, -64), (64, 64), (50, 64)],
        );
    }

    #[test]
    fn round_off() {
        round_cases(
            RoundMode::Off,
            &[(0, 0), (32, 32), (-32, -32), (64, 64), (50, 50)],
        );
    }

    fn round_cases(mode: RoundMode, cases: &[(i32, i32)]) {
        for (value, expected) in cases.iter().copied() {
            let value = F26Dot6::from_bits(value);
            let expected = F26Dot6::from_bits(expected);
            let state = RoundState {
                mode,
                ..Default::default()
            };
            let result = state.round(value);
            assert_eq!(
                result, expected,
                "mismatch in rounding: {mode:?}({value}) = {result} (expected {expected})"
            );
        }
    }
}
