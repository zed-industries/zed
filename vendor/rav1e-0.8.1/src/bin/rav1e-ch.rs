// Copyright (c) 2017-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[macro_use]
extern crate log;

mod common;
mod decoder;
mod error;
#[cfg(feature = "serialize")]
mod kv;
mod muxer;
mod stats;

use crate::common::*;
use crate::error::*;
use crate::stats::*;
use rav1e::config::CpuFeatureLevel;
use rav1e::prelude::*;

use crate::decoder::{Decoder, FrameBuilder, VideoDetails};
use crate::muxer::*;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::sync::Arc;

impl<T: Pixel> FrameBuilder<T> for FrameSender<T> {
  fn new_frame(&self) -> Frame<T> {
    FrameSender::new_frame(self)
  }
}

struct Source<D: Decoder> {
  limit: usize,
  count: usize,
  input: D,
  #[cfg(all(unix, feature = "signal-hook"))]
  exit_requested: Arc<std::sync::atomic::AtomicBool>,
}

impl<D: Decoder> Source<D> {
  cfg_if::cfg_if! {
    if #[cfg(all(unix, feature = "signal-hook"))] {
      fn new(limit: usize, input: D) -> Self {
        use signal_hook::{flag, consts};

        // Make sure double CTRL+C and similar kills
        let exit_requested = Arc::new(std::sync::atomic::AtomicBool::new(false));
        for sig in consts::TERM_SIGNALS {
            // When terminated by a second term signal, exit with exit code 1.
            // This will do nothing the first time (because term_now is false).
            flag::register_conditional_shutdown(*sig, 1, Arc::clone(&exit_requested)).unwrap();
            // But this will "arm" the above for the second time, by setting it to true.
            // The order of registering these is important, if you put this one first, it will
            // first arm and then terminate ‒ all in the first round.
            flag::register(*sig, Arc::clone(&exit_requested)).unwrap();
        }
        Self { limit, input, count: 0, exit_requested, }
      }
    } else {
      fn new(limit: usize, input: D) -> Self {
        Self { limit, input, count: 0, }
      }
    }
  }

  fn read_frame<T: Pixel>(
    &mut self, send_frame: &mut FrameSender<T>, video_info: VideoDetails,
  ) -> bool {
    if self.limit != 0 && self.count == self.limit {
      return false;
    }

    #[cfg(all(unix, feature = "signal-hook"))]
    {
      if self.exit_requested.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
      }
    }
    match self.input.read_frame(send_frame, &video_info) {
      Ok(frame) => {
        self.count += 1;
        let _ = send_frame.send(frame);
        true
      }
      _ => false,
    }
  }
}

