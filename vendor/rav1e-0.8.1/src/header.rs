// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::api::*;
use crate::context::*;
use crate::ec::*;
use crate::lrf::*;
use crate::partition::*;
use crate::tiling::MAX_TILE_WIDTH;
use crate::util::Fixed;
use crate::util::Pixel;

use crate::DeblockState;
use crate::FrameInvariants;
use crate::FrameState;
use crate::SegmentationState;
use crate::Sequence;

use arrayvec::ArrayVec;
use bitstream_io::{BigEndian, BitWrite, BitWriter, LittleEndian};

use std::io;

pub const PRIMARY_REF_NONE: u32 = 7;
pub const ALL_REF_FRAMES_MASK: u32 = (1 << REF_FRAMES) - 1;

const PRIMARY_REF_BITS: u32 = 3;

#[allow(unused)]
const OP_POINTS_IDC_BITS: usize = 12;
#[allow(unused)]
const LEVEL_MAJOR_MIN: usize = 2;
#[allow(unused)]
const LEVEL_MAJOR_BITS: usize = 3;
#[allow(unused)]
const LEVEL_MINOR_BITS: usize = 2;
#[allow(unused)]
const LEVEL_BITS: usize = LEVEL_MAJOR_BITS + LEVEL_MINOR_BITS;

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceMode {
  SINGLE = 0,
  COMPOUND = 1,
  SELECT = 2,
}

#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum ObuType {
  OBU_SEQUENCE_HEADER = 1,
  OBU_TEMPORAL_DELIMITER = 2,
  OBU_FRAME_HEADER = 3,
  OBU_TILE_GROUP = 4,
  OBU_METADATA = 5,
  OBU_FRAME = 6,
  OBU_REDUNDANT_FRAME_HEADER = 7,
  OBU_TILE_LIST = 8,
  OBU_PADDING = 15,
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum ObuMetaType {
  OBU_META_HDR_CLL = 1,
  OBU_META_HDR_MDCV = 2,
  OBU_META_SCALABILITY = 3,
  OBU_META_ITUT_T35 = 4,
  OBU_META_TIMECODE = 5,
}

impl ObuMetaType {
  const fn size(self) -> u64 {
    use self::ObuMetaType::*;
    match self {
      OBU_META_HDR_CLL => 4,
      OBU_META_HDR_MDCV => 24,
      _ => 0,
    }
  }
}

pub trait ULEB128Writer {
  fn write_uleb128(&mut self, payload: u64) -> io::Result<()>;
}

impl<W: io::Write> ULEB128Writer for BitWriter<W, BigEndian> {
  fn write_uleb128(&mut self, payload: u64) -> io::Result<()> {
    // NOTE from libaom:
    // Disallow values larger than 32-bits to ensure consistent behavior on 32 and
    // 64 bit targets: value is typically used to determine buffer allocation size
    // when decoded.
    let mut coded_value: ArrayVec<u8, 8> = ArrayVec::new();

    let mut value = payload as u32;
    loop {
      let mut byte = (value & 0x7f) as u8;
      value >>= 7u8;
      if value != 0 {
        // Signal that more bytes follow.
        byte |= 0x80;
      }
      coded_value.push(byte);

      if value == 0 {
        // We have to break at the end of the loop
        // because there must be at least one byte written.
        break;
      }
    }

    self.write_bytes(&coded_value)?;

    Ok(())
  }
}

pub trait LEWriter {
  fn write_le(&mut self, bytes: u32, payload: u64) -> io::Result<()>;
}

// to write little endian values in a globally big-endian BitWriter
impl<W: io::Write> LEWriter for BitWriter<W, BigEndian> {
  fn write_le(&mut self, bytes: u32, value: u64) -> io::Result<()> {
    let mut data = Vec::new();
    let mut bwle = BitWriter::endian(&mut data, LittleEndian);
    bwle.write_var(bytes * 8, value)?;
    self.write_bytes(&data)
  }
}

pub trait UncompressedHeader {
  // Start of OBU Headers
  fn write_obu_header(
    &mut self, obu_type: ObuType, obu_extension: u32,
  ) -> io::Result<()>;
  fn write_sequence_metadata_obu(
    &mut self, obu_meta_type: ObuMetaType, seq: &Sequence,
  ) -> io::Result<()>;
  fn write_sequence_header_obu<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_frame_header_obu<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, fs: &FrameState<T>,
    inter_cfg: &InterConfig,
  ) -> io::Result<()>;
  fn write_sequence_header<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_color_config(&mut self, seq: &Sequence) -> io::Result<()>;
  fn write_t35_metadata_obu(&mut self, t35: &T35) -> io::Result<()>;
  // End of OBU Headers

  fn write_max_frame_size<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_frame_size<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_render_size<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_frame_size_with_refs<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_deblock_filter_a<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, deblock: &DeblockState,
  ) -> io::Result<()>;
  fn write_deblock_filter_b<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, deblock: &DeblockState,
  ) -> io::Result<()>;
  fn write_frame_cdef<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()>;
  fn write_frame_lrf<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, rs: &RestorationState,
  ) -> io::Result<()>;
  fn write_segment_data<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, segmentation: &SegmentationState,
  ) -> io::Result<()>;
  fn write_delta_q(&mut self, delta_q: i8) -> io::Result<()>;
}

