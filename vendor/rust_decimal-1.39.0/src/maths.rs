use crate::prelude::*;
use num_traits::pow::Pow;

// Tolerance for inaccuracies when calculating exp
const EXP_TOLERANCE: Decimal = Decimal::from_parts(2, 0, 0, false, 7);
// Approximation of 1/ln(10) = 0.4342944819032518276511289189
const LN10_INVERSE: Decimal = Decimal::from_parts_raw(1763037029, 1670682625, 235431510, 1835008);
// Total iterations of taylor series for Trig.
const TRIG_SERIES_UPPER_BOUND: usize = 6;
// PI / 8
const EIGHTH_PI: Decimal = Decimal::from_parts_raw(2822163429, 3244459792, 212882598, 1835008);

// Table representing {index}!
const FACTORIAL: [Decimal; 28] = [
    Decimal::from_parts(1, 0, 0, false, 0),
    Decimal::from_parts(1, 0, 0, false, 0),
    Decimal::from_parts(2, 0, 0, false, 0),
    Decimal::from_parts(6, 0, 0, false, 0),
    Decimal::from_parts(24, 0, 0, false, 0),
    // 5!
    Decimal::from_parts(120, 0, 0, false, 0),
    Decimal::from_parts(720, 0, 0, false, 0),
    Decimal::from_parts(5040, 0, 0, false, 0),
    Decimal::from_parts(40320, 0, 0, false, 0),
    Decimal::from_parts(362880, 0, 0, false, 0),
    // 10!
    Decimal::from_parts(3628800, 0, 0, false, 0),
    Decimal::from_parts(39916800, 0, 0, false, 0),
    Decimal::from_parts(479001600, 0, 0, false, 0),
    Decimal::from_parts(1932053504, 1, 0, false, 0),
    Decimal::from_parts(1278945280, 20, 0, false, 0),
    // 15!
    Decimal::from_parts(2004310016, 304, 0, false, 0),
    Decimal::from_parts(2004189184, 4871, 0, false, 0),
    Decimal::from_parts(4006445056, 82814, 0, false, 0),
    Decimal::from_parts(3396534272, 1490668, 0, false, 0),
    Decimal::from_parts(109641728, 28322707, 0, false, 0),
    // 20!
    Decimal::from_parts(2192834560, 566454140, 0, false, 0),
    Decimal::from_parts(3099852800, 3305602358, 2, false, 0),
    Decimal::from_parts(3772252160, 4003775155, 60, false, 0),
    Decimal::from_parts(862453760, 1892515369, 1401, false, 0),
    Decimal::from_parts(3519021056, 2470695900, 33634, false, 0),
    // 25!
    Decimal::from_parts(2076180480, 1637855376, 840864, false, 0),
    Decimal::from_parts(2441084928, 3929534124, 21862473, false, 0),
    Decimal::from_parts(1484783616, 3018206259, 590286795, false, 0),
];

/// Trait exposing various mathematical operations that can be applied using a Decimal. This is only
/// present when the `maths` feature has been enabled, e.g. by adding the crate with
// `cargo add rust_decimal --features maths` and importing in your Rust file with `use rust_decimal::MathematicalOps;`
pub trait MathematicalOps {
    /// The estimated exponential function, e<sup>x</sup>. Stops calculating when it is within
    /// tolerance of roughly `0.0000002`.
    fn exp(&self) -> Decimal;

    /// The estimated exponential function, e<sup>x</sup>. Stops calculating when it is within
    /// tolerance of roughly `0.0000002`. Returns `None` on overflow.
    fn checked_exp(&self) -> Option<Decimal>;

    /// The estimated exponential function, e<sup>x</sup> using the `tolerance` provided as a hint
    /// as to when to stop calculating. A larger tolerance will cause the number to stop calculating
    /// sooner at the potential cost of a slightly less accurate result.
    fn exp_with_tolerance(&self, tolerance: Decimal) -> Decimal;

    /// The estimated exponential function, e<sup>x</sup> using the `tolerance` provided as a hint
    /// as to when to stop calculating. A larger tolerance will cause the number to stop calculating
    /// sooner at the potential cost of a slightly less accurate result.
    /// Returns `None` on overflow.
    fn checked_exp_with_tolerance(&self, tolerance: Decimal) -> Option<Decimal>;

