use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use anyhow::Result;
use av_scenechange::{
    decoder::Decoder,
    detect_scene_changes,
    DetectionOptions,
    SceneDetectionSpeed,
};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Sets the input file to use
    #[clap(value_parser)]
    pub input: String,

    /// Optional file to write results to
    #[clap(long, short, value_parser)]
    pub output: Option<String>,

    /// Speed level for scene-change detection, 0: best quality, 1: fastest mode
    #[clap(long, short, value_parser, default_value_t = 0)]
    pub speed: u8,

    /// Do not detect short scene flashes and exclude them as scene cuts
    #[clap(long)]
    pub no_flash_detection: bool,

    /// Sets a minimum interval between two consecutive scenecuts
    #[clap(long, value_parser)]
    pub min_scenecut: Option<usize>,

    /// Sets a maximum interval between two consecutive scenecuts,
    /// after which a scenecut will be forced
    #[clap(long, value_parser)]
    pub max_scenecut: Option<usize>,
}

fn main() -> Result<()> {
    init_logger();

    #[cfg(feature = "tracing")]
    let (chrome_layer, _guard) = tracing_chrome::ChromeLayerBuilder::new().build();

    #[cfg(feature = "tracing")]
    {
        use tracing_subscriber::layer::SubscriberExt;
        tracing::subscriber::set_global_default(tracing_subscriber::registry().with(chrome_layer))
            .expect("Could not initialize tracing subscriber");
    }

    let matches = Args::parse();
    let input = match matches.input.as_str() {
        "-" => Box::new(io::stdin()) as Box<dyn Read>,
        f => Box::new(File::open(f)?) as Box<dyn Read>,
    };
    let mut reader = BufReader::new(input);

    let mut opts = DetectionOptions {
        detect_flashes: !matches.no_flash_detection,
        min_scenecut_distance: matches.min_scenecut,
        max_scenecut_distance: matches.max_scenecut,
        ..DetectionOptions::default()
    };

    opts.analysis_speed = match matches.speed {
        0 => SceneDetectionSpeed::Standard,
        1 => SceneDetectionSpeed::Fast,
        _ => panic!("Speed mode must be in range [0; 1]"),
    };

    let mut dec = Decoder::Y4m(y4m::Decoder::new(&mut reader)?);
    let bit_depth = dec.get_video_details()?.bit_depth;
    let results = if bit_depth == 8 {
        detect_scene_changes::<_, u8>(&mut dec, opts, None, None)?
    } else {
        detect_scene_changes::<_, u16>(&mut dec, opts, None, None)?
    };
    print!("{}", serde_json::to_string(&results)?);

    if let Some(output_file) = matches.output {
        let mut file = File::create(output_file)?;

        let output = serde_json::to_string_pretty(&results)?;
        file.write_all(&output.into_bytes())?;
    }

    Ok(())
}

#[cfg(not(feature = "devel"))]
const fn init_logger() {
    // Do nothing
}

#[cfg(feature = "devel")]
fn init_logger() {
    use std::str::FromStr;
    fn level_colored(l: log::Level) -> console::StyledObject<&'static str> {
        use console::style;
        use log::Level;
        match l {
            Level::Trace => style("??").dim(),
            Level::Debug => style("? ").dim(),
            Level::Info => style("> ").green(),
            Level::Warn => style("! ").yellow(),
            Level::Error => style("!!").red(),
        }
    }

    let level = std::env::var("LOG")
        .ok()
        .and_then(|l| log::LevelFilter::from_str(&l).ok())
        .unwrap_or(log::LevelFilter::Warn);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{level} {message}",
                level = level_colored(record.level()),
                message = message,
            ));
        })
        // set the default log level. to filter out verbose log messages from dependencies, set
        // this to Warn and overwrite the log level for your crate.
        .level(level)
        // output to stdout
        .chain(std::io::stderr())
        .apply()
        .unwrap();
}
