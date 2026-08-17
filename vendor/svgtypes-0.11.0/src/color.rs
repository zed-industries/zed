use crate::{colors, ByteExt, Error, Stream};

/// Representation of the [`<color>`] type.
///
/// [`<color>`]: https://www.w3.org/TR/css-color-3/
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    /// Constructs a new `Color` from RGB values.
    #[inline]
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    /// Constructs a new `Color` from RGBA values.
    #[inline]
    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Constructs a new `Color` set to black.
    #[inline]
    pub fn black() -> Color {
        Color::new_rgb(0, 0, 0)
    }

    /// Constructs a new `Color` set to white.
    #[inline]
    pub fn white() -> Color {
        Color::new_rgb(255, 255, 255)
    }

    /// Constructs a new `Color` set to gray.
    #[inline]
    pub fn gray() -> Color {
        Color::new_rgb(128, 128, 128)
    }

    /// Constructs a new `Color` set to red.
    #[inline]
    pub fn red() -> Color {
        Color::new_rgb(255, 0, 0)
    }

    /// Constructs a new `Color` set to green.
    #[inline]
    pub fn green() -> Color {
        Color::new_rgb(0, 128, 0)
    }

    /// Constructs a new `Color` set to blue.
    #[inline]
    pub fn blue() -> Color {
        Color::new_rgb(0, 0, 255)
    }
}

impl std::str::FromStr for Color {
    type Err = Error;

    /// Parses [CSS3](https://www.w3.org/TR/css-color-3/) `Color` from a string.
    ///
    /// # Errors
    ///
    ///  - Returns error if a color has an invalid format.
    ///  - Returns error if `<color>` is followed by `<icccolor>`. It's not supported.
    ///
    /// # Notes
    ///
    ///  - Any non-`hexdigit` bytes will be treated as `0`.
    ///  - The [SVG 1.1 spec] has an error.
    ///    There should be a `number`, not an `integer` for percent values ([details]).
    ///  - It also supports 4 digits and 8 digits hex notation from the
    ///    [CSS Color Module Level 4][css-color-4-hex].
    ///
    /// [SVG 1.1 spec]: https://www.w3.org/TR/SVG11/types.html#DataTypeColor
    /// [details]: https://lists.w3.org/Archives/Public/www-svg/2014Jan/0109.html
    /// [css-color-4-hex]: https://www.w3.org/TR/css-color-4/#hex-notation
    fn from_str(text: &str) -> Result<Self, Error> {
        let mut s = Stream::from(text);
        let color = s.parse_color()?;

        // Check that we are at the end of the stream. Otherwise color can be followed by icccolor,
        // which is not supported.
        s.skip_spaces();
        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(color)
    }
}

impl<'a> Stream<'a> {
    /// Tries to parse a color, but doesn't advance on error.
    pub fn try_parse_color(&mut self) -> Option<Color> {
        let mut s = *self;
        if let Ok(color) = s.parse_color() {
            *self = s;
            Some(color)
        } else {
            None
        }
    }