impl<W: io::Write> UncompressedHeader for BitWriter<W, BigEndian> {
  // Start of OBU Headers
  // Write OBU Header syntax
  fn write_obu_header(
    &mut self, obu_type: ObuType, obu_extension: u32,
  ) -> io::Result<()> {
    self.write_bit(false)?; // forbidden bit.
    self.write::<4, u8>(obu_type as u8)?;
    self.write_bit(obu_extension != 0)?;
    self.write_bit(true)?; // obu_has_payload_length_field
    self.write_bit(false)?; // reserved

    if obu_extension != 0 {
      unimplemented!();
      //self.write(8, obu_extension & 0xFF)?; size += 8;
    }

    Ok(())
  }

  fn write_sequence_metadata_obu(
    &mut self, obu_meta_type: ObuMetaType, seq: &Sequence,
  ) -> io::Result<()> {
    // header
    self.write_obu_header(ObuType::OBU_METADATA, 0)?;

    // uleb128() - length
    // we use a constant value to avoid computing the OBU size every time
    // since it is fixed (depending on the metadata)
    // +2 is for the metadata_type field and the trailing bits byte
    self.write_uleb128(obu_meta_type.size() + 2)?;

    // uleb128() - metadata_type (1 byte)
    self.write_uleb128(obu_meta_type as u64)?;

    match obu_meta_type {
      ObuMetaType::OBU_META_HDR_CLL => {
        let cll = seq.content_light.unwrap();
        self.write::<16, u16>(cll.max_content_light_level)?;
        self.write::<16, u16>(cll.max_frame_average_light_level)?;
      }
      ObuMetaType::OBU_META_HDR_MDCV => {
        let mdcv = seq.mastering_display.unwrap();
        for i in 0..3 {
          self.write::<16, u16>(mdcv.primaries[i].x)?;
          self.write::<16, u16>(mdcv.primaries[i].y)?;
        }

        self.write::<16, u16>(mdcv.white_point.x)?;
        self.write::<16, u16>(mdcv.white_point.y)?;

        self.write::<32, u32>(mdcv.max_luminance)?;
        self.write::<32, u32>(mdcv.min_luminance)?;
      }
      _ => {}
    }

    // trailing bits (1 byte)
    self.write_bit(true)?;
    self.byte_align()?;

    Ok(())
  }

  fn write_t35_metadata_obu(&mut self, t35: &T35) -> io::Result<()> {
    self.write_obu_header(ObuType::OBU_METADATA, 0)?;

    // metadata type + country code + optional extension + trailing bits
    self.write_uleb128(
      t35.data.len() as u64 + if t35.country_code == 0xFF { 4 } else { 3 },
    )?;

    self.write_uleb128(ObuMetaType::OBU_META_ITUT_T35 as u64)?;

    self.write::<8, u8>(t35.country_code)?;
    if t35.country_code == 0xFF {
      self.write::<8, u8>(t35.country_code_extension_byte)?;
    }
    self.write_bytes(&t35.data)?;

    // trailing bits (1 byte)
    self.write_bit(true)?;
    self.byte_align()?;

    Ok(())
  }

  fn write_sequence_header_obu<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    assert!(
      !fi.sequence.reduced_still_picture_hdr || fi.sequence.still_picture
    );

    self.write::<3, u8>(fi.sequence.profile)?; // profile
    self.write_bit(fi.sequence.still_picture)?; // still_picture
    self.write_bit(fi.sequence.reduced_still_picture_hdr)?; // reduced_still_picture_header

    assert!(fi.sequence.level_idx[0] <= 31);
    if fi.sequence.reduced_still_picture_hdr {
      assert!(!fi.sequence.timing_info_present);
      assert!(!fi.sequence.decoder_model_info_present_flag);
      assert_eq!(fi.sequence.operating_points_cnt_minus_1, 0);
      assert_eq!(fi.sequence.operating_point_idc[0], 0);
      self.write::<5, u8>(fi.sequence.level_idx[0])?; // level
      assert_eq!(fi.sequence.tier[0], 0);
    } else {
      self.write_bit(fi.sequence.timing_info_present)?; // timing info present

      if fi.sequence.timing_info_present {
        self.write::<32, u64>(fi.sequence.time_base.num)?;
        self.write::<32, u64>(fi.sequence.time_base.den)?;

        self.write_bit(true)?; // equal picture interval
        self.write_bit(true)?; // zero interval
        self.write_bit(false)?; // decoder model info present flag
      }

      self.write_bit(false)?; // initial display delay present flag
      self.write::<5, u8>(0)?; // one operating point
      self.write::<12, u16>(0)?; // idc
      self.write::<5, u8>(fi.sequence.level_idx[0])?; // level
      if fi.sequence.level_idx[0] > 7 {
        self.write::<1, u8>(0)?; // tier
      }
    }

    self.write_sequence_header(fi)?;

    self.write_color_config(&fi.sequence)?;

    self.write_bit(fi.sequence.film_grain_params_present)?;

