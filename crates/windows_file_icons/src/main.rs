use anyhow::{Context as _, Result, anyhow, bail};
use image::{
    ExtendedColorType, ImageBuffer, ImageEncoder as _, RgbaImage,
    codecs::{
        ico::{IcoEncoder, IcoFrame},
        png::PngEncoder,
    },
};
use resvg::tiny_skia::{Pixmap, Transform};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const ICON_SIZES: &[u32] = &[256, 64, 48, 32, 16];
const CANVAS_SIZE: f32 = 256.;
const LARGE_GLYPH_LEFT: f32 = 64.;
const LARGE_GLYPH_TOP: f32 = 82.;
const LARGE_GLYPH_RIGHT: f32 = 192.;
const LARGE_GLYPH_BOTTOM: f32 = 210.;
const SMALL_GLYPH_LEFT: f32 = 48.;
const SMALL_GLYPH_TOP: f32 = 62.;
const SMALL_GLYPH_RIGHT: f32 = 210.;
const SMALL_GLYPH_BOTTOM: f32 = 224.;
const DOCUMENT_BASE_SVG: &[u8] = br##"
<svg width="256" height="256" viewBox="0 0 256 256" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M52 16h104l48 48v176H52V16z" fill="#F8F8F8"/>
  <path d="M156 16v48h48" fill="#E6E6E6"/>
  <path d="M52 16h104l48 48v176H52V16z" stroke="#C8C8C8" stroke-width="8" stroke-linejoin="round"/>
  <path d="M156 16v48h48" stroke="#C8C8C8" stroke-width="8" stroke-linejoin="round"/>
</svg>
"##;
const SMALL_DOCUMENT_BASE_SVG: &[u8] = br##"
<svg width="256" height="256" viewBox="0 0 256 256" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M30 10h140l56 56v180H30V10z" fill="#F8F8F8"/>
  <path d="M170 10v56h56" fill="#E6E6E6"/>
  <path d="M30 10h140l56 56v180H30V10z" stroke="#C8C8C8" stroke-width="16" stroke-linejoin="round"/>
  <path d="M170 10v56h56" stroke="#C8C8C8" stroke-width="16" stroke-linejoin="round"/>
</svg>
"##;

fn main() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let source_dir = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("missing source SVG directory argument"))?;
    let output_dir = args
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("missing output ICO directory argument"))?;

    if args.next().is_some() {
        bail!("usage: windows_file_icons <source-svg-dir> <output-ico-dir>");
    }

    generate_icons(&source_dir, &output_dir)
}

fn generate_icons(source_dir: &Path, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).with_context(|| format!("creating {}", output_dir.display()))?;

    let mut generated_count = 0;
    for entry in
        fs::read_dir(source_dir).with_context(|| format!("reading {}", source_dir.display()))?
    {
        let entry = entry?;
        let source_path = entry.path();
        if !source_path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"))
        {
            continue;
        }

        let file_stem = source_path
            .file_stem()
            .ok_or_else(|| anyhow!("missing file stem for {}", source_path.display()))?;
        let output_path = output_dir.join(file_stem).with_extension("ico");
        generate_icon(&source_path, &output_path)?;
        generated_count += 1;
    }

    if generated_count == 0 {
        bail!("no SVG icons found in {}", source_dir.display());
    }

    println!(
        "Generated {generated_count} Windows file icons in {}",
        output_dir.display()
    );
    Ok(())
}

fn generate_icon(source_path: &Path, output_path: &Path) -> Result<()> {
    let svg =
        fs::read(source_path).with_context(|| format!("reading {}", source_path.display()))?;
    let mut frames = Vec::new();

    for size in ICON_SIZES {
        let image = render_svg(&svg, *size)
            .with_context(|| format!("rendering {} at {size}px", source_path.display()))?;
        let png = encode_png(&image)?;
        frames.push(IcoFrame::with_encoded(
            png,
            image.width(),
            image.height(),
            ExtendedColorType::Rgba8,
        )?);
    }

    let mut output = fs::File::create(output_path)
        .with_context(|| format!("creating {}", output_path.display()))?;
    IcoEncoder::new(&mut output).encode_images(&frames)?;
    Ok(())
}

fn render_svg(svg: &[u8], output_size: u32) -> Result<RgbaImage> {
    let tree = usvg::Tree::from_data(svg, &usvg::Options::default())?;
    let layout = layout_for_size(output_size);
    let document_tree = usvg::Tree::from_data(layout.document_base_svg, &usvg::Options::default())?;

    let mut pixmap =
        Pixmap::new(output_size, output_size).ok_or_else(|| anyhow!("invalid icon size"))?;
    let document_scale = output_size as f32 / CANVAS_SIZE;
    render_tree(
        &document_tree,
        &mut pixmap,
        Transform::from_scale(document_scale, document_scale),
    );

    let svg_size = tree.size();
    let glyph_left = scale_for_size(layout.glyph_left, output_size);
    let glyph_top = scale_for_size(layout.glyph_top, output_size);
    let glyph_width = scale_for_size(layout.glyph_right - layout.glyph_left, output_size);
    let glyph_height = scale_for_size(layout.glyph_bottom - layout.glyph_top, output_size);
    let width_scale = glyph_width / svg_size.width();
    let height_scale = glyph_height / svg_size.height();
    let scale = width_scale.min(height_scale);
    let scaled_width = svg_size.width() * scale;
    let scaled_height = svg_size.height() * scale;
    let offset_x = glyph_left + (glyph_width - scaled_width) / 2.;
    let offset_y = glyph_top + (glyph_height - scaled_height) / 2.;

    let transform = Transform::from_translate(offset_x, offset_y).pre_scale(scale, scale);
    render_tree(&tree, &mut pixmap, transform);

    ImageBuffer::from_raw(output_size, output_size, pixmap.take())
        .ok_or_else(|| anyhow!("rendered pixmap had invalid dimensions"))
}

fn render_tree(tree: &usvg::Tree, pixmap: &mut Pixmap, transform: Transform) {
    resvg::render(tree, transform, &mut pixmap.as_mut());
}

fn scale_for_size(value: f32, output_size: u32) -> f32 {
    value * output_size as f32 / CANVAS_SIZE
}

struct Layout {
    document_base_svg: &'static [u8],
    glyph_left: f32,
    glyph_top: f32,
    glyph_right: f32,
    glyph_bottom: f32,
}

fn layout_for_size(output_size: u32) -> Layout {
    if output_size <= 32 {
        Layout {
            document_base_svg: SMALL_DOCUMENT_BASE_SVG,
            glyph_left: SMALL_GLYPH_LEFT,
            glyph_top: SMALL_GLYPH_TOP,
            glyph_right: SMALL_GLYPH_RIGHT,
            glyph_bottom: SMALL_GLYPH_BOTTOM,
        }
    } else {
        Layout {
            document_base_svg: DOCUMENT_BASE_SVG,
            glyph_left: LARGE_GLYPH_LEFT,
            glyph_top: LARGE_GLYPH_TOP,
            glyph_right: LARGE_GLYPH_RIGHT,
            glyph_bottom: LARGE_GLYPH_BOTTOM,
        }
    }
}

fn encode_png(image: &RgbaImage) -> Result<Vec<u8>> {
    let mut png = Vec::new();
    PngEncoder::new(&mut png).write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        ExtendedColorType::Rgba8,
    )?;
    Ok(png)
}
