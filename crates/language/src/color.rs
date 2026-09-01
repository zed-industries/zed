//! Detection and rewriting of color literals in source code.
//!
//! Colors are located with each language's `colors.scm` tree-sitter query, so
//! support for a new syntax is a query file rather than Rust code. A query
//! describes a color in one of two ways:
//!
//! * `@color.text` marks a node whose text is itself a color literal (`#ff00aa`,
//!   `rgb(255, 0, 170)`, `rebeccapurple`), parsed by [`parse_color_literal`].
//! * `@color.red`, `@color.green`, `@color.blue`, `@color.alpha` (or
//!   `@color.hue`, `@color.saturation`, `@color.lightness`) mark the individual
//!   numeric channels of a constructor call such as Bevy's
//!   `Color::srgb(0.2, 0.9, 0.4)`.
//!
//! Either way the pattern must also capture the whole construct as `@color`,
//! which is where the swatch is drawn.
//!
//! The channel scale is taken from `(#set! color.scale "unit" | "u8" |
//! "degrees")` when the query states it, and inferred from the matched text
//! otherwise: any channel written with a decimal point makes the whole match
//! unit-scaled.

use std::{iter, ops::Range};

use gpui::{Hsla, Rgba};
use language_core::{ColorCapture, ColorComponent, ColorScale};
use smallvec::SmallVec;
use text::BufferId;

use crate::BufferSnapshot;

