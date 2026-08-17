// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use av_metrics::video::*;
use rav1e::data::EncoderStats;
use rav1e::prelude::*;
use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct FrameSummary {
  /// Frame size in bytes
  pub size: usize,
  pub input_frameno: u64,
  pub frame_type: FrameType,
  /// Contains metrics such as PSNR, SSIM, etc.
  pub metrics: QualityMetrics,
  /// QP selected for the frame.
  pub qp: u8,
  /// Block-level encoding stats for the frame
  pub enc_stats: EncoderStats,
}

#[profiling::function]
pub fn build_frame_summary<T: Pixel>(
  packets: Packet<T>, bit_depth: usize, chroma_sampling: ChromaSampling,
  metrics_cli: MetricsEnabled,
) -> FrameSummary {
  let metrics_input_frame: &Frame<T> = packets.source.as_ref().unwrap();
  let metrics_output_frame: &Frame<T> = packets.rec.as_ref().unwrap();
  let encode_metrics: QualityMetrics = calculate_frame_metrics(
    metrics_input_frame,
    metrics_output_frame,
    bit_depth,
    chroma_sampling,
    metrics_cli,
  );
  FrameSummary {
    size: packets.data.len(),
    input_frameno: packets.input_frameno,
    frame_type: packets.frame_type,
    metrics: encode_metrics,
    qp: packets.qp,
    enc_stats: packets.enc_stats,
  }
}

impl fmt::Display for FrameSummary {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Input Frame {} - {} - {} bytes{}",
      self.input_frameno,
      self.frame_type,
      self.size,
      if let Some(psnr) = self.metrics.psnr {
        format!(
          " - PSNR: Y: {:.4}  Cb: {:.4}  Cr: {:.4}",
          psnr.y, psnr.u, psnr.v
        )
      } else {
        String::new()
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct ProgressInfo {
  // Frame rate of the video
  frame_rate: Rational,
  // The length of the whole video, in frames, if known
  total_frames: Option<usize>,
  // The time the encode was started
  time_started: Instant,
  // List of frames encoded so far
  frame_info: Vec<FrameSummary>,
  // Video size so far in bytes.
  //
  // This value will be updated in the CLI very frequently, so we cache the previous value
  // to reduce the overall complexity.
  encoded_size: usize,
  // Which Metrics to display during and at end of encode
  metrics_enabled: MetricsEnabled,
}

impl ProgressInfo {
  pub fn new(
    frame_rate: Rational, total_frames: Option<usize>,
    metrics_enabled: MetricsEnabled,
  ) -> Self {
    Self {
      frame_rate,
      total_frames,
      time_started: Instant::now(),
      frame_info: Vec::with_capacity(total_frames.unwrap_or_default()),
      encoded_size: 0,
      metrics_enabled,
    }
  }

  pub fn add_frame(&mut self, frame: FrameSummary) {
    self.encoded_size += frame.size;
    self.frame_info.push(frame);
  }

  pub fn frames_encoded(&self) -> usize {
    self.frame_info.len()
  }

  pub fn encoding_fps(&self) -> f64 {
    let duration = Instant::now().duration_since(self.time_started);
    self.frame_info.len() as f64
      / (duration.as_secs() as f64 + duration.subsec_millis() as f64 / 1000f64)
  }

  pub fn video_fps(&self) -> f64 {
    self.frame_rate.num as f64 / self.frame_rate.den as f64
  }

  // Returns the bitrate of the frames so far, in bits/second
  pub fn bitrate(&self) -> usize {
    let bits = self.encoded_size * 8;
    let seconds = self.frame_info.len() as f64 / self.video_fps();
    (bits as f64 / seconds) as usize
  }

  // Estimates the final filesize in bytes, if the number of frames is known
  pub fn estimated_size(&self) -> usize {
    self
      .total_frames
      .map(|frames| self.encoded_size * frames / self.frames_encoded())
      .unwrap_or_default()
  }

  // Estimates the remaining encoding time in seconds, if the number of frames is known
  pub fn estimated_time(&self) -> u64 {
    self
      .total_frames
      .map(|frames| {
        (frames - self.frames_encoded()) as f64 / self.encoding_fps()
      })
      .unwrap_or_default() as u64
  }

  // Elapsed time in seconds
  pub fn elapsed_time(&self) -> u64 {
    Instant::now().duration_since(self.time_started).as_secs()
  }

  // Number of frames of given type which appear in the video
  fn get_frame_type_count(&self, frame_type: FrameType) -> usize {
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .count()
  }

  fn get_frame_type_avg_size(&self, frame_type: FrameType) -> usize {
    let count = self.get_frame_type_count(frame_type);
    if count == 0 {
      return 0;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.size)
      .sum::<usize>()
      / count
  }

  fn get_frame_type_avg_qp(&self, frame_type: FrameType) -> f32 {
    let count = self.get_frame_type_count(frame_type);
    if count == 0 {
      return 0.;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.qp as f32)
      .sum::<f32>()
      / count as f32
  }

  fn get_block_count_by_frame_type(&self, frame_type: FrameType) -> usize {
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.block_size_counts.iter().sum::<usize>())
      .sum()
  }

  fn get_tx_count_by_frame_type(&self, frame_type: FrameType) -> usize {
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.tx_type_counts.iter().sum::<usize>())
      .sum()
  }