    /// Raise self to the given integer exponent: x<sup>y</sup>
    fn powi(&self, exp: i64) -> Decimal;

    /// Raise self to the given integer exponent x<sup>y</sup> returning `None` on overflow.
    fn checked_powi(&self, exp: i64) -> Option<Decimal>;

    /// Raise self to the given unsigned integer exponent: x<sup>y</sup>
    fn powu(&self, exp: u64) -> Decimal;

    /// Raise self to the given unsigned integer exponent x<sup>y</sup> returning `None` on overflow.
    fn checked_powu(&self, exp: u64) -> Option<Decimal>;

    /// Raise self to the given floating point exponent: x<sup>y</sup>
    fn powf(&self, exp: f64) -> Decimal;

    /// Raise self to the given floating point exponent x<sup>y</sup> returning `None` on overflow.
    fn checked_powf(&self, exp: f64) -> Option<Decimal>;

    /// Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not whole then the approximation
    /// e<sup>y*ln(x)</sup> is used.
    fn powd(&self, exp: Decimal) -> Decimal;

    /// Raise self to the given Decimal exponent x<sup>y</sup> returning `None` on overflow.
    /// If `exp` is not whole then the approximation e<sup>y*ln(x)</sup> is used.
    fn checked_powd(&self, exp: Decimal) -> Option<Decimal>;

    /// The square root of a Decimal. Uses a standard Babylonian method.
    fn sqrt(&self) -> Option<Decimal>;

    /// Calculates the natural logarithm for a Decimal calculated using Taylor's series.
    fn ln(&self) -> Decimal;

    /// Calculates the checked natural logarithm for a Decimal calculated using Taylor's series.
    /// Returns `None` for negative numbers or zero.
    fn checked_ln(&self) -> Option<Decimal>;

    /// Calculates the base 10 logarithm of a specified Decimal number.
    fn log10(&self) -> Decimal;

    /// Calculates the checked base 10 logarithm of a specified Decimal number.
    /// Returns `None` for negative numbers or zero.
    fn checked_log10(&self) -> Option<Decimal>;

    /// Abramowitz Approximation of Error Function from [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
    fn erf(&self) -> Decimal;

    /// The Cumulative distribution function for a Normal distribution
    fn norm_cdf(&self) -> Decimal;

    /// The Probability density function for a Normal distribution.
    fn norm_pdf(&self) -> Decimal;

    /// The Probability density function for a Normal distribution returning `None` on overflow.
    fn checked_norm_pdf(&self) -> Option<Decimal>;

    /// Computes the sine of a number (in radians).
    /// Panics upon overflow.
    fn sin(&self) -> Decimal;

    /// Computes the checked sine of a number (in radians).
    fn checked_sin(&self) -> Option<Decimal>;

    /// Computes the cosine of a number (in radians).
    /// Panics upon overflow.
    fn cos(&self) -> Decimal;

    /// Computes the checked cosine of a number (in radians).
    fn checked_cos(&self) -> Option<Decimal>;

    /// Computes the tangent of a number (in radians).
    /// Panics upon overflow or upon approaching a limit.
    fn tan(&self) -> Decimal;

    /// Computes the checked tangent of a number (in radians).
    /// Returns None on limit.
    fn checked_tan(&self) -> Option<Decimal>;
}

impl MathematicalOps for Decimal {
    fn exp(&self) -> Decimal {
        self.exp_with_tolerance(EXP_TOLERANCE)
    }

    fn checked_exp(&self) -> Option<Decimal> {
        self.checked_exp_with_tolerance(EXP_TOLERANCE)
    }

    fn exp_with_tolerance(&self, tolerance: Decimal) -> Decimal {
        match self.checked_exp_with_tolerance(tolerance) {
            Some(d) => d,
            None => {
                if self.is_sign_negative() {
                    panic!("Exp underflowed")
                } else {
                    panic!("Exp overflowed")
                }
            }
        }
    }

