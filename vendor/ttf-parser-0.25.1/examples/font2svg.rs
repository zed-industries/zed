#![allow(clippy::too_many_arguments)]

use base64::engine::general_purpose::STANDARD;

use std::io::Write;
use std::path::PathBuf;

use ttf_parser as ttf;
use ttf_parser::colr::{ClipBox, Paint};
use ttf_parser::{RgbaColor, Transform};

const FONT_SIZE: f64 = 128.0;
const COLUMNS: u32 = 100;

const HELP: &str = "\
Usage:
    font2svg font.ttf out.svg
    font2svg --variations 'wght:500;wdth:200' font.ttf out.svg
    font2svg --colr-palette 1 colr-font.ttf out.svg
";

struct Args {
    #[allow(dead_code)]
    variations: Vec<ttf::Variation>,
    colr_palette: u16,
    ttf_path: PathBuf,
    svg_path: PathBuf,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            print!("{}", HELP);
            std::process::exit(1);
        }
    };

    if let Err(e) = process(args) {
        eprintln!("Error: {}.", e);
        std::process::exit(1);
    }
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let variations = args.opt_value_from_fn("--variations", parse_variations)?;
    let colr_palette: u16 = args.opt_value_from_str("--colr-palette")?.unwrap_or(0);
    let free = args.finish();
    if free.len() != 2 {
        return Err("invalid number of arguments".into());
    }

    Ok(Args {
        variations: variations.unwrap_or_default(),
        colr_palette,
        ttf_path: PathBuf::from(&free[0]),
        svg_path: PathBuf::from(&free[1]),
    })
}

fn parse_variations(s: &str) -> Result<Vec<ttf::Variation>, &'static str> {
    let mut variations = Vec::new();
    for part in s.split(';') {
        let mut iter = part.split(':');

        let axis = iter.next().ok_or("failed to parse a variation")?;
        let axis = ttf::Tag::from_bytes_lossy(axis.as_bytes());

        let value = iter.next().ok_or("failed to parse a variation")?;
        let value: f32 = value.parse().map_err(|_| "failed to parse a variation")?;

        variations.push(ttf::Variation { axis, value });
    }

    Ok(variations)
}

fn process(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let font_data = std::fs::read(&args.ttf_path)?;

    // Exclude IO operations.
    let now = std::time::Instant::now();

    #[allow(unused_mut)]
    let mut face = ttf::Face::parse(&font_data, 0)?;
    if face.is_variable() {
        #[cfg(feature = "variable-fonts")]
        {
            for variation in args.variations {
                face.set_variation(variation.axis, variation.value)
                    .ok_or("failed to create variation coordinates")?;
            }
        }
    }

    if face.tables().colr.is_some() {
        if let Some(total) = face.color_palettes() {
            if args.colr_palette >= total.get() {
                return Err(format!("only {} palettes are available", total).into());
            }
        }
    }

    let num_glyphs = face.number_of_glyphs();

    let units_per_em = face.units_per_em();
    let scale = FONT_SIZE / units_per_em as f64;

    let cell_size = face.height() as f64 * FONT_SIZE / units_per_em as f64;
    let rows = (num_glyphs as f64 / COLUMNS as f64).ceil() as u32;

    let mut svg = xmlwriter::XmlWriter::with_capacity(
        num_glyphs as usize * 512,
        xmlwriter::Options::default(),
    );
    svg.start_element("svg");
    svg.write_attribute("xmlns", "http://www.w3.org/2000/svg");
    svg.write_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
    svg.write_attribute_fmt(
        "viewBox",
        format_args!(
            "{} {} {} {}",
            0,
            0,
            cell_size * COLUMNS as f64,
            cell_size * rows as f64
        ),
    );

    draw_grid(num_glyphs, cell_size, &mut svg);

    let mut path_buf = String::with_capacity(256);
    let mut row = 0;
    let mut column = 0;
    let mut gradient_index = 1;
    let mut clip_path_index = 1;
    for id in 0..num_glyphs {
        let gid = ttf::GlyphId(id);
        let x = column as f64 * cell_size;
        let y = row as f64 * cell_size;

        svg.start_element("text");
        svg.write_attribute("x", &(x + 2.0));
        svg.write_attribute("y", &(y + cell_size - 4.0));
        svg.write_attribute("font-size", "36");
        svg.write_attribute("fill", "gray");
        svg.write_text_fmt(format_args!("{}", &id));
        svg.end_element();

        if face.is_color_glyph(gid) {
            color_glyph(
                x,
                y,
                &face,
                args.colr_palette,
                gid,
                cell_size,
                scale,
                &mut gradient_index,
                &mut clip_path_index,
                &mut svg,
                &mut path_buf,
            );
        } else if let Some(img) = face.glyph_raster_image(gid, u16::MAX) {
            svg.start_element("image");
            svg.write_attribute("x", &(x + 2.0 + img.x as f64));
            svg.write_attribute("y", &(y - img.y as f64));
            svg.write_attribute("width", &img.width);
            svg.write_attribute("height", &img.height);
            svg.write_attribute_raw("xlink:href", |buf| {
                buf.extend_from_slice(b"data:image/png;base64, ");

                let mut enc = base64::write::EncoderWriter::new(buf, &STANDARD);
                enc.write_all(img.data).unwrap();
                enc.finish().unwrap();
            });
            svg.end_element();
        } else if let Some(img) = face.glyph_svg_image(gid) {
            svg.start_element("image");
            svg.write_attribute("x", &(x + 2.0));
            svg.write_attribute("y", &(y + cell_size));
            svg.write_attribute("width", &cell_size);
            svg.write_attribute("height", &cell_size);
            svg.write_attribute_raw("xlink:href", |buf| {
                buf.extend_from_slice(b"data:image/svg+xml;base64, ");

                let mut enc = base64::write::EncoderWriter::new(buf, &STANDARD);
                enc.write_all(img.data).unwrap();
                enc.finish().unwrap();
            });
            svg.end_element();
        } else {
            glyph_to_path(x, y, &face, gid, cell_size, scale, &mut svg, &mut path_buf);
        }

        column += 1;
        if column == COLUMNS {
            column = 0;
            row += 1;
        }
    }

    println!("Elapsed: {}ms", now.elapsed().as_micros() as f64 / 1000.0);

    std::fs::write(&args.svg_path, svg.end_document())?;

    Ok(())
}