/// A color literal found in a buffer, along with everything needed to rewrite
/// it in place when the user picks a different color.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorMatch {
    pub buffer_id: BufferId,
    /// The whole construct the swatch attaches to.
    pub range: Range<usize>,
    pub color: lsp::Color,
    pub replacement: ColorReplacement,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColorReplacement {
    /// The color occupies this exact span as a single literal.
    Literal {
        range: Range<usize>,
        notation: ColorNotation,
    },
    /// The color is spread across separate numeric channels.
    Channels(SmallVec<[ColorChannel; 4]>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorChannel {
    pub component: ColorComponent,
    pub range: Range<usize>,
    /// Decimal places the channel was written with, so rewrites keep the
    /// precision the author chose.
    pub precision: usize,
    /// The units this channel is written in. Alpha resolves separately from the
    /// other channels because CSS writes `rgba(255, 0, 0, 0.5)` — bytes for the
    /// color, a unit fraction for the alpha.
    pub scale: ColorScale,
}

/// How a textual color literal was written, so that a rewrite can preserve it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorNotation {
    Hex {
        digits_per_channel: usize,
        channels: usize,
        uppercase: bool,
    },
    /// `rgb(..)` / `rgba(..)`, with channels written as percentages or as 0-255.
    Rgb { alpha: bool, percent: bool },
    Hsl { alpha: bool },
    /// A CSS named color. There is no general way to name an arbitrary color,
    /// so edits fall back to hex.
    Named,
}

impl ColorMatch {
    /// Whether the construct can represent an alpha channel at all. Picking a
    /// translucent color for a construct that cannot store alpha would silently
    /// drop it, so callers should not offer alpha in that case.
    pub fn supports_alpha(&self) -> bool {
        match &self.replacement {
            ColorReplacement::Literal { notation, .. } => match notation {
                ColorNotation::Hex { channels, .. } => *channels == 4,
                ColorNotation::Rgb { alpha, .. } => *alpha,
                ColorNotation::Hsl { alpha } => *alpha,
                ColorNotation::Named => false,
            },
            ColorReplacement::Channels(channels) => channels
                .iter()
                .any(|channel| channel.component == ColorComponent::Alpha),
        }
    }

    /// The edits that rewrite this construct to `new_color`, as
    /// `(byte range, replacement text)` pairs in ascending order.
    pub fn edits(&self, new_color: lsp::Color) -> SmallVec<[(Range<usize>, String); 4]> {
        match &self.replacement {
            ColorReplacement::Literal { range, notation } => {
                let mut edits = SmallVec::new();
                edits.push((range.clone(), format_literal(new_color, *notation)));
                edits
            }
            ColorReplacement::Channels(channels) => {
                let hsla = Hsla::from(Rgba {
                    r: new_color.red,
                    g: new_color.green,
                    b: new_color.blue,
                    a: new_color.alpha,
                });
                let mut edits = channels
                    .iter()
                    .map(|channel| {
                        let unit_value = match channel.component {
                            ColorComponent::Red => new_color.red,
                            ColorComponent::Green => new_color.green,
                            ColorComponent::Blue => new_color.blue,
                            ColorComponent::Alpha => new_color.alpha,
                            ColorComponent::Hue => hsla.h,
                            ColorComponent::Saturation => hsla.s,
                            ColorComponent::Lightness => hsla.l,
                        };
                        let scaled = scale_out(unit_value, channel.component, channel.scale);
                        (
                            channel.range.clone(),
                            format_number(scaled, channel.precision),
                        )
                    })
                    .collect::<SmallVec<[_; 4]>>();
                edits.sort_by_key(|(range, _)| range.start);
                edits
            }
        }
    }
}

/// Convert a unit-scaled (0.0-1.0) channel into the units the source uses.
fn scale_out(unit_value: f32, component: ColorComponent, scale: ColorScale) -> f32 {
    match scale {
        ColorScale::Unit => unit_value,
        ColorScale::U8 => unit_value * 255.0,
        ColorScale::Degrees => match component {
            ColorComponent::Hue => unit_value * 360.0,
            ColorComponent::Alpha => unit_value,
            _ => unit_value * 100.0,
        },
    }
}

/// Convert a channel written in the source's units back to 0.0-1.0.
fn scale_in(value: f32, component: ColorComponent, scale: ColorScale) -> f32 {
    match scale {
        ColorScale::Unit => value,
        ColorScale::U8 => value / 255.0,
        ColorScale::Degrees => match component {
            ColorComponent::Hue => value / 360.0,
            ColorComponent::Alpha => value,
            _ => value / 100.0,
        },
    }
}

/// Format a channel value the way the source wrote it: `precision` of zero
/// means the author used an integer literal, anything else a float.
///
/// Floats are written with enough decimals to round-trip an 8-bit channel and
/// then trimmed, so `0.5` stays `0.5` rather than becoming `0.5000`. A trailing
/// decimal is always kept, because `1` does not parse as an `f32` in languages
/// like Rust where `1.0` does.
fn format_number(value: f32, precision: usize) -> String {
    if precision == 0 {
        return format!("{}", value.round() as i64);
    }
    let formatted = format!("{value:.4}");
    let trimmed = formatted.trim_end_matches('0');
    if trimmed.ends_with('.') {
        format!("{trimmed}0")
    } else {
        trimmed.to_string()
    }
}

fn format_literal(color: lsp::Color, notation: ColorNotation) -> String {
    let to_byte = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    match notation {
        ColorNotation::Named => format!(
            "#{:02x}{:02x}{:02x}",
            to_byte(color.red),
            to_byte(color.green),
            to_byte(color.blue)
        ),
        ColorNotation::Hex {
            digits_per_channel,
            channels,
            uppercase,
        } => {
            let mut values = vec![
                to_byte(color.red),
                to_byte(color.green),
                to_byte(color.blue),
            ];
            if channels == 4 {
                values.push(to_byte(color.alpha));
            }
            let mut out = String::from("#");
            for value in values {
                if digits_per_channel == 1 {
                    // `#abc` is `#aabbcc`, so only the high nibble survives.
                    let digit = (value as f32 / 17.0).round() as u8;
                    out.push(hex_digit(digit, uppercase));
                } else {
                    out.push(hex_digit(value >> 4, uppercase));
                    out.push(hex_digit(value & 0xf, uppercase));
                }
            }
            out
        }
        ColorNotation::Rgb { alpha, percent } => {
            let channel = |value: f32| {
                if percent {
                    format!("{}%", (value.clamp(0.0, 1.0) * 100.0).round() as i64)
                } else {
                    format!("{}", to_byte(value))
                }
            };
            if alpha {
                format!(
                    "rgba({}, {}, {}, {})",
                    channel(color.red),
                    channel(color.green),
                    channel(color.blue),
                    format_number(color.alpha, 2)
                )
            } else {
                format!(
                    "rgb({}, {}, {})",
                    channel(color.red),
                    channel(color.green),
                    channel(color.blue)
                )
            }
        }
        ColorNotation::Hsl { alpha } => {
            let hsla = Hsla::from(Rgba {
                r: color.red,
                g: color.green,
                b: color.blue,
                a: color.alpha,
            });
            let hue = (hsla.h * 360.0).round() as i64;
            let saturation = (hsla.s * 100.0).round() as i64;
            let lightness = (hsla.l * 100.0).round() as i64;
            if alpha {
                format!(
                    "hsla({hue}, {saturation}%, {lightness}%, {})",
                    format_number(color.alpha, 2)
                )
            } else {
                format!("hsl({hue}, {saturation}%, {lightness}%)")
            }
        }
    }
}

fn hex_digit(value: u8, uppercase: bool) -> char {
    let digit = char::from_digit(value as u32 & 0xf, 16).unwrap_or('0');
    if uppercase {
        digit.to_ascii_uppercase()
    } else {
        digit
    }
}

/// All colors that the language's `colors.scm` query finds within `offset_range`.
pub(crate) fn color_matches(
    buffer: &BufferSnapshot,
    offset_range: Range<usize>,
) -> impl Iterator<Item = ColorMatch> + '_ {
    let mut syntax_matches = buffer.matches(offset_range, |grammar| {
        grammar.colors_config.as_ref().map(|config| &config.query)
    });

    let configs = syntax_matches
        .grammars()
        .iter()
        .map(|grammar| grammar.colors_config.as_ref())
        .collect::<Vec<_>>();

    iter::from_fn(move || {
        let mat = syntax_matches.peek()?;
        let color_match = configs[mat.grammar_index].and_then(|config| {
            color_match_from_captures(buffer, mat.captures, config, mat.pattern_index)
        });
        syntax_matches.advance();
        Some(color_match)
    })
    .flatten()
}