    fn checked_exp_with_tolerance(&self, tolerance: Decimal) -> Option<Decimal> {
        if self.is_zero() {
            return Some(Decimal::ONE);
        }
        if self.is_sign_negative() {
            let mut flipped = *self;
            flipped.set_sign_positive(true);
            let exp = flipped.checked_exp_with_tolerance(tolerance)?;
            return Decimal::ONE.checked_div(exp);
        }

        // exp(x) = sum_i x^i / i!, let q_i := x^i/i!
        // Avoid computing x^i directly as it will quickly outgrow exp(x) for x > 1.
        // Instead we compute q_i = x*(x/2)*(x/3)*...*(x/i)

        // First two terms are done directly: y = 1 + x
        let mut result = self.checked_add(Decimal::ONE)?;

        // q_1 = x
        let mut term = *self;

        // Note: For smaller x the loop will terminate early due to the tolerance check. Only
        // for very large x it will run more iterations, for example: exp(66.5) runs up about 186
        // iterations.
        const ITERATION_COUNT: u32 = 200;

        for i in 2..ITERATION_COUNT {
            // SAFETY: `i` is always a valid Decimal (and it's trivial to construct it).
            let i_dec = Decimal::from_u32(i).unwrap();

            term = self.checked_mul(term.checked_div(i_dec)?)?;

            result = result.checked_add(term)?;

            // Note that term is positive
            if term <= tolerance {
                break;
            }
        }

        Some(result)
    }

    fn powi(&self, exp: i64) -> Decimal {
        match self.checked_powi(exp) {
            Some(result) => result,
            None => panic!("Pow overflowed"),
        }
    }

    fn checked_powi(&self, exp: i64) -> Option<Decimal> {
        // For negative exponents we change x^-y into 1 / x^y.
        // Otherwise, we calculate a standard unsigned exponent
        if exp >= 0 {
            return self.checked_powu(exp as u64);
        }

        // Get the unsigned exponent
        let exp = exp.unsigned_abs();
        let pow = match self.checked_powu(exp) {
            Some(v) => v,
            None => return None,
        };
        Decimal::ONE.checked_div(pow)
    }

    fn powu(&self, exp: u64) -> Decimal {
        match self.checked_powu(exp) {
            Some(result) => result,
            None => panic!("Pow overflowed"),
        }
    }

    fn checked_powu(&self, exp: u64) -> Option<Decimal> {
        if exp == 0 {
            return Some(Decimal::ONE);
        }
        if self.is_zero() {
            return Some(Decimal::ZERO);
        }
        if self.is_one() {
            return Some(Decimal::ONE);
        }

        match exp {
            0 => unreachable!(),
            1 => Some(*self),
            2 => self.checked_mul(*self),
            // Do the exponentiation by multiplying squares:
            //   y = Sum (for each 1 bit in binary representation) of (2 ^ bit)
            //   x ^ y = Sum (for each 1 bit in y) of (x ^ (2 ^ bit))
            // See: https://en.wikipedia.org/wiki/Exponentiation_by_squaring
            _ => {
                let mut product = Decimal::ONE;
                let mut mask = exp;
                let mut power = *self;

                // Run through just enough 1 bits
                for n in 0..(64 - exp.leading_zeros()) {
                    if n > 0 {
                        power = power.checked_mul(power)?;
                        mask >>= 1;
                    }
                    if mask & 0x01 > 0 {
                        match product.checked_mul(power) {
                            Some(r) => product = r,
                            None => return None,
                        };
                    }
                }
                product.normalize_assign();
                Some(product)
            }
        }
    }

    fn powf(&self, exp: f64) -> Decimal {
        match self.checked_powf(exp) {
            Some(result) => result,
            None => panic!("Pow overflowed"),
        }
    }

    fn checked_powf(&self, exp: f64) -> Option<Decimal> {
        let exp = match Decimal::from_f64(exp) {
            Some(f) => f,
            None => return None,
        };
        self.checked_powd(exp)
    }

    fn powd(&self, exp: Decimal) -> Decimal {
        match self.checked_powd(exp) {
            Some(result) => result,
            None => panic!("Pow overflowed"),
        }
    }

    fn checked_powd(&self, exp: Decimal) -> Option<Decimal> {
        if exp.is_zero() {
            return Some(Decimal::ONE);
        }
        if self.is_zero() {
            return Some(Decimal::ZERO);
        }
        if self.is_one() {
            return Some(Decimal::ONE);
        }
        if exp.is_one() {
            return Some(*self);
        }

        // If the scale is 0 then it's a trivial calculation
        let exp = exp.normalize();
        if exp.scale() == 0 {
            if exp.mid() != 0 || exp.hi() != 0 {
                // Exponent way too big
                return None;
            }

            return if exp.is_sign_negative() {
                self.checked_powi(-(exp.lo() as i64))
            } else {
                self.checked_powu(exp.lo() as u64)
            };
        }

        // We do some approximations since we've got a decimal exponent.
        // For positive bases: a^b = exp(b*ln(a))
        let negative = self.is_sign_negative();
        let e = match self.abs().ln().checked_mul(exp) {
            Some(e) => e,
            None => return None,
        };
        let mut result = e.checked_exp()?;
        result.set_sign_negative(negative);
        Some(result)
    }