fn draw_grid(n_glyphs: u16, cell_size: f64, svg: &mut xmlwriter::XmlWriter) {
    let columns = COLUMNS;
    let rows = (n_glyphs as f64 / columns as f64).ceil() as u32;

    let width = columns as f64 * cell_size;
    let height = rows as f64 * cell_size;

    svg.start_element("path");
    svg.write_attribute("fill", "none");
    svg.write_attribute("stroke", "black");
    svg.write_attribute("stroke-width", "5");

    let mut path = String::with_capacity(256);

    use std::fmt::Write;
    let mut x = 0.0;
    for _ in 0..=columns {
        write!(&mut path, "M {} {} L {} {} ", x, 0.0, x, height).unwrap();
        x += cell_size;
    }

    let mut y = 0.0;
    for _ in 0..=rows {
        write!(&mut path, "M {} {} L {} {} ", 0.0, y, width, y).unwrap();
        y += cell_size;
    }

    path.pop();

    svg.write_attribute("d", &path);
    svg.end_element();
}

struct Builder<'a>(&'a mut String);

impl Builder<'_> {
    fn finish(&mut self) {
        if !self.0.is_empty() {
            self.0.pop(); // remove trailing space
        }
    }
}

impl ttf::OutlineBuilder for Builder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        use std::fmt::Write;
        write!(self.0, "M {} {} ", x, y).unwrap()
    }

    fn line_to(&mut self, x: f32, y: f32) {
        use std::fmt::Write;
        write!(self.0, "L {} {} ", x, y).unwrap()
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        use std::fmt::Write;
        write!(self.0, "Q {} {} {} {} ", x1, y1, x, y).unwrap()
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        use std::fmt::Write;
        write!(self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap()
    }

    fn close(&mut self) {
        self.0.push_str("Z ")
    }
}

