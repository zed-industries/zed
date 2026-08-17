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
use crate::util::{CastFromPrimitive, Pixel};
use log::debug;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::{
  mem::{self, MaybeUninit},
  ptr, slice,
};

use dav1d_sys::*;

pub(crate) struct Dav1dDecoder<T: Pixel> {
  dec: *mut Dav1dContext,
  pixel: PhantomData<T>,
}

impl<T: Pixel> TestDecoder<T> for Dav1dDecoder<T> {
  fn setup_decoder(_w: usize, _h: usize) -> Self {
    unsafe {
      let mut settings = MaybeUninit::uninit();
      dav1d_default_settings(settings.as_mut_ptr());

      // Was initialized by dav1d_default_settings().
      let settings = settings.assume_init();

      let mut dec: Dav1dDecoder<T> =
        Dav1dDecoder { dec: ptr::null_mut(), pixel: PhantomData };
      let ret = dav1d_open(&mut dec.dec, &settings);

      if ret != 0 {
        panic!("Cannot instantiate the decoder {}", ret);
      }

      dec
    }
  }

  fn decode_packet(
    &mut self, packet: &[u8], rec_fifo: &mut VecDeque<Frame<T>>, w: usize,
    h: usize, chroma_sampling: ChromaSampling, bit_depth: usize, verify: bool,
  ) -> DecodeResult {
    let mut corrupted_count = 0;
    let mut data = SafeDav1dData::new(packet);
    let ret = data.send(self.dec);
    debug!("Decoded. -> {}", ret);
    if ret != 0 {
      corrupted_count += 1;
    }

    if ret == 0 {
      loop {
        let mut pic = SafeDav1dPicture::default();
        debug!("Retrieving frame");
        let ret = pic.get(self.dec);
        debug!("Retrieved.");
        if ret == DAV1D_ERR_AGAIN {
          return DecodeResult::Done;
        }
        if ret != 0 {
          panic!("Decode fail");
        }

        if verify {
          let rec = rec_fifo.pop_front().unwrap();
          compare_pic(&pic.0, &rec, bit_depth, w, h, chroma_sampling);
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

impl<T: Pixel> Drop for Dav1dDecoder<T> {
  fn drop(&mut self) {
    unsafe { dav1d_close(&mut self.dec) };
  }
}

struct SafeDav1dData(Dav1dData);

impl SafeDav1dData {
  fn new(packet: &[u8]) -> Self {
    unsafe {
      let mut data = Self { 0: mem::zeroed() };
      let ptr = dav1d_data_create(&mut data.0, packet.len());
      ptr::copy_nonoverlapping(packet.as_ptr(), ptr, packet.len());
      data
    }
  }

  fn send(&mut self, context: *mut Dav1dContext) -> c_int {
    unsafe { dav1d_send_data(context, &mut self.0) }
  }
}

impl Drop for SafeDav1dData {
  fn drop(&mut self) {
    unsafe { dav1d_data_unref(&mut self.0) };
  }
}

struct SafeDav1dPicture(Dav1dPicture);

impl Default for SafeDav1dPicture {
  fn default() -> Self {
    Self { 0: unsafe { mem::zeroed() } }
  }
}

impl SafeDav1dPicture {
  fn get(&mut self, context: *mut Dav1dContext) -> c_int {
    unsafe { dav1d_get_picture(context, &mut self.0) }
  }
}

impl Drop for SafeDav1dPicture {
  fn drop(&mut self) {
    unsafe { dav1d_picture_unref(&mut self.0) }
  }
}

fn compare_pic<T: Pixel>(
  pic: &Dav1dPicture, frame: &Frame<T>, bit_depth: usize, width: usize,
  height: usize, chroma_sampling: ChromaSampling,
) {
  use crate::frame::Plane;

  let cmp_plane = |data, stride, frame_plane: &Plane<T>, pli| {
    let w = width >> frame_plane.cfg.xdec;
    let h = height >> frame_plane.cfg.ydec;
    let rec_stride = frame_plane.cfg.stride;

    if bit_depth > 8 {
      let stride = stride / 2;
      let dec = unsafe {
        let data = data as *const u16;
        let size = stride * h;

        slice::from_raw_parts(data, size)
      };

      let rec: Vec<u16> =
        frame_plane.data_origin().iter().map(|&v| u16::cast_from(v)).collect();

      compare_plane::<u16>(&rec[..], rec_stride, dec, stride, w, h, pli);
    } else {
      let dec = unsafe {
        let data = data as *const u8;
        let size = stride * h;

        slice::from_raw_parts(data, size)
      };

      let rec: Vec<u8> =
        frame_plane.data_origin().iter().map(|&v| u8::cast_from(v)).collect();

      compare_plane::<u8>(&rec[..], rec_stride, dec, stride, w, h, pli);
    }
  };

  let lstride = pic.stride[0] as usize;
  cmp_plane(pic.data[0], lstride, &frame.planes[0], 0);

  if chroma_sampling != ChromaSampling::Cs400 {
    let cstride = pic.stride[1] as usize;
    cmp_plane(pic.data[1], cstride, &frame.planes[1], 1);
    cmp_plane(pic.data[2], cstride, &frame.planes[2], 2);
  }
}