    fn sqrt(&self) -> Option<Decimal> {
        if self.is_sign_negative() {
            return None;
        }

        if self.is_zero() {
            return Some(Decimal::ZERO);
        }

        // Start with an arbitrary number as the first guess
        let mut result = self / Decimal::TWO;
        // Too small to represent, so we start with self
        // Future iterations could actually avoid using a decimal altogether and use a buffered
        // vector, only combining back into a decimal on return
        if result.is_zero() {
            result = *self;
        }
        let mut last = result + Decimal::ONE;

        // Keep going while the difference is larger than the tolerance
        let mut circuit_breaker = 0;
        while last != result {
            circuit_breaker += 1;
            assert!(circuit_breaker < 1000, "geo mean circuit breaker");

            last = result;
            result = (result + self / result) / Decimal::TWO;
        }

        Some(result)
    }

    #[cfg(feature = "maths-nopanic")]
    fn ln(&self) -> Decimal {
        match self.checked_ln() {
            Some(result) => result,
            None => Decimal::ZERO,
        }
    }

    #[cfg(not(feature = "maths-nopanic"))]
    fn ln(&self) -> Decimal {
        match self.checked_ln() {
            Some(result) => result,
            None => {
                if self.is_sign_negative() {
                    panic!("Unable to calculate ln for negative numbers")
                } else if self.is_zero() {
                    panic!("Unable to calculate ln for zero")
                } else {
                    panic!("Calculation of ln failed for unknown reasons")
                }
            }
        }
    }

    fn checked_ln(&self) -> Option<Decimal> {
        if self.is_sign_negative() || self.is_zero() {
            return None;
        }
        if self.is_one() {
            return Some(Decimal::ZERO);
        }

        // Approximate using Taylor Series
        let mut x = *self;
        let mut count = 0;
        while x >= Decimal::ONE {
            x *= Decimal::E_INVERSE;
            count += 1;
        }
        while x <= Decimal::E_INVERSE {
            x *= Decimal::E;
            count -= 1;
        }
        x -= Decimal::ONE;
        if x.is_zero() {
            return Some(Decimal::new(count, 0));
        }
        let mut result = Decimal::ZERO;
        let mut iteration = 0;
        let mut y = Decimal::ONE;
        let mut last = Decimal::ONE;
        while last != result && iteration < 100 {
            iteration += 1;
            last = result;
            y *= -x;
            result += y / Decimal::new(iteration, 0);
        }
        Some(Decimal::new(count, 0) - result)
    }

    #[cfg(feature = "maths-nopanic")]
    fn log10(&self) -> Decimal {
        match self.checked_log10() {
            Some(result) => result,
            None => Decimal::ZERO,
        }
    }

    #[cfg(not(feature = "maths-nopanic"))]
    fn log10(&self) -> Decimal {
        match self.checked_log10() {
            Some(result) => result,
            None => {
                if self.is_sign_negative() {
                    panic!("Unable to calculate log10 for negative numbers")
                } else if self.is_zero() {
                    panic!("Unable to calculate log10 for zero")
                } else {
                    panic!("Calculation of log10 failed for unknown reasons")
                }
            }
        }
    }

