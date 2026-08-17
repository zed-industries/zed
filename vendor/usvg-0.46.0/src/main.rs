// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;
use std::sync::Arc;

use pico_args::Arguments;

const HELP: &str = "\
usvg (micro SVG) is an SVG simplification tool.

USAGE:
  usvg [OPTIONS] <in-svg> <out-svg>  # from file to file
  usvg [OPTIONS] <in-svg> -c         # from file to stdout
  usvg [OPTIONS] - <out-svg>         # from stdin to file
  usvg [OPTIONS] - -c                # from stdin to stdout

OPTIONS:
  -h, --help                        Prints help information
  -V, --version                     Prints version information
  -c                                Prints the output SVG to the stdout

  --dpi DPI                         Sets the resolution
                                    [default: 96] [possible values: 10..4000 (inclusive)]
  --stylesheet PATH                 Inject a stylesheet that should be used when resolving
                                    CSS attributes.
  --languages LANG                  Sets a comma-separated list of languages that
                                    will be used during the 'systemLanguage'
                                    attribute resolving
                                    Examples: 'en-US', 'en-US, ru-RU', 'en, ru'
                                    [default: en]
  --shape-rendering HINT            Selects the default shape rendering method
                                    [default: geometricPrecision]
                                    [possible values: optimizeSpeed, crispEdges,
                                    geometricPrecision]
  --text-rendering HINT             Selects the default text rendering method
                                    [default: optimizeLegibility]
                                    [possible values: optimizeSpeed, optimizeLegibility,
                                    geometricPrecision]
  --image-rendering HINT            Selects the default image rendering method
                                    [default: optimizeQuality]
                                    [possible values: optimizeQuality, optimizeSpeed, smooth, high-quality, crisp-edges, pixelated]
  --resources-dir DIR               Sets a directory that will be used during
                                    relative paths resolving.
                                    Expected to be the same as the directory that
                                    contains the SVG file, but can be set to any.
                                    [default: input file directory
                                    or none when reading from stdin]

  --font-family FAMILY              Sets the default font family that will be
                                    used when no 'font-family' is present
                                    [default: Times New Roman]
  --font-size SIZE                  Sets the default font size that will be
                                    used when no 'font-size' is present
                                    [default: 12] [possible values: 1..192 (inclusive)]
  --serif-family FAMILY             Sets the 'serif' font family.
                                    Will be used when no 'font-family' is present
                                    [default: Times New Roman]
  --sans-serif-family FAMILY        Sets the 'sans-serif' font family
                                    [default: Arial]
  --cursive-family FAMILY           Sets the 'cursive' font family
                                    [default: Comic Sans MS]
  --fantasy-family FAMILY           Sets the 'fantasy' font family
                                    [default: Impact]
  --monospace-family FAMILY         Sets the 'monospace' font family
                                    [default: Courier New]
  --use-font-file PATH              Load a specified font file into the fonts database.
                                    Will be used during text to path conversion.
                                    This option can be set multiple times
  --use-fonts-dir PATH              Loads all fonts from the specified directory
                                    into the fonts database.
                                    Will be used during text to path conversion.
                                    This option can be set multiple times
  --skip-system-fonts               Disables system fonts loading.
                                    You should add some fonts manually using
                                    --use-font-file and/or --use-fonts-dir
                                    Otherwise, text elements will not be processes
  --list-fonts                      Lists successfully loaded font faces.
                                    Useful for debugging
  --default-width LENGTH            Sets the default width of the SVG viewport. Like
                                    the '--default-height' option, this option
                                    controls what size relative units in the document
                                    will use as a base if there is no viewBox and
                                    document width or height are relative.
                                    [values: 1..4294967295 (inclusive)] [default: 100]
  --default-height LENGTH           Sets the default height of the SVG viewport.
                                    Refer to the explanation of the '--default-width'
                                    option. [values: 1..4294967295 (inclusive)] [default: 100]

  --preserve-text                   Do not convert text into paths.
  --id-prefix                       Adds a prefix to each ID attribute
  --indent INDENT                   Sets the XML nodes indent
                                    [values: none, 0, 1, 2, 3, 4, tabs] [default: 4]
  --attrs-indent INDENT             Sets the XML attributes indent
                                    [values: none, 0, 1, 2, 3, 4, tabs] [default: none]
  --coordinates-precision NUM       Set the coordinates numeric precision.
                                    Smaller precision can lead to a malformed output in some cases
                                    [values: 2..8 (inclusive)] [default: 8]
  --transforms-precision NUM        Set the transform values numeric precision.
                                    Smaller precision can lead to a malformed output in some cases
                                    [values: 2..8 (inclusive)] [default: 8]
  --quiet                           Disables warnings

