# Merman Render Assets

These files are packaged with `merman-render` because renderer code loads them relative to
`CARGO_MANIFEST_DIR`.

- `sequence_base_defs_11_12_2.svgfrag` is embedded into the Mermaid-parity sequence SVG renderer.
- `c4_database_d_11_12_2.txt` is embedded into the Mermaid-parity C4 database icon definition.
- `katex_flowchart_probe.cjs` is used by the optional Node.js KaTeX probe backend for HTML/math
  measurement audits.

Do not remove these files from crate packaging unless the corresponding renderer path is removed or
rewired.
