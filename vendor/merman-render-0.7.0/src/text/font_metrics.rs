//! Vendored browser/font metrics text measurer.

use super::{
    DeterministicTextMeasurer, FLOWCHART_DEFAULT_FONT_KEY, TextMeasurer, TextMetrics, TextStyle,
    WrapMode, flowchart_default_bold_delta_em, flowchart_default_bold_kern_delta_em,
    flowchart_default_bold_svg_right_overhang_em, font_key_uses_courier_metrics,
    is_flowchart_default_font, overrides, round_to_1_64_px, style_requests_bold_font_weight,
    svg_wrapped_first_line_bbox_height_px,
};

#[derive(Debug, Clone, Default)]
pub struct VendoredFontMetricsTextMeasurer {
    fallback: DeterministicTextMeasurer,
}

#[derive(Clone, Copy)]
struct FontMetricProfile<'a> {
    entries: &'a [(char, f64)],
    default_em: f64,
    kern_pairs: &'a [(u32, u32, f64)],
    space_trigrams: &'a [(u32, u32, f64)],
    trigrams: &'a [(u32, u32, u32, f64)],
    missing_v_comma_kern_em: f64,
    missing_t_o_kern_em: f64,
    missing_t_r_kern_em: f64,
    missing_space_before_capital_a_em: f64,
    missing_space_after_capital_a_before_open_paren_em: f64,
}