    fn checked_log10(&self) -> Option<Decimal> {
        use crate::ops::array::{div_by_u32, is_all_zero};
        // Early exits
        if self.is_sign_negative() || self.is_zero() {
            return None;
        }
        if self.is_one() {
            return Some(Decimal::ZERO);
        }

        // This uses a very basic method for calculating log10. We know the following is true:
        //   log10(n) = ln(n) / ln(10)
        // From this we can perform some small optimizations:
        //  1. ln(10) is a constant
        //  2. Multiplication is faster than division, so we can pre-calculate the constant 1/ln(10)
        // This allows us to then simplify log10(n) to:
        //   log10(n) = C * ln(n)

        // Before doing all of this however, we see if there are simple calculations to be made.
        let scale = self.scale();
        let mut working = self.mantissa_array3();

        // Check for scales less than 1 as an early exit
        if scale > 0 && working[2] == 0 && working[1] == 0 && working[0] == 1 {
            return Some(Decimal::from_parts(scale, 0, 0, true, 0));
        }

        // Loop for detecting bordering base 10 values
        let mut result = 0;
        let mut base10 = true;
        while !is_all_zero(&working) {
            let remainder = div_by_u32(&mut working, 10u32);
            if remainder != 0 {
                base10 = false;
                break;
            }
            result += 1;
            if working[2] == 0 && working[1] == 0 && working[0] == 1 {
                break;
            }
        }
        if base10 {
            return Some((result - scale as i32).into());
        }

        self.checked_ln().map(|result| LN10_INVERSE * result)
    }

    fn erf(&self) -> Decimal {
        if self.is_sign_positive() {
            let one = &Decimal::ONE;

            let xa1 = self * Decimal::from_parts(705230784, 0, 0, false, 10);
            let xa2 = self.powi(2) * Decimal::from_parts(422820123, 0, 0, false, 10);
            let xa3 = self.powi(3) * Decimal::from_parts(92705272, 0, 0, false, 10);
            let xa4 = self.powi(4) * Decimal::from_parts(1520143, 0, 0, false, 10);
            let xa5 = self.powi(5) * Decimal::from_parts(2765672, 0, 0, false, 10);
            let xa6 = self.powi(6) * Decimal::from_parts(430638, 0, 0, false, 10);

            let sum = one + xa1 + xa2 + xa3 + xa4 + xa5 + xa6;
            one - (one / sum.powi(16))
        } else {
            -self.abs().erf()
        }
    }

    fn norm_cdf(&self) -> Decimal {
        (Decimal::ONE + (self / Decimal::from_parts(2318911239, 3292722, 0, false, 16)).erf()) / Decimal::TWO
    }

    fn norm_pdf(&self) -> Decimal {
        match self.checked_norm_pdf() {
            Some(d) => d,
            None => panic!("Norm Pdf overflowed"),
        }
    }

    fn checked_norm_pdf(&self) -> Option<Decimal> {
        let sqrt2pi = Decimal::from_parts_raw(2133383024, 2079885984, 1358845910, 1835008);
        let factor = -self.checked_powi(2)?;
        let factor = factor.checked_div(Decimal::TWO)?;
        factor.checked_exp()?.checked_div(sqrt2pi)
    }

    fn sin(&self) -> Decimal {
        match self.checked_sin() {
            Some(x) => x,
            None => panic!("Sin overflowed"),
        }
    }

    fn checked_sin(&self) -> Option<Decimal> {
        if self.is_zero() {
            return Some(Decimal::ZERO);
        }
        if self.is_sign_negative() {
            // -Sin(-x)
            return (-self).checked_sin().map(|x| -x);
        }
        if self >= &Decimal::TWO_PI {
            // Reduce large numbers early - we can do this using rem to constrain to a range
            let adjusted = self.checked_rem(Decimal::TWO_PI)?;
            return adjusted.checked_sin();
        }
        if self >= &Decimal::PI {
            // -Sin(x-π)
            return (self - Decimal::PI).checked_sin().map(|x| -x);
        }
        if self > &Decimal::QUARTER_PI {
            // Cos(π2-x)
            return (Decimal::HALF_PI - self).checked_cos();
        }

        // Taylor series:
        // ∑(n=0 to ∞) : ((−1)^n / (2n + 1)!) * x^(2n + 1) , x∈R
        // First few expansions:
        // x^1/1! - x^3/3! + x^5/5! - x^7/7! + x^9/9!
        let mut result = Decimal::ZERO;
        for n in 0..TRIG_SERIES_UPPER_BOUND {
            let x = 2 * n + 1;
            let element = self.checked_powi(x as i64)?.checked_div(FACTORIAL[x])?;
            if n & 0x1 == 0 {
                result += element;
            } else {
                result -= element;
            }
        }
        Some(result)
    }

    fn cos(&self) -> Decimal {
        match self.checked_cos() {
            Some(x) => x,
            None => panic!("Cos overflowed"),
        }
    }