ARGS:
  <in-svg>                          Input file
  <out-svg>                         Output file
";

#[derive(Debug)]
struct Args {
    dpi: u32,
    languages: Vec<String>,
    shape_rendering: usvg::ShapeRendering,
    text_rendering: usvg::TextRendering,
    image_rendering: usvg::ImageRendering,
    resources_dir: Option<PathBuf>,

    font_family: Option<String>,
    font_size: u32,
    serif_family: Option<String>,
    sans_serif_family: Option<String>,
    cursive_family: Option<String>,
    fantasy_family: Option<String>,
    monospace_family: Option<String>,
    font_files: Vec<PathBuf>,
    font_dirs: Vec<PathBuf>,
    skip_system_fonts: bool,
    preserve_text: bool,
    list_fonts: bool,
    default_width: u32,
    default_height: u32,

    id_prefix: Option<String>,
    indent: xmlwriter::Indent,
    attrs_indent: xmlwriter::Indent,
    coordinates_precision: Option<u8>,
    transforms_precision: Option<u8>,
    style_sheet: Option<PathBuf>,

    quiet: bool,

    input: String,
    output: String,
}

fn collect_args() -> Result<Args, pico_args::Error> {
    let mut input = Arguments::from_env();

    if input.contains(["-h", "--help"]) {
        print!("{}", HELP);
        process::exit(0);
    }

    if input.contains(["-V", "--version"]) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    Ok(Args {
        dpi: input.opt_value_from_fn("--dpi", parse_dpi)?.unwrap_or(96),
        languages: input
            .opt_value_from_fn("--languages", parse_languages)?
            .unwrap_or(vec!["en".to_string()]), // TODO: use system language
        shape_rendering: input
            .opt_value_from_str("--shape-rendering")?
            .unwrap_or_default(),
        text_rendering: input
            .opt_value_from_str("--text-rendering")?
            .unwrap_or_default(),
        image_rendering: input
            .opt_value_from_str("--image-rendering")?
            .unwrap_or_default(),
        resources_dir: input
            .opt_value_from_str("--resources-dir")
            .unwrap_or_default(),

        font_family: input.opt_value_from_str("--font-family")?,
        font_size: input
            .opt_value_from_fn("--font-size", parse_font_size)?
            .unwrap_or(12),
        serif_family: input.opt_value_from_str("--serif-family")?,
        sans_serif_family: input.opt_value_from_str("--sans-serif-family")?,
        cursive_family: input.opt_value_from_str("--cursive-family")?,
        fantasy_family: input.opt_value_from_str("--fantasy-family")?,
        monospace_family: input.opt_value_from_str("--monospace-family")?,
        font_files: input.values_from_str("--use-font-file")?,
        font_dirs: input.values_from_str("--use-fonts-dir")?,
        skip_system_fonts: input.contains("--skip-system-fonts"),
        preserve_text: input.contains("--preserve-text"),
        list_fonts: input.contains("--list-fonts"),
        default_width: input
            .opt_value_from_fn("--default-width", parse_length)?
            .unwrap_or(100),
        default_height: input
            .opt_value_from_fn("--default-height", parse_length)?
            .unwrap_or(100),

        id_prefix: input.opt_value_from_str("--id-prefix")?,
        indent: input
            .opt_value_from_fn("--indent", parse_indent)?
            .unwrap_or(xmlwriter::Indent::Spaces(4)),
        attrs_indent: input
            .opt_value_from_fn("--attrs-indent", parse_indent)?
            .unwrap_or(xmlwriter::Indent::None),
        coordinates_precision: input
            .opt_value_from_fn("--coordinates-precision", parse_precision)?,
        transforms_precision: input.opt_value_from_fn("--transforms-precision", parse_precision)?,
        style_sheet: input.opt_value_from_str("--stylesheet").unwrap_or_default(),

        quiet: input.contains("--quiet"),

        input: input.free_from_str()?,
        output: input.free_from_str()?,
    })
}