fn glyph_to_path(
    x: f64,
    y: f64,
    face: &ttf::Face,
    glyph_id: ttf::GlyphId,
    cell_size: f64,
    scale: f64,
    svg: &mut xmlwriter::XmlWriter,
    path_buf: &mut String,
) {
    path_buf.clear();
    let mut builder = Builder(path_buf);
    let bbox = match face.outline_glyph(glyph_id, &mut builder) {
        Some(v) => v,
        None => return,
    };
    builder.finish();

    let bbox_w = (bbox.x_max as f64 - bbox.x_min as f64) * scale;
    let dx = (cell_size - bbox_w) / 2.0;
    let y = y + cell_size + face.descender() as f64 * scale;

    let transform = format!("matrix({} 0 0 {} {} {})", scale, -scale, x + dx, y);

    svg.start_element("path");
    svg.write_attribute("d", path_buf);
    svg.write_attribute("transform", &transform);
    svg.end_element();

    {
        let bbox_h = (bbox.y_max as f64 - bbox.y_min as f64) * scale;
        let bbox_x = x + dx + bbox.x_min as f64 * scale;
        let bbox_y = y - bbox.y_max as f64 * scale;

        svg.start_element("rect");
        svg.write_attribute("x", &bbox_x);
        svg.write_attribute("y", &bbox_y);
        svg.write_attribute("width", &bbox_w);
        svg.write_attribute("height", &bbox_h);
        svg.write_attribute("fill", "none");
        svg.write_attribute("stroke", "green");
        svg.end_element();
    }
}

// NOTE: this is not a feature-full implementation and just a demo.
struct GlyphPainter<'a> {
    face: &'a ttf::Face<'a>,
    svg: &'a mut xmlwriter::XmlWriter,
    path_buf: &'a mut String,
    gradient_index: usize,
    clip_path_index: usize,
    palette_index: u16,
    transform: ttf::Transform,
    outline_transform: ttf::Transform,
    transforms_stack: Vec<ttf::Transform>,
}

impl<'a> GlyphPainter<'a> {
    fn write_gradient_stops(&mut self, stops: ttf::colr::GradientStopsIter) {
        for stop in stops {
            self.svg.start_element("stop");
            self.svg.write_attribute("offset", &stop.stop_offset);
            self.svg.write_color_attribute("stop-color", stop.color);
            let opacity = f32::from(stop.color.alpha) / 255.0;
            self.svg.write_attribute("stop-opacity", &opacity);
            self.svg.end_element();
        }
    }

    fn paint_solid(&mut self, color: ttf::RgbaColor) {
        self.svg.start_element("path");
        self.svg.write_color_attribute("fill", color);
        let opacity = f32::from(color.alpha) / 255.0;
        self.svg.write_attribute("fill-opacity", &opacity);
        self.svg
            .write_transform_attribute("transform", self.outline_transform);
        self.svg.write_attribute("d", self.path_buf);
        self.svg.end_element();
    }

    fn paint_linear_gradient(&mut self, gradient: ttf::colr::LinearGradient<'a>) {
        let gradient_id = format!("lg{}", self.gradient_index);
        self.gradient_index += 1;

        let gradient_transform = paint_transform(self.outline_transform, self.transform);

        // TODO: We ignore x2, y2. Have to apply them somehow.
        // TODO: The way spreadMode works in ttf and svg is a bit different. In SVG, the spreadMode
        // will always be applied based on x1/y1 and x2/y2. However, in TTF the spreadMode will
        // be applied from the first/last stop. So if we have a gradient with x1=0 x2=1, and
        // a stop at x=0.4 and x=0.6, then in SVG we will always see a padding, while in ttf
        // we will see the actual spreadMode. We need to account for that somehow.
        self.svg.start_element("linearGradient");
        self.svg.write_attribute("id", &gradient_id);
        self.svg.write_attribute("x1", &gradient.x0);
        self.svg.write_attribute("y1", &gradient.y0);
        self.svg.write_attribute("x2", &gradient.x1);
        self.svg.write_attribute("y2", &gradient.y1);
        self.svg.write_attribute("gradientUnits", &"userSpaceOnUse");
        self.svg.write_spread_method_attribute(gradient.extend);
        self.svg
            .write_transform_attribute("gradientTransform", gradient_transform);
        self.write_gradient_stops(gradient.stops(
            self.palette_index,
            #[cfg(feature = "variable-fonts")]
            self.face.variation_coordinates(),
        ));
        self.svg.end_element();

        self.svg.start_element("path");
        self.svg
            .write_attribute_fmt("fill", format_args!("url(#{})", gradient_id));
        self.svg
            .write_transform_attribute("transform", self.outline_transform);
        self.svg.write_attribute("d", self.path_buf);
        self.svg.end_element();
    }

