# Theme-Aware LaTeX Math Rendering for Zed

Adds native LaTeX math rendering to Zed's markdown pipeline. Superscript/subscript fractions, integrals, sums, Greek letters, and the full KaTeX command set are rendered inline with the editor's buffer font and color theme. Designed for the Assistant panel but wired through `MarkdownOptions` so any markdown consumer can enable it.

## Pipeline

```
Markdown source ($...$ / $$...$$)
  → pulldown-cmark 0.13 (ENABLE_MATH)
    → MarkdownEvent::InlineMath / DisplayMath
      → ratex-parser   (LaTeX AST)
        → ratex-layout (display tree with placeholder color)
          → Two-phase cache entry
```

## Two-Phase Cache

Each math expression is stored in a `CachedMathExpression` with two independent, lazily populated slots:

| Slot | Contents | Invalidated on |
|------|----------|----------------|
| `display_tree` | `(DisplayList, baseline_y)` — the parsed and laid-out display tree. Color is a placeholder; the tree is semantically complete. | Content change, font size change |
| `rendered` | `MathRenderResult` — the final `(RenderImage, baseline_y)` rasterized from the recolored display tree using the current theme color. | Theme change only |

**On theme change** — `MathState::retheme()` iterates every cached entry and runs `recolor_and_render()`:
1. Read the cached `DisplayList` (no re-parse)
2. Replace every `DisplayItem`'s color field via `recolor_display_list()`
3. Feed the recolored list through `math_svg::display_list_to_svg()`
4. Rasterize the SVG with `SvgRenderer::render_single_frame()`
5. Store the new `MathRenderResult`

The expensive LaTeX parsing and box-layout steps are skipped entirely on theme switches. Only the SVG generation (string manipulation) and GPU rasterization run on each theme change, making even large documents with dozens of expressions switch themes instantly.

**On content or font size change** — `MathState::update()` clears the cache for expressions that no longer exist and creates new `CachedMathExpression` entries that run the full parse → layout → recolor → render pipeline on a background thread.

## Asynchronous Rendering

`CachedMathExpression::new()` spawns a two-phase task via `cx.spawn()`:

```
Phase 1 (background_spawn):
  ratex_parser::parse(latex)
  ratex_layout::layout(nodes, Default::default())
  ratex_layout::to_display_list(layout_box)
  → stores (DisplayList, baseline_y) in OnceLock<>

Phase 2 (same task, after phase 1):
  recolor_display_list(display_list, current_theme_color)
  math_svg::display_list_to_svg(recolored, font_size)
  svg_renderer.render_single_frame(svg_bytes, 1.0)
  → stores MathRenderResult in Mutex<Option<>>
```

While the task runs, `render_math_expression()` returns a fallback element showing the raw LaTeX source styled as inline text. No flicker or blocking occurs.

## Baseline Alignment

Inline math (`$...$`) must sit on the same baseline as surrounding text. The implementation:

1. Reads `ThemeSettings.buffer_font` to resolve the font family and weight.
2. Calls `cx.text_system().ascent(font_id, Pixels::from(font_size))` to get the font's ascent in pixels.
3. The SVG generation computes `baseline_y = display_list.height.as_f32 * font_size + 2.0` (the `2.0` is a padding constant).
4. The rendered image is shifted by `ascent - baseline_y` pixels using `mt(px(shift))`.

Display math (`$$...$$`) is block-displayed without baseline correction.

## File Reference

### `crates/markdown/src/math.rs`

| Item | Role |
|------|------|
| `ParsedMarkdownMathExpression` | Public data: source range, raw LaTeX content, display/inline flag |
| `MathCacheKey` | `(latex: SharedString, font_size_bits: u32)` — identifies a unique expression at a given font size |
| `MathState` | Owns the expression cache, font size, and text ascent. Provides `update()`, `retheme()`, `clear()`, `invalidate()` |
| `CachedMathExpression` | Per-expression storage: parsed display tree + rendered image + async task handle |
| `MathRenderResult` | Holds `Arc<RenderImage>` and `baseline_y: f32` |
| `gpui_color_to_ratex()` | Converts `gpui::Hsla` → `ratex_types::color::Color` |
| `recolor_display_list()` | Replaces every `DisplayItem.color` field with a given color — O(n) in display items |
| `render_math_expression()` | Public entry point: looks up expression in cache, returns image element or fallback |
| `extract_math_expressions()` | Walks pulldown-cmark events, collects `InlineMath`/`DisplayMath` into `BTreeMap` by source offset |

### `crates/markdown/src/math_svg.rs`

| Item | Role |
|------|------|
| `MathSvgOutput` | SVG bytes, logical width/height, baseline Y |
| `display_list_to_svg()` | Converts `DisplayList` → SVG string with font glyph outlines, lines, rects, and paths |
| `resolve_glyph_path()` | Extracts TrueType outline curves from KaTeX fonts using `ab_glyph` |
| `path_commands_to_svg_d()` | Converts ratex `PathCommand` sequence → SVG path `d` attribute |
| `color_to_svg()` | `ratex_color` → hex string or `rgba()` string |
| `tests` | Unit tests for SVG structure and empty display lists |

### `crates/markdown/src/markdown.rs`

| Change | Purpose |
|--------|---------|
| `_theme_subscription` | Single `Option<Subscription>` replacing separate mermaid/math subscriptions |
| `render_math` flag in `MarkdownOptions` | Enables/disables math rendering per-view |
| `inline_math` in `MarkdownStyle` | Fallback text style for raw LaTeX while rendering is pending |
| `math_state: MathState` | Cache and layout state owned by the `Markdown` entity |
| `invalidate_math_cache()` | Changed from `invalidate()` + `update()` to single `retheme(cx)` call |

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratex-parser` | 0.1.13 | LaTeX tokenizer and AST parser |
| `ratex-layout` | 0.1.13 | Box layout from parsed AST with `LayoutOptions` |
| `ratex-render` | 0.1.13 | Feature `embed-fonts` pulls in KaTeX TTF fonts |
| `ratex-font` | 0.1.13 | Font ID handling and glyph-to-char mapping |
| `ratex-font-loader` | 0.1.13 | Font file loading and glyph outline caching |
| `ratex-types` | 0.1.13 | Shared types: `DisplayList`, `DisplayItem`, `Color`, etc. |
| `ab_glyph` | 0.2 | TrueType outline extraction for SVG path generation |

All six ratex crates are declared as workspace dependencies in the root `Cargo.toml` and consumed by `crates/markdown/Cargo.toml`.

## Testing

```
cargo test -p markdown
```

Unit tests cover:

- **Extraction**: inline math, display math, multiple expressions, code block exclusion, source ranges, empty input, cache key equality — 11 tests in `math.rs`.
- **SVG generation**: rect element structure, empty display list — 2 tests in `math_svg.rs`.

Integration tests (gpui rendering) require a platform with GPU access and are not included in the lib test suite.