    fn checked_cos(&self) -> Option<Decimal> {
        if self.is_zero() {
            return Some(Decimal::ONE);
        }
        if self.is_sign_negative() {
            // Cos(-x)
            return (-self).checked_cos();
        }
        if self >= &Decimal::TWO_PI {
            // Reduce large numbers early - we can do this using rem to constrain to a range
            let adjusted = self.checked_rem(Decimal::TWO_PI)?;
            return adjusted.checked_cos();
        }
        if self >= &Decimal::PI {
            // -Cos(x-π)
            return (self - Decimal::PI).checked_cos().map(|x| -x);
        }
        if self > &Decimal::QUARTER_PI {
            // Sin(π2-x)
            return (Decimal::HALF_PI - self).checked_sin();
        }

        // Taylor series:
        // ∑(n=0 to ∞) : ((−1)^n / (2n)!) * x^(2n) , x∈R
        // First few expansions:
        // x^0/0! - x^2/2! + x^4/4! - x^6/6! + x^8/8!
        let mut result = Decimal::ZERO;
        for n in 0..TRIG_SERIES_UPPER_BOUND {
            let x = 2 * n;
            let element = self.checked_powi(x as i64)?.checked_div(FACTORIAL[x])?;
            if n & 0x1 == 0 {
                result += element;
            } else {
                result -= element;
            }
        }
        Some(result)
    }

    fn tan(&self) -> Decimal {
        match self.checked_tan() {
            Some(x) => x,
            None => panic!("Tan overflowed"),
        }
    }

    fn checked_tan(&self) -> Option<Decimal> {
        if self.is_zero() {
            return Some(Decimal::ZERO);
        }
        if self.is_sign_negative() {
            // -Tan(-x)
            return (-self).checked_tan().map(|x| -x);
        }
        if self >= &Decimal::TWO_PI {
            // Reduce large numbers early - we can do this using rem to constrain to a range
            let adjusted = self.checked_rem(Decimal::TWO_PI)?;
            return adjusted.checked_tan();
        }
        // Reduce to 0 <= x <= PI
        if self >= &Decimal::PI {
            // Tan(x-π)
            return (self - Decimal::PI).checked_tan();
        }
        // Reduce to 0 <= x <= PI/2
        if self > &Decimal::HALF_PI {
            // We can use the symmetrical function inside the first quadrant
            // e.g. tan(x) = -tan((PI/2 - x) + PI/2)
            return ((Decimal::HALF_PI - self) + Decimal::HALF_PI).checked_tan().map(|x| -x);
        }

        // It has now been reduced to 0 <= x <= PI/2. If it is >= PI/4 we can make it even smaller
        // by calculating tan(PI/2 - x) and taking the reciprocal
        if self > &Decimal::QUARTER_PI {
            return match (Decimal::HALF_PI - self).checked_tan() {
                Some(x) => Decimal::ONE.checked_div(x),
                None => None,
            };
        }

        // Due the way that tan(x) sharply tends towards infinity, we try to optimize
        // the resulting accuracy by using Trigonometric identity when > PI/8. We do this by
        // replacing the angle with one that is half as big.
        if self > &EIGHTH_PI {
            // Work out tan(x/2)
            let tan_half = (self / Decimal::TWO).checked_tan()?;
            // Work out the dividend i.e. 2tan(x/2)
            let dividend = Decimal::TWO.checked_mul(tan_half)?;

            // Work out the divisor i.e. 1 - tan^2(x/2)
            let squared = tan_half.checked_mul(tan_half)?;
            let divisor = Decimal::ONE - squared;
            // Treat this as infinity
            if divisor.is_zero() {
                return None;
            }
            return dividend.checked_div(divisor);
        }

        // Do a polynomial approximation based upon the Maclaurin series.
        // This can be simplified to something like:
        //
        // ∑(n=1,3,5,7,9)(f(n)(0)/n!)x^n
        //
        // First few expansions (which we leverage):
        // (f'(0)/1!)x^1 + (f'''(0)/3!)x^3 + (f'''''(0)/5!)x^5 + (f'''''''/7!)x^7
        //
        // x + (1/3)x^3 + (2/15)x^5 + (17/315)x^7 + (62/2835)x^9 + (1382/155925)x^11
        //
        // (Generated by https://www.wolframalpha.com/widgets/view.jsp?id=fe1ad8d4f5dbb3cb866d0c89beb527a6)
        // The more terms, the better the accuracy. This generates accuracy within approx 10^-8 for angles
        // less than PI/8.
        const SERIES: [(Decimal, u64); 6] = [
            // 1 / 3
            (Decimal::from_parts_raw(89478485, 347537611, 180700362, 1835008), 3),
            // 2 / 15
            (Decimal::from_parts_raw(894784853, 3574988881, 72280144, 1835008), 5),
            // 17 / 315
            (Decimal::from_parts_raw(905437054, 3907911371, 2925624, 1769472), 7),
            // 62 / 2835
            (Decimal::from_parts_raw(3191872741, 2108928381, 11855473, 1835008), 9),
            // 1382 / 155925
            (Decimal::from_parts_raw(3482645539, 2612995122, 4804769, 1835008), 11),
            // 21844 / 6081075
            (Decimal::from_parts_raw(4189029078, 2192791200, 1947296, 1835008), 13),
        ];
        let mut result = *self;
        for (fraction, pow) in SERIES {
            result += fraction * self.powu(pow);
        }
        Some(result)
    }
}