    fn paint_radial_gradient(&mut self, gradient: ttf::colr::RadialGradient<'a>) {
        let gradient_id = format!("rg{}", self.gradient_index);
        self.gradient_index += 1;

        self.svg.start_element("radialGradient");
        self.svg.write_attribute("id", &gradient_id);
        self.svg.write_attribute("cx", &gradient.x1);
        self.svg.write_attribute("cy", &gradient.y1);
        self.svg.write_attribute("r", &gradient.r1);
        self.svg.write_attribute("fr", &gradient.r0);
        self.svg.write_attribute("fx", &gradient.x0);
        self.svg.write_attribute("fy", &gradient.y0);
        self.svg.write_attribute("gradientUnits", &"userSpaceOnUse");
        self.svg.write_spread_method_attribute(gradient.extend);
        self.svg
            .write_transform_attribute("gradientTransform", self.transform);
        self.write_gradient_stops(gradient.stops(
            self.palette_index,
            #[cfg(feature = "variable-fonts")]
            self.face.variation_coordinates(),
        ));
        self.svg.end_element();

        self.svg.start_element("path");
        self.svg
            .write_attribute_fmt("fill", format_args!("url(#{})", gradient_id));
        self.svg
            .write_transform_attribute("transform", self.outline_transform);
        self.svg.write_attribute("d", self.path_buf);
        self.svg.end_element();
    }

    fn paint_sweep_gradient(&mut self, _: ttf::colr::SweepGradient<'a>) {
        println!("Warning: sweep gradients are not supported.")
    }
}

fn paint_transform(outline_transform: Transform, transform: Transform) -> Transform {
    let outline_transform = tiny_skia_path::Transform::from_row(
        outline_transform.a,
        outline_transform.b,
        outline_transform.c,
        outline_transform.d,
        outline_transform.e,
        outline_transform.f,
    );

    let gradient_transform = tiny_skia_path::Transform::from_row(
        transform.a,
        transform.b,
        transform.c,
        transform.d,
        transform.e,
        transform.f,
    );

    let gradient_transform = outline_transform
        .invert()
        .unwrap()
        .pre_concat(gradient_transform);

    ttf_parser::Transform {
        a: gradient_transform.sx,
        b: gradient_transform.ky,
        c: gradient_transform.kx,
        d: gradient_transform.sy,
        e: gradient_transform.tx,
        f: gradient_transform.ty,
    }
}

impl GlyphPainter<'_> {
    fn clip_with_path(&mut self, path: &str) {
        let clip_id = format!("cp{}", self.clip_path_index);
        self.clip_path_index += 1;

        self.svg.start_element("clipPath");
        self.svg.write_attribute("id", &clip_id);
        self.svg.start_element("path");
        self.svg
            .write_transform_attribute("transform", self.outline_transform);
        self.svg.write_attribute("d", &path);
        self.svg.end_element();
        self.svg.end_element();

        self.svg.start_element("g");
        self.svg
            .write_attribute_fmt("clip-path", format_args!("url(#{})", clip_id));
    }
}