fn do_encode<T: Pixel, D: Decoder>(
  cfg: Config, verbose: Verboseness, mut progress: ProgressInfo,
  output: &mut dyn Muxer, mut source: Source<D>, pass1file: Option<File>,
  pass2file: Option<File>,
  mut y4m_enc: Option<y4m::Encoder<Box<dyn Write + Send>>>,
  metrics_enabled: MetricsEnabled,
) -> Result<(), CliError> {
  let ((mut send_frame, receive_packet), (send_rc, receive_rc)) =
    match (pass1file.is_some(), pass2file.is_some()) {
      (true, true) => {
        let (channel, (send_rc, receive_rc)) = cfg
          .new_multipass_channel::<T>()
          .map_err(|e| e.context("Invalid setup"))?;
        (channel, (Some(send_rc), Some(receive_rc)))
      }
      (true, false) => {
        let (channel, receive_rc) = cfg
          .new_firstpass_channel()
          .map_err(|e| e.context("Invalid setup"))?;
        (channel, (None, Some(receive_rc)))
      }
      (false, true) => {
        let (channel, send_rc) = cfg
          .new_secondpass_channel()
          .map_err(|e| e.context("Invalid setup"))?;
        (channel, (Some(send_rc), None))
      }
      (false, false) => {
        let channel =
          cfg.new_channel().map_err(|e| e.context("Invalid setup"))?;
        (channel, (None, None))
      }
    };

  let y4m_details = source.input.get_video_details();

  crossbeam::thread::scope(move |s| -> Result<(), CliError> {
    // Receive pass data
    let receive_pass_data = s.spawn(move |_| -> Result<(), CliError> {
      if let (Some(mut passfile), Some(receive_rc)) = (pass1file, receive_rc) {
        let len = receive_rc.summary_size();
        let buf = vec![0u8; len];

        passfile
          .write_all(&(len as u64).to_be_bytes())
          .map_err(|e| e.context("Unable to write to two-pass data file."))?;

        passfile
          .write_all(&buf)
          .map_err(|e| e.context("Unable to write to two-pass data file."))?;
        for data in receive_rc.iter() {
          match data {
            RcData::Frame(outbuf) => {
              let len = outbuf.len() as u64;
              passfile.write_all(&len.to_be_bytes()).map_err(|e| {
                e.context("Unable to write to two-pass data file.")
              })?;

              passfile.write_all(&outbuf).map_err(|e| {
                e.context("Unable to write to two-pass data file.")
              })?;
            }
            RcData::Summary(outbuf) => {
              // Write an end marker
              passfile.write_all(&0u64.to_be_bytes()).map_err(|e| {
                e.context("Unable to write to two-pass data file.")
              })?;

              // The last packet of rate control data we get is the summary data.
              // Let's put it at the start of the file.
              passfile.seek(std::io::SeekFrom::Start(0)).map_err(|e| {
                e.context("Unable to seek in the two-pass data file.")
              })?;
              let len = outbuf.len() as u64;

              passfile.write_all(&len.to_be_bytes()).map_err(|e| {
                e.context("Unable to write to two-pass data file.")
              })?;

              passfile.write_all(&outbuf).map_err(|e| {
                e.context("Unable to write to two-pass data file.")
              })?;
            }
          }
        }
      }
      Ok(())
    });

    // Send frames
    let send_frames = s.spawn(move |_| -> Result<(), CliError> {
      while source.read_frame(&mut send_frame, y4m_details) {}

      // send_frame.result()
      Ok(())
    });

    // Send pass data
    let send_pass_data = s.spawn(move |_| -> Result<(), CliError> {
      if let (Some(mut passfile), Some(mut send_rc)) = (pass2file, send_rc) {
        let mut buflen = [0u8; 8];

        passfile
          .read_exact(&mut buflen)
          .map_err(|e| e.context("Unable to read the two-pass data file."))?;

        let mut data = vec![0u8; u64::from_be_bytes(buflen) as usize];

        passfile
          .read_exact(&mut data)
          .map_err(|e| e.context("Unable to read the two-pass data file."))?;

        while send_rc.send(RcData::Frame(data.into_boxed_slice())).is_ok() {
          passfile.read_exact(&mut buflen).map_err(|e| {
            e.context("Unable to read the two-pass data file.")
          })?;

          if u64::from_be_bytes(buflen) == 0 {
            break;
          }

          data = vec![0u8; u64::from_be_bytes(buflen) as usize];

          passfile.read_exact(&mut data).map_err(|e| {
            e.context("Unable to read the two-pass data file.")
          })?;
        }
      }

      Ok(())
    });

    // Receive Packets
    let receive_packets = s.spawn(move |_| -> Result<(), CliError> {
      for pkt in receive_packet.iter() {
        output.write_frame(
          pkt.input_frameno as u64,
          pkt.data.as_ref(),
          pkt.frame_type,
        );
        output.flush().unwrap();
        if let (Some(ref mut y4m_enc_uw), Some(ref rec)) =
          (y4m_enc.as_mut(), &pkt.rec)
        {
          write_y4m_frame(y4m_enc_uw, rec, y4m_details);
        }
        let summary = build_frame_summary(
          pkt,
          y4m_details.bit_depth,
          y4m_details.chroma_sampling,
          metrics_enabled,
        );

        if verbose != Verboseness::Quiet {
          progress.add_frame(summary.clone());
          if verbose == Verboseness::Verbose {
            info!("{} - {}", summary, progress);
          } else {
            // Print a one-line progress indicator that overrides itself with every update
            eprint!("\r{}                    ", progress);
          };
        }
      }

      if verbose != Verboseness::Quiet {
        if verbose == Verboseness::Verbose {
          // Clear out the temporary progress indicator
          eprint!("\r");
        }
        progress.print_summary(verbose == Verboseness::Verbose);
      }

      // receive_packet.result()
      Ok(())
    });

    send_pass_data.join().expect("The send pass data thread panicked ")?;
    receive_pass_data
      .join()
      .expect("The receive pass data thread panicked")?;
    send_frames.join().expect("The send frames thread panicked")?;
    receive_packets.join().expect("The receive packets thread panicked")?;

    Ok(())
  })
  .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logger();

  #[cfg(feature = "tracing")]
  let (chrome_layer, _guard) =
    tracing_chrome::ChromeLayerBuilder::new().build();

  #[cfg(feature = "tracing")]
  {
    use tracing_subscriber::layer::SubscriberExt;
    tracing::subscriber::set_global_default(
      tracing_subscriber::registry().with(chrome_layer),
    )
    .unwrap();
  }

  run().map_err(|e| {
    error::print_error(&e);
    Box::new(e) as Box<dyn std::error::Error>
  })
}

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

  let level = std::env::var("RAV1E_LOG")
    .ok()
    .and_then(|l| log::LevelFilter::from_str(&l).ok())
    .unwrap_or(log::LevelFilter::Info);

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
    .level(log::LevelFilter::Warn)
    // change log levels for individual modules. Note: This looks for the record's target
    // field which defaults to the module path but can be overwritten with the `target`
    // parameter:
    // `info!(target="special_target", "This log message is about special_target");`
    .level_for("rav1e", level)
    .level_for("rav1e_ch", level)
    // output to stdout
    .chain(std::io::stderr())
    .apply()
    .unwrap();
}

