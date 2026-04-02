use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use gpui::rgba;
use language::build_highlight_map;
use theme::SyntaxTheme;

fn syntax_theme(highlight_names: &[&str]) -> SyntaxTheme {
    SyntaxTheme::new(highlight_names.iter().enumerate().map(|(i, name)| {
        let r = ((i * 37) % 256) as u8;
        let g = ((i * 53) % 256) as u8;
        let b = ((i * 71) % 256) as u8;
        let color = rgba(u32::from_be_bytes([r, g, b, 0xff]));
        (name.to_string(), color.into())
    }))
}

static SMALL_THEME_KEYS: &[&str] = &[
    "comment", "function", "keyword", "string", "type", "variable",
];

static LARGE_THEME_KEYS: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.doc",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "emphasis",
    "emphasis.strong",
    "function",
    "function.builtin",
    "function.method",
    "function.method.builtin",
    "function.special.definition",
    "keyword",
    "keyword.control",
    "keyword.control.conditional",
    "keyword.control.import",
    "keyword.control.repeat",
    "keyword.control.return",
    "keyword.modifier",
    "keyword.operator",
    "label",
    "link_text",
    "link_uri",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.list_marker",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "string.special.symbol",
    "tag",
    "text.literal",
    "title",
    "type",
    "type.builtin",
    "type.super",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
    "variable.special",
];

static SMALL_CAPTURE_NAMES: &[&str] = &[
    "function",
    "keyword",
    "string.escape",
    "type.builtin",
    "variable.builtin",
];

static LARGE_CAPTURE_NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.doc",
    "constant",
    "constant.builtin",
    "constructor",
    "function",
    "function.builtin",
    "function.method",
    "keyword",
    "keyword.control",
    "keyword.control.conditional",
    "keyword.control.import",
    "keyword.modifier",
    "keyword.operator",
    "label",
    "number",
    "operator",
    "property",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
];

fn bench_build_highlight_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_highlight_map");

    for (capture_label, capture_names) in [
        ("small_captures", SMALL_CAPTURE_NAMES as &[&str]),
        ("large_captures", LARGE_CAPTURE_NAMES as &[&str]),
    ] {
        for (theme_label, theme_keys) in [
            ("small_theme", SMALL_THEME_KEYS as &[&str]),
            ("large_theme", LARGE_THEME_KEYS as &[&str]),
        ] {
            let theme = syntax_theme(theme_keys);
            group.bench_with_input(
                BenchmarkId::new(capture_label, theme_label),
                &(capture_names, &theme),
                |b, (capture_names, theme)| {
                    b.iter(|| build_highlight_map(black_box(capture_names), black_box(theme)));
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_build_highlight_map);
criterion_main!(benches);