    /// Parses a color.
    pub fn parse_color(&mut self) -> Result<Color, Error> {
        self.skip_spaces();

        let mut color = Color::black();

        if self.curr_byte()? == b'#' {
            // See https://www.w3.org/TR/css-color-4/#hex-notation
            self.advance(1);
            let color_str = self.consume_bytes(|_, c| c.is_hex_digit()).as_bytes();
            // get color data len until first space or stream end
            match color_str.len() {
                6 => {
                    // #rrggbb
                    color.red = hex_pair(color_str[0], color_str[1]);
                    color.green = hex_pair(color_str[2], color_str[3]);
                    color.blue = hex_pair(color_str[4], color_str[5]);
                }
                8 => {
                    // #rrggbbaa
                    color.red = hex_pair(color_str[0], color_str[1]);
                    color.green = hex_pair(color_str[2], color_str[3]);
                    color.blue = hex_pair(color_str[4], color_str[5]);
                    color.alpha = hex_pair(color_str[6], color_str[7]);
                }
                3 => {
                    // #rgb
                    color.red = short_hex(color_str[0]);
                    color.green = short_hex(color_str[1]);
                    color.blue = short_hex(color_str[2]);
                }
                4 => {
                    // #rgba
                    color.red = short_hex(color_str[0]);
                    color.green = short_hex(color_str[1]);
                    color.blue = short_hex(color_str[2]);
                    color.alpha = short_hex(color_str[3]);
                }
                _ => {
                    return Err(Error::InvalidValue);
                }
            }
        } else {
            // TODO: remove allocation
            let name = self.consume_ident().to_ascii_lowercase();
            if name == "rgb" || name == "rgba" {
                self.consume_byte(b'(')?;

                let mut is_percent = false;
                let value = self.parse_number()?;
                if self.starts_with(b"%") {
                    self.advance(1);
                    is_percent = true;
                }
                self.skip_spaces();
                self.parse_list_separator();

                if is_percent {
                    fn from_percent(v: f64) -> u8 {
                        let n = (v * 255.0).round() as i32;
                        bound(0, n, 255) as u8
                    }

                    color.red = from_percent(value / 100.0);
                    color.green = from_percent(self.parse_list_number_or_percent()?);
                    color.blue = from_percent(self.parse_list_number_or_percent()?);
                } else {
                    color.red = bound(0, value as i32, 255) as u8;
                    color.green = bound(0, self.parse_list_integer()?, 255) as u8;
                    color.blue = bound(0, self.parse_list_integer()?, 255) as u8;
                }

                self.skip_spaces();
                if !self.starts_with(b")") {
                    color.alpha = (f64_bound(0.0, self.parse_list_number()?, 1.0) * 255.0) as u8;
                }

                self.skip_spaces();
                self.consume_byte(b')')?;
            } else if name == "hsl" || name == "hsla" {
                self.consume_byte(b'(')?;

                let mut hue = self.parse_list_integer()?;
                hue = ((hue % 360) + 360) % 360;

                let saturation = f64_bound(0.0, self.parse_list_number_or_percent()?, 1.0);
                let lightness = f64_bound(0.0, self.parse_list_number_or_percent()?, 1.0);

                color = hsl_to_rgb(hue as f32 / 60.0, saturation as f32, lightness as f32);

                self.skip_spaces();
                if !self.starts_with(b")") {
                    color.alpha = (f64_bound(0.0, self.parse_list_number()?, 1.0) * 255.0) as u8;
                }

                self.skip_spaces();
                self.consume_byte(b')')?;
            } else {
                match colors::from_str(&name) {
                    Some(c) => {
                        color = c;
                    }
                    None => {
                        return Err(Error::InvalidValue);
                    }
                }
            }
        }

        Ok(color)
    }
}

#[inline]
fn from_hex(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => b'0',
    }
}

#[inline]
fn short_hex(c: u8) -> u8 {
    let h = from_hex(c);
    (h << 4) | h
}

#[inline]
fn hex_pair(c1: u8, c2: u8) -> u8 {
    let h1 = from_hex(c1);
    let h2 = from_hex(c2);
    (h1 << 4) | h2
}

// `hue` is in a 0..6 range, while `saturation` and `lightness` are in a 0..=1 range.
// Based on https://www.w3.org/TR/css-color-3/#hsl-color
fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> Color {
    let t2 = if lightness <= 0.5 {
        lightness * (saturation + 1.0)
    } else {
        lightness + saturation - (lightness * saturation)
    };

    let t1 = lightness * 2.0 - t2;
    let red = hue_to_rgb(t1, t2, hue + 2.0);
    let green = hue_to_rgb(t1, t2, hue);
    let blue = hue_to_rgb(t1, t2, hue - 2.0);
    Color::new_rgb(
        (red * 255.0) as u8,
        (green * 255.0) as u8,
        (blue * 255.0) as u8,
    )
}

fn hue_to_rgb(t1: f32, t2: f32, mut hue: f32) -> f32 {
    if hue < 0.0 {
        hue += 6.0;
    }
    if hue >= 6.0 {
        hue -= 6.0;
    }

    if hue < 1.0 {
        (t2 - t1) * hue + t1
    } else if hue < 3.0 {
        t2
    } else if hue < 4.0 {
        (t2 - t1) * (4.0 - hue) + t1
    } else {
        t1
    }
}

#[inline]
fn bound<T: Ord>(min: T, val: T, max: T) -> T {
    std::cmp::max(min, std::cmp::min(max, val))
}