cfg_if::cfg_if! {
  if #[cfg(any(target_os = "windows", target_arch = "wasm32"))] {
    fn print_rusage() {
      eprintln!("Resource usage reporting is not currently supported on this platform");
    }
  } else {
    fn print_rusage() {
      // SAFETY: This uses an FFI, it is safe because we call it correctly.
      let (utime, stime, maxrss) = unsafe {
        let mut usage = std::mem::zeroed();
        let _ = libc::getrusage(libc::RUSAGE_SELF, &mut usage);
        (usage.ru_utime, usage.ru_stime, usage.ru_maxrss)
      };
      eprintln!(
        "user time: {} s",
        utime.tv_sec as f64 + utime.tv_usec as f64 / 1_000_000f64
      );
      eprintln!(
        "system time: {} s",
        stime.tv_sec as f64 + stime.tv_usec as f64 / 1_000_000f64
      );
      eprintln!("maximum rss: {} KB", maxrss);
    }
  }
}

fn run() -> Result<(), error::CliError> {
  let mut cli = parse_cli()?;
  // Maximum frame size by specification + maximum y4m header
  let limit = y4m::Limits {
    // Use saturating operations to gracefully handle 32-bit architectures
    bytes: 64usize
      .saturating_mul(64)
      .saturating_mul(4096)
      .saturating_mul(2304)
      .saturating_add(1024),
  };
  let mut y4m_dec = match y4m::Decoder::new_with_limits(cli.io.input, limit) {
    Err(e) => {
      return Err(CliError::new(match e {
        y4m::Error::ParseError(_) => {
          "Could not parse input video. Is it a y4m file?"
        }
        y4m::Error::IoError(_) => {
          "Could not read input file. Check that the path is correct and you have read permissions."
        }
        y4m::Error::UnknownColorspace => {
          "Unknown colorspace or unsupported bit depth."
        }
        y4m::Error::OutOfMemory => "The video's frame size exceeds the limit.",
        y4m::Error::EOF => "Unexpected end of input.",
        y4m::Error::BadInput => "Bad y4m input parameters provided.",
      }))
    }
    Ok(d) => d,
  };
  let video_info = y4m_dec.get_video_details();
  let y4m_enc = cli.io.rec.map(|rec| {
    y4m::encode(
      video_info.width,
      video_info.height,
      y4m::Ratio::new(
        video_info.time_base.den as usize,
        video_info.time_base.num as usize,
      ),
    )
    .with_colorspace(y4m_dec.get_colorspace())
    .with_pixel_aspect(y4m::Ratio {
      num: video_info.sample_aspect_ratio.num as usize,
      den: video_info.sample_aspect_ratio.den as usize,
    })
    .write_header(rec)
    .unwrap()
  });

  match video_info.bit_depth {
    8 | 10 | 12 => {}
    _ => return Err(CliError::new("Unsupported bit depth")),
  }

  cli.enc.width = video_info.width;
  cli.enc.height = video_info.height;
  cli.enc.bit_depth = video_info.bit_depth;
  cli.enc.sample_aspect_ratio = video_info.sample_aspect_ratio;
  cli.enc.chroma_sampling = video_info.chroma_sampling;
  cli.enc.chroma_sample_position = video_info.chroma_sample_position;

  // If no pixel range is specified via CLI, assume limited,
  // as it is the default for the Y4M format.
  if !cli.color_range_specified {
    cli.enc.pixel_range = PixelRange::Limited;
  }

  if !cli.override_time_base {
    cli.enc.time_base = video_info.time_base;
  }

  if cli.photon_noise > 0 && cli.enc.film_grain_params.is_none() {
    cli.enc.film_grain_params = Some(vec![generate_photon_noise_params(
      0,
      u64::MAX,
      NoiseGenArgs {
        iso_setting: cli.photon_noise as u32 * 100,
        width: video_info.width as u32,
        height: video_info.height as u32,
        transfer_function: if cli.enc.is_hdr() {
          TransferFunction::SMPTE2084
        } else {
          TransferFunction::BT1886
        },
        chroma_grain: false,
        random_seed: None,
      },
    )]);
  }

  let mut rc = RateControlConfig::new();

  let pass2file = match cli.pass2file_name {
    Some(f) => {
      let mut f = File::open(f).map_err(|e| {
        e.context("Unable to open file for reading two-pass data")
      })?;
      let mut buflen = [0u8; 8];
      f.read_exact(&mut buflen)
        .map_err(|e| e.context("Summary data too short"))?;
      let len = i64::from_be_bytes(buflen);
      let mut buf = vec![0u8; len as usize];

      f.read_exact(&mut buf)
        .map_err(|e| e.context("Summary data too short"))?;

      rc = RateControlConfig::from_summary_slice(&buf)
        .map_err(|e| e.context("Invalid summary"))?;

      Some(f)
    }
    None => None,
  };

  let pass1file = match cli.pass1file_name {
    Some(f) => {
      let f = File::create(f).map_err(|e| {
        e.context("Unable to open file for writing two-pass data")
      })?;
      rc = rc.with_emit_data(true);
      Some(f)
    }
    None => None,
  };

  let cfg = Config::new()
    .with_encoder_config(cli.enc.clone())
    .with_threads(cli.threads)
    .with_rate_control(rc)
    .with_parallel_gops(cli.slots);

  #[cfg(feature = "serialize")]
  {
    if let Some(save_config) = cli.save_config {
      let mut out = File::create(save_config)
        .map_err(|e| e.context("Cannot create configuration file"))?;
      let s = toml::to_string(&cli.enc).unwrap();
      out
        .write_all(s.as_bytes())
        .map_err(|e| e.context("Cannot write the configuration file"))?
    }
  }

  cli.io.output.write_header(
    video_info.width,
    video_info.height,
    cli.enc.time_base.den as usize,
    cli.enc.time_base.num as usize,
  );

  let tiling =
    cfg.tiling_info().map_err(|e| e.context("Invalid configuration"))?;
  if cli.verbose != Verboseness::Quiet {
    info!("CPU Feature Level: {}", CpuFeatureLevel::default());

    info!(
      "Using y4m decoder: {}x{}p @ {}/{} fps, {}, {}-bit",
      video_info.width,
      video_info.height,
      video_info.time_base.den,
      video_info.time_base.num,
      video_info.chroma_sampling,
      video_info.bit_depth
    );
    info!("Encoding settings: {}", cli.enc);

    if tiling.tile_count() == 1 {
      info!("Using 1 tile");
    } else {
      info!(
        "Using {} tiles ({}x{})",
        tiling.tile_count(),
        tiling.cols,
        tiling.rows
      );
    }
  }

  let progress = ProgressInfo::new(
    Rational { num: video_info.time_base.den, den: video_info.time_base.num },
    if cli.limit == 0 { None } else { Some(cli.limit) },
    cli.metrics_enabled,
  );

  for _ in 0..cli.skip {
    match y4m_dec.read_frame() {
      Ok(f) => f,
      Err(_) => {
        return Err(CliError::new("Skipped more frames than in the input"))
      }
    };
  }

  let source = Source::new(cli.limit, y4m_dec);

  if video_info.bit_depth == 8 && !cli.force_highbitdepth {
    do_encode::<u8, y4m::Decoder<Box<dyn Read + Send>>>(
      cfg,
      cli.verbose,
      progress,
      &mut *cli.io.output,
      source,
      pass1file,
      pass2file,
      y4m_enc,
      cli.metrics_enabled,
    )?
  } else {
    do_encode::<u16, y4m::Decoder<Box<dyn Read + Send>>>(
      cfg,
      cli.verbose,
      progress,
      &mut *cli.io.output,
      source,
      pass1file,
      pass2file,
      y4m_enc,
      cli.metrics_enabled,
    )?
  }
  if cli.benchmark {
    print_rusage();
  }

  Ok(())
}
