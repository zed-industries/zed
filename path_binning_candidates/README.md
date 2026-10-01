# Path binning measurement tools

Not part of the PR. Each script applies `hooks.patch`, builds, and reverts it,
so the tree is clean between runs. Outputs go to `target/path_sweep` and
`target/profile`.

| File | What it is |
| --- | --- |
| `hooks.patch` | The temporary in-crate hooks: mounts `measure.rs` as a test module, lets `PATH_SWEEP` override the cost model's constants or force a cell size, counts listings, fakes the display scale (`GPUI_SCALE_FACTOR`), bins for a fixed scale (`PATH_BIN_SCALE`), and lets the painting example pick a scene (`PAINTING_SCENE`) at a fixed size and redraw every frame. |
| `measure.rs` | Fixtures (painting scenes and Zed-shaped paths), the `measure` table (build and bin µs, tiles, pixels, shader visits at `PATH_SCALE`, default 1.5; set `PATH_BIN_SCALE=1.5` with another `PATH_SCALE` to reproduce the old fixed-scale binner), the `calibrate` table (actual cells, listings and visits against the cost model's estimates), and the `profile` loop. |
| `tessellator.rs` | Our build plus binning at 1, 1.5 and 2x against lyon's tessellator, fresh and with reused buffers, on `measure.rs`'s fixtures. Not covered by the patch; see below. |
| `sweep.ps1` | CPU sweep through `measure`, GPU sweep through PresentMon on the painting example. |
| `profile.ps1`, `symbolize.py` | 8 kHz xperf profile of one fixture's binning, attributed to binner phases and source lines. Needs `sudo`; takes a few minutes. |

```powershell
# Configs are "visits_per_listing,min_tile_size,max_grid_cells[,forced_cell_size[,visits_per_cell]]";
# omitted fields keep the constants.
./path_binning_candidates/sweep.ps1 -Configs '1000','2000','2000,16' [-Cpu] [-Gpu] [-Rounds 2]
./path_binning_candidates/profile.ps1 -Fixture 'rust logo' -Seconds 5
```

`tessellator` has no script either. With the patch applied:

1. `cargo add --offline --dev -p gpui lyon_tessellation@1.0.22 lyon_algorithms@1.0.21`
   (back up `Cargo.lock` and the root `Cargo.toml` first, and restore them by
   copying: `cargo remove` rewrites the root manifest).
2. Add a `#[cfg(test)] pub(crate) fn into_lyon(self) -> (lyon_path::Path,
   PathStyle, Option<Vec<Pixels>>)` to `PathBuilder` returning the transformed
   raw path, style and dash array.
3. Mount `tessellator.rs` in `gpui.rs` next to `measure.rs`, as
   `path_binning_tessellator`, and run
   `path_binning_tessellator::tessellator --exact --ignored --nocapture`.

`calibrate` has no script: apply the patch, build the test binary, and run
`path_binning_measure::calibrate --exact --ignored --nocapture` with
`PATH_SWEEP=",,,<size>"` for each cell size.

Findings so far: the model's listing and visit estimates are within about
0.5–1.5× of the actual counts. CPU time fits about 10.8 ns per listing plus
3.2 ns per cell walked; GPU time about 14–21 ps per visit. Adding a per-cell
term in place of the exchange rate didn't separate fills from strokes, since
strokes in large bounding boxes walk many empty cells too. Charging every cell
(`VISITS_PER_CELL`) helped the scribbles and the full-screen selection but
made dashed grids coarser for +0.12–0.14 ms of GPU time at 1x and 2x.
Charging only the estimated empty cells, `cells - listings`, keeps the gains
and leaves the dashed grid alone; that's what the binner does now.

`hooks.patch` also has `GPUI_SCALE_FACTOR`, which fakes the display scale in
`gpui_windows` (the painting window is fixed at 1200x800 logical pixels, so
2x is 2400x1600 device pixels), and `PATH_BIN_SCALE`, which makes `paint_path`
bin for a fixed scale, as the old binner did. Set them in the environment
around `sweep.ps1 -Gpu`.

If `git apply` fails, the hooks' context drifted: re-apply them by hand and
regenerate the patch with `diff -u` against the clean files.
