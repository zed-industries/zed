use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use gpui::rgba;
use language::{HighlightMap, Language};
use rope::Rope;
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

/// Interning a grammar's capture names, which happens once per grammar load.
fn bench_build_highlight_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_highlight_map");

    for (capture_label, capture_names) in [
        ("small_captures", SMALL_CAPTURE_NAMES as &[&str]),
        ("large_captures", LARGE_CAPTURE_NAMES as &[&str]),
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(capture_label),
            &capture_names,
            |b, capture_names| {
                b.iter(|| {
                    HighlightMap::from_capture_names(black_box(*capture_names).iter().copied())
                });
            },
        );
    }

    group.finish();
}

/// Resolving a token to a style, which happens for every highlighted span on
/// every frame. This is the path that pays for theme-independent ids: it
/// resolves through the capture name rather than indexing a precomputed slot.
fn bench_style_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_lookup");

    for (capture_label, capture_names) in [
        ("small_captures", SMALL_CAPTURE_NAMES as &[&str]),
        ("large_captures", LARGE_CAPTURE_NAMES as &[&str]),
    ] {
        let tokens = capture_names
            .iter()
            .map(|name| syntax_token::intern(name))
            .collect::<Vec<_>>();

        for (theme_label, theme_keys) in [
            ("small_theme", SMALL_THEME_KEYS as &[&str]),
            ("large_theme", LARGE_THEME_KEYS as &[&str]),
        ] {
            let theme = syntax_theme(theme_keys);
            group.bench_with_input(
                BenchmarkId::new(capture_label, theme_label),
                &(&tokens, &theme),
                |b, (tokens, theme)| {
                    b.iter(|| {
                        for token in black_box(*tokens) {
                            black_box(theme.get(*token));
                        }
                    });
                },
            );
        }
    }

    group.finish();
}

/// Iterating highlighted chunks, which happens for every visible span on every
/// frame. Each chunk carries the captures enclosing it so a theme can fall back
/// to a more general one, so this covers the cost of building that list.
fn bench_highlighted_chunks(c: &mut Criterion) {
    let queries = grammars::load_queries("rust");
    let Some(highlights) = queries.highlights.as_deref() else {
        return;
    };
    let language = std::sync::Arc::new(
        Language::new(
            grammars::load_config("rust"),
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_highlights_query(highlights)
        .expect("rust highlights query should compile"),
    );

    let source = Rope::from(SAMPLE.repeat(32).as_str());
    let mut group = c.benchmark_group("highlighted_chunks");
    group.bench_function("rust_source", |b| {
        b.iter(|| black_box(language.highlight_text(black_box(&source), 0..source.len())));
    });
    group.finish();
}

static SAMPLE: &str = r#"
pub fn resolve(theme: &SyntaxTheme, token: SyntaxTokenId) -> Option<HighlightStyle> {
    let name = syntax_token::name_for(token)?;
    match theme.highlight_id(&name) {
        Some(index) => theme.styles.get(index as usize).copied(),
        None => None,
    }
}
"#;

criterion_group!(
    benches,
    bench_build_highlight_map,
    bench_style_lookup,
    bench_highlighted_chunks
);
criterion_main!(benches);
