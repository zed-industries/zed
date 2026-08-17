// The code that is related to float number handling
fn find_minimal_repr(n: f64, eps: f64) -> (f64, usize) {
    if eps >= 1.0 {
        return (n, 0);
    }
    if n - n.floor() < eps {
        (n.floor(), 0)
    } else if n.ceil() - n < eps {
        (n.ceil(), 0)
    } else {
        let (rem, pre) = find_minimal_repr((n - n.floor()) * 10.0, eps * 10.0);
        (n.floor() + rem / 10.0, pre + 1)
    }
}

#[allow(clippy::never_loop)]
fn float_to_string(n: f64, max_precision: usize, min_decimal: usize) -> String {
    let (mut result, mut count) = loop {
        let (sign, n) = if n < 0.0 { ("-", -n) } else { ("", n) };
        let int_part = n.floor();

        let dec_part =
            ((n.abs() - int_part.abs()) * (10.0f64).powi(max_precision as i32)).round() as u64;

        if dec_part == 0 || max_precision == 0 {
            break (format!("{}{:.0}", sign, int_part), 0);
        }

        let mut leading = "".to_string();
        let mut dec_result = format!("{}", dec_part);

        for _ in 0..(max_precision - dec_result.len()) {
            leading.push('0');
        }

        while let Some(c) = dec_result.pop() {
            if c != '0' {
                dec_result.push(c);
                break;
            }
        }

        break (
            format!("{}{:.0}.{}{}", sign, int_part, leading, dec_result),
            leading.len() + dec_result.len(),
        );
    };

    if count == 0 && min_decimal > 0 {
        result.push('.');
    }

    while count < min_decimal {
        result.push('0');
        count += 1;
    }
    result
}

/// Handles printing of floating point numbers
pub struct FloatPrettyPrinter {
    /// Whether scientific notation is allowed
    pub allow_scientific: bool,
    /// Minimum allowed number of decimal digits
    pub min_decimal: i32,
    /// Maximum allowed number of decimal digits
    pub max_decimal: i32,
}

impl FloatPrettyPrinter {
    /// Handles printing of floating point numbers
    pub fn print(&self, n: f64) -> String {
        let (tn, p) = find_minimal_repr(n, (10f64).powi(-self.max_decimal));
        let d_repr = float_to_string(tn, p, self.min_decimal as usize);
        if !self.allow_scientific {
            d_repr
        } else {
            if n == 0.0 {
                return "0".to_string();
            }

            let mut idx = n.abs().log10().floor();
            let mut exp = (10.0f64).powf(idx);

            if n.abs() / exp + 1e-5 >= 10.0 {
                idx += 1.0;
                exp *= 10.0;
            }

            if idx.abs() < 3.0 {
                return d_repr;
            }

            let (sn, sp) = find_minimal_repr(n / exp, 1e-5);
            let s_repr = format!(
                "{}e{}",
                float_to_string(sn, sp, self.min_decimal as usize),
                float_to_string(idx, 0, 0)
            );
            if s_repr.len() + 1 < d_repr.len() || (tn == 0.0 && n != 0.0) {
                s_repr
            } else {
                d_repr
            }
        }
    }
}

/// The function that pretty prints the floating number
/// Since rust doesn't have anything that can format a float with out appearance, so we just
/// implement a float pretty printing function, which finds the shortest representation of a
/// floating point number within the allowed error range.
///
/// - `n`: The float number to pretty-print
/// - `allow_sn`: Should we use scientific notation when possible
/// - **returns**: The pretty printed string
pub fn pretty_print_float(n: f64, allow_sn: bool) -> String {
    (FloatPrettyPrinter {
        allow_scientific: allow_sn,
        min_decimal: 0,
        max_decimal: 10,
    })
    .print(n)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pretty_printing() {
        assert_eq!(pretty_print_float(0.99999999999999999999, false), "1");
        assert_eq!(pretty_print_float(0.9999, false), "0.9999");
        assert_eq!(
            pretty_print_float(-1e-5 - 0.00000000000000001, true),
            "-1e-5"
        );
        assert_eq!(
            pretty_print_float(-1e-5 - 0.00000000000000001, false),
            "-0.00001"
        );
        assert_eq!(pretty_print_float(1e100, true), "1e100");
        assert_eq!(pretty_print_float(1234567890f64, true), "1234567890");
        assert_eq!(pretty_print_float(1000000001f64, true), "1e9");
    }
}