fn color_match_from_captures(
    buffer: &BufferSnapshot,
    captures: &[tree_sitter::QueryCapture<'_>],
    config: &language_core::ColorsConfig,
    pattern_index: usize,
) -> Option<ColorMatch> {
    let mut whole_range = None;
    let mut literal_range = None;
    let mut component_ranges = SmallVec::<[(ColorComponent, Range<usize>); 4]>::new();

    for capture in captures {
        match config
            .captures
            .get(capture.index as usize)
            .copied()
            .flatten()
        {
            Some(ColorCapture::Whole) => whole_range = Some(capture.node.byte_range()),
            Some(ColorCapture::Text) => literal_range = Some(capture.node.byte_range()),
            Some(ColorCapture::Component(component)) => {
                component_ranges.push((component, capture.node.byte_range()))
            }
            None => {}
        }
    }
    let whole_range = whole_range?;
    let buffer_id = buffer.remote_id();

    if let Some(literal_range) = literal_range {
        let text = buffer
            .text_for_range(literal_range.clone())
            .collect::<String>();
        let (span, color, notation) = parse_color_literal(&text)?;
        return Some(ColorMatch {
            buffer_id,
            range: whole_range,
            color,
            replacement: ColorReplacement::Literal {
                range: literal_range.start + span.start..literal_range.start + span.end,
                notation,
            },
        });
    }

    if component_ranges.is_empty() {
        // A pattern that captures only `@color` describes where a swatch goes
        // but not what color it is, so there is nothing to show.
        return None;
    }

    let mut parsed = SmallVec::<[(ColorComponent, Range<usize>, f32, usize); 4]>::new();
    for (component, range) in component_ranges {
        let text = buffer.text_for_range(range.clone()).collect::<String>();
        let (value, precision) = parse_channel(&text)?;
        parsed.push((component, range, value, precision));
    }

    let declared_scales = config.scales.get(pattern_index).copied().unwrap_or_default();
    let is_hsl = parsed.iter().any(|(component, ..)| {
        matches!(
            component,
            ColorComponent::Hue | ColorComponent::Saturation | ColorComponent::Lightness
        )
    });
    let inferred_scale = infer_scale(&parsed, is_hsl);

    let mut channels = SmallVec::<[ColorChannel; 4]>::new();
    let mut unit_values = [None; ColorComponent::COUNT];
    for (component, range, value, precision) in parsed {
        let scale = declared_scales.for_component(component).unwrap_or({
            // CSS writes alpha as a unit fraction even alongside 0-255
            // channels, so a fractional alpha is unit-scaled whatever the rest
            // of the match uses.
            if component == ColorComponent::Alpha && precision > 0 {
                ColorScale::Unit
            } else {
                inferred_scale
            }
        });
        unit_values[component.index()] = Some(scale_in(value, component, scale));
        channels.push(ColorChannel {
            component,
            range,
            precision,
            scale,
        });
    }

    let alpha = unit_values[ColorComponent::Alpha.index()]
        .unwrap_or(1.0)
        .clamp(0.0, 1.0);
    let rgba = if is_hsl {
        Rgba::from(Hsla {
            h: unit_values[ColorComponent::Hue.index()]?.rem_euclid(1.0),
            s: unit_values[ColorComponent::Saturation.index()]?.clamp(0.0, 1.0),
            l: unit_values[ColorComponent::Lightness.index()]?.clamp(0.0, 1.0),
            a: alpha,
        })
    } else {
        Rgba {
            r: unit_values[ColorComponent::Red.index()]?.clamp(0.0, 1.0),
            g: unit_values[ColorComponent::Green.index()]?.clamp(0.0, 1.0),
            b: unit_values[ColorComponent::Blue.index()]?.clamp(0.0, 1.0),
            a: alpha,
        }
    };

    channels.sort_by_key(|channel| channel.range.start);
    Some(ColorMatch {
        buffer_id,
        range: whole_range,
        color: lsp::Color {
            red: rgba.r,
            green: rgba.g,
            blue: rgba.b,
            alpha: rgba.a,
        },
        replacement: ColorReplacement::Channels(channels),
    })
}

/// Guess the units of a match whose query did not declare them.
fn infer_scale(
    parsed: &[(ColorComponent, Range<usize>, f32, usize)],
    is_hsl: bool,
) -> ColorScale {
    let written_as_fraction = parsed
        .iter()
        .any(|(component, _, _, precision)| *component != ColorComponent::Alpha && *precision > 0);
    if written_as_fraction {
        return ColorScale::Unit;
    }
    if is_hsl {
        // Whole-number HSL is degrees and percent: `hsl(210, 60, 40)`.
        ColorScale::Degrees
    } else {
        ColorScale::U8
    }
}

/// Parse one numeric channel, returning its value and how many decimal places
/// it was written with. Type suffixes and digit separators (`0.2_f32`) are
/// ignored so that the same query works across languages.
fn parse_channel(text: &str) -> Option<(f32, usize)> {
    let mut numeric = String::with_capacity(text.len());
    for character in text.trim().chars() {
        match character {
            '_' => continue,
            '0'..='9' | '.' | '-' | '+' => numeric.push(character),
            _ => break,
        }
    }
    let value = numeric.parse::<f32>().ok()?;
    let precision = numeric
        .split_once('.')
        .map(|(_, decimals)| decimals.len())
        .unwrap_or(0);
    Some((value, precision))
}

/// Locate the first color literal inside `text` and parse it.
///
/// Returns the byte span of the literal within `text`, so that a node which
/// wraps the color in other syntax (a quoted string, a CSS declaration) still
/// yields an edit range covering only the color itself.
pub fn parse_color_literal(text: &str) -> Option<(Range<usize>, lsp::Color, ColorNotation)> {
    let bytes = text.as_bytes();
    for start in 0..bytes.len() {
        if !text.is_char_boundary(start) {
            continue;
        }
        let rest = &text[start..];
        let parsed = if rest.starts_with('#') {
            parse_hex(rest)
        } else if starts_with_ignore_case(rest, "rgb") || starts_with_ignore_case(rest, "hsl") {
            parse_color_function(rest)
        } else if rest
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_alphabetic())
        {
            // Only try a named color at the start of an identifier, so that the
            // `red` inside `border-red` is not mistaken for a color.
            let preceded_by_identifier = start > 0
                && bytes
                    .get(start - 1)
                    .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'-' || *byte == b'_');
            if preceded_by_identifier {
                None
            } else {
                parse_named(rest)
            }
        } else {
            None
        };

        if let Some((length, color, notation)) = parsed {
            return Some((start..start + length, color, notation));
        }
    }
    None
}