#[inline]
fn f64_bound(min: f64, val: f64, max: f64) -> f64 {
    debug_assert!(min.is_finite());
    debug_assert!(val.is_finite());
    debug_assert!(max.is_finite());
    val.max(min).min(max)
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::Color;

    macro_rules! test {
        ($name:ident, $text:expr, $color:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Color::from_str($text).unwrap(), $color);
            }
        };
    }

    test!(
        rrggbb,
        "#ff0000",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        rrggbb_upper,
        "#FF0000",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        rgb_hex,
        "#f00",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        rrggbbaa,
        "#ff0000ff",
        Color::new_rgba(255, 0, 0, 255)
    );

    test!(
        rrggbbaa_upper,
        "#FF0000FF",
        Color::new_rgba(255, 0, 0, 255)
    );

    test!(
        rgba_hex,
        "#f00f",
        Color::new_rgba(255, 0, 0, 255)
    );

    test!(
        rrggbb_spaced,
        "  #ff0000  ",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        rgb_numeric,
        "rgb(254, 203, 231)",
        Color::new_rgb(254, 203, 231)
    );

    test!(
        rgb_numeric_spaced,
        " rgb( 77 , 77 , 77 ) ",
        Color::new_rgb(77, 77, 77)
    );

    test!(
        rgb_percentage,
        "rgb(50%, 50%, 50%)",
        Color::new_rgb(128, 128, 128)
    );

    test!(
        rgb_percentage_overflow,
        "rgb(140%, -10%, 130%)",
        Color::new_rgb(255, 0, 255)
    );

    test!(
        rgb_percentage_float,
        "rgb(33.333%,46.666%,93.333%)",
        Color::new_rgb(85, 119, 238)
    );

    test!(
        rgb_numeric_upper_case,
        "RGB(254, 203, 231)",
        Color::new_rgb(254, 203, 231)
    );

    test!(
        rgb_numeric_mixed_case,
        "RgB(254, 203, 231)",
        Color::new_rgb(254, 203, 231)
    );

    test!(
        name_red,
        "red",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        name_red_spaced,
        " red ",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        name_red_upper_case,
        "RED",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        name_red_mixed_case,
        "ReD",
        Color::new_rgb(255, 0, 0)
    );

    test!(
        name_cornflowerblue,
        "cornflowerblue",
        Color::new_rgb(100, 149, 237)
    );

    test!(
        transparent,
        "transparent",
        Color::new_rgba(0, 0, 0, 0)
    );

    test!(
        rgba_half,
        "rgba(10, 20, 30, 0.5)",
        Color::new_rgba(10, 20, 30, 127)
    );

    test!(
        rgba_negative,
        "rgba(10, 20, 30, -2)",
        Color::new_rgba(10, 20, 30, 0)
    );

    test!(
        rgba_large_alpha,
        "rgba(10, 20, 30, 2)",
        Color::new_rgba(10, 20, 30, 255)
    );

    test!(
        rgb_with_alpha,
        "rgb(10, 20, 30, 0.5)",
        Color::new_rgba(10, 20, 30, 127)
    );

    test!(
        hsl_green,
        "hsl(120, 100%, 75%)",
        Color::new_rgba(127, 255, 127, 255)
    );

    test!(
        hsl_yellow,
        "hsl(60, 100%, 50%)",
        Color::new_rgba(255, 255, 0, 255)
    );

    test!(
        hsl_hue_360,
        "hsl(360, 100%, 100%)",
        Color::new_rgba(255, 255, 255, 255)
    );

    test!(
        hsl_out_of_bounds,
        "hsl(800, 150%, -50%)",
        Color::new_rgba(0, 0, 0, 255)
    );

    test!(
        hsla_green,
        "hsla(120, 100%, 75%, 0.5)",
        Color::new_rgba(127, 255, 127, 127)
    );

    test!(
        hsl_with_alpha,
        "hsl(120, 100%, 75%, 0.5)",
        Color::new_rgba(127, 255, 127, 127)
    );

    macro_rules! test_err {
        ($name:ident, $text:expr, $err:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Color::from_str($text).unwrap_err().to_string(), $err);
            }
        };
    }

    test_err!(
        not_a_color_1,
        "text",
        "invalid value"
    );

    test_err!(
        icc_color_not_supported_1,
        "#CD853F icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
        "unexpected data at position 9"
    );

    test_err!(
        icc_color_not_supported_2,
        "red icc-color(acmecmyk, 0.11, 0.48, 0.83, 0.00)",
        "unexpected data at position 5"
    );

    test_err!(
        invalid_input_1,
        "rgb(-0\x0d",
        "unexpected end of stream"
    );

    test_err!(
        invalid_input_2,
        "#9ߞpx! ;",
        "invalid value"
    );

    test_err!(
        rgba_with_percent_alpha,
        "rgba(10, 20, 30, 5%)",
        "expected ')' not '%' at position 19"
    );

    test_err!(
        rgb_mixed_units,
        "rgb(140%, -10mm, 130pt)",
        "invalid number at position 14"
    );
}
