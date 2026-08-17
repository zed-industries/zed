// Copyright (c) 2017-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::error::*;
use crate::muxer::{create_muxer, Muxer};
use crate::stats::MetricsEnabled;
use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::{CommandFactory, Parser as Clap, Subcommand};
use clap_complete::{generate, Shell};
use rav1e::prelude::*;
use scan_fmt::scan_fmt;

use rav1e::config::CpuFeatureLevel;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::OnceLock;

pub mod built_info {
  // The file has been placed there by the build script.
  include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Clap)]
#[clap(
  name = "rav1e",
  styles = get_styles(),
  version = get_version(),
  long_version = get_long_version(),
  about = "AV1 video encoder",
  long_about = None
)]
pub struct CliOptions {
  /// Uncompressed YUV4MPEG2 video input
  #[clap(value_parser, help_heading = "INPUT/OUTPUT")]
  pub input: PathBuf,
  /// Compressed AV1 in IVF video output
  #[clap(long, short, value_parser, help_heading = "INPUT/OUTPUT")]
  pub output: PathBuf,
  /// Overwrite output file.
  #[clap(short = 'y', help_heading = "INPUT/OUTPUT")]
  pub overwrite: bool,

  /// Set the threadpool size. If 0, will use the number of logical CPUs.
  /// rav1e will use up to this many threads. Additional tiles may be needed to
  /// increase thread utilization.
  #[clap(long, value_parser, default_value_t = 0, help_heading = "THREADING")]
  pub threads: usize,
  /// Number of tile rows. Must be a power of 2. rav1e may override this based on video resolution.
  #[clap(long, value_parser, default_value_t = 0, help_heading = "THREADING")]
  pub tile_rows: usize,
  /// Number of tile columns. Must be a power of 2. rav1e may override this based on video resolution.
  #[clap(long, value_parser, default_value_t = 0, help_heading = "THREADING")]
  pub tile_cols: usize,
  /// Number of tiles. Tile-cols and tile-rows are overridden
  /// so that the video has at least this many tiles.
  #[clap(
    long,
    value_parser,
    conflicts_with = "tile_rows",
    conflicts_with = "tile_cols",
    help_heading = "THREADING"
  )]
  pub tiles: Option<usize>,
  /// Maximum number of GOPs that can be encoded in parallel
  #[cfg(feature = "unstable")]
  #[clap(long, value_parser, default_value_t = 0, help_heading = "THREADING")]
  pub slots: usize,

  /// Perform the first pass of a two-pass encode,
  /// saving the pass data to the specified file for future passes
  #[clap(
    long,
    value_parser,
    value_name = "STATS_FILE",
    help_heading = "ENCODE SETTINGS"
  )]
  pub first_pass: Option<PathBuf>,
  /// Perform the second pass of a two-pass encode,
  /// reading the pass data saved from a previous pass from the specified file
  #[clap(
    long,
    value_parser,
    value_name = "STATS_FILE",
    help_heading = "ENCODE SETTINGS"
  )]
  pub second_pass: Option<PathBuf>,
  /// Maximum number of frames to encode
  #[clap(
    long,
    short,
    value_parser,
    default_value_t = 0,
    help_heading = "ENCODE SETTINGS"
  )]
  pub limit: usize,
  /// Skip n number of frames and encode
  #[clap(
    long,
    value_parser,
    default_value_t = 0,
    help_heading = "ENCODE SETTINGS"
  )]
  pub skip: usize,
  /// Quantizer (0-255), smaller values are higher quality [default: 100]
  #[clap(long, value_parser, help_heading = "ENCODE SETTINGS")]
  pub quantizer: Option<u8>,
  /// Minimum quantizer (0-255) to use in bitrate mode [default: 0]
  #[clap(long, value_parser, help_heading = "ENCODE SETTINGS")]
  pub min_quantizer: Option<u8>,
  /// Bitrate (kbps)
  #[clap(long, short, value_parser, help_heading = "ENCODE SETTINGS")]
  pub bitrate: Option<i32>,
  /// Speed level (0 is best quality, 10 is fastest).
  /// Speeds 10 and 0 are extremes and are generally not recommended.
  #[clap(long, short, value_parser = clap::value_parser!(u8).range(0..=10), default_value_t = 6, help_heading = "ENCODE SETTINGS", long_help = build_speed_long_help())]
  pub speed: u8,
  /// Speed level for scene-change detection, 0: best quality, 1: fastest mode.
  /// [default: 0 for s0-s9, 1 for s10]
  #[clap(long, value_parser = clap::value_parser!(u8).range(0..=1), help_heading = "ENCODE SETTINGS")]
  pub scd_speed: Option<u8>,
  /// Minimum interval between keyframes
  #[clap(
    long,
    short = 'i',
    value_parser,
    default_value_t = 12,
    help_heading = "ENCODE SETTINGS"
  )]
  pub min_keyint: u64,
  /// Maximum interval between keyframes. When set to 0, disables fixed-interval keyframes.
  #[clap(
    long,
    short = 'I',
    value_parser,
    default_value_t = 240,
    help_heading = "ENCODE SETTINGS"
  )]
  pub keyint: u64,
  /// Maximum interval between switch frames. When set to 0, disables switch frames.
  #[clap(
    long,
    short = 'S',
    value_parser,
    default_value_t = 0,
    help_heading = "ENCODE SETTINGS"
  )]
  pub switch_frame_interval: u64,
  /// "Number of frames over which rate control should distribute the reservoir
  /// [default: min(240, 1.5x keyint)]
  /// A minimum value of 12 is enforced.
  #[clap(long, value_parser = clap::value_parser!(i32).range(12..), help_heading = "ENCODE SETTINGS")]
  pub reservoir_frame_delay: Option<i32>,
  /// Low latency mode; disables frame reordering.
  /// Has a significant speed-to-quality trade-off
  #[clap(long, help_heading = "ENCODE SETTINGS")]
  pub low_latency: bool,
  /// Disables scene detection entirely.
  /// Has a significant speed-to-quality trade-off in full encodes.
  /// Useful for chunked encoding.
  #[clap(long, help_heading = "ENCODE SETTINGS")]
  pub no_scene_detection: bool,
  /// Number of frames encoder should lookahead for RDO purposes\n\
  /// [default value for speed levels: 10,9 - 10; 8,7,6 - 20; 5,4,3 - 30; 2,1,0 - 40]
  #[clap(long, value_parser, help_heading = "ENCODE SETTINGS")]
  pub rdo_lookahead_frames: Option<usize>,
  /// Quality tuning
  #[clap(long, value_parser, default_value_t = Tune::Psychovisual, help_heading = "ENCODE SETTINGS")]
  pub tune: Tune,
  /// Still picture mode
  #[clap(long, help_heading = "ENCODE SETTINGS")]
  pub still_picture: bool,
  /// Uses grain synthesis to add photon noise to the resulting encode.
  /// Takes a strength value 0-64.
  #[clap(
    long,
    conflicts_with = "film_grain_table",
    value_parser = clap::value_parser!(u8).range(0..=64),
    default_value_t = 0,
    help_heading = "ENCODE SETTINGS"
  )]
  pub photon_noise: u8,
  /// Uses a film grain table file to apply grain synthesis to the encode.
  /// Uses the same table file format as aomenc and svt-av1.
  #[clap(
    long,
    alias = "photon-noise-table",
    value_parser,
    help_heading = "ENCODE SETTINGS"
  )]
  pub film_grain_table: Option<PathBuf>,
  /// Force the high bitdepth codepath even for 8bit content.
  /// Mainly for debugging purposes.
  #[clap(long, help_heading = "ENCODE SETTINGS")]
  pub high_bitdepth: bool,

  /// Pixel range
  #[clap(long, value_parser, help_heading = "VIDEO METADATA")]
  pub range: Option<PixelRange>,
  /// Color primaries used to describe color parameters
  #[clap(long, value_parser, help_heading = "VIDEO METADATA")]
  pub primaries: Option<ColorPrimaries>,
  /// Transfer characteristics used to describe color parameters
  #[clap(long, value_parser, help_heading = "VIDEO METADATA")]
  pub transfer: Option<TransferCharacteristics>,
  /// Matrix coefficients used to describe color parameters
  #[clap(long, value_parser, help_heading = "VIDEO METADATA")]
  pub matrix: Option<MatrixCoefficients>,
  /// Mastering display primaries in the form of G(x,y)B(x,y)R(x,y)WP(x,y)L(max,min)
  #[clap(long, help_heading = "VIDEO METADATA")]
  pub mastering_display: Option<String>,
  /// Content light level used to describe content luminosity (cll,fall)
  #[clap(long, help_heading = "VIDEO METADATA")]
  pub content_light: Option<String>,
  /// AV1 level to target in the form <major>.<minor>, e.g. 3.1.
  /// Specify "unconstrained" for no level constraints or "auto" to let
  /// the encoder choose (default)
  #[clap(long, help_heading = "LEVEL")]
  pub level: Option<String>,
  /// Constant frame rate to set at the output (inferred from input when omitted)
  #[clap(long, value_parser, help_heading = "VIDEO METADATA")]
  pub frame_rate: Option<u64>,
  /// The time scale associated with the frame rate if provided (ignored otherwise)
  #[clap(
    long,
    value_parser,
    default_value_t = 0,
    help_heading = "VIDEO METADATA"
  )]
  pub time_scale: u64,

  /// Provide a benchmark report at the end of the encoding
  #[clap(long, help_heading = "DEBUGGING")]
  pub benchmark: bool,
  /// Verbose logging; outputs info for every frame
  #[clap(long, short, help_heading = "DEBUGGING")]
  pub verbose: bool,
  /// Do not output any status message
  #[clap(long, short, conflicts_with = "verbose", help_heading = "DEBUGGING")]
  pub quiet: bool,
  /// Calculate and display PSNR metrics
  #[clap(long, help_heading = "DEBUGGING")]
  pub psnr: bool,
  /// Calculate and display several metrics including PSNR, SSIM, CIEDE2000 etc
  #[clap(long, conflicts_with = "psnr", help_heading = "DEBUGGING")]
  pub metrics: bool,
  /// Outputs a Y4M file containing the output from the decoder
  #[clap(long, short, value_parser, help_heading = "DEBUGGING")]
  pub reconstruction: Option<PathBuf>,

  #[clap(subcommand)]
  pub command: Option<Commands>,
}

