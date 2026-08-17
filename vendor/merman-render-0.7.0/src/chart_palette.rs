const CHART_ACCENT_FALLBACK: &str = "#3b82f6";

#[derive(Debug, Clone)]
pub(crate) struct XyChartPaletteConfig {
    pub(crate) theme_name: String,
    pub(crate) plot_color_palette: Option<String>,
    pub(crate) accent_color: Option<String>,
    pub(crate) background_color: Option<String>,
}

fn default_plot_color_palette() -> Vec<String> {
    "#ECECFF,#8493A6,#FFC3A0,#DCDDE1,#B8E994,#D1A36F,#C3CDE6,#FFB6C1,#496078,#F8F3E3"
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn parse_palette_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty())
        .collect()
}

fn is_valid_hex(color: &str) -> bool {
    let bytes = color.as_bytes();
    bytes.len() == 7
        && bytes[0] == b'#'
        && bytes[1..]
            .iter()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'))
}

fn is_dark_background(bg_hex: &str) -> bool {
    hex_to_hsl(bg_hex).is_some_and(|(_, _, l)| l < 50.0)
}

fn hex_to_hsl(hex: &str) -> Option<(f64, f64, f64)> {
    let h = hex.strip_prefix('#')?;
    let (r, g, b) = match h.len() {
        6 => (
            u8::from_str_radix(&h[0..2], 16).ok()? as f64 / 255.0,
            u8::from_str_radix(&h[2..4], 16).ok()? as f64 / 255.0,
            u8::from_str_radix(&h[4..6], 16).ok()? as f64 / 255.0,
        ),
        _ => return None,
    };

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() <= f64::EPSILON {
        return Some((0.0, 0.0, l * 100.0));
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let mut hue = if (max - r).abs() <= f64::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() <= f64::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    hue /= 6.0;

    Some((hue * 360.0, s * 100.0, l * 100.0))
}

fn hsl_to_hex(h: f64, s: f64, l: f64) -> String {
    let h = ((h % 360.0) + 360.0) % 360.0;
    let s = (s / 100.0).clamp(0.0, 1.0);
    let l = (l / 100.0).clamp(0.0, 1.0);

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h as i64 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let to_hex = |v: f64| -> String {
        let byte = ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        format!("{:02x}", byte)
    };

    format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}

fn series_color(index: usize, accent_color: &str, bg_color: Option<&str>) -> String {
    if index == 0 {
        return accent_color.to_string();
    }

    let safe_accent = if is_valid_hex(accent_color) {
        accent_color
    } else {
        CHART_ACCENT_FALLBACK
    };
    let safe_bg = bg_color.filter(|c| is_valid_hex(c));
    let Some((h, s, _)) = hex_to_hsl(safe_accent) else {
        return CHART_ACCENT_FALLBACK.to_string();
    };
    let chart_s = s.clamp(55.0, 85.0);
    let tier = index.div_ceil(2) as f64;
    let odd_index = index % 2 == 1;
    let dark = safe_bg.is_some_and(is_dark_background);
    let dark = if dark { !odd_index } else { odd_index };

    let l = if dark {
        (48.0 - tier * 13.0).max(25.0)
    } else {
        (55.0 + tier * 11.0).min(78.0)
    };
    let h_shift = if dark { -8.0 } else { 12.0 } * tier;

    hsl_to_hex(h + h_shift, chart_s, l)
}

pub(crate) fn resolve_xychart_plot_palette(config: XyChartPaletteConfig) -> Vec<String> {
    if let Some(raw) = config.plot_color_palette {
        let palette = parse_palette_list(&raw);
        if !palette.is_empty() {
            return palette;
        }
    }

    if config.theme_name == "default" {
        return default_plot_color_palette();
    }

    let accent = config
        .accent_color
        .unwrap_or_else(|| CHART_ACCENT_FALLBACK.to_string());
    let background = config.background_color;

    (0..10)
        .map(|i| series_color(i, accent.as_str(), background.as_deref()))
        .collect()
}

pub(crate) fn plot_color_from_palette(palette: &[String], plot_index: usize) -> String {
    if palette.is_empty() {
        return String::new();
    }
    let idx = if plot_index == 0 {
        0
    } else {
        plot_index % palette.len()
    };
    palette[idx].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_keeps_the_legacy_palette_shape() {
        let palette = resolve_xychart_plot_palette(XyChartPaletteConfig {
            theme_name: "default".to_string(),
            plot_color_palette: None,
            accent_color: None,
            background_color: None,
        });

        assert_eq!(palette.len(), 10);
        assert_eq!(palette[0], "#ECECFF");
        assert_eq!(palette[1], "#8493A6");
    }

    #[test]
    fn non_default_theme_derives_from_primary_color() {
        let palette = resolve_xychart_plot_palette(XyChartPaletteConfig {
            theme_name: "neo".to_string(),
            plot_color_palette: None,
            accent_color: Some("#123456".to_string()),
            background_color: Some("#ffffff".to_string()),
        });

        assert_eq!(palette.len(), 10);
        assert_eq!(palette[0], "#123456");
        assert_ne!(palette[1], "#8493A6");
    }

    #[test]
    fn explicit_plot_palette_wins_over_derivation() {
        let palette = resolve_xychart_plot_palette(XyChartPaletteConfig {
            theme_name: "neo".to_string(),
            plot_color_palette: Some("#001122, #334455".to_string()),
            accent_color: Some("#123456".to_string()),
            background_color: None,
        });

        assert_eq!(palette, vec!["#001122".to_string(), "#334455".to_string()]);
    }
}