fn starts_with_ignore_case(text: &str, prefix: &str) -> bool {
    text.len() >= prefix.len() && text[..prefix.len()].eq_ignore_ascii_case(prefix)
}

fn parse_hex(text: &str) -> Option<(usize, lsp::Color, ColorNotation)> {
    let digits = text[1..]
        .bytes()
        .take_while(u8::is_ascii_hexdigit)
        .count();
    // Reject a run that is longer than any valid form: `#1234567` is not a
    // color, and truncating it to `#123456` would show a swatch for something
    // the author did not write.
    let channels = match digits {
        3 | 6 => 3,
        4 | 8 => 4,
        _ => return None,
    };
    let digits_per_channel = digits / channels;
    let hex = &text[1..1 + digits];
    let uppercase = hex.bytes().any(|byte| byte.is_ascii_uppercase());

    let mut values = [0u8; 4];
    for (index, value) in values.iter_mut().enumerate().take(channels) {
        let start = index * digits_per_channel;
        let chunk = hex.get(start..start + digits_per_channel)?;
        let parsed = u8::from_str_radix(chunk, 16).ok()?;
        *value = if digits_per_channel == 1 {
            parsed * 17
        } else {
            parsed
        };
    }

    let color = lsp::Color {
        red: values[0] as f32 / 255.0,
        green: values[1] as f32 / 255.0,
        blue: values[2] as f32 / 255.0,
        alpha: if channels == 4 {
            values[3] as f32 / 255.0
        } else {
            1.0
        },
    };
    Some((
        1 + digits,
        color,
        ColorNotation::Hex {
            digits_per_channel,
            channels,
            uppercase,
        },
    ))
}