static VERSION_STR: OnceLock<String> = OnceLock::new();
static LONG_VERSION_STR: OnceLock<String> = OnceLock::new();

fn get_styles() -> Styles {
  Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default())
}

fn get_version() -> &'static str {
  VERSION_STR.get_or_init(|| {
    format!(
      "{} ({})",
      rav1e::version::full(),
      // We cannot use `built_info::DEBUG` because that tells us if there are debug symbols,
      // not if there are optimizations.
      if cfg!(debug_assertions) { "debug" } else { "release" }
    )
  })
}

fn get_long_version() -> &'static str {
  LONG_VERSION_STR.get_or_init(|| {
    let rustflags = env!("CARGO_ENCODED_RUSTFLAGS");
    let rustflags = if rustflags.trim().is_empty() {
      "(None)".to_string()
    } else {
      // Replace non-printable ASCII Unit Separator with whitespace
      rustflags.replace(0x1F as char, " ")
    };
    format!(
      "{}\n{} {}\nCompiled CPU Features: {}\nRuntime Assembly Support: {}{}\nThreading: {}\nUnstable Features: {}\nCompiler Flags: {}",
      get_version(),
      built_info::RUSTC_VERSION,
      built_info::TARGET,
      option_env!("CARGO_CFG_TARGET_FEATURE").unwrap_or("(None)"),
      if cfg!(feature = "asm") { "Enabled" } else { "Disabled" },
      if cfg!(feature = "asm") {
        format!("\nRuntime Assembly Level: {}", CpuFeatureLevel::default())
      } else {
        String::new()
      },
      if cfg!(feature = "threading") { "Enabled" } else { "Disabled" },
      if cfg!(feature = "unstable") { "Enabled" } else { "Disabled" },
      rustflags
    )
  })
}

