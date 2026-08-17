// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;
use crate::test_encode_decode::{compare_plane, DecodeResult, TestDecoder};
use crate::util::Pixel;
use aom_sys::*;
use log::debug;
use std::collections::VecDeque;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::{mem::MaybeUninit, ptr, slice};

pub(crate) struct AomDecoder<T: Pixel> {
  dec: aom_codec_ctx,
  iter: aom_codec_iter_t,
  pixel: PhantomData<T>,
}

impl<T: Pixel> TestDecoder<T> for AomDecoder<T> {
  fn setup_decoder(w: usize, h: usize) -> Self {
    unsafe {
      let interface = aom_codec_av1_dx();
      let cfg = aom_codec_dec_cfg_t {
        threads: 1,
        w: w as u32,
        h: h as u32,
        allow_lowbitdepth: 1,
      };

      let mut dec = MaybeUninit::uninit();
      let ret = aom_codec_dec_init_ver(
        dec.as_mut_ptr(),
        interface,
        &cfg,
        0,
        AOM_DECODER_ABI_VERSION as i32,
      );
      if ret != 0 {
        panic!("Cannot instantiate the decoder {}", ret);
      }

      // Was initialized by aom_codec_dec_init_ver().
      let dec = dec.assume_init();
      AomDecoder { dec, iter: ptr::null_mut(), pixel: PhantomData }
    }
  }

  fn decode_packet(
    &mut self, packet: &[u8], rec_fifo: &mut VecDeque<Frame<T>>, w: usize,
    h: usize, chroma_sampling: ChromaSampling, bit_depth: usize, verify: bool,
  ) -> DecodeResult {
    let mut corrupted_count = 0;
    unsafe {
      let ret = aom_codec_decode(
        &mut self.dec,
        packet.as_ptr(),
        packet.len(),
        ptr::null_mut(),
      );
      debug!("Decoded. -> {}", ret);
      if ret != 0 {
        let error_msg = aom_codec_error(&mut self.dec);
        debug!(
          "  Decode codec_decode failed: {}",
          CStr::from_ptr(error_msg).to_string_lossy()
        );
        let detail = aom_codec_error_detail(&mut self.dec);
        if !detail.is_null() {
          debug!(
            "  Decode codec_decode failed {}",
            CStr::from_ptr(detail).to_string_lossy()
          );
        }

        corrupted_count += 1;
      }

      if ret == 0 {
        loop {
          debug!("Retrieving frame");
          let img = aom_codec_get_frame(&mut self.dec, &mut self.iter);
          debug!("Retrieved.");
          if img.is_null() {
            return DecodeResult::Done;
          }
          let mut corrupted = 0;
          let ret = aom_codec_control(
            &mut self.dec,
            aom_dec_control_id::AOMD_GET_FRAME_CORRUPTED as i32,
            &mut corrupted,
          );
          if ret != 0 {
            let detail = aom_codec_error_detail(&mut self.dec);
            panic!(
              "Decode codec_control failed {}",
              CStr::from_ptr(detail).to_string_lossy()
            );
          }
          corrupted_count += corrupted;

          if verify {
            let rec = rec_fifo.pop_front().unwrap();
            compare_img(img, &rec, bit_depth, w, h, chroma_sampling);
          }
        }
      }
    }
    if corrupted_count > 0 {
      DecodeResult::Corrupted(corrupted_count)
    } else {
      DecodeResult::NotDone
    }
  }
}

impl<T: Pixel> Drop for AomDecoder<T> {
  fn drop(&mut self) {
    unsafe { aom_codec_destroy(&mut self.dec) };
  }
}

fn compare_img<T: Pixel>(
  img: *const aom_image_t, frame: &Frame<T>, bit_depth: usize, width: usize,
  height: usize, chroma_sampling: ChromaSampling,
) {
  let img = unsafe { *img };
  let img_iter = img.planes.iter().zip(img.stride.iter());
  let planes = if chroma_sampling == ChromaSampling::Cs400 { 1 } else { 3 };

  for (pli, (img_plane, frame_plane)) in
    img_iter.zip(frame.planes.iter()).enumerate().take(planes)
  {
    let w = width >> frame_plane.cfg.xdec;
    let h = height >> frame_plane.cfg.ydec;
    let rec_stride = frame_plane.cfg.stride;

    if bit_depth > 8 {
      let dec_stride = *img_plane.1 as usize / 2;

      let dec = unsafe {
        let data = *img_plane.0 as *const u16;
        let size = dec_stride * h;

        slice::from_raw_parts(data, size)
      };

      let rec: Vec<u16> =
        frame_plane.data_origin().iter().map(|&v| u16::cast_from(v)).collect();

      compare_plane::<u16>(&rec[..], rec_stride, dec, dec_stride, w, h, pli);
    } else {
      let dec_stride = *img_plane.1 as usize;

      let dec = unsafe {
        let data = *img_plane.0 as *const u8;
        let size = dec_stride * h;

        slice::from_raw_parts(data, size)
      };

      let rec: Vec<u8> =
        frame_plane.data_origin().iter().map(|&v| u8::cast_from(v)).collect();

      compare_plane::<u8>(&rec[..], rec_stride, dec, dec_stride, w, h, pli);
    }
  }
}