  fn get_bsize_pct_by_frame_type(
    &self, bsize: BlockSize, frame_type: FrameType,
  ) -> f32 {
    let count = self.get_block_count_by_frame_type(frame_type);
    if count == 0 {
      return 0.;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.block_size_counts[bsize as usize])
      .sum::<usize>() as f32
      / count as f32
      * 100.
  }

  fn get_skip_pct_by_frame_type(&self, frame_type: FrameType) -> f32 {
    let count = self.get_block_count_by_frame_type(frame_type);
    if count == 0 {
      return 0.;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.skip_block_count)
      .sum::<usize>() as f32
      / count as f32
      * 100.
  }

  fn get_txtype_pct_by_frame_type(
    &self, txtype: TxType, frame_type: FrameType,
  ) -> f32 {
    let count = self.get_tx_count_by_frame_type(frame_type);
    if count == 0 {
      return 0.;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.tx_type_counts[txtype as usize])
      .sum::<usize>() as f32
      / count as f32
      * 100.
  }

  fn get_luma_pred_count_by_frame_type(&self, frame_type: FrameType) -> usize {
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.luma_pred_mode_counts.iter().sum::<usize>())
      .sum()
  }

  fn get_chroma_pred_count_by_frame_type(
    &self, frame_type: FrameType,
  ) -> usize {
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| {
        frame.enc_stats.chroma_pred_mode_counts.iter().sum::<usize>()
      })
      .sum()
  }

  fn get_luma_pred_mode_pct_by_frame_type(
    &self, pred_mode: PredictionMode, frame_type: FrameType,
  ) -> f32 {
    let count = self.get_luma_pred_count_by_frame_type(frame_type);
    if count == 0 {
      return 0.;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.luma_pred_mode_counts[pred_mode as usize])
      .sum::<usize>() as f32
      / count as f32
      * 100.
  }

  fn get_chroma_pred_mode_pct_by_frame_type(
    &self, pred_mode: PredictionMode, frame_type: FrameType,
  ) -> f32 {
    let count = self.get_chroma_pred_count_by_frame_type(frame_type);
    if count == 0 {
      return 0.;
    }
    self
      .frame_info
      .iter()
      .filter(|frame| frame.frame_type == frame_type)
      .map(|frame| frame.enc_stats.chroma_pred_mode_counts[pred_mode as usize])
      .sum::<usize>() as f32
      / count as f32
      * 100.
  }

  pub fn print_summary(&self, verbose: bool) {
    eprint!("\r");
    info!("{}", self);
    info!("----------");
    self.print_frame_type_summary(FrameType::KEY);
    self.print_frame_type_summary(FrameType::INTER);
    self.print_frame_type_summary(FrameType::INTRA_ONLY);
    self.print_frame_type_summary(FrameType::SWITCH);
    if verbose {
      self.print_block_type_summary();
      self.print_transform_type_summary();
      self.print_prediction_modes_summary();
    }
    match self.metrics_enabled {
      MetricsEnabled::None => info!("----"),
      MetricsEnabled::Psnr => self.print_video_psnr(),
      MetricsEnabled::All => {
        self.print_video_psnr();
        self.print_video_all();
      }
    }
  }

  fn print_frame_type_summary(&self, frame_type: FrameType) {
    let count = self.get_frame_type_count(frame_type);
    let size = self.get_frame_type_avg_size(frame_type);
    let avg_qp = self.get_frame_type_avg_qp(frame_type);
    info!(
      "{:17} {:>6} | avg QP: {:6.2} | avg size: {:>7} B",
      format!("{frame_type}:"),
      count,
      avg_qp,
      size
    );
  }

  fn print_video_psnr(&self) {
    info!("----------");
    let psnr_y = sum_metric(&self.frame_info, |fi| fi.metrics.psnr.unwrap().y);
    let psnr_u = sum_metric(&self.frame_info, |fi| fi.metrics.psnr.unwrap().u);
    let psnr_v = sum_metric(&self.frame_info, |fi| fi.metrics.psnr.unwrap().v);
    let psnr_avg =
      sum_metric(&self.frame_info, |fi| fi.metrics.psnr.unwrap().avg);
    info!(
      "Mean PSNR: Avg: {:.4}  Y: {:.4}  Cb: {:.4}  Cr: {:.4}",
      psnr_avg, psnr_y, psnr_u, psnr_v
    );
  }
  fn print_video_all(&self) {
    info!("----------");
    let psnr_hvs =
      sum_metric(&self.frame_info, |fi| fi.metrics.psnr_hvs.unwrap().avg);
    let ssim = sum_metric(&self.frame_info, |fi| fi.metrics.ssim.unwrap().avg);
    let ms_ssim =
      sum_metric(&self.frame_info, |fi| fi.metrics.ms_ssim.unwrap().avg);
    let ciede = sum_metric(&self.frame_info, |fi| fi.metrics.ciede.unwrap());
    info!("PSNR HVS: {:.4}", psnr_hvs);
    info!("SSIM: {:.4}  MS SSIM: {:.4}", ssim, ms_ssim);
    info!("CIEDE2000: {:.4}", ciede);
    info!("----------");
  }

  fn print_block_type_summary(&self) {
    self.print_block_type_summary_for_frame_type(FrameType::KEY, 'I');
    self.print_block_type_summary_for_frame_type(FrameType::INTER, 'P');
  }

  fn print_block_type_summary_for_frame_type(
    &self, frame_type: FrameType, type_label: char,
  ) {
    info!("----------");
    info!(
      "bsize {}: {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
      type_label, "x128", "x64", "x32", "x16", "x8", "x4"
    );
    info!(
      "   128x: {:>5.1}% {:>5.1}%                              {}",
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_128X128, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_128X64, frame_type),
      if frame_type == FrameType::INTER {
        format!("skip: {:>5.1}%", self.get_skip_pct_by_frame_type(frame_type))
      } else {
        String::new()
      }
    );
    info!(
      "    64x: {:>5.1}% {:>5.1}% {:>5.1}% {:>5.1}%",
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_64X128, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_64X64, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_64X32, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_64X16, frame_type),
    );
    info!(
      "    32x:        {:>5.1}% {:>5.1}% {:>5.1}% {:>5.1}%",
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_32X64, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_32X32, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_32X16, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_32X8, frame_type),
    );
    info!(
      "    16x:        {:>5.1}% {:>5.1}% {:>5.1}% {:>5.1}% {:>5.1}%",
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_16X64, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_16X32, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_16X16, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_16X8, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_16X4, frame_type),
    );
    info!(
      "     8x:               {:>5.1}% {:>5.1}% {:>5.1}% {:>5.1}%",
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_8X32, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_8X16, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_8X8, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_8X4, frame_type),
    );
    info!(
      "     4x:                      {:>5.1}% {:>5.1}% {:>5.1}%",
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_4X16, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_4X8, frame_type),
      self.get_bsize_pct_by_frame_type(BlockSize::BLOCK_4X4, frame_type),
    );
  }

  fn print_transform_type_summary(&self) {
    info!("----------");
    self.print_transform_type_summary_by_frame_type(FrameType::KEY, 'I');
    self.print_transform_type_summary_by_frame_type(FrameType::INTER, 'P');
  }

  fn print_transform_type_summary_by_frame_type(
    &self, frame_type: FrameType, type_label: char,
  ) {
    info!(
      "txtypes {}: DCT_DCT: {:.1}% | ADST_DCT: {:.1}% | DCT_ADST: {:.1}% | ADST_ADST: {:.1}%",
      type_label,
      self.get_txtype_pct_by_frame_type(TxType::DCT_DCT, frame_type),
      self.get_txtype_pct_by_frame_type(TxType::ADST_DCT, frame_type),
      self.get_txtype_pct_by_frame_type(TxType::DCT_ADST, frame_type),
      self.get_txtype_pct_by_frame_type(TxType::ADST_ADST, frame_type)
    );
    info!(
      "           IDTX: {:.1}% | V_DCT: {:.1}% | H_DCT: {:.1}%",
      self.get_txtype_pct_by_frame_type(TxType::IDTX, frame_type),
      self.get_txtype_pct_by_frame_type(TxType::V_DCT, frame_type),
      self.get_txtype_pct_by_frame_type(TxType::H_DCT, frame_type),
    )
  }

  fn print_prediction_modes_summary(&self) {
    info!("----------");
    self.print_luma_prediction_mode_summary_by_frame_type(FrameType::KEY, 'I');
    self
      .print_chroma_prediction_mode_summary_by_frame_type(FrameType::KEY, 'I');
    info!("----------");
    self
      .print_luma_prediction_mode_summary_by_frame_type(FrameType::INTER, 'P');
    self.print_chroma_prediction_mode_summary_by_frame_type(
      FrameType::INTER,
      'P',
    );
  }

  fn print_luma_prediction_mode_summary_by_frame_type(
    &self, frame_type: FrameType, type_label: char,
  ) {
    if frame_type == FrameType::KEY {
      info!(
        "y modes {}: DC: {:.1}% | V: {:.1}% | H: {:.1}% | Paeth: {:.1}%",
        type_label,
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::DC_PRED,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::V_PRED,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::H_PRED,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::PAETH_PRED,
          frame_type
        ),
      );
      info!(
        "           Smooth: {:.1}% | Smooth V: {:.1}% | Smooth H: {:.1}%",
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::SMOOTH_PRED,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::SMOOTH_V_PRED,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::SMOOTH_H_PRED,
          frame_type
        ),
      );
      // Keep angular order for presentation here, rather than enum order.
      info!(
      "        D: 45: {:.1}% | 67: {:.1}% | 113: {:.1}% | 135: {:.1}% | 157: {:.1}% | 203: {:.1}%",
      self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::D45_PRED, frame_type),
      self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::D67_PRED, frame_type),
      self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::D113_PRED, frame_type),
      self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::D135_PRED, frame_type),
      self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::D157_PRED, frame_type),
      self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::D203_PRED, frame_type),
      );
    } else if frame_type == FrameType::INTER {
      info!(
        "y modes {}: Nearest: {:.1}% | Near0: {:.1}% | Near1: {:.1}% | NearNear0: {:.1}% | NearNear1: {:.1}% | NearNear2: {:.1}%",
        type_label,
        self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::NEARESTMV, frame_type),
        self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::NEAR0MV, frame_type),
        self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::NEAR1MV, frame_type),
        self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::NEAR_NEAR0MV, frame_type),
        self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::NEAR_NEAR1MV, frame_type),
        self.get_luma_pred_mode_pct_by_frame_type(PredictionMode::NEAR_NEAR2MV, frame_type),
      );
      info!(
        "y modes {}: NearNew0: {:.1}% | NearNew1: {:.1}% | NearNew2: {:.1}%",
        type_label,
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::NEAR_NEW0MV,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::NEAR_NEW1MV,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::NEAR_NEW2MV,
          frame_type
        ),
      );
      info!(
        "y modes {}: NewNear0: {:.1}% | NewNear1: {:.1}% | NewNear2: {:.1}%",
        type_label,
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::NEW_NEAR0MV,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::NEW_NEAR1MV,
          frame_type
        ),
        self.get_luma_pred_mode_pct_by_frame_type(
          PredictionMode::NEW_NEAR2MV,
          frame_type
        ),
      );
    }
  }

  fn print_chroma_prediction_mode_summary_by_frame_type(
    &self, frame_type: FrameType, type_label: char,
  ) {
    if frame_type == FrameType::KEY {
      info!(
        "uv modes {}: DC: {:.1}% | V: {:.1}% | H: {:.1}% | Paeth: {:.1}%",
        type_label,
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::DC_PRED,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::V_PRED,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::H_PRED,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::PAETH_PRED,
          frame_type
        ),
      );
      info!(
        "            Smooth: {:.1}% | Smooth V: {:.1}% | Smooth H: {:.1}% | UV CFL: {:.1}%",
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::SMOOTH_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::SMOOTH_V_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::SMOOTH_H_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::UV_CFL_PRED, frame_type),
      );
      // Keep angular order for presentation here, rather than enum order.
      info!(
        "         D: 45: {:.1}% | 67: {:.1}% | 113: {:.1}% | 135: {:.1}% | 157: {:.1}% | 203: {:.1}%",
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::D45_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::D67_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::D113_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::D135_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::D157_PRED, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::D203_PRED, frame_type),
      );
    } else if frame_type == FrameType::INTER {
      info!(
        "uv modes {}: Nearest: {:.1}% | Near0: {:.1}% | Near1: {:.1}% | NearNear0: {:.1}% | NearNear1: {:.1}% | NearNear2: {:.1}%",
        type_label,
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEARESTMV, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEAR0MV, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEAR1MV, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEAR_NEAR0MV, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEAR_NEAR1MV, frame_type),
        self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEAR_NEAR2MV, frame_type),
      );
      info!(
        "uv modes {}: NearNew0: {:.1}% | NearNew1: {:.1}% | NearNew2: {:.1}%",
        type_label,
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::NEAR_NEW0MV,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::NEAR_NEW1MV,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::NEAR_NEW2MV,
          frame_type
        ),
      );
      info!(
        "uv modes {}: NewNear0: {:.1}% | NewNear1: {:.1}% | NewNear2: {:.1}%",
        type_label,
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::NEW_NEAR0MV,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::NEW_NEAR1MV,
          frame_type
        ),
        self.get_chroma_pred_mode_pct_by_frame_type(
          PredictionMode::NEW_NEAR2MV,
          frame_type
        ),
      );
      info!("            New: {:.1}% | NewNew: {:.1}% | NearestNearest: {:.1}% | GlobalGlobal: {:.1}%",
            self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEWMV, frame_type),
            self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEW_NEWMV, frame_type),
            self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::NEAREST_NEARESTMV, frame_type),
            self.get_chroma_pred_mode_pct_by_frame_type(PredictionMode::GLOBAL_GLOBALMV, frame_type),);
    }
  }
}