fn parse_color_function(text: &str) -> Option<(usize, lsp::Color, ColorNotation)> {
    let open = text.find('(')?;
    let name = text[..open].trim();
    let is_hsl = name.eq_ignore_ascii_case("hsl") || name.eq_ignore_ascii_case("hsla");
    if !is_hsl && !name.eq_ignore_ascii_case("rgb") && !name.eq_ignore_ascii_case("rgba") {
        return None;
    }
    let close = text[open..].find(')')? + open;
    let arguments = &text[open + 1..close];

    let mut percent = false;
    let mut values = Vec::with_capacity(4);
    for argument in arguments
        .split([',', '/', ' ', '\t', '\n'])
        .filter(|argument| !argument.trim().is_empty())
    {
        let argument = argument.trim();
        let (number, is_percent) = match argument.strip_suffix('%') {
            Some(number) => (number, true),
            None => (argument, false),
        };
        percent |= is_percent;
        values.push((number.parse::<f32>().ok()?, is_percent));
    }
    if values.len() < 3 {
        return None;
    }

    let alpha = values
        .get(3)
        .map(|(value, is_percent)| {
            if *is_percent {
                value / 100.0
            } else {
                *value
            }
        })
        .unwrap_or(1.0)
        .clamp(0.0, 1.0);
    let has_alpha = values.len() > 3;

    let color = if is_hsl {
        Hsla {
            h: (values[0].0 / 360.0).rem_euclid(1.0),
            s: (values[1].0 / 100.0).clamp(0.0, 1.0),
            l: (values[2].0 / 100.0).clamp(0.0, 1.0),
            a: alpha,
        }
        .into()
    } else {
        let channel = |(value, is_percent): (f32, bool)| {
            if is_percent {
                value / 100.0
            } else {
                value / 255.0
            }
            .clamp(0.0, 1.0)
        };
        Rgba {
            r: channel(values[0]),
            g: channel(values[1]),
            b: channel(values[2]),
            a: alpha,
        }
    };
    let color = lsp::Color {
        red: color.r,
        green: color.g,
        blue: color.b,
        alpha: color.a,
    };

    let notation = if is_hsl {
        ColorNotation::Hsl { alpha: has_alpha }
    } else {
        ColorNotation::Rgb {
            alpha: has_alpha,
            percent,
        }
    };
    Some((close + 1, color, notation))
}

fn parse_named(text: &str) -> Option<(usize, lsp::Color, ColorNotation)> {
    let length = text
        .bytes()
        .take_while(u8::is_ascii_alphabetic)
        .count();
    let name = text.get(..length)?;
    let index = NAMED_COLORS
        .binary_search_by(|(candidate, _)| compare_ignore_case(candidate, name))
        .ok()?;
    let (_, rgb) = NAMED_COLORS[index];
    let color = lsp::Color {
        red: ((rgb >> 16) & 0xff) as f32 / 255.0,
        green: ((rgb >> 8) & 0xff) as f32 / 255.0,
        blue: (rgb & 0xff) as f32 / 255.0,
        alpha: 1.0,
    };
    Some((length, color, ColorNotation::Named))
}