impl VendoredFontMetricsTextMeasurer {
    fn metric_profile(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
    ) -> FontMetricProfile<'_> {
        FontMetricProfile {
            entries: table.entries,
            default_em: table.default_em.max(0.1),
            kern_pairs: table.kern_pairs,
            space_trigrams: table.space_trigrams,
            trigrams: table.trigrams,
            missing_v_comma_kern_em: if table.font_key == FLOWCHART_DEFAULT_FONT_KEY {
                -140.0 / 1024.0
            } else {
                0.0
            },
            missing_t_o_kern_em: if table.font_key == FLOWCHART_DEFAULT_FONT_KEY {
                -128.0 / 1024.0
            } else {
                0.0
            },
            missing_t_r_kern_em: if table.font_key == FLOWCHART_DEFAULT_FONT_KEY {
                -113.0 / 1024.0
            } else {
                0.0
            },
            missing_space_before_capital_a_em: if table.font_key
                == "trebuchetms,verdana,arial,sans-serif"
            {
                -57.0 / 1024.0
            } else {
                0.0
            },
            missing_space_after_capital_a_before_open_paren_em: if table.font_key
                == "trebuchetms,verdana,arial,sans-serif"
            {
                -57.0 / 1024.0
            } else {
                0.0
            },
        }
    }

    pub(super) fn quantize_svg_bbox_px_nearest(v: f64) -> f64 {
        if !(v.is_finite() && v >= 0.0) {
            return 0.0;
        }
        // Title/label `getBBox()` extents in upstream fixtures frequently land on 1/1024px
        // increments. Quantize after applying svg-overrides so (em * font_size) does not leak FP
        // noise into viewBox/max-width comparisons.
        let x = v * 1024.0;
        let f = x.floor();
        let frac = x - f;
        let i = if frac < 0.5 {
            f
        } else if frac > 0.5 {
            f + 1.0
        } else {
            let fi = f as i64;
            if fi % 2 == 0 { f } else { f + 1.0 }
        };
        i / 1024.0
    }

    fn quantize_svg_half_px_nearest(half_px: f64) -> f64 {
        if !(half_px.is_finite() && half_px >= 0.0) {
            return 0.0;
        }
        // SVG `getBBox()` metrics in upstream Mermaid baselines tend to behave like a truncation
        // on a power-of-two grid for the anchored half-advance. Using `floor` here avoids a
        // systematic +1/256px drift in wide titles that can bubble up into `viewBox`/`max-width`.
        (half_px * 256.0).floor() / 256.0
    }

    fn normalize_font_key(s: &str) -> String {
        s.chars()
            .filter_map(|ch| {
                // Mermaid config strings occasionally embed the trailing CSS `;` in `fontFamily`.
                // We treat it as syntactic noise so lookups work with both `...sans-serif` and
                // `...sans-serif;`.
                if ch.is_whitespace() || ch == '"' || ch == '\'' || ch == ';' {
                    None
                } else {
                    Some(ch.to_ascii_lowercase())
                }
            })
            .collect()
    }

    fn lookup_table(
        &self,
        style: &TextStyle,
    ) -> Option<&'static crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables> {
        let key = style
            .font_family
            .as_deref()
            .map(Self::normalize_font_key)
            .unwrap_or_default();
        let key = if key.is_empty() {
            // Mermaid defaults to `"trebuchet ms", verdana, arial, sans-serif`. Many headless
            // layout call sites omit `font_family` and rely on that implicit default.
            FLOWCHART_DEFAULT_FONT_KEY
        } else {
            key.as_str()
        };
        if let Some(t) = crate::generated::font_metrics_flowchart_11_12_2::lookup_font_metrics(key)
        {
            return Some(t);
        }

        // Best-effort aliases for common stacks in upstream fixtures (Mermaid measures via DOM,
        // while our vendored tables cover a small set of representative families).
        let key_lower = key;
        if font_key_uses_courier_metrics(key_lower) {
            return crate::generated::font_metrics_flowchart_11_12_2::lookup_font_metrics(
                "courier",
            );
        }
        // Prefer explicit generic stacks. If the font family does not match a known table and
        // does not include an explicit fallback token like `sans-serif`, fall back to the
        // deterministic measurer (unknown fonts vary widely across environments).
        if key_lower.contains("sans-serif") {
            return crate::generated::font_metrics_flowchart_11_12_2::lookup_font_metrics(
                "sans-serif",
            );
        }
        None
    }

    fn lookup_char_em(entries: &[(char, f64)], default_em: f64, ch: char) -> f64 {
        fn find_entry_em(entries: &[(char, f64)], ch: char) -> Option<f64> {
            let mut lo = 0usize;
            let mut hi = entries.len();
            while lo < hi {
                let mid = (lo + hi) / 2;
                match entries[mid].0.cmp(&ch) {
                    std::cmp::Ordering::Equal => return Some(entries[mid].1),
                    std::cmp::Ordering::Less => lo = mid + 1,
                    std::cmp::Ordering::Greater => hi = mid,
                }
            }
            None
        }

        if let Some(em) = find_entry_em(entries, ch) {
            return em;
        }

        // Browser-measured metric tables are generated from observed fixture text, so a table can
        // contain one side of a mirrored ASCII punctuation pair but not the other. Use the measured
        // counterpart before falling back to the broad average; this keeps ordinary punctuation
        // labels deterministic without adding fixture-specific width lookups.
        let paired = match ch {
            '(' => Some(')'),
            ')' => Some('('),
            '[' => Some(']'),
            ']' => Some('['),
            '{' => Some('}'),
            '}' => Some('{'),
            _ => None,
        };
        if let Some(other) = paired {
            if let Some(other_em) = find_entry_em(entries, other) {
                return other_em;
            }
        }
        if ch.is_ascii() {
            return default_em;
        }

        if ('\u{80}'..='\u{9f}').contains(&ch) {
            // Mermaid/Chromium preserves C1 control bytes that appear in mojibake labels from
            // upstream fixtures and measures the rendered replacement glyph. Chromium 11.15
            // reports these glyphs closer to a narrow fallback than a full CJK cell for Flowchart
            // HTML labels, so keep the fallback near 0.6em.
            return 0.598_7;
        }

        Self::lookup_non_ascii_fallback_em(default_em, ch)
    }

    fn lookup_non_ascii_fallback_em(default_em: f64, ch: char) -> f64 {
        let code = ch as u32;

        // Mermaid's default font stack is `"trebuchet ms", verdana, arial, sans-serif`.
        // In browser rendering, non-Latin glyphs frequently fall back to script-specific fonts
        // rather than inheriting Trebuchet's Latin average. Keep the model at Unicode block
        // granularity: this mirrors browser fallback classes without adding per-fixture strings
        // or glyph lookup tables.
        if unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) == 0
            || (0x1f3fb..=0x1f3ff).contains(&code)
        {
            return 0.0;
        }
        if (0x0590..=0x05ff).contains(&code) {
            return 0.479_980_468_75;
        }
        if (0x1f300..=0x1faff).contains(&code) || (0x2600..=0x27bf).contains(&code) {
            return 1.249_67;
        }
        if (0xac00..=0xd7af).contains(&code) {
            return 0.864_257_812_5;
        }

        match unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) {
            2.. => 1.0,
            _ => default_em,
        }
    }

    fn lookup_kern_em(kern_pairs: &[(u32, u32, f64)], a: char, b: char) -> f64 {
        let key_a = a as u32;
        let key_b = b as u32;
        let mut lo = 0usize;
        let mut hi = kern_pairs.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (ma, mb, v) = kern_pairs[mid];
            match (ma.cmp(&key_a), mb.cmp(&key_b)) {
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => return v,
                (std::cmp::Ordering::Less, _) => lo = mid + 1,
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => lo = mid + 1,
                _ => hi = mid,
            }
        }
        0.0
    }

    fn lookup_profile_kern_em(profile: FontMetricProfile<'_>, a: char, b: char) -> f64 {
        let explicit = Self::lookup_kern_em(profile.kern_pairs, a, b);
        if explicit != 0.0 {
            return explicit;
        }

        if a == 'v' && b == ',' {
            // The generated default-font table captures strong comma kerning for nearby lowercase
            // terminal shapes such as `r,` and `y,`, but fixture coverage does not always observe
            // `v,`. Keep this as a narrow missing-pair fallback instead of adding literal label
            // overrides for JSON-like prose.
            return profile.missing_v_comma_kern_em;
        }
        if a == 'T' && b == 'o' {
            return profile.missing_t_o_kern_em;
        }
        if a == 'T' && b == 'r' {
            return profile.missing_t_r_kern_em;
        }

        0.0
    }

    fn lookup_space_trigram_em(space_trigrams: &[(u32, u32, f64)], a: char, b: char) -> f64 {
        let key_a = a as u32;
        let key_b = b as u32;
        let mut lo = 0usize;
        let mut hi = space_trigrams.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (ma, mb, v) = space_trigrams[mid];
            match (ma.cmp(&key_a), mb.cmp(&key_b)) {
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => return v,
                (std::cmp::Ordering::Less, _) => lo = mid + 1,
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => lo = mid + 1,
                _ => hi = mid,
            }
        }
        0.0
    }

    fn lookup_trigram_em(trigrams: &[(u32, u32, u32, f64)], a: char, b: char, c: char) -> f64 {
        let key_a = a as u32;
        let key_b = b as u32;
        let key_c = c as u32;
        let mut lo = 0usize;
        let mut hi = trigrams.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (ma, mb, mc, v) = trigrams[mid];
            match (ma.cmp(&key_a), mb.cmp(&key_b), mc.cmp(&key_c)) {
                (
                    std::cmp::Ordering::Equal,
                    std::cmp::Ordering::Equal,
                    std::cmp::Ordering::Equal,
                ) => return v,
                (std::cmp::Ordering::Less, _, _) => lo = mid + 1,
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less, _) => lo = mid + 1,
                (
                    std::cmp::Ordering::Equal,
                    std::cmp::Ordering::Equal,
                    std::cmp::Ordering::Less,
                ) => lo = mid + 1,
                _ => hi = mid,
            }
        }
        0.0
    }

    fn is_tiny_lattice_residual_em(v: f64) -> bool {
        // At Mermaid's 16px default font size, Chromium's 1/64px DOM lattice is 1/1024em.
        // Generated two-character samples can capture that quantization as a tiny "kerning"
        // residual. For same-glyph runs, browser layout accumulates it per glyph pair cell
        // (`ss`, `ssss`, ...), not per overlapping pair (`ss`, `sss`, ...).
        v.abs() <= (1.0 / 1024.0) + 1e-12
    }

    fn same_glyph_pair_kern_em(
        profile: FontMetricProfile<'_>,
        a: char,
        b: char,
        same_run_len_after: usize,
    ) -> f64 {
        let kern = Self::lookup_profile_kern_em(profile, a, b);
        if a == b && Self::is_tiny_lattice_residual_em(kern) && same_run_len_after % 2 == 1 {
            0.0
        } else {
            kern
        }
    }

    fn same_glyph_trigram_em(profile: FontMetricProfile<'_>, a: char, b: char, c: char) -> f64 {
        let delta = Self::lookup_trigram_em(profile.trigrams, a, b, c);
        if a == b && b == c && Self::is_tiny_lattice_residual_em(delta) {
            0.0
        } else {
            delta
        }
    }

    fn lookup_html_override_em(overrides: &[(&'static str, f64)], text: &str) -> Option<f64> {
        let mut lo = 0usize;
        let mut hi = overrides.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (k, v) = overrides[mid];
            match k.cmp(text) {
                std::cmp::Ordering::Equal => return Some(v),
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }

    fn lookup_svg_override_em(
        overrides: &[(&'static str, f64, f64)],
        text: &str,
    ) -> Option<(f64, f64)> {
        let mut lo = 0usize;
        let mut hi = overrides.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (k, l, r) = overrides[mid];
            match k.cmp(text) {
                std::cmp::Ordering::Equal => return Some((l, r)),
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }

    fn lookup_overhang_em(entries: &[(char, f64)], default_em: f64, ch: char) -> f64 {
        let mut lo = 0usize;
        let mut hi = entries.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            match entries[mid].0.cmp(&ch) {
                std::cmp::Ordering::Equal => return entries[mid].1,
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        default_em
    }

    fn line_svg_bbox_extents_px(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
    ) -> (f64, f64) {
        let profile = Self::metric_profile(table);
        let t = text.trim_end();
        if t.is_empty() {
            return (0.0, 0.0);
        }

        if let Some((left_em, right_em)) = Self::lookup_svg_override_em(table.svg_overrides, t) {
            let left = Self::quantize_svg_bbox_px_nearest((left_em * font_size).max(0.0));
            let right = Self::quantize_svg_bbox_px_nearest((right_em * font_size).max(0.0));
            return (left, right);
        }

        if let Some((left, right)) =
            overrides::lookup_flowchart_svg_bbox_x_px(table.font_key, font_size, t)
        {
            return (left, right);
        }

        let first = t.chars().next().unwrap_or(' ');
        let last = t.chars().last().unwrap_or(' ');

        // Mermaid's SVG label renderer tokenizes whitespace into multiple inner `<tspan>` runs
        // (one word per run, with a leading space on subsequent runs).
        //
        // These boundaries can affect shaping/kerning vs treating the text as one run, and those
        // small differences bubble into Dagre layout and viewBox parity. Mirror the upstream
        // behavior by summing per-run advances when whitespace tokenization would occur.
        let advance_px_unscaled = {
            let words: Vec<&str> = t.split_whitespace().filter(|s| !s.is_empty()).collect();
            if words.len() >= 2 {
                let mut sum_px = 0.0f64;
                for (idx, w) in words.iter().enumerate() {
                    if idx == 0 {
                        sum_px += Self::line_width_px(profile, w, false, font_size);
                    } else {
                        let seg = format!(" {w}");
                        sum_px += Self::line_width_px(profile, &seg, false, font_size);
                    }
                }
                sum_px
            } else {
                Self::line_width_px(profile, t, false, font_size)
            }
        };

        let advance_px = advance_px_unscaled * table.svg_scale;
        let half = Self::quantize_svg_half_px_nearest((advance_px / 2.0).max(0.0));
        // In upstream Mermaid fixtures, SVG `getBBox()` overhang at the ends of ASCII labels tends
        // to behave like `0` after quantization/hinting, even for glyphs with a non-zero outline
        // overhang (e.g. `s`). To avoid systematic `viewBox`/`max-width` drift, treat ASCII
        // overhang as zero and only apply per-glyph overhang for non-ASCII.
        // Most ASCII glyph overhang tends to quantize away in upstream SVG `getBBox()` fixtures,
        // but frame labels (e.g. `[opt ...]`, `[loop ...]`) start/end with bracket-like glyphs
        // where keeping overhang improves wrapping parity.
        let left_oh_em = if first.is_ascii() && !matches!(first, '[' | '(' | '{') {
            0.0
        } else {
            Self::lookup_overhang_em(
                table.svg_bbox_overhang_left,
                table.svg_bbox_overhang_left_default_em,
                first,
            )
        };
        let right_oh_em = if last.is_ascii() && !matches!(last, ']' | ')' | '}') {
            0.0
        } else {
            Self::lookup_overhang_em(
                table.svg_bbox_overhang_right,
                table.svg_bbox_overhang_right_default_em,
                last,
            )
        };

        let left = (half + left_oh_em * font_size).max(0.0);
        let right = (half + right_oh_em * font_size).max(0.0);
        (left, right)
    }

    fn line_svg_bbox_extents_px_single_run(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
    ) -> (f64, f64) {
        let profile = Self::metric_profile(table);
        let t = text.trim_end();
        if t.is_empty() {
            return (0.0, 0.0);
        }

        if let Some((left_em, right_em)) = Self::lookup_svg_override_em(table.svg_overrides, t) {
            let left = Self::quantize_svg_bbox_px_nearest((left_em * font_size).max(0.0));
            let right = Self::quantize_svg_bbox_px_nearest((right_em * font_size).max(0.0));
            return (left, right);
        }

        let first = t.chars().next().unwrap_or(' ');
        let last = t.chars().last().unwrap_or(' ');

        // Mermaid titles (e.g. flowchartTitleText) are rendered as a single `<text>` run, without
        // whitespace-tokenized `<tspan>` segments. Measure as one run to keep viewport parity.
        let advance_px_unscaled = Self::line_width_px(profile, t, false, font_size);

        let advance_px = advance_px_unscaled * table.svg_scale;
        let half = Self::quantize_svg_half_px_nearest((advance_px / 2.0).max(0.0));

        let left_oh_em = if first.is_ascii() && !matches!(first, '[' | '(' | '{') {
            0.0
        } else {
            Self::lookup_overhang_em(
                table.svg_bbox_overhang_left,
                table.svg_bbox_overhang_left_default_em,
                first,
            )
        };
        let right_oh_em = if last.is_ascii() && !matches!(last, ']' | ')' | '}') {
            0.0
        } else {
            Self::lookup_overhang_em(
                table.svg_bbox_overhang_right,
                table.svg_bbox_overhang_right_default_em,
                last,
            )
        };

        let left = (half + left_oh_em * font_size).max(0.0);
        let right = (half + right_oh_em * font_size).max(0.0);
        (left, right)
    }

    fn line_svg_bbox_extents_px_single_run_with_ascii_overhang(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
    ) -> (f64, f64) {
        Self::line_svg_bbox_extents_px_single_run_with_ascii_overhang_and_weight(
            table, text, font_size, false,
        )
    }

    fn line_svg_bbox_extents_px_single_run_with_ascii_overhang_and_weight(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
        bold: bool,
    ) -> (f64, f64) {
        let profile = Self::metric_profile(table);
        let t = text.trim_end();
        if t.is_empty() {
            return (0.0, 0.0);
        }

        if let Some((left_em, right_em)) = Self::lookup_svg_override_em(table.svg_overrides, t) {
            let left = Self::quantize_svg_bbox_px_nearest((left_em * font_size).max(0.0));
            let right = Self::quantize_svg_bbox_px_nearest((right_em * font_size).max(0.0));
            return (left, right);
        }

        let first = t.chars().next().unwrap_or(' ');
        let last = t.chars().last().unwrap_or(' ');

        let advance_px_unscaled = Self::line_width_px(profile, t, bold, font_size);

        let advance_px = advance_px_unscaled * table.svg_scale;
        let half = Self::quantize_svg_half_px_nearest((advance_px / 2.0).max(0.0));

        let left_oh_em = Self::lookup_overhang_em(
            table.svg_bbox_overhang_left,
            table.svg_bbox_overhang_left_default_em,
            first,
        );
        let mut right_oh_em = Self::lookup_overhang_em(
            table.svg_bbox_overhang_right,
            table.svg_bbox_overhang_right_default_em,
            last,
        );
        if bold && table.font_key == FLOWCHART_DEFAULT_FONT_KEY {
            right_oh_em = right_oh_em.max(flowchart_default_bold_svg_right_overhang_em(last));
        }

        let left = (half + left_oh_em * font_size).max(0.0);
        let right = (half + right_oh_em * font_size).max(0.0);
        (left, right)
    }

    fn line_svg_bbox_width_px(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
    ) -> f64 {
        let (l, r) = Self::line_svg_bbox_extents_px(table, text, font_size);
        (l + r).max(0.0)
    }

    fn line_svg_bbox_width_single_run_px(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
    ) -> f64 {
        let t = text.trim_end();
        if !t.is_empty() {
            if let Some((left_em, right_em)) =
                overrides::lookup_sequence_svg_override_em(table.font_key, t)
            {
                let left = Self::quantize_svg_bbox_px_nearest((left_em * font_size).max(0.0));
                let right = Self::quantize_svg_bbox_px_nearest((right_em * font_size).max(0.0));
                return (left + right).max(0.0);
            }
        }

        let (l, r) = Self::line_svg_bbox_extents_px_single_run(table, text, font_size);
        (l + r).max(0.0)
    }

    fn line_svg_title_bbox_extents_px(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        font_size: f64,
    ) -> (f64, f64) {
        let profile = Self::metric_profile(table);
        let t = text.trim_end();
        if t.is_empty() {
            return (0.0, 0.0);
        }

        // Flowchart titles are emitted as a centered single `<text>` node. The final upstream
        // root bbox behaves as a symmetric title advance, while the generic SVG override table
        // captures simple-text probes with per-edge overhang. Keep title measurement separate so
        // those simple-text asymmetries do not force fixture root viewport pins.
        let advance_px = if let Some(em) = Self::lookup_html_override_em(table.html_overrides, t) {
            em * font_size
        } else {
            Self::line_width_px(profile, t, false, font_size) * table.svg_scale
        };
        let half = Self::quantize_svg_half_px_nearest((advance_px / 2.0).max(0.0));
        (half, half)
    }

    fn split_token_to_svg_bbox_width_px(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        tok: &str,
        max_width_px: f64,
        font_size: f64,
    ) -> (String, String) {
        if max_width_px <= 0.0 {
            return (tok.to_string(), String::new());
        }
        let chars = tok.chars().collect::<Vec<_>>();
        if chars.is_empty() {
            return (String::new(), String::new());
        }

        let first = chars[0];
        let left_oh_em = if first.is_ascii() {
            0.0
        } else {
            Self::lookup_overhang_em(
                table.svg_bbox_overhang_left,
                table.svg_bbox_overhang_left_default_em,
                first,
            )
        };

        let mut em = 0.0;
        let mut prev: Option<char> = None;
        let mut split_at = 1usize;
        for (idx, ch) in chars.iter().enumerate() {
            em += Self::lookup_char_em(table.entries, table.default_em.max(0.1), *ch);
            if let Some(p) = prev {
                em += Self::lookup_kern_em(table.kern_pairs, p, *ch);
            }
            prev = Some(*ch);

            let right_oh_em = if ch.is_ascii() {
                0.0
            } else {
                Self::lookup_overhang_em(
                    table.svg_bbox_overhang_right,
                    table.svg_bbox_overhang_right_default_em,
                    *ch,
                )
            };
            let half_px = Self::quantize_svg_half_px_nearest(
                (em * font_size * table.svg_scale / 2.0).max(0.0),
            );
            let w_px = 2.0 * half_px + (left_oh_em + right_oh_em) * font_size;
            if w_px.is_finite() && w_px <= max_width_px {
                split_at = idx + 1;
            } else if idx > 0 {
                break;
            }
        }
        let head = chars[..split_at].iter().collect::<String>();
        let tail = chars[split_at..].iter().collect::<String>();
        (head, tail)
    }

    fn wrap_text_lines_svg_bbox_px(
        table: &crate::generated::font_metrics_flowchart_11_12_2::FontMetricsTables,
        text: &str,
        max_width_px: Option<f64>,
        font_size: f64,
        tokenize_whitespace: bool,
    ) -> Vec<String> {
        const EPS_PX: f64 = 0.125;
        let max_width_px = max_width_px.filter(|w| w.is_finite() && *w > 0.0);
        let width_fn = if tokenize_whitespace {
            Self::line_svg_bbox_width_px
        } else {
            Self::line_svg_bbox_width_single_run_px
        };

        let mut lines = Vec::new();
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let Some(w) = max_width_px else {
                lines.push(line);
                continue;
            };

            let mut tokens = std::collections::VecDeque::from(
                DeterministicTextMeasurer::split_line_to_words(&line),
            );
            let mut out: Vec<String> = Vec::new();
            let mut cur = String::new();

            while let Some(tok) = tokens.pop_front() {
                if cur.is_empty() && tok == " " {
                    continue;
                }

                let candidate = format!("{cur}{tok}");
                let candidate_trimmed = candidate.trim_end();
                if width_fn(table, candidate_trimmed, font_size) <= w + EPS_PX {
                    cur = candidate;
                    continue;
                }

                if !cur.trim().is_empty() {
                    out.push(cur.trim_end().to_string());
                    cur.clear();
                    tokens.push_front(tok);
                    continue;
                }

                if tok == " " {
                    continue;
                }

                if width_fn(table, tok.as_str(), font_size) <= w + EPS_PX {
                    cur = tok;
                    continue;
                }

                // Mermaid's SVG wrapping breaks long words.
                let (head, tail) =
                    Self::split_token_to_svg_bbox_width_px(table, &tok, w + EPS_PX, font_size);
                out.push(head);
                if !tail.is_empty() {
                    tokens.push_front(tail);
                }
            }

            if !cur.trim().is_empty() {
                out.push(cur.trim_end().to_string());
            }

            if out.is_empty() {
                lines.push("".to_string());
            } else {
                lines.extend(out);
            }
        }

        if lines.is_empty() {
            vec!["".to_string()]
        } else {
            lines
        }
    }

    fn line_width_px(
        profile: FontMetricProfile<'_>,
        text: &str,
        bold: bool,
        font_size: f64,
    ) -> f64 {
        fn normalize_whitespace_like(ch: char) -> (char, f64) {
            // Mermaid frequently uses `&nbsp;` inside HTML labels (e.g. block arrows). In SVG
            // exports this becomes U+00A0. Treat it as a regular space for width/kerning models
            // so it does not fall back to `default_em`.
            //
            // Empirically, for Mermaid@11.12.2 fixtures, U+00A0 measures slightly narrower than
            // U+0020 in the default font stack. Model that as a tiny delta in `em` space so
            // repeated `&nbsp;` placeholders land on the same 1/64px lattice as upstream.
            const NBSP_DELTA_EM: f64 = -1.0 / 3072.0;
            if ch == '\u{00A0}' {
                (' ', NBSP_DELTA_EM)
            } else {
                (ch, 0.0)
            }
        }

        let mut em = 0.0;
        let mut prevprev: Option<char> = None;
        let mut prev: Option<char> = None;
        let mut same_run_len = 0usize;
        for ch in text.chars() {
            let (ch, delta_em) = normalize_whitespace_like(ch);
            let next_same_run_len = if prev == Some(ch) {
                same_run_len + 1
            } else {
                1
            };
            em += Self::lookup_char_em(profile.entries, profile.default_em, ch) + delta_em;
            if let Some(p) = prev {
                em += Self::same_glyph_pair_kern_em(profile, p, ch, next_same_run_len);
            }
            if bold {
                if let Some(p) = prev {
                    em += flowchart_default_bold_kern_delta_em(p, ch);
                }
                em += flowchart_default_bold_delta_em(ch);
            }
            if let (Some(a), Some(b)) = (prevprev, prev) {
                if b == ' ' {
                    if !(a.is_whitespace() || ch.is_whitespace()) {
                        let space_delta =
                            Self::lookup_space_trigram_em(profile.space_trigrams, a, ch);
                        if space_delta != 0.0 {
                            em += space_delta;
                        } else if a == 'A' && ch == '(' {
                            em += profile.missing_space_after_capital_a_before_open_paren_em;
                        } else if ch == 'A' && a.is_ascii_alphanumeric() {
                            // The default Mermaid stack consistently tightens a preceding word
                            // space before capital `A`. The generated table captures this for
                            // observed pairs such as `r A`; use the same profile delta as a
                            // fallback for missing pairs instead of carrying per-label overrides.
                            em += profile.missing_space_before_capital_a_em;
                        }
                    }
                } else if !(a.is_whitespace() || b.is_whitespace() || ch.is_whitespace()) {
                    em += Self::same_glyph_trigram_em(profile, a, b, ch);
                }
            }
            prevprev = prev;
            prev = Some(ch);
            same_run_len = next_same_run_len;
        }
        em * font_size
    }

    fn split_token_to_width_px(
        profile: FontMetricProfile<'_>,
        tok: &str,
        max_width_px: f64,
        bold: bool,
        font_size: f64,
    ) -> (String, String) {
        fn normalize_whitespace_like(ch: char) -> (char, f64) {
            const NBSP_DELTA_EM: f64 = -1.0 / 3072.0;
            if ch == '\u{00A0}' {
                (' ', NBSP_DELTA_EM)
            } else {
                (ch, 0.0)
            }
        }

        if max_width_px <= 0.0 {
            return (tok.to_string(), String::new());
        }
        let max_em = max_width_px / font_size.max(1.0);
        let mut em = 0.0;
        let mut prevprev: Option<char> = None;
        let mut prev: Option<char> = None;
        let mut same_run_len = 0usize;
        let chars = tok.chars().collect::<Vec<_>>();
        let mut split_at = 0usize;
        for (idx, ch) in chars.iter().enumerate() {
            let (ch_norm, delta_em) = normalize_whitespace_like(*ch);
            let next_same_run_len = if prev == Some(ch_norm) {
                same_run_len + 1
            } else {
                1
            };
            em += Self::lookup_char_em(profile.entries, profile.default_em, ch_norm) + delta_em;
            if let Some(p) = prev {
                em += Self::same_glyph_pair_kern_em(profile, p, ch_norm, next_same_run_len);
            }
            if bold {
                if let Some(p) = prev {
                    em += flowchart_default_bold_kern_delta_em(p, ch_norm);
                }
                em += flowchart_default_bold_delta_em(ch_norm);
            }
            if let (Some(a), Some(b)) = (prevprev, prev) {
                if !(a.is_whitespace() || b.is_whitespace() || ch_norm.is_whitespace()) {
                    em += Self::same_glyph_trigram_em(profile, a, b, ch_norm);
                }
            }
            prevprev = prev;
            prev = Some(ch_norm);
            same_run_len = next_same_run_len;
            if em > max_em && idx > 0 {
                break;
            }
            split_at = idx + 1;
            if em >= max_em {
                break;
            }
        }
        if split_at == 0 {
            split_at = 1.min(chars.len());
        }
        let head = chars.iter().take(split_at).collect::<String>();
        let tail = chars.iter().skip(split_at).collect::<String>();
        (head, tail)
    }

    fn wrap_line_to_width_px(
        profile: FontMetricProfile<'_>,
        line: &str,
        max_width_px: f64,
        font_size: f64,
        break_long_words: bool,
        bold: bool,
    ) -> Vec<String> {
        fn split_html_breakable_segments(tok: &str) -> Vec<String> {
            // Browser HTML line breaking (UAX #14) provides extra break opportunities inside
            // path/URL-like tokens. Keep this deliberately narrow: short prose punctuation such
            // as `(a/b/c)` in subgraph titles should still wrap at spaces first, matching upstream
            // Mermaid's rendered 200px HTML title boxes.
            //
            // Intentionally *exclude* '=': upstream fixtures show tokens like `wrappingWidth=120`
            // overflowing rather than breaking at '='.
            let hyphen_count = tok.chars().filter(|ch| *ch == '-').count();
            let char_count = tok.chars().count();
            let is_hyphenated_compound = hyphen_count >= 2 && char_count >= 16;
            let is_url_like = tok.starts_with("http://") || tok.starts_with("https://");
            let is_path_like = is_hyphenated_compound
                || is_url_like
                || tok.len() >= 24
                    && tok
                        .chars()
                        .filter(|ch| {
                            matches!(ch, '/' | '\\' | '-' | ':' | '?' | '&' | '#' | '[' | ']')
                        })
                        .count()
                        >= 2;
            if !is_path_like {
                return vec![tok.to_string()];
            }

            fn is_break_after(ch: char, is_url_like: bool) -> bool {
                matches!(ch, '/' | '-' | ':' | '?' | '&' | '#' | ')' | ']' | '}')
                    || (is_url_like && ch == '.')
            }

            let mut out: Vec<String> = Vec::new();
            let mut cur = String::new();
            for ch in tok.chars() {
                cur.push(ch);
                if is_break_after(ch, is_url_like) && !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            if !cur.is_empty() {
                out.push(cur);
            }
            if out.len() <= 1 {
                vec![tok.to_string()]
            } else {
                out
            }
        }

        // HTML measurement in upstream Mermaid comes from the browser layout engine and tends to
        // be slightly more permissive at wrap boundaries than our glyph-advance sum (especially
        // after the 1/64px lattice quantization seen in fixtures). Add a tiny slack to reduce
        // off-by-one-line wrapping deltas near the threshold.
        let max_width_px = if break_long_words {
            max_width_px
        } else {
            max_width_px + (1.0 / 64.0)
        };

        let mut tokens =
            std::collections::VecDeque::from(DeterministicTextMeasurer::split_line_to_words(line));
        let mut out: Vec<String> = Vec::new();
        let mut cur = String::new();

        while let Some(tok) = tokens.pop_front() {
            if cur.is_empty() && tok == " " {
                continue;
            }

            let candidate = format!("{cur}{tok}");
            let candidate_trimmed = candidate.trim_end();
            if Self::line_width_px(profile, candidate_trimmed, bold, font_size) <= max_width_px {
                cur = candidate;
                continue;
            }

            if !break_long_words && tok != " " && !cur.trim().is_empty() {
                // Browser HTML layout uses punctuation-aware break opportunities even when a token
                // would fit on its own line (e.g. URLs inside parentheses). Try to consume a
                // breakable prefix before forcing the whole token onto the next line.
                let segments = split_html_breakable_segments(&tok);
                if segments.len() > 1 {
                    let mut cur_candidate = cur.clone();
                    let mut consumed = 0usize;
                    for seg in &segments {
                        let candidate = format!("{cur_candidate}{seg}");
                        let candidate_trimmed = candidate.trim_end();
                        if Self::line_width_px(profile, candidate_trimmed, bold, font_size)
                            <= max_width_px
                        {
                            cur_candidate = candidate;
                            consumed += 1;
                        } else {
                            break;
                        }
                    }
                    if consumed > 0 {
                        cur = cur_candidate;
                        for seg in segments.into_iter().skip(consumed).rev() {
                            tokens.push_front(seg);
                        }
                        continue;
                    }
                }
            }

            if !cur.trim().is_empty() {
                out.push(cur.trim_end().to_string());
                cur.clear();
            }

            if tok == " " {
                continue;
            }

            if Self::line_width_px(profile, tok.as_str(), bold, font_size) <= max_width_px {
                cur = tok;
                continue;
            }

            if !break_long_words {
                let segments = split_html_breakable_segments(&tok);
                if segments.len() > 1 {
                    for seg in segments.into_iter().rev() {
                        tokens.push_front(seg);
                    }
                    continue;
                }
                out.push(tok);
                continue;
            }

            let (head, tail) =
                Self::split_token_to_width_px(profile, &tok, max_width_px, bold, font_size);
            out.push(head);
            if !tail.is_empty() {
                tokens.push_front(tail);
            }
        }

        if !cur.trim().is_empty() {
            out.push(cur.trim_end().to_string());
        }

        if out.is_empty() {
            vec!["".to_string()]
        } else {
            out
        }
    }

    fn wrap_text_lines_px(
        profile: FontMetricProfile<'_>,
        text: &str,
        style: &TextStyle,
        bold: bool,
        max_width_px: Option<f64>,
        wrap_mode: WrapMode,
    ) -> Vec<String> {
        let font_size = style.font_size.max(1.0);
        let max_width_px = max_width_px.filter(|w| w.is_finite() && *w > 0.0);
        let break_long_words = wrap_mode == WrapMode::SvgLike;

        let mut lines = Vec::new();
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            if let Some(w) = max_width_px {
                lines.extend(Self::wrap_line_to_width_px(
                    profile,
                    &line,
                    w,
                    font_size,
                    break_long_words,
                    bold,
                ));
            } else {
                lines.push(line);
            }
        }

        if lines.is_empty() {
            vec!["".to_string()]
        } else {
            lines
        }
    }
}

fn vendored_measure_wrapped_impl(
    measurer: &VendoredFontMetricsTextMeasurer,
    text: &str,
    style: &TextStyle,
    max_width: Option<f64>,
    wrap_mode: WrapMode,
    use_html_overrides: bool,
) -> (TextMetrics, Option<f64>) {
    let Some(table) = measurer.lookup_table(style) else {
        return measurer
            .fallback
            .measure_wrapped_with_raw_width(text, style, max_width, wrap_mode);
    };

    let bold = is_flowchart_default_font(style) && style_requests_bold_font_weight(style);
    let font_size = style.font_size.max(1.0);
    let max_width = max_width.filter(|w| w.is_finite() && *w > 0.0);
    let line_height_factor = match wrap_mode {
        WrapMode::SvgLike | WrapMode::SvgLikeSingleRun => 1.1,
        WrapMode::HtmlLike => 1.5,
    };

    let html_overrides: &[(&'static str, f64)] = if use_html_overrides && !bold {
        table.html_overrides
    } else {
        &[]
    };
    let profile = VendoredFontMetricsTextMeasurer::metric_profile(table);

    let html_override_px = |em: f64| -> f64 {
        // `html_overrides` entries are generated from upstream fixtures by dividing the measured
        // pixel width by `base_font_size_px`. When a fixture applies a non-default `font-size`
        // via CSS (e.g. flowchart class definitions), the recorded width already reflects that
        // larger font size, so we must *not* scale it again by `font_size`.
        //
        // Empirically (Mermaid@11.12.2), upstream HTML label widths in those cases match
        // `em * base_font_size_px` rather than `em * font_size`.
        if (font_size - table.base_font_size_px).abs() < 0.01 {
            em * font_size
        } else {
            em * table.base_font_size_px
        }
    };

    let html_width_override_px = |line: &str| -> Option<f64> {
        // Flowchart labels still flow through the generic text API, so the few remaining
        // root-viewport guard widths stay here instead of in the Flowchart renderer.
        overrides::lookup_flowchart_html_width_px(table.font_key, font_size, line)
    };

    // Mermaid HTML labels behave differently depending on whether the content "needs" wrapping:
    // - if the unwrapped line width exceeds the configured wrapping width, Mermaid constrains
    //   the element to `width=max_width` and lets HTML wrapping determine line breaks
    //   (`white-space: break-spaces` / `width: 200px` patterns in upstream SVGs).
    // - otherwise, Mermaid uses an auto-sized container and measures the natural width.
    //
    // In headless mode we model this by computing the unwrapped width first, then forcing the
    // measured width to `max_width` when it would overflow.
    let raw_width_unscaled = if wrap_mode == WrapMode::HtmlLike {
        let mut raw_w: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            if let Some(w) = html_width_override_px(&line) {
                raw_w = raw_w.max(w);
                continue;
            }
            if let Some(em) =
                VendoredFontMetricsTextMeasurer::lookup_html_override_em(html_overrides, &line)
            {
                raw_w = raw_w.max(html_override_px(em));
            } else {
                raw_w = raw_w.max(VendoredFontMetricsTextMeasurer::line_width_px(
                    profile, &line, bold, font_size,
                ));
            }
        }
        Some(raw_w)
    } else {
        None
    };

    // Mermaid's HTML label measurements are taken from a `<div style="max-width: wpx">` that is
    // later switched to `display: table; width: wpx; white-space: break-spaces` when it hits the
    // max width.
    //
    // When a "word" (space-delimited token) is wider than the configured max width, browsers may
    // still wrap other parts of the paragraph, but the element's measured bounding box can expand
    // to accommodate the token's min-content width. Upstream Mermaid records that via
    // `getBoundingClientRect()` into `foreignObject width="..."`.
    //
    // Model this by tracking the widest space-delimited token width as a separate "min-content"
    // contributor to the final measured width, without changing the wrapping width used for line
    // breaking.
    fn split_html_min_content_segments(tok: &str) -> Vec<String> {
        // HTML min-content sizing for `display: table` tends to treat URL query separators as
        // break opportunities, but does not behave like a full `word-break: break-all`.
        //
        // Keep this conservative: avoid splitting on `/`/`.`/`:` so we still model wide URL path
        // segments that expand the measured bounding box beyond `wrappingWidth`.
        fn is_break_after(ch: char) -> bool {
            matches!(ch, '-' | '?' | '&' | '#')
        }

        let mut out: Vec<String> = Vec::new();
        let mut cur = String::new();
        for ch in tok.chars() {
            cur.push(ch);
            if is_break_after(ch) && !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        }
        if !cur.is_empty() {
            out.push(cur);
        }
        if out.len() <= 1 {
            vec![tok.to_string()]
        } else {
            out
        }
    }

    let html_min_content_width = if wrap_mode == WrapMode::HtmlLike && max_width.is_some() {
        let mut max_word_w: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            for part in line.split(' ') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                for seg in split_html_min_content_segments(part) {
                    let seg_w = html_width_override_px(&seg).unwrap_or_else(|| {
                        VendoredFontMetricsTextMeasurer::line_width_px(
                            profile,
                            seg.as_str(),
                            bold,
                            font_size,
                        )
                    });
                    max_word_w = max_word_w.max(seg_w);
                }
            }
        }
        if max_word_w.is_finite() && max_word_w > 0.0 {
            Some(max_word_w)
        } else {
            None
        }
    } else {
        None
    };

    let lines = match wrap_mode {
        WrapMode::HtmlLike => VendoredFontMetricsTextMeasurer::wrap_text_lines_px(
            profile, text, style, bold, max_width, wrap_mode,
        ),
        WrapMode::SvgLike => VendoredFontMetricsTextMeasurer::wrap_text_lines_svg_bbox_px(
            table, text, max_width, font_size, true,
        ),
        WrapMode::SvgLikeSingleRun => VendoredFontMetricsTextMeasurer::wrap_text_lines_svg_bbox_px(
            table, text, max_width, font_size, false,
        ),
    };

    let mut width: f64 = 0.0;
    match wrap_mode {
        WrapMode::HtmlLike => {
            for line in &lines {
                if let Some(w) = html_width_override_px(line) {
                    width = width.max(w);
                    continue;
                }
                if let Some(em) =
                    VendoredFontMetricsTextMeasurer::lookup_html_override_em(html_overrides, line)
                {
                    width = width.max(html_override_px(em));
                } else {
                    width = width.max(VendoredFontMetricsTextMeasurer::line_width_px(
                        profile, line, bold, font_size,
                    ));
                }
            }
        }
        WrapMode::SvgLike => {
            for line in &lines {
                width = width.max(VendoredFontMetricsTextMeasurer::line_svg_bbox_width_px(
                    table, line, font_size,
                ));
            }
        }
        WrapMode::SvgLikeSingleRun => {
            for line in &lines {
                width = width.max(
                    VendoredFontMetricsTextMeasurer::line_svg_bbox_width_single_run_px(
                        table, line, font_size,
                    ),
                );
            }
        }
    }

    // Mermaid HTML labels use `max-width` and can visually overflow for long words, but their
    // layout width is at least the max width in "wrapped" mode (tables), and may exceed it for
    // long unbreakable tokens.
    if wrap_mode == WrapMode::HtmlLike {
        let needs_wrap = max_width.is_some_and(|w| raw_width_unscaled.is_some_and(|rw| rw > w));
        if let Some(w) = max_width {
            if needs_wrap {
                width = width.max(w);
            } else {
                width = width.min(w);
            }
        }
        if needs_wrap {
            if let Some(w) = html_min_content_width {
                width = width.max(w);
            }
        }
        // Empirically, upstream HTML label widths (via `getBoundingClientRect()`) land on a 1/64px
        // lattice. Quantize to that grid to keep our layout math stable.
        width = round_to_1_64_px(width);
        if let Some(w) = max_width {
            width = if needs_wrap {
                width.max(w)
            } else {
                width.min(w)
            };
        }
    }

    let height = match wrap_mode {
        WrapMode::HtmlLike => lines.len() as f64 * font_size * line_height_factor,
        WrapMode::SvgLike | WrapMode::SvgLikeSingleRun => {
            if lines.is_empty() {
                0.0
            } else {
                // Mermaid's SVG `<text>.getBBox().height` behaves as "one taller first line"
                // plus 1.1em per additional wrapped line (observed in upstream fixtures at
                // Mermaid@11.12.2).
                // Chromium often reports an integer first-line bbox height; keep ties-to-even
                // rounding so `28.5px` becomes `28px` (matching upstream class SVG probes).
                let first_line_h = svg_wrapped_first_line_bbox_height_px(style);
                let additional = (lines.len().saturating_sub(1)) as f64 * font_size * 1.1;
                first_line_h + additional
            }
        }
    };

    let metrics = TextMetrics {
        width,
        height,
        line_count: lines.len(),
    };
    let raw_width_px = if wrap_mode == WrapMode::HtmlLike {
        raw_width_unscaled
    } else {
        None
    };
    (metrics, raw_width_px)
}

impl TextMeasurer for VendoredFontMetricsTextMeasurer {
    fn measure(&self, text: &str, style: &TextStyle) -> TextMetrics {
        self.measure_wrapped(text, style, None, WrapMode::SvgLike)
    }

    fn measure_svg_text_computed_length_px(&self, text: &str, style: &TextStyle) -> f64 {
        let Some(table) = self.lookup_table(style) else {
            return self
                .fallback
                .measure_svg_text_computed_length_px(text, style);
        };

        let bold = is_flowchart_default_font(style) && style_requests_bold_font_weight(style);
        let font_size = style.font_size.max(1.0);
        let profile = VendoredFontMetricsTextMeasurer::metric_profile(table);
        let mut width: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            width = width.max(VendoredFontMetricsTextMeasurer::line_width_px(
                profile, &line, bold, font_size,
            ));
        }
        if width.is_finite() && width >= 0.0 {
            width
        } else {
            0.0
        }
    }

    fn measure_svg_text_bbox_x(&self, text: &str, style: &TextStyle) -> (f64, f64) {
        let Some(table) = self.lookup_table(style) else {
            return self.fallback.measure_svg_text_bbox_x(text, style);
        };

        let font_size = style.font_size.max(1.0);
        let mut left: f64 = 0.0;
        let mut right: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let (l, r) = Self::line_svg_bbox_extents_px(table, &line, font_size);
            left = left.max(l);
            right = right.max(r);
        }
        (left, right)
    }

    fn measure_svg_text_bbox_x_with_ascii_overhang(
        &self,
        text: &str,
        style: &TextStyle,
    ) -> (f64, f64) {
        let Some(table) = self.lookup_table(style) else {
            return self
                .fallback
                .measure_svg_text_bbox_x_with_ascii_overhang(text, style);
        };

        let font_size = style.font_size.max(1.0);
        let mut left: f64 = 0.0;
        let mut right: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let (l, r) = Self::line_svg_bbox_extents_px_single_run_with_ascii_overhang(
                table, &line, font_size,
            );
            left = left.max(l);
            right = right.max(r);
        }
        (left, right)
    }

    fn measure_svg_title_bbox_x(&self, text: &str, style: &TextStyle) -> (f64, f64) {
        let Some(table) = self.lookup_table(style) else {
            return self.fallback.measure_svg_title_bbox_x(text, style);
        };

        let font_size = style.font_size.max(1.0);
        let mut left: f64 = 0.0;
        let mut right: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let (l, r) = Self::line_svg_title_bbox_extents_px(table, &line, font_size);
            left = left.max(l);
            right = right.max(r);
        }
        (left, right)
    }

    fn measure_svg_simple_text_bbox_width_px(&self, text: &str, style: &TextStyle) -> f64 {
        let Some(table) = self.lookup_table(style) else {
            return self
                .fallback
                .measure_svg_simple_text_bbox_width_px(text, style);
        };

        let font_size = style.font_size.max(1.0);
        let t = text.trim_end();
        if !t.is_empty() {
            if let Some((left_em, right_em)) =
                overrides::lookup_sequence_svg_override_em(table.font_key, t)
            {
                let left = Self::quantize_svg_bbox_px_nearest((left_em * font_size).max(0.0));
                let right = Self::quantize_svg_bbox_px_nearest((right_em * font_size).max(0.0));
                return (left + right).max(0.0);
            }
        }

        let mut width: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let (l, r) = Self::line_svg_bbox_extents_px_single_run_with_ascii_overhang(
                table, &line, font_size,
            );
            width = width.max((l + r).max(0.0));
        }
        width
    }

    fn measure_svg_simple_text_bbox_width_for_wrap_px(&self, text: &str, style: &TextStyle) -> f64 {
        let Some(table) = self.lookup_table(style) else {
            return self
                .fallback
                .measure_svg_simple_text_bbox_width_for_wrap_px(text, style);
        };

        let font_size = style.font_size.max(1.0);
        let mut width: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let (l, r) = Self::line_svg_bbox_extents_px_single_run_with_ascii_overhang(
                table, &line, font_size,
            );
            width = width.max((l + r).max(0.0));
        }
        width
    }

    fn measure_svg_raw_text_bbox_width_px(&self, text: &str, style: &TextStyle) -> f64 {
        let Some(table) = self.lookup_table(style) else {
            return self
                .fallback
                .measure_svg_raw_text_bbox_width_px(text, style);
        };

        let font_size = style.font_size.max(1.0);
        let bold = is_flowchart_default_font(style) && style_requests_bold_font_weight(style);
        let mut width: f64 = 0.0;
        for line in DeterministicTextMeasurer::normalized_text_lines(text) {
            let (l, r) = Self::line_svg_bbox_extents_px_single_run_with_ascii_overhang_and_weight(
                table, &line, font_size, bold,
            );
            width = width.max((l + r).max(0.0));
        }
        width
    }

    fn measure_svg_simple_text_bbox_height_px(&self, text: &str, style: &TextStyle) -> f64 {
        let t = text.trim_end();
        if t.is_empty() {
            return 0.0;
        }
        // Upstream gitGraph uses `<text>.getBBox().height` for commit/tag labels, and those values
        // land on a tighter ~`1.1em` height compared to our wrapped SVG text heuristic.
        let font_size = style.font_size.max(1.0);
        (font_size * 1.1).max(0.0)
    }

    fn measure_wrapped(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> TextMetrics {
        vendored_measure_wrapped_impl(self, text, style, max_width, wrap_mode, true).0
    }

    fn measure_wrapped_with_raw_width(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> (TextMetrics, Option<f64>) {
        vendored_measure_wrapped_impl(self, text, style, max_width, wrap_mode, true)
    }

    fn measure_wrapped_raw(
        &self,
        text: &str,
        style: &TextStyle,
        max_width: Option<f64>,
        wrap_mode: WrapMode,
    ) -> TextMetrics {
        vendored_measure_wrapped_impl(self, text, style, max_width, wrap_mode, false).0
    }
}
