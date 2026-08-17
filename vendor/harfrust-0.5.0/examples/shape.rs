use std::path::PathBuf;
use std::str::FromStr;

use harfrust::{FontRef, ShaperData, ShaperInstance};

const HELP: &str = "\
USAGE:
    shape [OPTIONS] <FONT-FILE> [TEXT]

OPTIONS:
    -h, --help                                  Show help options
        --version                               Show version number
        --font-file PATH                        Set font file-name
        --face-index INDEX                      Set face index [default: 0]
        --font-ptem NUMBER                      Set font point-size
        --variations LIST                       Set comma-separated list of font variations
        --text TEXT                             Set input text
        --text-file PATH                        Set input text file
    -u, --unicodes LIST                         Set comma-separated list of input Unicode codepoints
                                                Examples: 'U+0056,U+0057'
        --direction DIRECTION                   Set text direction
                                                [possible values: ltr, rtl, ttb, btt]
        --language LANG                         Set text language [default: LC_CTYPE]
        --script TAG                            Set text script as ISO-15924 tag
        --not-found-variation-selector-glyph N  Glyph value to replace not-found variation-selector characters with
        --utf8-clusters                         Use UTF-8 byte indices, not char indices
        --cluster-level N                       Cluster merging level [default: 0]
                                                [possible values: 0, 1, 2]
        --features LIST                         Set comma-separated list of font features
        --no-glyph-names                        Output glyph indices instead of names
        --no-positions                          Do not output glyph positions
        --no-advances                           Do not output glyph advances
        --no-clusters                           Do not output cluster indices
        --show-extents                          Output glyph extents
        --show-flags                            Output glyph flags
        --single-par                            Treat the input string as a single paragraph
        --ned                                   No Extra Data; Do not output clusters or advances

ARGS:
    <FONT-FILE>                         A font file
    [TEXT]                              An optional text
";