fn parse_dpi(s: &str) -> Result<u32, String> {
    let n: u32 = s.parse().map_err(|_| "invalid number")?;

    if n >= 10 && n <= 4000 {
        Ok(n)
    } else {
        Err("DPI out of bounds".to_string())
    }
}

fn parse_font_size(s: &str) -> Result<u32, String> {
    let n: u32 = s.parse().map_err(|_| "invalid number")?;

    if n > 0 && n <= 192 {
        Ok(n)
    } else {
        Err("font size out of bounds".to_string())
    }
}

fn parse_languages(s: &str) -> Result<Vec<String>, String> {
    let mut langs = Vec::new();
    for lang in s.split(',') {
        langs.push(lang.trim().to_string());
    }

    if langs.is_empty() {
        return Err("languages list cannot be empty".to_string());
    }

    Ok(langs)
}

fn parse_indent(s: &str) -> Result<xmlwriter::Indent, String> {
    let indent = match s {
        "none" => xmlwriter::Indent::None,
        "0" => xmlwriter::Indent::Spaces(0),
        "1" => xmlwriter::Indent::Spaces(1),
        "2" => xmlwriter::Indent::Spaces(2),
        "3" => xmlwriter::Indent::Spaces(3),
        "4" => xmlwriter::Indent::Spaces(4),
        "tabs" => xmlwriter::Indent::Tabs,
        _ => return Err("invalid INDENT value".to_string()),
    };

    Ok(indent)
}

fn parse_length(s: &str) -> Result<u32, String> {
    let n: u32 = s.parse().map_err(|_| "invalid length")?;

    if n > 0 {
        Ok(n)
    } else {
        Err("LENGTH cannot be zero".to_string())
    }
}

fn parse_precision(s: &str) -> Result<u8, String> {
    let n: u8 = s.parse().map_err(|_| "invalid precision NUM value")?;

    if (2..=8).contains(&n) {
        Ok(n)
    } else {
        Err("precision NUM cannot be smaller than 2 or larger than 8".to_string())
    }
}

#[derive(Clone, PartialEq, Debug)]
enum InputFrom<'a> {
    Stdin,
    File(&'a str),
}

#[derive(Clone, PartialEq, Debug)]
enum OutputTo<'a> {
    Stdout,
    File(&'a str),
}

fn main() {
    let args = match collect_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            process::exit(1);
        }
    };

    if !args.quiet {
        if let Ok(()) = log::set_logger(&LOGGER) {
            log::set_max_level(log::LevelFilter::Warn);
        }
    }

    if let Err(e) = process(args) {
        eprintln!("Error: {}.", e);
        process::exit(1);
    }
}

