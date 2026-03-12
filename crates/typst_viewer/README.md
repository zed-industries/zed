# Typst Viewer

Live preview for [Typst](https://typst.app/) documents in Zed, powered by
[tinymist](https://github.com/Myriad-Dreamin/tinymist)'s incremental
compilation and server-side SVG rendering streamed over WebSocket.

Requires the [Zed typst extension](https://github.com/zed-industries/zed/tree/main/extensions/typst)
which launches tinymist as the language server. Our
[fork of tinymist](https://github.com/dsturnbull/tinymist) adds the
`--server-svg` flag for server-side SVG rendering and glyph defs stripping.

## Architecture

The preview leverages infrastructure already running: the tinymist LSP
(launched by the Zed typst extension) handles compilation with
[comemo](https://github.com/typst/typst/tree/main/crates/typst-library)
memoisation (~26ms incremental recompile). Rather than pulling tinymist's
~250 reflexo rendering crates into Zed, we have tinymist render SVG
server-side and stream it to Zed over WebSocket. Zed rasterises with
[resvg](https://github.com/nicubugarin/resvg) (already in the tree) and
displays via GPUI.

```
tinymist (native binary, launched by Zed typst extension)
 ├─ LSP server (textDocument/didChange on every keystroke)
 ├─ comemo incremental compiler (~26ms recompile)
 ├─ typst_svg::svg(page) → complete SVG per page
 ├─ glyph defs stripping (hash-based, ~200KB vs ~2MB per frame)
 └─ WebSocket data plane on localhost:PORT
     └─ Text messages: page:{idx}:{total}\n<svg ...>

Zed (typst_viewer crate)
 ├─ svg_stream.rs
 │   ├─ WebSocket client (async-tungstenite + smol)
 │   ├─ connect(), PreviewSocket type
 │   └─ Mock server for testing
 ├─ typst_viewer_view.rs
 │   ├─ Receive loop with frame dropping (now_or_never drain)
 │   ├─ Glyph defs caching + injection for stripped frames
 │   ├─ SVG rasterisation: usvg parse → resvg render → BGRA swap
 │   ├─ Multi-page display with vertical scroll
 │   └─ GPUI view with key_context + track_focus
 ├─ typst_viewer.rs
 │   ├─ LSP integration: find_tinymist_server, start_preview_via_lsp
 │   ├─ tinymist.doStartPreview with --server-svg
 │   └─ Workspace action registration
 └─ bench_preview.rs
     ├─ bench_preview_loop: cold compile via tinymist CLI
     └─ bench_preview_lsp: warm compile via LSP with comemo
```

## Data flow

```
keystroke
  → textDocument/didChange (LSP, buffer content)
  → tinymist comemo recompile (~26ms)
  → typst_svg::svg(page) per page
  → strip unchanged glyph defs (hash comparison)
  → WebSocket text message: page:{idx}:{total}\n<svg ...>
  → Zed receive loop
  → drain queued messages (frame dropping)
  → inject cached glyph defs if stripped
  → usvg::Tree::from_data → resvg::render → BGRA swap
  → Arc<RenderImage> with scale_factor
  → GPUI img() display, multi-page vertical scroll
```

## Multi-page wire protocol

Each page is sent as a separate WebSocket text message with a header:

```
page:0:5\n<svg class="typst-doc" viewBox="0 0 595 842" ...>...</svg>
page:1:5\n<svg class="typst-doc" viewBox="0 0 595 842" ...>...</svg>
...
```

Where `page:{0-based index}:{total pages}`. Legacy messages without the
prefix (plain `<svg>`) are treated as page 0 of 1.

## Glyph defs caching

Typst SVGs embed all glyph outlines as `<symbol>` elements inside
`<defs id="glyph">`. This is typically 80–90% of the SVG size (~1.5MB
of ~2MB). The tinymist fork strips unchanged glyph defs after the first
frame using a hash comparison, reducing per-frame transfer to ~200KB.

On the Zed side, the first frame's defs are cached. Subsequent frames
with stripped defs have them injected before rasterisation so usvg can
resolve all `<use>` references.

## Performance

Benchmarked in release mode against a complex A4 2-column legal reference
sheet (536 glyphs, 7704 glyph uses, 2566 text groups, 2MB SVG):

| Stage | Cold (CLI) | Warm (LSP + comemo) |
|-------|-----------|-------------------|
| Compile | 538ms | 26ms |
| Rasterise | 82ms | 82ms |
| **Total** | **624ms** | **109ms** |

At 80 wpm (~150ms between keystrokes), the warm pipeline keeps up.
Frame dropping ensures bursts don't queue up.

## Testing

```sh
# Unit tests (15 tests)
cargo test -p typst_viewer

# Pipeline benchmarks (requires tinymist binary + .typ document)
cargo test -p typst_viewer --release -- bench_preview --nocapture --ignored
```

Tests:
- **WebSocket**: roundtrip, live updates, rapid-fire (10 updates), clean shutdown
- **Multi-page**: mock generation, page header parse/reject, glyph defs injection
- **GPUI layout**: image bounds consistency across updates, display size verification

Benchmarks:
- **bench_preview_loop**: cold compile via `tinymist compile --format svg`
- **bench_preview_lsp**: warm compile via LSP stdio with comemo memoisation

## tinymist fork changes

The [tinymist fork](https://github.com/dsturnbull/tinymist) adds:

- `--server-svg` flag: renders complete SVG server-side instead of streaming
  vector IR. Sends as `WsMessage::Text` instead of `WsMessage::Binary`.
- `render_svg()` in `RenderActor`: iterates all `doc.pages`, renders each
  with `typst_svg::svg(page)`, prepends `page:{idx}:{total}\n` header.
- `strip_cached_glyph_defs()`: hash-based stripping of unchanged
  `<defs id="glyph">` sections after the first frame. Controlled by
  the `--strip-svg-glyph-defs` CLI flag so it doesn't affect the
  default tinymist behaviour.

Usage:

```sh
tinymist preview --server-svg --strip-svg-glyph-defs my-document.typ
```

When started via the LSP (`tinymist.doStartPreview`), the Zed client
passes both `--server-svg` and `--strip-svg-glyph-defs` automatically.