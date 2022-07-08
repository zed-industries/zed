use alacritty_terminal::{ansi::Color as AnsiColor, term::color::Rgb as AlacRgb};
use gpui::color::Color;
use theme::TerminalStyle;

///Converts a 2, 8, or 24 bit color ANSI color to the GPUI equivalent
pub fn convert_color(alac_color: &AnsiColor, style: &TerminalStyle) -> Color {
    match alac_color {
        //Named and theme defined colors
        alacritty_terminal::ansi::Color::Named(n) => match n {
            alacritty_terminal::ansi::NamedColor::Black => style.black,
            alacritty_terminal::ansi::NamedColor::Red => style.red,
            alacritty_terminal::ansi::NamedColor::Green => style.green,
            alacritty_terminal::ansi::NamedColor::Yellow => style.yellow,
            alacritty_terminal::ansi::NamedColor::Blue => style.blue,
            alacritty_terminal::ansi::NamedColor::Magenta => style.magenta,
            alacritty_terminal::ansi::NamedColor::Cyan => style.cyan,
            alacritty_terminal::ansi::NamedColor::White => style.white,
            alacritty_terminal::ansi::NamedColor::BrightBlack => style.bright_black,
            alacritty_terminal::ansi::NamedColor::BrightRed => style.bright_red,
            alacritty_terminal::ansi::NamedColor::BrightGreen => style.bright_green,
            alacritty_terminal::ansi::NamedColor::BrightYellow => style.bright_yellow,
            alacritty_terminal::ansi::NamedColor::BrightBlue => style.bright_blue,
            alacritty_terminal::ansi::NamedColor::BrightMagenta => style.bright_magenta,
            alacritty_terminal::ansi::NamedColor::BrightCyan => style.bright_cyan,
            alacritty_terminal::ansi::NamedColor::BrightWhite => style.bright_white,
            alacritty_terminal::ansi::NamedColor::Foreground => style.foreground,
            alacritty_terminal::ansi::NamedColor::Background => style.background,
            alacritty_terminal::ansi::NamedColor::Cursor => style.cursor,
            alacritty_terminal::ansi::NamedColor::DimBlack => style.dim_black,
            alacritty_terminal::ansi::NamedColor::DimRed => style.dim_red,
            alacritty_terminal::ansi::NamedColor::DimGreen => style.dim_green,
            alacritty_terminal::ansi::NamedColor::DimYellow => style.dim_yellow,
            alacritty_terminal::ansi::NamedColor::DimBlue => style.dim_blue,
            alacritty_terminal::ansi::NamedColor::DimMagenta => style.dim_magenta,
            alacritty_terminal::ansi::NamedColor::DimCyan => style.dim_cyan,
            alacritty_terminal::ansi::NamedColor::DimWhite => style.dim_white,
            alacritty_terminal::ansi::NamedColor::BrightForeground => style.bright_foreground,
            alacritty_terminal::ansi::NamedColor::DimForeground => style.dim_foreground,
        },
        //'True' colors
        alacritty_terminal::ansi::Color::Spec(rgb) => Color::new(rgb.r, rgb.g, rgb.b, u8::MAX),
        //8 bit, indexed colors
        alacritty_terminal::ansi::Color::Indexed(i) => get_color_at_index(&(*i as usize), style),
    }
}

///Converts an 8 bit ANSI color to it's GPUI equivalent.
///Accepts usize for compatability with the alacritty::Colors interface,
///Other than that use case, should only be called with values in the [0,255] range
pub fn get_color_at_index(index: &usize, style: &TerminalStyle) -> Color {
    match index {
        //0-15 are the same as the named colors above
        0 => style.black,
        1 => style.red,
        2 => style.green,
        3 => style.yellow,
        4 => style.blue,
        5 => style.magenta,
        6 => style.cyan,
        7 => style.white,
        8 => style.bright_black,
        9 => style.bright_red,
        10 => style.bright_green,
        11 => style.bright_yellow,
        12 => style.bright_blue,
        13 => style.bright_magenta,
        14 => style.bright_cyan,
        15 => style.bright_white,
        //16-231 are mapped to their RGB colors on a 0-5 range per channel
        16..=231 => {
            let (r, g, b) = rgb_for_index(&(*index as u8)); //Split the index into it's ANSI-RGB components
            let step = (u8::MAX as f32 / 5.).floor() as u8; //Split the RGB range into 5 chunks, with floor so no overflow
            Color::new(r * step, g * step, b * step, u8::MAX) //Map the ANSI-RGB components to an RGB color
        }
        //232-255 are a 24 step grayscale from black to white
        232..=255 => {
            let i = *index as u8 - 232; //Align index to 0..24
            let step = (u8::MAX as f32 / 24.).floor() as u8; //Split the RGB grayscale values into 24 chunks
            Color::new(i * step, i * step, i * step, u8::MAX) //Map the ANSI-grayscale components to the RGB-grayscale
        }
        //For compatability with the alacritty::Colors interface
        256 => style.foreground,
        257 => style.background,
        258 => style.cursor,
        259 => style.dim_black,
        260 => style.dim_red,
        261 => style.dim_green,
        262 => style.dim_yellow,
        263 => style.dim_blue,
        264 => style.dim_magenta,
        265 => style.dim_cyan,
        266 => style.dim_white,
        267 => style.bright_foreground,
        268 => style.black, //'Dim Background', non-standard color
        _ => Color::new(0, 0, 0, 255),
    }
}
///Generates the rgb channels in [0, 5] for a given index into the 6x6x6 ANSI color cube
///See: [8 bit ansi color](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
///
///Wikipedia gives a formula for calculating the index for a given color:
///
///index = 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5)
///
///This function does the reverse, calculating the r, g, and b components from a given index.
fn rgb_for_index(i: &u8) -> (u8, u8, u8) {
    debug_assert!(i >= &16 && i <= &231);
    let i = i - 16;
    let r = (i - (i % 36)) / 36;
    let g = ((i % 36) - (i % 6)) / 6;
    let b = (i % 36) % 6;
    (r, g, b)
}

//Convenience method to convert from a GPUI color to an alacritty Rgb
pub fn to_alac_rgb(color: Color) -> AlacRgb {
    AlacRgb {
        r: color.r,
        g: color.g,
        b: color.g,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rgb_for_index() {
        //Test every possible value in the color cube
        for i in 16..=231 {
            let (r, g, b) = crate::color_translation::rgb_for_index(&(i as u8));
            assert_eq!(i, 16 + 36 * r + 6 * g + b);
        }
    }
}