impl<'a> ttf::colr::Painter<'a> for GlyphPainter<'a> {
    fn outline_glyph(&mut self, glyph_id: ttf::GlyphId) {
        self.path_buf.clear();
        let mut builder = Builder(self.path_buf);
        match self.face.outline_glyph(glyph_id, &mut builder) {
            Some(v) => v,
            None => return,
        };
        builder.finish();

        // We have to write outline using the current transform.
        self.outline_transform = self.transform;
    }

    fn push_layer(&mut self, mode: ttf::colr::CompositeMode) {
        self.svg.start_element("g");

        use ttf::colr::CompositeMode;
        // TODO: Need to figure out how to represent the other blend modes
        // in SVG.
        let mode = match mode {
            CompositeMode::SourceOver => "normal",
            CompositeMode::Screen => "screen",
            CompositeMode::Overlay => "overlay",
            CompositeMode::Darken => "darken",
            CompositeMode::Lighten => "lighten",
            CompositeMode::ColorDodge => "color-dodge",
            CompositeMode::ColorBurn => "color-burn",
            CompositeMode::HardLight => "hard-light",
            CompositeMode::SoftLight => "soft-light",
            CompositeMode::Difference => "difference",
            CompositeMode::Exclusion => "exclusion",
            CompositeMode::Multiply => "multiply",
            CompositeMode::Hue => "hue",
            CompositeMode::Saturation => "saturation",
            CompositeMode::Color => "color",
            CompositeMode::Luminosity => "luminosity",
            _ => {
                println!("Warning: unsupported blend mode: {:?}", mode);
                "normal"
            }
        };
        self.svg.write_attribute_fmt(
            "style",
            format_args!("mix-blend-mode: {}; isolation: isolate", mode),
        );
    }

    fn pop_layer(&mut self) {
        self.svg.end_element(); // g
    }

    fn push_transform(&mut self, transform: ttf::Transform) {
        self.transforms_stack.push(self.transform);
        self.transform = ttf::Transform::combine(self.transform, transform);
    }

    fn paint(&mut self, paint: Paint<'a>) {
        match paint {
            Paint::Solid(color) => self.paint_solid(color),
            Paint::LinearGradient(lg) => self.paint_linear_gradient(lg),
            Paint::RadialGradient(rg) => self.paint_radial_gradient(rg),
            Paint::SweepGradient(sg) => self.paint_sweep_gradient(sg),
        }
    }

    fn pop_transform(&mut self) {
        if let Some(ts) = self.transforms_stack.pop() {
            self.transform = ts
        }
    }

    fn push_clip(&mut self) {
        self.clip_with_path(&self.path_buf.clone());
    }

    fn pop_clip(&mut self) {
        self.svg.end_element();
    }

    fn push_clip_box(&mut self, clipbox: ClipBox) {
        let x_min = clipbox.x_min;
        let x_max = clipbox.x_max;
        let y_min = clipbox.y_min;
        let y_max = clipbox.y_max;

        let clip_path = format!(
            "M {} {} L {} {} L {} {} L {} {} Z",
            x_min, y_min, x_max, y_min, x_max, y_max, x_min, y_max
        );

        self.clip_with_path(&clip_path);
    }
}

fn color_glyph(
    x: f64,
    y: f64,
    face: &ttf::Face,
    palette_index: u16,
    glyph_id: ttf::GlyphId,
    cell_size: f64,
    scale: f64,
    gradient_index: &mut usize,
    clip_path_index: &mut usize,
    svg: &mut xmlwriter::XmlWriter,
    path_buf: &mut String,
) {
    let y = y + cell_size + face.descender() as f64 * scale;
    let transform = format!("matrix({} 0 0 {} {} {})", scale, -scale, x, y);

    svg.start_element("g");
    svg.write_attribute("transform", &transform);

    let mut painter = GlyphPainter {
        face,
        svg,
        path_buf,
        gradient_index: *gradient_index,
        clip_path_index: *clip_path_index,
        palette_index,
        transform: ttf::Transform::default(),
        outline_transform: ttf::Transform::default(),
        transforms_stack: vec![ttf::Transform::default()],
    };
    face.paint_color_glyph(
        glyph_id,
        palette_index,
        RgbaColor::new(0, 0, 0, 255),
        &mut painter,
    );
    *gradient_index = painter.gradient_index;
    *clip_path_index = painter.clip_path_index;

    svg.end_element();
}

trait XmlWriterExt {
    fn write_color_attribute(&mut self, name: &str, ts: ttf::RgbaColor);
    fn write_transform_attribute(&mut self, name: &str, ts: ttf::Transform);
    fn write_spread_method_attribute(&mut self, method: ttf::colr::GradientExtend);
}

impl XmlWriterExt for xmlwriter::XmlWriter {
    fn write_color_attribute(&mut self, name: &str, color: ttf::RgbaColor) {
        self.write_attribute_fmt(
            name,
            format_args!("rgb({}, {}, {})", color.red, color.green, color.blue),
        );
    }

    fn write_transform_attribute(&mut self, name: &str, ts: ttf::Transform) {
        if ts.is_default() {
            return;
        }

        self.write_attribute_fmt(
            name,
            format_args!(
                "matrix({} {} {} {} {} {})",
                ts.a, ts.b, ts.c, ts.d, ts.e, ts.f
            ),
        );
    }

    fn write_spread_method_attribute(&mut self, extend: ttf::colr::GradientExtend) {
        self.write_attribute(
            "spreadMethod",
            match extend {
                ttf::colr::GradientExtend::Pad => &"pad",
                ttf::colr::GradientExtend::Repeat => &"repeat",
                ttf::colr::GradientExtend::Reflect => &"reflect",
            },
        );
    }
}