fn sum_metric<F: FnMut(&FrameSummary) -> f64>(
  frame_info: &[FrameSummary], map_fn: F,
) -> f64 {
  frame_info.iter().map(map_fn).sum::<f64>() / frame_info.len() as f64
}

impl fmt::Display for ProgressInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(total_frames) = self.total_frames {
      write!(
        f,
        "encoded {}/{} frames, {:.3} fps, {:.2} Kb/s, est: {}, {:.2} MB, elapsed: {}",
        self.frames_encoded(),
        total_frames,
        self.encoding_fps(),
        self.bitrate() as f64 / 1000f64,
        secs_to_human_time(self.estimated_time()),
        self.estimated_size() as f64 / (1024 * 1024) as f64,
        secs_to_human_time(self.elapsed_time())
      )
    } else {
      write!(
        f,
        "encoded {} frames, {:.3} fps, {:.2} Kb/s, elapsed: {}",
        self.frames_encoded(),
        self.encoding_fps(),
        self.bitrate() as f64 / 1000f64,
        secs_to_human_time(self.elapsed_time())
      )
    }
  }
}

fn secs_to_human_time(mut secs: u64) -> String {
  let mut mins = secs / 60;
  secs %= 60;
  let hours = mins / 60;
  mins %= 60;
  if hours > 0 {
    format!("{hours}h {mins}m {secs}s")
  } else if mins > 0 {
    format!("{mins}m {secs}s")
  } else {
    format!("{secs}s")
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct QualityMetrics {
  /// Peak Signal-to-Noise Ratio for Y, U, and V planes
  pub psnr: Option<PlanarMetrics>,
  /// Peak Signal-to-Noise Ratio as perceived by the Human Visual System--
  /// taking into account Contrast Sensitivity Function (CSF)
  pub psnr_hvs: Option<PlanarMetrics>,
  /// Structural Similarity
  pub ssim: Option<PlanarMetrics>,
  /// Multi-Scale Structural Similarity
  pub ms_ssim: Option<PlanarMetrics>,
  /// CIEDE 2000 [color difference algorithm](https://en.wikipedia.org/wiki/Color_difference#CIEDE2000)
  pub ciede: Option<f64>,
  /// Aligned Peak Signal-to-Noise Ratio for Y, U, and V planes
  pub apsnr: Option<PlanarMetrics>,
  /// Netflix's Video Multimethod Assessment Fusion
  pub vmaf: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricsEnabled {
  /// Don't calculate any metrics.
  None,
  /// Calculate the PSNR of each plane, but no other metrics.
  Psnr,
  /// Calculate all implemented metrics. Currently implemented metrics match what is available via AWCY.
  All,
}

pub fn calculate_frame_metrics<T: Pixel>(
  frame1: &Frame<T>, frame2: &Frame<T>, bit_depth: usize, cs: ChromaSampling,
  metrics: MetricsEnabled,
) -> QualityMetrics {
  match metrics {
    MetricsEnabled::None => QualityMetrics::default(),
    MetricsEnabled::Psnr => QualityMetrics {
      psnr: Some(
        psnr::calculate_frame_psnr(frame1, frame2, bit_depth, cs).unwrap(),
      ),
      ..Default::default()
    },
    MetricsEnabled::All => {
      let mut metrics = QualityMetrics {
        psnr: Some(
          psnr::calculate_frame_psnr(frame1, frame2, bit_depth, cs).unwrap(),
        ),
        psnr_hvs: Some(
          psnr_hvs::calculate_frame_psnr_hvs(frame1, frame2, bit_depth, cs)
            .unwrap(),
        ),
        ..Default::default()
      };
      let ssim = ssim::calculate_frame_ssim(frame1, frame2, bit_depth, cs);
      metrics.ssim = Some(ssim.unwrap());
      let ms_ssim =
        ssim::calculate_frame_msssim(frame1, frame2, bit_depth, cs);
      metrics.ms_ssim = Some(ms_ssim.unwrap());
      let ciede = ciede::calculate_frame_ciede(frame1, frame2, bit_depth, cs);
      metrics.ciede = Some(ciede.unwrap());
      // TODO APSNR
      // TODO VMAF
      metrics
    }
  }
}