impl Pow<Decimal> for Decimal {
    type Output = Decimal;

    fn pow(self, rhs: Decimal) -> Self::Output {
        MathematicalOps::powd(&self, rhs)
    }
}

impl Pow<u64> for Decimal {
    type Output = Decimal;

    fn pow(self, rhs: u64) -> Self::Output {
        MathematicalOps::powu(&self, rhs)
    }
}

impl Pow<i64> for Decimal {
    type Output = Decimal;

    fn pow(self, rhs: i64) -> Self::Output {
        MathematicalOps::powi(&self, rhs)
    }
}

impl Pow<f64> for Decimal {
    type Output = Decimal;

    fn pow(self, rhs: f64) -> Self::Output {
        MathematicalOps::powf(&self, rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;

    #[test]
    fn test_factorials() {
        assert_eq!("1", FACTORIAL[0].to_string(), "0!");
        assert_eq!("1", FACTORIAL[1].to_string(), "1!");
        assert_eq!("2", FACTORIAL[2].to_string(), "2!");
        assert_eq!("6", FACTORIAL[3].to_string(), "3!");
        assert_eq!("24", FACTORIAL[4].to_string(), "4!");
        assert_eq!("120", FACTORIAL[5].to_string(), "5!");
        assert_eq!("720", FACTORIAL[6].to_string(), "6!");
        assert_eq!("5040", FACTORIAL[7].to_string(), "7!");
        assert_eq!("40320", FACTORIAL[8].to_string(), "8!");
        assert_eq!("362880", FACTORIAL[9].to_string(), "9!");
        assert_eq!("3628800", FACTORIAL[10].to_string(), "10!");
        assert_eq!("39916800", FACTORIAL[11].to_string(), "11!");
        assert_eq!("479001600", FACTORIAL[12].to_string(), "12!");
        assert_eq!("6227020800", FACTORIAL[13].to_string(), "13!");
        assert_eq!("87178291200", FACTORIAL[14].to_string(), "14!");
        assert_eq!("1307674368000", FACTORIAL[15].to_string(), "15!");
        assert_eq!("20922789888000", FACTORIAL[16].to_string(), "16!");
        assert_eq!("355687428096000", FACTORIAL[17].to_string(), "17!");
        assert_eq!("6402373705728000", FACTORIAL[18].to_string(), "18!");
        assert_eq!("121645100408832000", FACTORIAL[19].to_string(), "19!");
        assert_eq!("2432902008176640000", FACTORIAL[20].to_string(), "20!");
        assert_eq!("51090942171709440000", FACTORIAL[21].to_string(), "21!");
        assert_eq!("1124000727777607680000", FACTORIAL[22].to_string(), "22!");
        assert_eq!("25852016738884976640000", FACTORIAL[23].to_string(), "23!");
        assert_eq!("620448401733239439360000", FACTORIAL[24].to_string(), "24!");
        assert_eq!("15511210043330985984000000", FACTORIAL[25].to_string(), "25!");
        assert_eq!("403291461126605635584000000", FACTORIAL[26].to_string(), "26!");
        assert_eq!("10888869450418352160768000000", FACTORIAL[27].to_string(), "27!");
    }
}