#[derive(Subcommand)]
pub enum Commands {
  /// Advanced features
  Advanced {
    /// Output to stdout the completion definition for the shell
    #[clap(long, short, value_parser)]
    completion: Option<Shell>,
    /// Save the current configuration in a toml file
    #[clap(long, short, value_parser)]
    save_config: Option<PathBuf>,
    /// Load the encoder configuration from a toml file
    #[clap(long, short, value_parser, conflicts_with = "save-config")]
    load_config: Option<PathBuf>,
  },
}

pub struct EncoderIO {
  pub input: Box<dyn Read + Send>,
  pub output: Box<dyn Muxer + Send>,
  pub rec: Option<Box<dyn Write + Send>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Verboseness {
  Quiet,
  Normal,
  Verbose,
}

pub struct ParsedCliOptions {
  pub io: EncoderIO,
  pub enc: EncoderConfig,
  pub limit: usize,
  pub color_range_specified: bool,
  pub override_time_base: bool,
  pub skip: usize,
  pub verbose: Verboseness,
  pub benchmark: bool,
  pub threads: usize,
  pub metrics_enabled: MetricsEnabled,
  pub pass1file_name: Option<PathBuf>,
  pub pass2file_name: Option<PathBuf>,
  #[cfg(feature = "serialize")]
  pub save_config: Option<PathBuf>,
  pub photon_noise: u8,
  #[cfg(feature = "unstable")]
  #[allow(unused)]
  pub slots: usize,
  pub force_highbitdepth: bool,
}

#[cfg(feature = "serialize")]
static HELP_TEXT: OnceLock<String> = OnceLock::new();

#[cfg(feature = "serialize")]
fn build_speed_long_help() -> Option<&'static str> {
  let help = HELP_TEXT.get_or_init(|| {
    let levels = (0..=10)
      .map(|speed| {
        let s = SpeedSettings::from_preset(speed);
        let o = crate::kv::to_string(&s).unwrap().replace(", ", "\n    ");
        format!("{:2} :\n    {}", speed, o)
      })
      .collect::<Vec<String>>()
      .join("\n");

    format!(
      "Speed level (0 is best quality, 10 is fastest)\n\
   Speeds 10 and 0 are extremes and are generally not recommended\n\
   {}",
      levels
    )
  });