fn process(args: Args) -> Result<(), String> {
    let (in_svg, out_svg) = {
        let in_svg = args.input.as_str();
        let out_svg = args.output.as_str();

        let svg_from = if in_svg == "-" {
            InputFrom::Stdin
        } else if in_svg == "-c" {
            return Err("-c should be set after input".to_string());
        } else {
            InputFrom::File(in_svg)
        };

        let svg_to = if out_svg == "-c" {
            OutputTo::Stdout
        } else {
            OutputTo::File(out_svg)
        };

        (svg_from, svg_to)
    };

    let mut fontdb = usvg::fontdb::Database::new();
    if !args.skip_system_fonts {
        // TODO: only when needed
        fontdb.load_system_fonts();
    }

    for path in &args.font_files {
        if let Err(e) = fontdb.load_font_file(path) {
            log::warn!("Failed to load '{}' cause {}.", path.display(), e);
        }
    }

    for path in &args.font_dirs {
        fontdb.load_fonts_dir(path);
    }

    let take_or = |mut family: Option<String>, fallback: &str| {
        family.take().unwrap_or_else(|| fallback.to_string())
    };

    fontdb.set_serif_family(take_or(args.serif_family, "Times New Roman"));
    fontdb.set_sans_serif_family(take_or(args.sans_serif_family, "Arial"));
    fontdb.set_cursive_family(take_or(args.cursive_family, "Comic Sans MS"));
    fontdb.set_fantasy_family(take_or(args.fantasy_family, "Impact"));
    fontdb.set_monospace_family(take_or(args.monospace_family, "Courier New"));

    if args.list_fonts {
        for face in fontdb.faces() {
            if let usvg::fontdb::Source::File(path) = &face.source {
                let families: Vec<_> = face
                    .families
                    .iter()
                    .map(|f| format!("{} ({}, {})", f.0, f.1.primary_language(), f.1.region()))
                    .collect();

                println!(
                    "{}: '{}', {}, {:?}, {:?}, {:?}",
                    path.display(),
                    families.join("', '"),
                    face.index,
                    face.style,
                    face.weight.0,
                    face.stretch
                );
            }
        }
    }

    let resources_dir = match args.resources_dir {
        Some(v) => Some(v),
        None => {
            match in_svg {
                InputFrom::Stdin => None,
                InputFrom::File(ref f) => {
                    // Get input file absolute directory.
                    std::fs::canonicalize(f)
                        .ok()
                        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                }
            }
        }
    };

    let style_sheet = match args.style_sheet.as_ref() {
        Some(p) => Some(
            std::fs::read(&p)
                .ok()
                .and_then(|s| std::str::from_utf8(&s).ok().map(|s| s.to_string()))
                .ok_or("failed to read stylesheet".to_string())?,
        ),
        None => None,
    };

    let re_opt = usvg::Options {
        resources_dir,
        dpi: args.dpi as f32,
        font_family: args
            .font_family
            .as_deref()
            .unwrap_or("Times New Roman")
            .to_string(),
        font_size: args.font_size as f32,
        languages: args.languages,
        shape_rendering: args.shape_rendering,
        text_rendering: args.text_rendering,
        image_rendering: args.image_rendering,
        default_size: usvg::Size::from_wh(args.default_width as f32, args.default_height as f32)
            .unwrap(),
        image_href_resolver: usvg::ImageHrefResolver::default(),
        font_resolver: usvg::FontResolver::default(),
        fontdb: Arc::new(fontdb),
        style_sheet,
    };

    let input_svg = match in_svg {
        InputFrom::Stdin => load_stdin(),
        InputFrom::File(ref path) => std::fs::read(path).map_err(|e| e.to_string()),
    }?;

    let tree = usvg::Tree::from_data(&input_svg, &re_opt).map_err(|e| format!("{}", e))?;

    let xml_opt = usvg::WriteOptions {
        id_prefix: args.id_prefix,
        preserve_text: args.preserve_text,
        coordinates_precision: args.coordinates_precision.unwrap_or(8),
        transforms_precision: args.transforms_precision.unwrap_or(8),
        use_single_quote: false,
        indent: args.indent,
        attributes_indent: args.attrs_indent,
    };

    let s = tree.to_string(&xml_opt);
    match out_svg {
        OutputTo::Stdout => {
            io::stdout()
                .write_all(s.as_bytes())
                .map_err(|_| "failed to write to the stdout".to_string())?;
        }
        OutputTo::File(path) => {
            let mut f =
                File::create(path).map_err(|_| "failed to create the output file".to_string())?;
            f.write_all(s.as_bytes())
                .map_err(|_| "failed to write to the output file".to_string())?;
        }
    }

    Ok(())
}

fn load_stdin() -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_to_end(&mut buf)
        .map_err(|_| "failed to read from stdin".to_string())?;

    Ok(buf)
}

/// A simple stderr logger.
static LOGGER: SimpleLogger = SimpleLogger;
struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::LevelFilter::Warn
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let target = if record.target().len() > 0 {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            let line = record.line().unwrap_or(0);
            let args = record.args();

            match record.level() {
                log::Level::Error => eprintln!("Error (in {}:{}): {}", target, line, args),
                log::Level::Warn => eprintln!("Warning (in {}:{}): {}", target, line, args),
                log::Level::Info => eprintln!("Info (in {}:{}): {}", target, line, args),
                log::Level::Debug => eprintln!("Debug (in {}:{}): {}", target, line, args),
                log::Level::Trace => eprintln!("Trace (in {}:{}): {}", target, line, args),
            }
        }
    }

    fn flush(&self) {}
}