    Ok(())
  }

  fn write_sequence_header<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    self.write_max_frame_size(fi)?;

    let seq = &fi.sequence;

    if seq.reduced_still_picture_hdr {
      assert!(!seq.frame_id_numbers_present_flag);
    } else {
      self.write_bit(seq.frame_id_numbers_present_flag)?;
    }

    if seq.frame_id_numbers_present_flag {
      // We must always have delta_frame_id_length < frame_id_length,
      // in order for a frame to be referenced with a unique delta.
      // Avoid wasting bits by using a coding that enforces this restriction.
      self.write::<4, u32>(seq.delta_frame_id_length - 2)?;
      self.write::<3, u32>(
        seq.frame_id_length - seq.delta_frame_id_length - 1,
      )?;
    }

    self.write_bit(seq.use_128x128_superblock)?;
    self.write_bit(seq.enable_filter_intra)?;
    self.write_bit(seq.enable_intra_edge_filter)?;

    if seq.reduced_still_picture_hdr {
      assert!(!seq.enable_interintra_compound);
      assert!(!seq.enable_masked_compound);
      assert!(!seq.enable_warped_motion);
      assert!(!seq.enable_dual_filter);
      assert!(!seq.enable_order_hint);
      assert!(!seq.enable_jnt_comp);
      assert!(!seq.enable_ref_frame_mvs);
      assert!(seq.force_screen_content_tools == 2);
      assert!(seq.force_integer_mv == 2);
    } else {
      self.write_bit(seq.enable_interintra_compound)?;
      self.write_bit(seq.enable_masked_compound)?;
      self.write_bit(seq.enable_warped_motion)?;
      self.write_bit(seq.enable_dual_filter)?;
      self.write_bit(seq.enable_order_hint)?;

      if seq.enable_order_hint {
        self.write_bit(seq.enable_jnt_comp)?;
        self.write_bit(seq.enable_ref_frame_mvs)?;
      }

      if seq.force_screen_content_tools == 2 {
        self.write_bit(true)?;
      } else {
        self.write_bit(false)?;
        self.write_bit(seq.force_screen_content_tools != 0)?;
      }
      if seq.force_screen_content_tools > 0 {
        if seq.force_integer_mv == 2 {
          self.write_bit(true)?;
        } else {
          self.write_bit(false)?;
          self.write_bit(seq.force_integer_mv != 0)?;
        }
      } else {
        assert!(seq.force_integer_mv == 2);
      }
      if seq.enable_order_hint {
        self.write::<3, u32>(seq.order_hint_bits_minus_1)?;
      }
    }

    self.write_bit(seq.enable_superres)?;
    self.write_bit(seq.enable_cdef)?;
    self.write_bit(seq.enable_restoration)?;

    Ok(())
  }

  // <https://aomediacodec.github.io/av1-spec/#color-config-syntax>
  fn write_color_config(&mut self, seq: &Sequence) -> io::Result<()> {
    let high_bitdepth = seq.bit_depth > 8;
    self.write_bit(high_bitdepth)?;
    if seq.profile == 2 && high_bitdepth {
      self.write_bit(seq.bit_depth == 12)?; // twelve_bit
    }

    let monochrome = seq.chroma_sampling == ChromaSampling::Cs400;
    if seq.profile == 1 {
      assert!(!monochrome);
    } else {
      self.write_bit(monochrome)?; // mono_chrome
    }

    // color_description_present_flag
    self.write_bit(seq.color_description.is_some())?;
    let mut srgb_triple = false;
    if let Some(color_description) = seq.color_description {
      self.write::<8, _>(color_description.color_primaries as u8)?;
      self.write::<8, _>(color_description.transfer_characteristics as u8)?;
      self.write::<8, _>(color_description.matrix_coefficients as u8)?;
      srgb_triple = color_description.is_srgb_triple();
    }

    if monochrome || !srgb_triple {
      self.write_bit(seq.pixel_range == PixelRange::Full)?; // color_range
    }
    if monochrome {
      return Ok(());
    } else if srgb_triple {
      assert!(seq.pixel_range == PixelRange::Full);
      assert!(seq.chroma_sampling == ChromaSampling::Cs444);
    } else {
      if seq.profile == 0 {
        assert!(seq.chroma_sampling == ChromaSampling::Cs420);
      } else if seq.profile == 1 {
        assert!(seq.chroma_sampling == ChromaSampling::Cs444);
      } else if seq.bit_depth == 12 {
        let subsampling_x = seq.chroma_sampling != ChromaSampling::Cs444;
        let subsampling_y = seq.chroma_sampling == ChromaSampling::Cs420;
        self.write_bit(subsampling_x)?;
        if subsampling_x {
          self.write_bit(subsampling_y)?;
        }
      } else {
        assert!(seq.chroma_sampling == ChromaSampling::Cs422);
      }
      if seq.chroma_sampling == ChromaSampling::Cs420 {
        self.write::<2, u8>(seq.chroma_sample_position as u8)?;
      }
    }
    self.write_bit(true)?; // separate_uv_delta_q

    Ok(())
  }

  #[allow(unused)]
  fn write_frame_header_obu<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, fs: &FrameState<T>,
    inter_cfg: &InterConfig,
  ) -> io::Result<()> {
    if fi.sequence.reduced_still_picture_hdr {
      assert!(!fi.is_show_existing_frame());
      assert!(fi.frame_type == FrameType::KEY);
      assert!(fi.show_frame);
      assert!(!fi.showable_frame);
    } else {
      self.write_bit(fi.is_show_existing_frame())?;

      if fi.is_show_existing_frame() {
        self.write::<3, u32>(fi.frame_to_show_map_idx)?;

        //TODO:
        /* temporal_point_info();
        if fi.sequence.decoder_model_info_present_flag &&
           timing_info.equal_picture_interval == 0 {
          // write frame_presentation_delay;
        }
        if fi.sequence.frame_id_numbers_present_flag {
          // write display_frame_id;
        }*/

        self.write_bit(true)?; // trailing bit
        self.byte_align()?;
        return Ok(());
      }

      self.write::<2, u8>(fi.frame_type as u8)?;
      self.write_bit(fi.show_frame)?; // show frame

      if fi.show_frame {
        //TODO:
        /* temporal_point_info();
        if fi.sequence.decoder_model_info_present_flag &&
           timing_info.equal_picture_interval == 0 {
          // write frame_presentation_delay;
        }*/
      } else {
        self.write_bit(fi.showable_frame)?;
      }

      if fi.error_resilient {
        assert!(fi.primary_ref_frame == PRIMARY_REF_NONE);
      }
      if fi.frame_type == FrameType::SWITCH {
        assert!(fi.error_resilient);
      } else if !(fi.frame_type == FrameType::KEY && fi.show_frame) {
        self.write_bit(fi.error_resilient)?; // error resilient
      }
    }

    self.write_bit(fi.disable_cdf_update)?;

    if fi.sequence.force_screen_content_tools == 2 {
      self.write_bit(fi.allow_screen_content_tools != 0)?;
    } else {
      assert!(
        fi.allow_screen_content_tools
          == fi.sequence.force_screen_content_tools
      );
    }

    if fi.allow_screen_content_tools > 0 {
      if fi.sequence.force_integer_mv == 2 {
        self.write_bit(fi.force_integer_mv != 0)?;
      } else {
        assert!(fi.force_integer_mv == fi.sequence.force_integer_mv);
      }
    }

    assert!(
      fi.force_integer_mv
        == u32::from(fi.frame_type == FrameType::KEY || fi.intra_only)
    );

    if fi.sequence.frame_id_numbers_present_flag {
      unimplemented!();

      //TODO:
      //let frame_id_len = fi.sequence.frame_id_length;
      //self.write(frame_id_len, fi.current_frame_id);
    }

    if fi.frame_type != FrameType::SWITCH
      && !fi.sequence.reduced_still_picture_hdr
    {
      self.write_bit(fi.frame_size_override_flag)?; // frame size overhead flag
    }

    if fi.sequence.enable_order_hint {
      let n = fi.sequence.order_hint_bits_minus_1 + 1;
      let mask = (1 << n) - 1;
      self.write_var(n, fi.order_hint & mask)?;
    }

    if !fi.error_resilient && !fi.intra_only {
      self.write::<PRIMARY_REF_BITS, u32>(fi.primary_ref_frame)?;
    }

    if fi.sequence.decoder_model_info_present_flag {
      unimplemented!();
    }

    if fi.frame_type == FrameType::KEY {
      if !fi.show_frame {
        // unshown keyframe (forward keyframe)
        unimplemented!();
        self.write_var(REF_FRAMES as u32, fi.refresh_frame_flags)?;
      } else {
        assert!(fi.refresh_frame_flags == ALL_REF_FRAMES_MASK);
      }
    } else if fi.frame_type == FrameType::SWITCH {
      assert!(fi.refresh_frame_flags == ALL_REF_FRAMES_MASK);
    } else {
      // Inter frame info goes here
      if fi.intra_only {
        assert!(fi.refresh_frame_flags != ALL_REF_FRAMES_MASK);
      } else {
        // TODO: This should be set once inter mode is used
      }
      self.write_var(REF_FRAMES as u32, fi.refresh_frame_flags)?;
    };

    if (!fi.intra_only || fi.refresh_frame_flags != ALL_REF_FRAMES_MASK) {
      // Write all ref frame order hints if error_resilient_mode == 1
      if (fi.error_resilient && fi.sequence.enable_order_hint) {
        for i in 0..REF_FRAMES {
          let n = fi.sequence.order_hint_bits_minus_1 + 1;
          let mask = (1 << n) - 1;
          if let Some(ref rec) = fi.rec_buffer.frames[i] {
            let ref_hint = rec.order_hint;
            self.write_var(n, ref_hint & mask)?;
          } else {
            self.write_var(n, 0)?;
          }
        }
      }
    }

    // if KEY or INTRA_ONLY frame
    if fi.intra_only {
      self.write_frame_size(fi)?;
      self.write_render_size(fi)?;
      if fi.allow_screen_content_tools != 0 {
        // TODO: && UpscaledWidth == FrameWidth.
        self.write_bit(fi.allow_intrabc)?;
      }
    }

    let frame_refs_short_signaling = false;
    if fi.frame_type == FrameType::KEY || fi.intra_only {
      // Done by above
    } else {
      if fi.sequence.enable_order_hint {
        self.write_bit(frame_refs_short_signaling)?;
        if frame_refs_short_signaling {
          unimplemented!();
        }
      }

      for i in 0..INTER_REFS_PER_FRAME {
        if !frame_refs_short_signaling {
          self.write_var(REF_FRAMES_LOG2 as u32, fi.ref_frames[i])?;
        }
        if fi.sequence.frame_id_numbers_present_flag {
          unimplemented!();
        }
      }

      if !fi.error_resilient && fi.frame_size_override_flag {
        self.write_frame_size_with_refs(fi)?;
      } else {
        self.write_frame_size(fi)?;
        self.write_render_size(fi)?;
      }

      if fi.force_integer_mv == 0 {
        self.write_bit(fi.allow_high_precision_mv);
      }

      self.write_bit(fi.is_filter_switchable)?;
      if !fi.is_filter_switchable {
        self.write::<2, u8>(fi.default_filter as u8)?;
      }
      self.write_bit(fi.is_motion_mode_switchable)?;

      if (!fi.error_resilient && fi.sequence.enable_ref_frame_mvs) {
        self.write_bit(fi.use_ref_frame_mvs)?;
      }
    }

    if fi.sequence.reduced_still_picture_hdr || fi.disable_cdf_update {
      assert!(fi.disable_frame_end_update_cdf);
    } else {
      self.write_bit(fi.disable_frame_end_update_cdf)?;
    }

    // tile
    // <https://aomediacodec.github.io/av1-spec/#tile-info-syntax>

    // Can we use the uniform spacing tile syntax?  'Uniform spacing'
    // is a slight misnomer; it's more constrained than just a uniform
    // spacing.
    let ti = &fi.sequence.tiling;

    if fi.sb_width.align_power_of_two_and_shift(ti.tile_cols_log2)
      == ti.tile_width_sb
      && fi.sb_height.align_power_of_two_and_shift(ti.tile_rows_log2)
        == ti.tile_height_sb
    {
      // yes; our actual tile width/height setting (which is always
      // currently uniform) also matches the constrained width/height
      // calculation implicit in the uniform spacing flag.

      self.write_bit(true)?; // uniform_tile_spacing_flag

      let cols_ones = ti.tile_cols_log2 - ti.min_tile_cols_log2;
      for _ in 0..cols_ones {
        self.write_bit(true);
      }
      if ti.tile_cols_log2 < ti.max_tile_cols_log2 {
        self.write_bit(false);
      }

      let rows_ones = ti.tile_rows_log2 - ti.min_tile_rows_log2;
      for _ in 0..rows_ones {
        self.write_bit(true);
      }
      if ti.tile_rows_log2 < ti.max_tile_rows_log2 {
        self.write_bit(false);
      }
    } else {
      self.write_bit(false)?; // uniform_tile_spacing_flag
      let mut sofar = 0;
      let mut widest_tile_sb = 0;
      for _ in 0..ti.cols {
        let max = (MAX_TILE_WIDTH
          >> if fi.sequence.use_128x128_superblock { 7 } else { 6 })
        .min(fi.sb_width - sofar) as u16;
        let this_sb_width = ti.tile_width_sb.min(fi.sb_width - sofar);
        self.write_quniform(max, (this_sb_width - 1) as u16);
        sofar += this_sb_width;
        widest_tile_sb = widest_tile_sb.max(this_sb_width);
      }

      let max_tile_area_sb = if ti.min_tiles_log2 > 0 {
        (fi.sb_height * fi.sb_width) >> (ti.min_tiles_log2 + 1)
      } else {
        fi.sb_height * fi.sb_width
      };

      let max_tile_height_sb = (max_tile_area_sb / widest_tile_sb).max(1);

      sofar = 0;
      for i in 0..ti.rows {
        let max = max_tile_height_sb.min(fi.sb_height - sofar) as u16;
        let this_sb_height = ti.tile_height_sb.min(fi.sb_height - sofar);

        self.write_quniform(max, (this_sb_height - 1) as u16);
        sofar += this_sb_height;
      }
    }

    let tiles_log2 = ti.tile_cols_log2 + ti.tile_rows_log2;
    if tiles_log2 > 0 {
      // context_update_tile_id
      // for now, always use the first tile CDF
      self.write_var(tiles_log2 as u32, fs.context_update_tile_id as u32)?;

      // tile_size_bytes_minus_1
      self.write::<2, u32>(fs.max_tile_size_bytes - 1)?;
    }

    // quantization
    assert!(fi.base_q_idx > 0);
    self.write::<8, u8>(fi.base_q_idx)?; // base_q_idx
    self.write_delta_q(fi.dc_delta_q[0])?;
    if fi.sequence.chroma_sampling != ChromaSampling::Cs400 {
      assert!(fi.ac_delta_q[0] == 0);
      let diff_uv_delta = fi.dc_delta_q[1] != fi.dc_delta_q[2]
        || fi.ac_delta_q[1] != fi.ac_delta_q[2];
      self.write_bit(diff_uv_delta)?;
      self.write_delta_q(fi.dc_delta_q[1])?;
      self.write_delta_q(fi.ac_delta_q[1])?;
      if diff_uv_delta {
        self.write_delta_q(fi.dc_delta_q[2])?;
        self.write_delta_q(fi.ac_delta_q[2])?;
      }
    }
    self.write_bit(false)?; // no qm

    // segmentation
    self.write_segment_data(fi, &fs.segmentation)?;

    // delta_q
    self.write_bit(false)?; // delta_q_present_flag: no delta q

    // delta_lf_params in the spec
    self.write_deblock_filter_a(fi, &fs.deblock)?;

    // code for features not yet implemented....

    // loop_filter_params in the spec
    self.write_deblock_filter_b(fi, &fs.deblock)?;

    // cdef
    self.write_frame_cdef(fi)?;

    // loop restoration
    self.write_frame_lrf(fi, &fs.restoration)?;

    self.write_bit(fi.tx_mode_select)?; // tx mode

    let mut reference_select = false;
    if !fi.intra_only {
      reference_select = fi.reference_mode != ReferenceMode::SINGLE;
      self.write_bit(reference_select)?;
    }

    let skip_mode_allowed =
      fi.sequence.get_skip_mode_allowed(fi, inter_cfg, reference_select);
    if skip_mode_allowed {
      self.write_bit(false)?; // skip_mode_present
    }

    if fi.intra_only || fi.error_resilient || !fi.sequence.enable_warped_motion
    {
    } else {
      self.write_bit(fi.allow_warped_motion)?; // allow_warped_motion
    }

    self.write_bit(fi.use_reduced_tx_set)?; // reduced tx

    // global motion
    if !fi.intra_only {
      for i in 0..7 {
        let mode = fi.globalmv_transformation_type[i];
        self.write_bit(mode != GlobalMVMode::IDENTITY)?;
        if mode != GlobalMVMode::IDENTITY {
          self.write_bit(mode == GlobalMVMode::ROTZOOM)?;
          if mode != GlobalMVMode::ROTZOOM {
            self.write_bit(mode == GlobalMVMode::TRANSLATION)?;
          }
        }
        match mode {
          GlobalMVMode::IDENTITY => { /* Nothing to do */ }
          GlobalMVMode::TRANSLATION => {
            let mv_x = 0;
            let mv_x_ref = 0;
            let mv_y = 0;
            let mv_y_ref = 0;
            let bits = 12 - 6 + 3 - !fi.allow_high_precision_mv as u8;
            let bits_diff = 12 - 3 + fi.allow_high_precision_mv as u8;
            BCodeWriter::write_s_refsubexpfin(
              self,
              (1 << bits) + 1,
              3,
              mv_x_ref >> bits_diff,
              mv_x >> bits_diff,
            )?;
            BCodeWriter::write_s_refsubexpfin(
              self,
              (1 << bits) + 1,
              3,
              mv_y_ref >> bits_diff,
              mv_y >> bits_diff,
            )?;
          }
          GlobalMVMode::ROTZOOM => unimplemented!(),
          GlobalMVMode::AFFINE => unimplemented!(),
        };
      }
    }

    if fi.sequence.film_grain_params_present {
      if let Some(grain_params) = fi.film_grain_params() {
        // Apply grain
        self.write_bit(true)?;
        self.write::<16, u16>(grain_params.random_seed)?;
        if fi.frame_type == FrameType::INTER {
          // For the purposes of photon noise,
          // it's simpler to always update the params,
          // and the output will be the same.
          self.write_bit(true)?;
        }

        self.write::<4, u8>(grain_params.scaling_points_y.len() as u8)?;
        for point in &grain_params.scaling_points_y {
          self.write::<8, u8>(point[0])?;
          self.write::<8, u8>(point[1])?;
        }

        let chroma_scaling_from_luma =
          if fi.sequence.chroma_sampling != ChromaSampling::Cs400 {
            self.write_bit(grain_params.chroma_scaling_from_luma)?;
            grain_params.chroma_scaling_from_luma
          } else {
            false
          };
        if !(fi.sequence.chroma_sampling == ChromaSampling::Cs400
          || chroma_scaling_from_luma
          || (fi.sequence.chroma_sampling == ChromaSampling::Cs420
            && grain_params.scaling_points_y.is_empty()))
        {
          self.write::<4, u8>(grain_params.scaling_points_cb.len() as u8)?;
          for point in &grain_params.scaling_points_cb {
            self.write::<8, u8>(point[0])?;
            self.write::<8, u8>(point[1])?;
          }
          self.write::<4, u8>(grain_params.scaling_points_cr.len() as u8)?;
          for point in &grain_params.scaling_points_cr {
            self.write::<8, u8>(point[0])?;
            self.write::<8, u8>(point[1])?;
          }
        }

        self.write::<2, u8>(grain_params.scaling_shift - 8)?;
        self.write::<2, u8>(grain_params.ar_coeff_lag)?;

        let mut num_pos_luma =
          (2 * grain_params.ar_coeff_lag * (grain_params.ar_coeff_lag + 1))
            as usize;
        let mut num_pos_chroma;
        if !grain_params.scaling_points_y.is_empty() {
          num_pos_chroma = num_pos_luma + 1;
          for i in 0..num_pos_luma {
            self.write::<8, u8>(
              (grain_params.ar_coeffs_y[i] as i16 + 128) as u8,
            )?;
          }
        } else {
          num_pos_chroma = num_pos_luma;
        }

        if chroma_scaling_from_luma
          || !grain_params.scaling_points_cb.is_empty()
        {
          for i in 0..num_pos_chroma {
            self.write::<8, u8>(
              (grain_params.ar_coeffs_cb[i] as i16 + 128) as u8,
            )?;
          }
        }
        if chroma_scaling_from_luma
          || !grain_params.scaling_points_cr.is_empty()
        {
          for i in 0..num_pos_chroma {
            self.write::<8, u8>(
              (grain_params.ar_coeffs_cr[i] as i16 + 128) as u8,
            )?;
          }
        }

        self.write::<2, u8>(grain_params.ar_coeff_shift - 6)?;
        self.write::<2, u8>(grain_params.grain_scale_shift)?;
        if !grain_params.scaling_points_cb.is_empty() {
          self.write::<8, u8>(grain_params.cb_mult)?;
          self.write::<8, u8>(grain_params.cb_luma_mult)?;
          self.write::<9, u16>(grain_params.cb_offset)?;
        }
        if !grain_params.scaling_points_cr.is_empty() {
          self.write::<8, u8>(grain_params.cr_mult)?;
          self.write::<8, u8>(grain_params.cr_luma_mult)?;
          self.write::<9, u16>(grain_params.cr_offset)?;
        }
        self.write_bit(grain_params.overlap_flag)?;
        self.write_bit(fi.sequence.pixel_range == PixelRange::Limited)?;
      } else {
        // No film grain for this frame
        self.write_bit(false)?;
      }
    }

    if fi.large_scale_tile {
      unimplemented!();
    }
    self.byte_align()?;

    Ok(())
  }
  // End of OBU Headers

  fn write_max_frame_size<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    // width_bits and height_bits will have to be moved to the sequence header OBU
    // when we add support for it.
    let width = fi.width - 1;
    let height = fi.height - 1;
    let width_bits = log_in_base_2(width as u32) as u32 + 1;
    let height_bits = log_in_base_2(height as u32) as u32 + 1;
    assert!(width_bits <= 16);
    assert!(height_bits <= 16);
    self.write::<4, u32>(width_bits - 1)?;
    self.write::<4, u32>(height_bits - 1)?;
    self.write_var(width_bits, width as u16)?;
    self.write_var(height_bits, height as u16)?;
    Ok(())
  }

  fn write_frame_size<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    // width_bits and height_bits will have to be moved to the sequence header OBU
    // when we add support for it.
    if fi.frame_size_override_flag {
      let width = fi.width - 1;
      let height = fi.height - 1;
      let width_bits = log_in_base_2(width as u32) as u32 + 1;
      let height_bits = log_in_base_2(height as u32) as u32 + 1;
      assert!(width_bits <= 16);
      assert!(height_bits <= 16);
      self.write_var(width_bits, width as u16)?;
      self.write_var(height_bits, height as u16)?;
    }
    if fi.sequence.enable_superres {
      unimplemented!();
    }
    Ok(())
  }

  fn write_render_size<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    self.write_bit(fi.render_and_frame_size_different)?;
    if fi.render_and_frame_size_different {
      self.write::<16, u32>(fi.render_width - 1)?;
      self.write::<16, u32>(fi.render_height - 1)?;
    }
    Ok(())
  }

  fn write_frame_size_with_refs<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    let mut found_ref = false;
    for i in 0..INTER_REFS_PER_FRAME {
      if let Some(ref rec) = fi.rec_buffer.frames[fi.ref_frames[i] as usize] {
        if rec.width == fi.width as u32
          && rec.height == fi.height as u32
          && rec.render_width == fi.render_width
          && rec.render_height == fi.render_height
        {
          self.write_bit(true)?;
          found_ref = true;
          break;
        } else {
          self.write_bit(false)?;
        }
      } else {
        self.write_bit(false)?;
      }
    }
    if !found_ref {
      self.write_frame_size(fi)?;
      self.write_render_size(fi)?;
    } else if fi.sequence.enable_superres {
      unimplemented!();
    }
    Ok(())
  }

  fn write_deblock_filter_a<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, deblock: &DeblockState,
  ) -> io::Result<()> {
    if fi.delta_q_present {
      if !fi.allow_intrabc {
        self.write_bit(deblock.block_deltas_enabled)?;
      }
      if deblock.block_deltas_enabled {
        self.write::<2, u8>(deblock.block_delta_shift)?;
        self.write_bit(deblock.block_delta_multi)?;
      }
    }
    Ok(())
  }

  fn write_deblock_filter_b<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, deblock: &DeblockState,
  ) -> io::Result<()> {
    let planes = if fi.sequence.chroma_sampling == ChromaSampling::Cs400 {
      1
    } else {
      MAX_PLANES
    };
    assert!(deblock.levels[0] < 64);
    self.write::<6, u8>(deblock.levels[0])?; // loop deblocking filter level 0
    assert!(deblock.levels[1] < 64);
    self.write::<6, u8>(deblock.levels[1])?; // loop deblocking filter level 1
    if planes > 1 && (deblock.levels[0] > 0 || deblock.levels[1] > 0) {
      assert!(deblock.levels[2] < 64);
      self.write::<6, u8>(deblock.levels[2])?; // loop deblocking filter level 2
      assert!(deblock.levels[3] < 64);
      self.write::<6, u8>(deblock.levels[3])?; // loop deblocking filter level 3
    }
    self.write::<3, u8>(deblock.sharpness)?; // deblocking filter sharpness
    self.write_bit(deblock.deltas_enabled)?; // loop deblocking filter deltas enabled
    if deblock.deltas_enabled {
      self.write_bit(deblock.delta_updates_enabled)?; // deltas updates enabled
      if deblock.delta_updates_enabled {
        // conditionally write ref delta updates
        let prev_ref_deltas = if fi.primary_ref_frame == PRIMARY_REF_NONE {
          [1, 0, 0, 0, 0, -1, -1, -1]
        } else {
          fi.rec_buffer.deblock
            [fi.ref_frames[fi.primary_ref_frame as usize] as usize]
            .ref_deltas
        };
        for i in 0..REF_FRAMES {
          let update = deblock.ref_deltas[i] != prev_ref_deltas[i];
          self.write_bit(update)?;
          if update {
            self.write_signed::<7, i8>(deblock.ref_deltas[i])?;
          }
        }
        // conditionally write mode delta updates
        let prev_mode_deltas = if fi.primary_ref_frame == PRIMARY_REF_NONE {
          [0, 0]
        } else {
          fi.rec_buffer.deblock
            [fi.ref_frames[fi.primary_ref_frame as usize] as usize]
            .mode_deltas
        };
        for i in 0..2 {
          let update = deblock.mode_deltas[i] != prev_mode_deltas[i];
          self.write_bit(update)?;
          if update {
            self.write_signed::<7, i8>(deblock.mode_deltas[i])?;
          }
        }
      }
    }
    Ok(())
  }

  fn write_frame_cdef<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>,
  ) -> io::Result<()> {
    if fi.sequence.enable_cdef && !fi.allow_intrabc {
      assert!(fi.cdef_damping >= 3);
      assert!(fi.cdef_damping <= 6);
      self.write::<2, u8>(fi.cdef_damping - 3)?;
      assert!(fi.cdef_bits < 4);
      self.write::<2, u8>(fi.cdef_bits)?; // cdef bits
      for i in 0..(1 << fi.cdef_bits) {
        assert!(fi.cdef_y_strengths[i] < 64);
        assert!(fi.cdef_uv_strengths[i] < 64);
        self.write::<6, u8>(fi.cdef_y_strengths[i])?; // cdef y strength
        if fi.sequence.chroma_sampling != ChromaSampling::Cs400 {
          self.write::<6, u8>(fi.cdef_uv_strengths[i])?; // cdef uv strength
        }
      }
    }
    Ok(())
  }

  fn write_frame_lrf<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, rs: &RestorationState,
  ) -> io::Result<()> {
    if fi.sequence.enable_restoration && !fi.allow_intrabc {
      // && !self.lossless
      let planes = if fi.sequence.chroma_sampling == ChromaSampling::Cs400 {
        1
      } else {
        MAX_PLANES
      };
      let mut use_lrf = false;
      let mut use_chroma_lrf = false;
      for i in 0..planes {
        self.write::<2, u8>(rs.planes[i].cfg.lrf_type)?; // filter type by plane
        if rs.planes[i].cfg.lrf_type != RESTORE_NONE {
          use_lrf = true;
          if i > 0 {
            use_chroma_lrf = true;
          }
        }
      }
      if use_lrf {
        // The Y shift value written here indicates shift up from superblock size
        if !fi.sequence.use_128x128_superblock {
          self.write::<1, u8>(u8::from(rs.planes[0].cfg.unit_size > 64))?;
        }

        if rs.planes[0].cfg.unit_size > 64 {
          self.write::<1, u8>(u8::from(rs.planes[0].cfg.unit_size > 128))?;
        }

        if use_chroma_lrf
          && fi.sequence.chroma_sampling == ChromaSampling::Cs420
        {
          self.write::<1, u8>(u8::from(
            rs.planes[0].cfg.unit_size > rs.planes[1].cfg.unit_size,
          ))?;
        }
      }
    }
    Ok(())
  }

  fn write_segment_data<T: Pixel>(
    &mut self, fi: &FrameInvariants<T>, segmentation: &SegmentationState,
  ) -> io::Result<()> {
    assert_eq!(fi.enable_segmentation, segmentation.enabled);
    self.write_bit(fi.enable_segmentation)?;

    if segmentation.enabled {
      if fi.primary_ref_frame == PRIMARY_REF_NONE {
        assert!(segmentation.update_map);
        assert!(segmentation.update_data);
      } else {
        self.write_bit(segmentation.update_map)?;
        if segmentation.update_map {
          self.write_bit(false)?; /* Without using temporal prediction */
        }
        self.write_bit(segmentation.update_data)?;
      }
      if segmentation.update_data {
        for i in 0..8 {
          for j in 0..SegLvl::SEG_LVL_MAX as usize {
            self.write_bit(segmentation.features[i][j])?;
            if segmentation.features[i][j] {
              let bits = seg_feature_bits[j];
              let data = segmentation.data[i][j];
              if seg_feature_is_signed[j] {
                self.write_signed_var(bits + 1, data)?;
              } else {
                self.write_var(bits, data)?;
              }
            }
          }
        }
      }
    }
    Ok(())
  }

  fn write_delta_q(&mut self, delta_q: i8) -> io::Result<()> {
    self.write_bit(delta_q != 0)?;
    if delta_q != 0 {
      assert!((-63..=63).contains(&delta_q));
      self.write_signed::<7, i8>(delta_q)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::ULEB128Writer;
  use bitstream_io::{BigEndian, BitWriter};
  use nom::error::Error;
  use nom::IResult;
  use quickcheck::quickcheck;

  fn leb128(mut input: &[u8]) -> IResult<&[u8], u64, Error<&[u8]>> {
    use nom::bytes::complete::take;

    let mut value = 0u64;
    for i in 0..8u8 {
      let result = take(1usize)(input)?;
      input = result.0;
      let leb128_byte = result.1[0];
      value |= u64::from(leb128_byte & 0x7f) << (i * 7);
      if (leb128_byte & 0x80) == 0 {
        break;
      }
    }
    Ok((input, value))
  }

  quickcheck! {
    fn validate_leb128_write(val: u32) -> bool {
      let mut buf1 = Vec::new();
      let mut bw1 = BitWriter::endian(&mut buf1, BigEndian);
      bw1.write_uleb128(val as u64).unwrap();
      let result = leb128(&buf1).unwrap();
      u64::from(val) == result.1 && result.0.is_empty()
    }
  }
}