  Some(&help)
}

#[cfg(not(feature = "serialize"))]
#[allow(clippy::missing_const_for_fn)]
fn build_speed_long_help() -> Option<&'static str> {
  Some(
    "Speed level (0 is best quality, 10 is fastest)\n\
 Speeds 10 and 0 are extremes and are generally not recommended",
  )
}

#[allow(unused_mut)]
/// Only call this once at the start of the app,
/// otherwise bad things will happen.
pub fn parse_cli() -> Result<ParsedCliOptions, CliError> {
  let matches = CliOptions::parse();

  #[cfg(feature = "serialize")]
  let mut save_config_path = None;
  let mut enc = None;

  if let Some(command) = matches.command.as_ref() {
    match command {
      Commands::Advanced { completion, save_config, load_config } => {
        if let Some(shell) = completion {
          let mut app = CliOptions::command();
          let app_name = app.get_name().to_string();
          generate(*shell, &mut app, app_name, &mut std::io::stdout());
          std::process::exit(0);
        }

        #[cfg(feature = "serialize")]
        {
          save_config_path = save_config.clone();
          if let Some(load_config) = load_config {
            let mut config = String::new();
            File::open(load_config)
              .and_then(|mut f| f.read_to_string(&mut config))
              .map_err(|e| e.context("Cannot open the configuration file"))?;

            enc = Some(toml::from_str(&config).unwrap());
          }
        }
        #[cfg(not(feature = "serialize"))]
        {
          if save_config.is_some() || load_config.is_some() {
            let e: io::Error = io::ErrorKind::InvalidInput.into();
            return Err(e.context(
              "The load/save config advanced option requires the
            `serialize` feature, rebuild adding it.",
            ));
          }
        }
      }
    }
  }

  let rec = match matches.reconstruction.as_ref() {
    Some(f) => Some(Box::new(
      File::create(f)
        .map_err(|e| e.context("Cannot create reconstruction file"))?,
    ) as Box<dyn Write + Send>),
    None => None,
  };

  let os_input = &matches.input;
  let io = EncoderIO {
    input: match os_input.to_str() {
      Some("-") => Box::new(io::stdin()) as Box<dyn Read + Send>,
      _ => Box::new(
        File::open(os_input)
          .map_err(|e| e.context("Cannot open input file"))?,
      ) as Box<dyn Read + Send>,
    },
    output: create_muxer(&matches.output, matches.overwrite)?,
    rec,
  };

  let enc = enc.map_or_else(|| parse_config(&matches), Ok)?;

  let verbose = if matches.quiet {
    Verboseness::Quiet
  } else if matches.verbose {
    Verboseness::Verbose
  } else {
    Verboseness::Normal
  };

  let metrics_enabled = if matches.metrics {
    MetricsEnabled::All
  } else if matches.psnr {
    MetricsEnabled::Psnr
  } else {
    MetricsEnabled::None
  };

  let limit = matches.limit;
  if enc.still_picture && limit > 1 {
    panic!("A limit cannot be set above 1 in still picture mode");
  }

  #[cfg(feature = "unstable")]
  let slots = matches.slots;

  Ok(ParsedCliOptions {
    io,
    enc,
    limit,
    color_range_specified: matches.range.is_some(),
    override_time_base: matches.frame_rate.is_some(),
    metrics_enabled,
    skip: matches.skip,
    benchmark: matches.benchmark,
    verbose,
    threads: matches.threads,
    pass1file_name: matches.first_pass.clone(),
    pass2file_name: matches.second_pass.clone(),
    #[cfg(feature = "serialize")]
    save_config: save_config_path,
    photon_noise: matches.photon_noise,
    force_highbitdepth: matches.high_bitdepth,
    #[cfg(feature = "unstable")]
    slots,
  })
}