struct Args {
    help: bool,
    version: bool,
    font_file: Option<PathBuf>,
    face_index: u32,
    font_ptem: Option<f32>,
    variations: Vec<harfrust::Variation>,
    text: Option<String>,
    text_file: Option<PathBuf>,
    unicodes: Option<String>,
    direction: Option<harfrust::Direction>,
    language: harfrust::Language,
    script: Option<harfrust::Script>,
    not_found_variation_selector_glyph: Option<u32>,
    utf8_clusters: bool,
    cluster_level: harfrust::BufferClusterLevel,
    features: Vec<harfrust::Feature>,
    no_glyph_names: bool,
    no_positions: bool,
    no_advances: bool,
    no_clusters: bool,
    show_extents: bool,
    show_flags: bool,
    single_par: bool,
    ned: bool,
    free: Vec<String>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        help: args.contains(["-h", "--help"]),
        version: args.contains("--version"),
        font_file: args.opt_value_from_str("--font-file")?,
        face_index: args.opt_value_from_str("--face-index")?.unwrap_or(0),
        font_ptem: args.opt_value_from_str("--font-ptem")?,
        variations: args
            .opt_value_from_fn("--variations", parse_variations)?
            .unwrap_or_default(),
        text: args.opt_value_from_str("--text")?,
        text_file: args.opt_value_from_str("--text-file")?,
        unicodes: args.opt_value_from_fn(["-u", "--unicodes"], parse_unicodes)?,
        direction: args.opt_value_from_str("--direction")?,
        language: args
            .opt_value_from_str("--language")?
            .unwrap_or(system_language()),
        script: args.opt_value_from_str("--script")?,
        utf8_clusters: args.contains("--utf8-clusters"),
        not_found_variation_selector_glyph: args
            .opt_value_from_str("--not-found-variation-selector-glyph")?,
        cluster_level: args
            .opt_value_from_fn("--cluster-level", parse_cluster)?
            .unwrap_or_default(),
        features: args
            .opt_value_from_fn("--features", parse_features)?
            .unwrap_or_default(),
        no_glyph_names: args.contains("--no-glyph-names"),
        no_positions: args.contains("--no-positions"),
        no_advances: args.contains("--no-advances"),
        no_clusters: args.contains("--no-clusters"),
        show_extents: args.contains("--show-extents"),
        show_flags: args.contains("--show-flags"),
        single_par: args.contains("--single-par"),
        ned: args.contains("--ned"),
        free: args
            .finish()
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect(),
    };

    Ok(args)
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}.");
            std::process::exit(1);
        }
    };

    if args.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.help {
        print!("{HELP}");
        return;
    }

    let mut font_set_as_free_arg = false;
    let font_path = if let Some(path) = args.font_file {
        path.clone()
    } else if !args.free.is_empty() {
        font_set_as_free_arg = true;
        PathBuf::from(&args.free[0])
    } else {
        eprintln!("Error: font is not set.");
        std::process::exit(1);
    };

    if !font_path.exists() {
        eprintln!("Error: '{}' does not exist.", font_path.display());
        std::process::exit(1);
    }

    let font_data = std::fs::read(font_path).unwrap();
    let font = FontRef::from_index(&font_data, args.face_index).unwrap();
    let data = ShaperData::new(&font);
    let instance = ShaperInstance::from_variations(&font, &args.variations);
    let shaper = data
        .shaper(&font)
        .instance(Some(&instance))
        .point_size(args.font_ptem)
        .build();

    let text = if let Some(path) = args.text_file {
        std::fs::read_to_string(path).unwrap()
    } else if args.free.len() == 2 && font_set_as_free_arg {
        args.free[1].clone()
    } else if args.free.len() == 1 && !font_set_as_free_arg {
        args.free[0].clone()
    } else if let Some(ref text) = args.unicodes {
        text.clone()
    } else if let Some(ref text) = args.text {
        text.clone()
    } else {
        eprintln!("Error: text is not set.");
        std::process::exit(1);
    };

    let lines = if args.single_par {
        vec![text.as_str()]
    } else {
        text.split('\n').filter(|s| !s.is_empty()).collect()
    };

    for text in lines {
        let mut buffer = harfrust::UnicodeBuffer::new();
        buffer.push_str(text);

        if let Some(d) = args.direction {
            buffer.set_direction(d);
        }

        buffer.set_language(args.language.clone());

        if let Some(script) = args.script {
            buffer.set_script(script);
        }

        buffer.set_cluster_level(args.cluster_level);

        if !args.utf8_clusters {
            buffer.reset_clusters();
        }

        if let Some(g) = args.not_found_variation_selector_glyph {
            buffer.set_not_found_variation_selector_glyph(g);
        }

        buffer.guess_segment_properties();

        let glyph_buffer = shaper.shape(buffer, &args.features);

        let mut format_flags = harfrust::SerializeFlags::default();
        if args.no_glyph_names {
            format_flags |= harfrust::SerializeFlags::NO_GLYPH_NAMES;
        }

        if args.no_clusters || args.ned {
            format_flags |= harfrust::SerializeFlags::NO_CLUSTERS;
        }

        if args.no_positions {
            format_flags |= harfrust::SerializeFlags::NO_POSITIONS;
        }

        if args.no_advances || args.ned {
            format_flags |= harfrust::SerializeFlags::NO_ADVANCES;
        }

        if args.show_extents {
            format_flags |= harfrust::SerializeFlags::GLYPH_EXTENTS;
        }

        if args.show_flags {
            format_flags |= harfrust::SerializeFlags::GLYPH_FLAGS;
        }

        println!("{}", glyph_buffer.serialize(&shaper, format_flags));
    }
}

fn parse_unicodes(s: &str) -> Result<String, String> {
    let mut text = String::new();
    for u in s.split(',') {
        let u = u32::from_str_radix(&u[2..], 16)
            .map_err(|_| format!("'{u}' is not a valid codepoint"))?;

        let c = char::try_from(u).map_err(|_| format!("{u} is not a valid codepoint"))?;

        text.push(c);
    }

    Ok(text)
}

fn parse_features(s: &str) -> Result<Vec<harfrust::Feature>, String> {
    let mut features = Vec::new();
    for f in s.split(',') {
        features.push(harfrust::Feature::from_str(f)?);
    }

    Ok(features)
}

fn parse_variations(s: &str) -> Result<Vec<harfrust::Variation>, String> {
    let mut variations = Vec::new();
    for v in s.split(',') {
        variations.push(harfrust::Variation::from_str(v)?);
    }

    Ok(variations)
}

fn parse_cluster(s: &str) -> Result<harfrust::BufferClusterLevel, String> {
    match s {
        "0" => Ok(harfrust::BufferClusterLevel::MonotoneGraphemes),
        "1" => Ok(harfrust::BufferClusterLevel::MonotoneCharacters),
        "2" => Ok(harfrust::BufferClusterLevel::Characters),
        _ => Err("invalid cluster level".to_string()),
    }
}

fn system_language() -> harfrust::Language {
    unsafe {
        libc::setlocale(libc::LC_ALL, c"".as_ptr());
        let s = libc::setlocale(libc::LC_CTYPE, std::ptr::null());
        let s = std::ffi::CStr::from_ptr(s);
        let s = s.to_str().expect("locale must be ASCII");
        harfrust::Language::from_str(s).unwrap()
    }
}