fn compare_ignore_case(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

/// The CSS named colors, sorted by name so that lookups can binary search.
static NAMED_COLORS: &[(&str, u32)] = &[
    ("aliceblue", 0xF0F8FF),
    ("antiquewhite", 0xFAEBD7),
    ("aqua", 0x00FFFF),
    ("aquamarine", 0x7FFFD4),
    ("azure", 0xF0FFFF),
    ("beige", 0xF5F5DC),
    ("bisque", 0xFFE4C4),
    ("black", 0x000000),
    ("blanchedalmond", 0xFFEBCD),
    ("blue", 0x0000FF),
    ("blueviolet", 0x8A2BE2),
    ("brown", 0xA52A2A),
    ("burlywood", 0xDEB887),
    ("cadetblue", 0x5F9EA0),
    ("chartreuse", 0x7FFF00),
    ("chocolate", 0xD2691E),
    ("coral", 0xFF7F50),
    ("cornflowerblue", 0x6495ED),
    ("cornsilk", 0xFFF8DC),
    ("crimson", 0xDC143C),
    ("cyan", 0x00FFFF),
    ("darkblue", 0x00008B),
    ("darkcyan", 0x008B8B),
    ("darkgoldenrod", 0xB8860B),
    ("darkgray", 0xA9A9A9),
    ("darkgreen", 0x006400),
    ("darkgrey", 0xA9A9A9),
    ("darkkhaki", 0xBDB76B),
    ("darkmagenta", 0x8B008B),
    ("darkolivegreen", 0x556B2F),
    ("darkorange", 0xFF8C00),
    ("darkorchid", 0x9932CC),
    ("darkred", 0x8B0000),
    ("darksalmon", 0xE9967A),
    ("darkseagreen", 0x8FBC8F),
    ("darkslateblue", 0x483D8B),
    ("darkslategray", 0x2F4F4F),
    ("darkslategrey", 0x2F4F4F),
    ("darkturquoise", 0x00CED1),
    ("darkviolet", 0x9400D3),
    ("deeppink", 0xFF1493),
    ("deepskyblue", 0x00BFFF),
    ("dimgray", 0x696969),
    ("dimgrey", 0x696969),
    ("dodgerblue", 0x1E90FF),
    ("firebrick", 0xB22222),
    ("floralwhite", 0xFFFAF0),
    ("forestgreen", 0x228B22),
    ("fuchsia", 0xFF00FF),
    ("gainsboro", 0xDCDCDC),
    ("ghostwhite", 0xF8F8FF),
    ("gold", 0xFFD700),
    ("goldenrod", 0xDAA520),
    ("gray", 0x808080),
    ("green", 0x008000),
    ("greenyellow", 0xADFF2F),
    ("grey", 0x808080),
    ("honeydew", 0xF0FFF0),
    ("hotpink", 0xFF69B4),
    ("indianred", 0xCD5C5C),
    ("indigo", 0x4B0082),
    ("ivory", 0xFFFFF0),
    ("khaki", 0xF0E68C),
    ("lavender", 0xE6E6FA),
    ("lavenderblush", 0xFFF0F5),
    ("lawngreen", 0x7CFC00),
    ("lemonchiffon", 0xFFFACD),
    ("lightblue", 0xADD8E6),
    ("lightcoral", 0xF08080),
    ("lightcyan", 0xE0FFFF),
    ("lightgoldenrodyellow", 0xFAFAD2),
    ("lightgray", 0xD3D3D3),
    ("lightgreen", 0x90EE90),
    ("lightgrey", 0xD3D3D3),
    ("lightpink", 0xFFB6C1),
    ("lightsalmon", 0xFFA07A),
    ("lightseagreen", 0x20B2AA),
    ("lightskyblue", 0x87CEFA),
    ("lightslategray", 0x778899),
    ("lightslategrey", 0x778899),
    ("lightsteelblue", 0xB0C4DE),
    ("lightyellow", 0xFFFFE0),
    ("lime", 0x00FF00),
    ("limegreen", 0x32CD32),
    ("linen", 0xFAF0E6),
    ("magenta", 0xFF00FF),
    ("maroon", 0x800000),
    ("mediumaquamarine", 0x66CDAA),
    ("mediumblue", 0x0000CD),
    ("mediumorchid", 0xBA55D3),
    ("mediumpurple", 0x9370DB),
    ("mediumseagreen", 0x3CB371),
    ("mediumslateblue", 0x7B68EE),
    ("mediumspringgreen", 0x00FA9A),
    ("mediumturquoise", 0x48D1CC),
    ("mediumvioletred", 0xC71585),
    ("midnightblue", 0x191970),
    ("mintcream", 0xF5FFFA),
    ("mistyrose", 0xFFE4E1),
    ("moccasin", 0xFFE4B5),
    ("navajowhite", 0xFFDEAD),
    ("navy", 0x000080),
    ("oldlace", 0xFDF5E6),
    ("olive", 0x808000),
    ("olivedrab", 0x6B8E23),
    ("orange", 0xFFA500),
    ("orangered", 0xFF4500),
    ("orchid", 0xDA70D6),
    ("palegoldenrod", 0xEEE8AA),
    ("palegreen", 0x98FB98),
    ("paleturquoise", 0xAFEEEE),
    ("palevioletred", 0xDB7093),
    ("papayawhip", 0xFFEFD5),
    ("peachpuff", 0xFFDAB9),
    ("peru", 0xCD853F),
    ("pink", 0xFFC0CB),
    ("plum", 0xDDA0DD),
    ("powderblue", 0xB0E0E6),
    ("purple", 0x800080),
    ("rebeccapurple", 0x663399),
    ("red", 0xFF0000),
    ("rosybrown", 0xBC8F8F),
    ("royalblue", 0x4169E1),
    ("saddlebrown", 0x8B4513),
    ("salmon", 0xFA8072),
    ("sandybrown", 0xF4A460),
    ("seagreen", 0x2E8B57),
    ("seashell", 0xFFF5EE),
    ("sienna", 0xA0522D),
    ("silver", 0xC0C0C0),
    ("skyblue", 0x87CEEB),
    ("slateblue", 0x6A5ACD),
    ("slategray", 0x708090),
    ("slategrey", 0x708090),
    ("snow", 0xFFFAFA),
    ("springgreen", 0x00FF7F),
    ("steelblue", 0x4682B4),
    ("tan", 0xD2B48C),
    ("teal", 0x008080),
    ("thistle", 0xD8BFD8),
    ("tomato", 0xFF6347),
    ("turquoise", 0x40E0D0),
    ("violet", 0xEE82EE),
    ("wheat", 0xF5DEB3),
    ("white", 0xFFFFFF),
    ("whitesmoke", 0xF5F5F5),
    ("yellow", 0xFFFF00),
    ("yellowgreen", 0x9ACD32),];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Buffer, json_lang, rust_lang};
    use gpui::{AppContext as _, TestAppContext};

    #[track_caller]
    fn colors_in(source: &str, cx: &mut TestAppContext) -> Vec<ColorMatch> {
        colors_in_language(source, rust_lang(), cx)
    }

    #[track_caller]
    fn colors_in_language(
        source: &str,
        language: std::sync::Arc<crate::Language>,
        cx: &mut TestAppContext,
    ) -> Vec<ColorMatch> {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(source, cx);
            buffer.set_language(Some(language), cx);
            buffer
        });
        cx.executor().run_until_parked();
        buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot.color_matches(0..snapshot.len()).collect()
        })
    }

    #[track_caller]
    fn assert_color(actual: lsp::Color, expected: [f32; 4]) {
        let actual = [actual.red, actual.green, actual.blue, actual.alpha];
        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert!(
                (actual - expected).abs() < 0.005,
                "expected {expected:?}, got {actual:?}"
            );
        }
    }

    #[gpui::test]
    fn test_float_channel_constructor(cx: &mut TestAppContext) {
        let source = "fn draw() { gizmos.line(a, b, Color::srgb(0.2, 0.9, 0.4)); }";
        let colors = colors_in(source, cx);
        assert_eq!(colors.len(), 1, "{colors:?}");
        assert_color(colors[0].color, [0.2, 0.9, 0.4, 1.0]);
        assert_eq!(&source[colors[0].range.clone()], "Color::srgb(0.2, 0.9, 0.4)");
        assert!(!colors[0].supports_alpha());

        let edits = colors[0].edits(lsp::Color {
            red: 1.0,
            green: 0.5,
            blue: 0.0,
            alpha: 1.0,
        });
        let edited = apply(source, &edits);
        assert!(
            edited.contains("Color::srgb(1.0, 0.5, 0.0)"),
            "unexpected edit result: {edited}"
        );
    }

    #[gpui::test]
    fn test_alpha_constructor_is_preferred_over_the_three_channel_pattern(cx: &mut TestAppContext) {
        let source = "fn f() { let c = Color::srgba(0.2, 0.9, 0.4, 0.25); }";
        let colors = colors_in(source, cx);
        assert_eq!(colors.len(), 1, "{colors:?}");
        assert_color(colors[0].color, [0.2, 0.9, 0.4, 0.25]);
        assert!(colors[0].supports_alpha());
    }

    #[gpui::test]
    fn test_u8_channels_are_inferred(cx: &mut TestAppContext) {
        let source = "fn f() { let c = Color::srgb_u8(51, 230, 102); }";
        let colors = colors_in(source, cx);
        assert_eq!(colors.len(), 1, "{colors:?}");
        assert_color(colors[0].color, [0.2, 0.902, 0.4, 1.0]);

        let edits = colors[0].edits(lsp::Color {
            red: 1.0,
            green: 0.0,
            blue: 0.5,
            alpha: 1.0,
        });
        assert!(apply(source, &edits).contains("Color::srgb_u8(255, 0, 128)"));
    }

    #[gpui::test]
    fn test_hue_is_scaled_separately_from_saturation(cx: &mut TestAppContext) {
        let source = "fn f() { let c = Color::hsl(120.0, 1.0, 0.5); }";
        let colors = colors_in(source, cx);
        assert_eq!(colors.len(), 1, "{colors:?}");
        assert_color(colors[0].color, [0.0, 1.0, 0.0, 1.0]);
    }

    #[gpui::test]
    fn test_hsl_constructors_match_exactly_once(cx: &mut TestAppContext) {
        for source in [
            "fn f() { let c = Color::hsla(120.0, 1.0, 0.5, 0.5); }",
            "fn f() { let c = Hsla::new(120.0, 1.0, 0.5, 0.5); }",
        ] {
            let colors = colors_in(source, cx);
            assert_eq!(colors.len(), 1, "{source}: {colors:?}");
            assert_color(colors[0].color, [0.0, 1.0, 0.0, 0.5]);
        }
    }

    #[gpui::test]
    fn test_hex_string_literal(cx: &mut TestAppContext) {
        let source = "const ACCENT: &str = \"#FF00AA\";";
        let colors = colors_in(source, cx);
        assert_eq!(colors.len(), 1, "{colors:?}");
        assert_color(colors[0].color, [1.0, 0.0, 0.667, 1.0]);
        assert_eq!(&source[colors[0].range.clone()], "\"#FF00AA\"");

        let edits = colors[0].edits(lsp::Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        });
        // The quotes are outside the replaced span, and the original casing is kept.
        assert_eq!(apply(source, &edits), "const ACCENT: &str = \"#0000FF\";");
    }

    #[gpui::test]
    fn test_unrelated_calls_are_not_colors(cx: &mut TestAppContext) {
        let source = "fn f() { let v = Vec3::new(0.2, 0.9, 0.4); Foo::srgb(1, 2, 3); }";
        let colors = colors_in(source, cx);
        assert_eq!(colors.len(), 0, "{colors:?}");
    }

    #[gpui::test]
    fn test_hex_colors_in_json(cx: &mut TestAppContext) {
        let source = "{ \"accent\": \"#0af\", \"name\": \"not a color\" }";
        let colors = colors_in_language(source, json_lang(), cx);
        assert_eq!(colors.len(), 1, "{colors:?}");
        assert_color(colors[0].color, [0.0, 0.667, 1.0, 1.0]);
        assert_eq!(&source[colors[0].range.clone()], "\"#0af\"");

        // A three-digit hex stays three digits.
        let edits = colors[0].edits(lsp::Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        });
        assert!(apply(source, &edits).contains("\"#fff\""));
    }

    fn apply(source: &str, edits: &[(Range<usize>, String)]) -> String {
        let mut edited = source.to_string();
        for (range, replacement) in edits.iter().rev() {
            edited.replace_range(range.clone(), replacement);
        }
        edited
    }

    #[test]
    fn test_parse_color_literal() {
        let (range, color, notation) = parse_color_literal("\"#abc\"").unwrap();
        assert_eq!(range, 1..5);
        assert_color(color, [0.667, 0.733, 0.8, 1.0]);
        assert_eq!(
            notation,
            ColorNotation::Hex {
                digits_per_channel: 1,
                channels: 3,
                uppercase: false
            }
        );

        let (_, color, notation) = parse_color_literal("rgba(255, 0, 128, 0.5)").unwrap();
        assert_color(color, [1.0, 0.0, 0.502, 0.5]);
        assert_eq!(
            notation,
            ColorNotation::Rgb {
                alpha: true,
                percent: false
            }
        );

        let (_, color, _) = parse_color_literal("hsl(120, 100%, 50%)").unwrap();
        assert_color(color, [0.0, 1.0, 0.0, 1.0]);

        let (range, color, notation) = parse_color_literal("color: rebeccapurple;").unwrap();
        assert_eq!(range, 7..20);
        assert_color(color, [0.4, 0.2, 0.6, 1.0]);
        assert_eq!(notation, ColorNotation::Named);

        // Too many digits to be a color, and an identifier that merely ends in
        // a color name, are both rejected.
        assert!(parse_color_literal("#1234567").is_none());
        assert!(parse_color_literal("border-red").is_none());
    }

    #[test]
    fn test_named_colors_are_sorted_for_binary_search() {
        assert!(
            NAMED_COLORS.windows(2).all(|pair| pair[0].0 < pair[1].0),
            "NAMED_COLORS must stay sorted by name"
        );
    }
}