fn parse_config(matches: &CliOptions) -> Result<EncoderConfig, CliError> {
  let maybe_quantizer = matches.quantizer;
  let maybe_bitrate = matches.bitrate;
  let quantizer = maybe_quantizer.unwrap_or_else(|| {
    if maybe_bitrate.is_some() {
      // If a bitrate is specified, the quantizer is the maximum allowed (e.g.,
      //  the minimum quality allowed), which by default should be
      //  unconstrained.
      255
    } else {
      100
    }
  }) as usize;
  let bitrate: i32 = maybe_bitrate.unwrap_or(0);
  if bitrate <= 0
    && (matches.first_pass.is_some() || matches.second_pass.is_some())
  {
    panic!("A target bitrate must be specified when using passes");
  }

  if quantizer == 0 {
    unimplemented!("Lossless encoding not yet implemented");
  } else if quantizer > 255 {
    panic!("Quantizer must be between 0-255");
  }

  let speed = matches.speed;
  let scene_detection_speed = matches.scd_speed;
  let max_interval = matches.keyint;
  let min_interval = matches.min_keyint.min(max_interval);

  if speed > 10 {
    panic!("Speed must be between 0-10");
  } else if min_interval > max_interval {
    panic!("Maximum keyframe interval must be greater than or equal to minimum keyframe interval");
  }

  let color_primaries = matches.primaries.unwrap_or_default();
  let transfer_characteristics = matches.transfer.unwrap_or_default();
  let matrix_coefficients = matches.matrix.unwrap_or_default();

  let mut cfg = EncoderConfig::with_speed_preset(speed);

  if let Some(level_str) = &matches.level {
    cfg.level_idx = match level_str.as_str() {
      "auto" => None,
      "unconstrained" => Some(31),
      _ => {
        let (major, minor) = scan_fmt!(level_str, "{}.{}", u8, u8)
          .expect("Could not parse AV1 level");
        if major > 7 || minor > 3 {
          panic!("Invalid AV1 level")
        }
        Some(((major - 2) << 2) + minor)
      }
    };
  };

  if let Some(scd_speed) = scene_detection_speed {
    cfg.speed_settings.scene_detection_mode = if scd_speed == 0 {
      SceneDetectionSpeed::Standard
    } else {
      SceneDetectionSpeed::Fast
    };
  }

  cfg.set_key_frame_interval(min_interval, max_interval);
  cfg.switch_frame_interval = matches.switch_frame_interval;

  cfg.pixel_range = matches.range.unwrap_or_default();
  cfg.color_description = if color_primaries == ColorPrimaries::Unspecified
    && transfer_characteristics == TransferCharacteristics::Unspecified
    && matrix_coefficients == MatrixCoefficients::Unspecified
  {
    // No need to set a color description with all parameters unspecified.
    None
  } else {
    Some(ColorDescription {
      color_primaries,
      transfer_characteristics,
      matrix_coefficients,
    })
  };

  cfg.mastering_display = matches.mastering_display.as_ref().map(|mastering_display| {
    let (g_x, g_y, b_x, b_y, r_x, r_y, wp_x, wp_y, max_lum, min_lum) =
      scan_fmt!(
        mastering_display,
        "G({},{})B({},{})R({},{})WP({},{})L({},{})",
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64,
        f64
      )
      .expect("Cannot parse the mastering display option");

    /* AV1 spec sec. 6.7.4 "Metadata high dynamic range mastering display color volume semantics"
     * specifies chromaticity coords as 0.16 fixed-point numbers, which have a max float value
     * of 0.9999847412109375 (rounding to 1).
     */
    let chromaticity_range = 0.0..=1.0;
    if !chromaticity_range.contains(&g_x)
      || !chromaticity_range.contains(&g_y)
      || !chromaticity_range.contains(&b_x)
      || !chromaticity_range.contains(&b_y)
      || !chromaticity_range.contains(&r_x)
      || !chromaticity_range.contains(&r_y)
      || !chromaticity_range.contains(&wp_x)
      || !chromaticity_range.contains(&wp_y)
    {
      warn!(
        "Chromaticity coordinates will be trimmed to the range 0.0 to 1.0 (see AV1 spec sec. 6.7.4)."
      );
    }

    MasteringDisplay {
      primaries: [
        ChromaticityPoint {
          x: (r_x * ((1 << 16) as f64)).round() as u16,
          y: (r_y * ((1 << 16) as f64)).round() as u16,
        },
        ChromaticityPoint {
          x: (g_x * ((1 << 16) as f64)).round() as u16,
          y: (g_y * ((1 << 16) as f64)).round() as u16,
        },
        ChromaticityPoint {
          x: (b_x * ((1 << 16) as f64)).round() as u16,
          y: (b_y * ((1 << 16) as f64)).round() as u16,
        },
      ],
      white_point: ChromaticityPoint {
        x: (wp_x * ((1 << 16) as f64)).round() as u16,
        y: (wp_y * ((1 << 16) as f64)).round() as u16,
      },
      max_luminance: (max_lum * ((1 << 8) as f64)).round() as u32,
      min_luminance: (min_lum * ((1 << 14) as f64)).round() as u32,
    }
  });

  cfg.content_light =
    matches.content_light.as_ref().and_then(|content_light| {
      let (cll, fall) = scan_fmt!(content_light, "{},{}", u16, u16)
        .expect("Cannot parse the content light option");
      if cll == 0 && fall == 0 {
        None
      } else {
        Some(ContentLight {
          max_content_light_level: cll,
          max_frame_average_light_level: fall,
        })
      }
    });

  cfg.still_picture = matches.still_picture;

  cfg.quantizer = quantizer;
  cfg.min_quantizer = matches.min_quantizer.unwrap_or(0);
  cfg.bitrate = bitrate.checked_mul(1000).expect("Bitrate too high");
  cfg.reservoir_frame_delay = matches.reservoir_frame_delay;

  if let Some(rdo_frames) = matches.rdo_lookahead_frames {
    cfg.speed_settings.rdo_lookahead_frames = rdo_frames;
  }

  cfg.tune = matches.tune;

  if cfg.tune == Tune::Psychovisual {
    cfg.speed_settings.transform.tx_domain_distortion = false;
  }

  cfg.tile_cols = matches.tile_cols;
  cfg.tile_rows = matches.tile_rows;

  cfg.tiles = matches.tiles.unwrap_or(0);

  if cfg.tile_cols > 64 || cfg.tile_rows > 64 {
    panic!("Tile columns and rows may not be greater than 64");
  }

  if let Some(table_file) = matches.film_grain_table.as_ref() {
    let contents = std::fs::read_to_string(table_file)
      .expect("Failed to read film grain table file");
    let table = av1_grain::parse_grain_table(&contents)
      .expect("Failed to parse film grain table");
    if !table.is_empty() {
      cfg.film_grain_params = Some(table);
    }
  }

  if let Some(frame_rate) = matches.frame_rate {
    cfg.time_base = Rational::new(matches.time_scale, frame_rate);
  }

  cfg.low_latency = matches.low_latency;
  // Disables scene_detection
  if matches.no_scene_detection {
    cfg.speed_settings.scene_detection_mode = SceneDetectionSpeed::None;
  }

  Ok(cfg)
}
