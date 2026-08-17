// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::encoder::FrameInvariants;
use crate::prelude::*;

use std::sync::Arc;

use interpolate_name::interpolate_test;

fn setup_config(
  w: usize, h: usize, speed: u8, quantizer: usize, bit_depth: usize,
  chroma_sampling: ChromaSampling, min_keyint: u64, max_keyint: u64,
  bitrate: i32, low_latency: bool, switch_frame_interval: u64,
  no_scene_detection: bool, rdo_lookahead_frames: usize,
  min_quantizer: Option<u8>,
) -> Config {
  let mut enc = EncoderConfig::with_speed_preset(speed);
  enc.quantizer = quantizer;
  enc.min_key_frame_interval = min_keyint;
  enc.max_key_frame_interval = max_keyint;
  enc.low_latency = low_latency;
  enc.switch_frame_interval = switch_frame_interval;
  enc.width = w;
  enc.height = h;
  enc.bit_depth = bit_depth;
  enc.chroma_sampling = chroma_sampling;
  enc.bitrate = bitrate;
  if no_scene_detection {
    enc.speed_settings.scene_detection_mode = SceneDetectionSpeed::None;
  }
  enc.speed_settings.rdo_lookahead_frames = rdo_lookahead_frames;
  if let Some(min_quantizer) = min_quantizer {
    enc.min_quantizer = min_quantizer;
  }

  Config::new().with_encoder_config(enc).with_threads(1)
}

fn setup_encoder<T: Pixel>(
  w: usize, h: usize, speed: u8, quantizer: usize, bit_depth: usize,
  chroma_sampling: ChromaSampling, min_keyint: u64, max_keyint: u64,
  bitrate: i32, low_latency: bool, switch_frame_interval: u64,
  no_scene_detection: bool, rdo_lookahead_frames: usize,
  min_quantizer: Option<u8>,
) -> Context<T> {
  let cfg = setup_config(
    w,
    h,
    speed,
    quantizer,
    bit_depth,
    chroma_sampling,
    min_keyint,
    max_keyint,
    bitrate,
    low_latency,
    switch_frame_interval,
    no_scene_detection,
    rdo_lookahead_frames,
    min_quantizer,
  );
  cfg.new_context().unwrap()
}

/*
fn fill_frame<T: Pixel>(ra: &mut ChaChaRng, frame: &mut Frame<T>) {
  for plane in frame.planes.iter_mut() {
    let stride = plane.cfg.stride;
    for row in plane.data.chunks_mut(stride) {
      for pixel in row {
        let v: u8 = ra.random();
        *pixel = T::cast_from(v);
      }
    }
  }
}
*/

fn fill_frame_const<T: Pixel>(frame: &mut Frame<T>, value: T) {
  for plane in frame.planes.iter_mut() {
    let stride = plane.cfg.stride;
    for row in plane.data.chunks_mut(stride) {
      for pixel in row {
        *pixel = value;
      }
    }
  }
}

#[cfg(feature = "channel-api")]
mod channel {
  use super::*;

  #[interpolate_test(low_latency_no_scene_change, true, true)]
  #[interpolate_test(reorder_no_scene_change, false, true)]
  #[interpolate_test(low_latency_scene_change_detection, true, false)]
  #[interpolate_test(reorder_scene_change_detection, false, false)]
  fn flush(low_lantency: bool, no_scene_detection: bool) {
    let cfg = setup_config(
      64,
      80,
      10,
      100,
      8,
      ChromaSampling::Cs420,
      150,
      200,
      0,
      low_lantency,
      0,
      no_scene_detection,
      10,
      None,
    );

    let limit = 41;

    let (mut sf, rp) = cfg.new_channel::<u8>().unwrap();

    for _ in 0..limit {
      let input = sf.new_frame();
      let _ = sf.send(input);
    }

    drop(sf);

    let mut count = 0;

    for _ in 0..limit {
      let _ = rp
        .recv()
        .map(|_| {
          eprintln!("Packet Received {}/{}", count, limit);
          count += 1;
        })
        .unwrap();
    }

    assert_eq!(limit, count);
  }
}

#[interpolate_test(low_latency_no_scene_change, true, true)]
#[interpolate_test(reorder_no_scene_change, false, true)]
#[interpolate_test(low_latency_scene_change_detection, true, false)]
#[interpolate_test(reorder_scene_change_detection, false, false)]
fn flush(low_lantency: bool, no_scene_detection: bool) {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    150,
    200,
    0,
    low_lantency,
    0,
    no_scene_detection,
    10,
    None,
  );
  let limit = 41;

  for _ in 0..limit {
    let input = ctx.new_frame();
    let _ = ctx.send_frame(input);
  }

  ctx.flush();

  let mut count = 0;

  'out: for _ in 0..limit {
    loop {
      match ctx.receive_packet() {
        Ok(_) => {
          eprintln!("Packet Received {}/{}", count, limit);
          count += 1;
        }
        Err(EncoderStatus::EnoughData) => {
          eprintln!("{:?}", EncoderStatus::EnoughData);

          break 'out;
        }
        Err(e) => {
          eprintln!("{:?}", e);
          break;
        }
      }
    }
  }

  assert_eq!(limit, count);
}

#[interpolate_test(low_latency_no_scene_change, true, true)]
#[interpolate_test(reorder_no_scene_change, false, true)]
#[interpolate_test(low_latency_scene_change_detection, true, false)]
#[interpolate_test(reorder_scene_change_detection, false, false)]
fn flush_unlimited(low_lantency: bool, no_scene_detection: bool) {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    150,
    200,
    0,
    low_lantency,
    0,
    no_scene_detection,
    10,
    None,
  );
  let limit = 41;

  for _ in 0..limit {
    let input = ctx.new_frame();
    let _ = ctx.send_frame(input);
  }

  ctx.flush();

  let mut count = 0;

  'out: for _ in 0..limit {
    loop {
      match ctx.receive_packet() {
        Ok(_) => {
          eprintln!("Packet Received {}/{}", count, limit);
          count += 1;
        }
        Err(EncoderStatus::EnoughData) => {
          eprintln!("{:?}", EncoderStatus::EnoughData);

          break 'out;
        }
        Err(e) => {
          eprintln!("{:?}", e);
          break;
        }
      }
    }
  }

  assert_eq!(limit, count);
}

fn send_frames<T: Pixel>(
  ctx: &mut Context<T>, limit: u64, scene_change_at: u64,
) {
  for i in 0..limit {
    if i < scene_change_at {
      send_test_frame(ctx, T::min_value());
    } else {
      send_test_frame(ctx, T::max_value());
    }
  }
}

fn send_test_frame<T: Pixel>(ctx: &mut Context<T>, content_value: T) {
  let mut input = ctx.new_frame();
  fill_frame_const(&mut input, content_value);
  let _ = ctx.send_frame(Arc::new(input));
}

fn get_frame_invariants<T: Pixel>(
  ctx: Context<T>,
) -> impl Iterator<Item = Option<FrameInvariants<T>>> {
  ctx.inner.frame_data.into_values().map(|v| v.map(|v| v.fi))
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
fn output_frameno_low_latency_minus(missing: u64) {
  // Test output_frameno configurations when there are <missing> less frames
  // than the perfect subgop size, in no-reorder mode.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    true,
    0,
    true,
    10,
    None,
  );
  let limit = 10 - missing;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match missing {
      0 => {
        &[
          Some(0), // I-frame
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          Some(6),
          Some(7),
          Some(8),
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[test]
fn switch_frame_interval() {
  // Test output_frameno configurations when there are <missing> less frames
  // than the perfect subgop size, in no-reorder mode.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    true,
    2,
    true,
    10,
    None,
  );
  let limit = 10;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| (fi.input_frameno, fi.frame_type)))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    &[
      Some((0, FrameType::KEY)),
      Some((1, FrameType::INTER)),
      Some((2, FrameType::SWITCH)),
      Some((3, FrameType::INTER)),
      Some((4, FrameType::SWITCH)),
      Some((5, FrameType::KEY)),
      Some((6, FrameType::INTER)),
      Some((7, FrameType::SWITCH)),
      Some((8, FrameType::INTER)),
      Some((9, FrameType::SWITCH)),
    ][..]
  );
}

#[test]
fn minimum_frame_delay() {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    true,
    0,
    true,
    1,
    None,
  );

  let limit = 4; // 4 frames in for 1 frame out (delay of 3 frames)
  send_frames(&mut ctx, limit, 0);

  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| (fi.input_frameno, fi.frame_type)))
    .collect::<Vec<_>>();

  assert_eq!(&data[..], &[Some((0, FrameType::KEY))][..]);
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
fn pyramid_level_low_latency_minus(missing: u64) {
  // Test pyramid_level configurations when there are <missing> less frames
  // than the perfect subgop size, in no-reorder mode.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    true,
    0,
    true,
    10,
    None,
  );
  let limit = 10 - missing;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  // data[output_frameno] = pyramid_level

  assert!(get_frame_invariants(ctx)
    .map(|fi| fi.unwrap().pyramid_level)
    .all(|pyramid_level| pyramid_level == 0));
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn output_frameno_reorder_minus(missing: u64) {
  // Test output_frameno configurations when there are <missing> less frames
  // than the perfect subgop size.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    false,
    0,
    true,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 10 - missing;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  // data[output_frameno] = (input_frameno, !invalid)
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match missing {
      0 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          Some(9),
          Some(7),
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          None,
          Some(7),
          Some(6),
          Some(7),
          Some(8),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          None,
          Some(7),
          Some(6),
          Some(7),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          None,
          None,
          Some(6),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn pyramid_level_reorder_minus(missing: u64) {
  // Test pyramid_level configurations when there are <missing> less frames
  // than the perfect subgop size.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    false,
    0,
    true,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 10 - missing;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  // data[output_frameno] = pyramid_level
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.pyramid_level))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match missing {
      0 => {
        &[
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0), // I-frame
          None,
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0), // I-frame
          None,
          Some(1),
          Some(2),
          Some(1),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0), // I-frame
          None,
          None,
          Some(2),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0), // I-frame
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn output_frameno_reorder_scene_change_at(scene_change_at: u64) {
  // Test output_frameno configurations when there's a scene change at the
  // <scene_change_at>th frame.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 10;
  send_frames(&mut ctx, limit, scene_change_at);
  ctx.flush();

  // data[output_frameno] = (input_frameno, !invalid)
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match scene_change_at {
      0 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5),
          Some(9),
          Some(7),
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(1), // I-frame
          Some(5),
          Some(3),
          Some(2),
          Some(3),
          Some(4),
          Some(5),
          Some(6),
          None,
          Some(8),
          Some(7),
          Some(8),
          Some(9),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          None,
          None,
          Some(1),
          None,
          None,
          None,
          Some(2), // I-frame
          Some(6),
          Some(4),
          Some(3),
          Some(4),
          Some(5),
          Some(6),
          Some(7),
          None,
          Some(9),
          Some(8),
          Some(9),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          None,
          Some(2),
          Some(1),
          Some(2),
          None,
          None,
          Some(3), // I-frame
          Some(7),
          Some(5),
          Some(4),
          Some(5),
          Some(6),
          Some(7),
          Some(8),
          None,
          None,
          Some(9),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          None,
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          None,
          Some(4), // I-frame
          Some(8),
          Some(6),
          Some(5),
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn pyramid_level_reorder_scene_change_at(scene_change_at: u64) {
  // Test pyramid_level configurations when there's a scene change at the
  // <scene_change_at>th frame.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 10;
  send_frames(&mut ctx, limit, scene_change_at);
  ctx.flush();

  // data[output_frameno] = pyramid_level
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.pyramid_level))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match scene_change_at {
      0 => {
        &[
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0),
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0),
          None,
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          None,
          None,
          Some(2),
          None,
          None,
          None,
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0),
          None,
          Some(1),
          Some(2),
          Some(1),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          None,
          Some(1),
          Some(2),
          Some(1),
          None,
          None,
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0),
          None,
          None,
          Some(2),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          None,
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          None,
          Some(0), // I-frame
          Some(0),
          Some(1),
          Some(2),
          Some(1),
          Some(2),
          Some(0),
          Some(0),
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn output_frameno_incremental_reorder_minus(missing: u64) {
  // Test output_frameno configurations when there are <missing> less frames
  // than the perfect subgop size, computing the lookahead data incrementally.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    5,
    5,
    0,
    false,
    0,
    true,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 10 - missing;
  for _ in 0..limit {
    send_frames(&mut ctx, 1, 0);
  }
  ctx.flush();

  // data[output_frameno] = (input_frameno, !invalid)
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match missing {
      0 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          Some(9),
          Some(7),
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          None,
          Some(7),
          Some(6),
          Some(7),
          Some(8),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          None,
          Some(7),
          Some(6),
          Some(7),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
          None,
          None,
          Some(6),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5), // I-frame
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn output_frameno_incremental_reorder_scene_change_at(scene_change_at: u64) {
  // Test output_frameno configurations when there's a scene change at the
  // <scene_change_at>th frame, computing the lookahead data incrementally.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 10;
  for i in 0..limit {
    send_frames(&mut ctx, 1, scene_change_at.saturating_sub(i));
  }
  ctx.flush();

  // data[output_frameno] = (input_frameno, !invalid)
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match scene_change_at {
      0 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
          Some(5),
          Some(9),
          Some(7),
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(1), // I-frame
          Some(5),
          Some(3),
          Some(2),
          Some(3),
          Some(4),
          Some(5),
          Some(6),
          None,
          Some(8),
          Some(7),
          Some(8),
          Some(9),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          None,
          None,
          Some(1),
          None,
          None,
          None,
          Some(2), // I-frame
          Some(6),
          Some(4),
          Some(3),
          Some(4),
          Some(5),
          Some(6),
          Some(7),
          None,
          Some(9),
          Some(8),
          Some(9),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          None,
          Some(2),
          Some(1),
          Some(2),
          None,
          None,
          Some(3), // I-frame
          Some(7),
          Some(5),
          Some(4),
          Some(5),
          Some(6),
          Some(7),
          Some(8),
          None,
          None,
          Some(9),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          None,
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          None,
          Some(4), // I-frame
          Some(8),
          Some(6),
          Some(5),
          Some(6),
          Some(7),
          Some(8),
          Some(9),
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

fn send_frame_kf<T: Pixel>(ctx: &mut Context<T>, keyframe: bool) {
  let input = ctx.new_frame();

  let frame_type_override =
    if keyframe { FrameTypeOverride::Key } else { FrameTypeOverride::No };

  let opaque = Some(Opaque::new(keyframe));

  let fp = FrameParameters {
    frame_type_override,
    opaque,
    t35_metadata: Box::new([]),
  };

  let _ = ctx.send_frame((input, fp));
}

#[test]
fn test_opaque_delivery() {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  let kf_at = 3;

  let limit = 5;
  for i in 0..limit {
    send_frame_kf(&mut ctx, kf_at == i);
  }
  ctx.flush();

  while let Ok(pkt) = ctx.receive_packet() {
    let Packet { opaque, input_frameno, .. } = pkt;
    if let Some(opaque) = opaque {
      let kf = opaque.downcast::<bool>().unwrap();
      assert_eq!(kf, Box::new(input_frameno == kf_at));
    }
  }
}

fn send_frame_t35<T: Pixel>(ctx: &mut Context<T>) {
  let input = ctx.new_frame();

  let frame_type_override = FrameTypeOverride::No;

  let opaque = None;

  let t35_metadata = Box::new([T35 {
    country_code: 0xFF,
    country_code_extension_byte: 0x00,
    data: Box::new(*b"AYAYA"),
  }]);

  let fp = FrameParameters { frame_type_override, opaque, t35_metadata };

  let _ = ctx.send_frame((input, fp));
}

#[test]
fn test_t35_parameter() {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  let limit = 2;
  for _ in 0..limit {
    send_frame_t35(&mut ctx);
  }
  ctx.flush();

  while ctx.receive_packet().is_ok() {}
}

#[interpolate_test(0, 0)]
#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
#[interpolate_test(4, 4)]
fn output_frameno_incremental_reorder_keyframe_at(kf_at: u64) {
  // Test output_frameno configurations when there's a forced keyframe at the
  // <kf_at>th frame, computing the lookahead data incrementally.

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 5;
  for i in 0..limit {
    send_frame_kf(&mut ctx, kf_at == i);
  }
  ctx.flush();

  // data[output_frameno] = (input_frameno, !invalid)
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    match kf_at {
      0 => {
        &[
          Some(0), // I-frame
          Some(4),
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          Some(4),
        ][..]
      }
      1 => {
        &[
          Some(0), // I-frame
          Some(1), // I-frame
          None,
          Some(3),
          Some(2),
          Some(3),
          Some(4),
          None,
        ][..]
      }
      2 => {
        &[
          Some(0), // I-frame
          None,
          None,
          Some(1),
          None,
          None,
          None,
          Some(2), // I-frame
          None,
          Some(4),
          Some(3),
          Some(4),
          None,
          None,
        ][..]
      }
      3 => {
        &[
          Some(0), // I-frame
          None,
          Some(2),
          Some(1),
          Some(2),
          None,
          None,
          Some(3), // I-frame
          None,
          None,
          Some(4),
          None,
          None,
          None,
        ][..]
      }
      4 => {
        &[
          Some(0), // I-frame
          None,
          Some(2),
          Some(1),
          Some(2),
          Some(3),
          None,
          Some(4), // I-frame
        ][..]
      }
      _ => unreachable!(),
    }
  );
}

#[interpolate_test(1, 1)]
#[interpolate_test(2, 2)]
#[interpolate_test(3, 3)]
fn output_frameno_no_scene_change_at_short_flash(flash_at: u64) {
  // Test output_frameno configurations when there's a single-frame flash at the
  // <flash_at>th frame.
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    5,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);

  let limit = 5;
  for i in 0..limit {
    if i == flash_at {
      send_test_frame(&mut ctx, u8::MIN);
    } else {
      send_test_frame(&mut ctx, u8::MAX);
    }
  }
  ctx.flush();

  // data[output_frameno] = (input_frameno, !invalid)
  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    &[
      Some(0), // I-frame
      Some(4),
      Some(2),
      Some(1),
      Some(2),
      Some(3),
      Some(4),
    ]
  );
}

#[test]
fn output_frameno_no_scene_change_at_flash_smaller_than_max_len_flash() {
  // Test output_frameno configurations when there's a multi-frame flash
  // with length equal to the max flash length

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    10,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);
  assert_eq!(ctx.inner.inter_cfg.group_input_len, 4);

  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  ctx.flush();

  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    &[
      Some(0), // I-frame
      Some(4),
      Some(2),
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      None,
      Some(6),
      Some(5),
      Some(6),
      Some(7),
      None,
    ]
  );
}

#[test]
fn output_frameno_scene_change_before_flash_longer_than_max_flash_len() {
  // Test output_frameno configurations when there's a multi-frame flash
  // with length greater than the max flash length

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    10,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);
  assert_eq!(ctx.inner.inter_cfg.group_input_len, 4);

  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MAX);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  ctx.flush();

  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    &[
      Some(0), // I-frame
      None,
      None,
      Some(1),
      None,
      None,
      None,
      Some(2), // I-frame
      Some(6),
      Some(4),
      Some(3),
      Some(4),
      Some(5),
      Some(6),
      Some(10),
      Some(8),
      Some(7),
      Some(8),
      Some(9),
      Some(10),
      None,
      None,
      Some(11),
      None,
      None,
      None,
    ]
  );
}

#[test]
fn output_frameno_scene_change_after_multiple_flashes() {
  // Test output_frameno configurations when there are multiple consecutive flashes

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    10,
    0,
    false,
    0,
    false,
    10,
    None,
  );

  // TODO: when we support more pyramid depths, this test will need tweaks.
  assert_eq!(ctx.inner.inter_cfg.pyramid_depth, 2);
  assert_eq!(ctx.inner.inter_cfg.group_input_len, 4);

  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, u8::MIN);
  send_test_frame(&mut ctx, 40);
  send_test_frame(&mut ctx, 100);
  send_test_frame(&mut ctx, 160);
  send_test_frame(&mut ctx, 240);
  send_test_frame(&mut ctx, 240);
  send_test_frame(&mut ctx, 240);
  send_test_frame(&mut ctx, 240);
  send_test_frame(&mut ctx, 240);
  send_test_frame(&mut ctx, 240);
  ctx.flush();

  let data = get_frame_invariants(ctx)
    .map(|fi| fi.map(|fi| fi.input_frameno))
    .collect::<Vec<_>>();

  assert_eq!(
    &data[..],
    &[
      Some(0), // I-frame
      Some(4),
      Some(2),
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      Some(5),
      Some(9),
      Some(7),
      Some(6),
      Some(7),
      Some(8),
      Some(9),
      None,
      None,
      Some(10),
      None,
      None,
      None,
    ]
  );
}

#[derive(Clone, Copy)]
struct LookaheadTestExpectations {
  pre_receive_frame_q_lens: [usize; 60],
  pre_receive_fi_lens: [usize; 60],
  post_receive_frame_q_lens: [usize; 60],
  post_receive_fi_lens: [usize; 60],
}

#[test]
fn lookahead_size_properly_bounded_8() {
  const LOOKAHEAD_SIZE: usize = 8;
  const EXPECTATIONS: LookaheadTestExpectations = LookaheadTestExpectations {
    pre_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 19, 20, 20, 21, 19, 20, 20, 21, 19, 20, 20, 21, 19, 20, 20, 21, 19,
      20, 20, 21, 19, 20, 20, 21, 19, 20, 20, 21, 19, 20, 20, 21, 19, 20, 20,
      21, 19, 20, 20,
    ],
    pre_receive_fi_lens: [
      0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 7, 7, 7, 7, 13, 13, 13, 13, 19, 19, 19,
      14, 20, 19, 19, 14, 20, 19, 19, 14, 20, 19, 19, 14, 20, 19, 19, 14, 20,
      19, 19, 14, 20, 19, 19, 14, 20, 19, 19, 14, 20, 19, 19, 14, 20, 19, 19,
      14, 20, 19,
    ],
    post_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      18, 19, 19, 20, 18, 19, 19, 20, 18, 19, 19, 20, 18, 19, 19, 20, 18, 19,
      19, 20, 18, 19, 19, 20, 18, 19, 19, 20, 18, 19, 19, 20, 18, 19, 19, 20,
      18, 19, 19, 20,
    ],
    post_receive_fi_lens: [
      0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 7, 7, 7, 7, 13, 13, 13, 13, 19, 19, 14,
      14, 19, 19, 14, 14, 19, 19, 14, 14, 19, 19, 14, 14, 19, 19, 14, 14, 19,
      19, 14, 14, 19, 19, 14, 14, 19, 19, 14, 14, 19, 19, 14, 14, 19, 19, 14,
      14, 19, 19,
    ],
  };
  lookahead_size_properly_bounded(LOOKAHEAD_SIZE, false, &EXPECTATIONS);
}

#[test]
fn lookahead_size_properly_bounded_10() {
  const LOOKAHEAD_SIZE: usize = 10;
  const EXPECTATIONS: LookaheadTestExpectations = LookaheadTestExpectations {
    pre_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23, 20, 21, 22, 23, 20, 21, 22, 23, 20, 21, 22, 23, 20, 21, 22,
      23, 20, 21, 22, 23, 20, 21, 22, 23, 20, 21, 22, 23, 20, 21, 22, 23, 20,
      21, 22, 23, 20,
    ],
    pre_receive_fi_lens: [
      0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 7, 7, 7, 7, 13, 13, 13, 13, 19, 19, 19,
      19, 25, 19, 19, 19, 25, 19, 19, 19, 25, 19, 19, 19, 25, 19, 19, 19, 25,
      19, 19, 19, 25, 19, 19, 19, 25, 19, 19, 19, 25, 19, 19, 19, 25, 19, 19,
      19, 25, 19,
    ],
    post_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 19, 20, 21, 22, 19, 20, 21, 22, 19, 20, 21, 22, 19, 20, 21, 22,
      19, 20, 21, 22, 19, 20, 21, 22, 19, 20, 21, 22, 19, 20, 21, 22, 19, 20,
      21, 22, 19, 20,
    ],
    post_receive_fi_lens: [
      0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 7, 7, 7, 7, 13, 13, 13, 13, 19, 19, 19,
      19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19,
      19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19,
      19, 19, 19,
    ],
  };
  lookahead_size_properly_bounded(LOOKAHEAD_SIZE, false, &EXPECTATIONS);
}

#[test]
fn lookahead_size_properly_bounded_16() {
  const LOOKAHEAD_SIZE: usize = 16;
  const EXPECTATIONS: LookaheadTestExpectations = LookaheadTestExpectations {
    pre_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23, 24, 25, 26, 27, 28, 29, 27, 28, 28, 29, 27, 28, 28, 29, 27,
      28, 28, 29, 27, 28, 28, 29, 27, 28, 28, 29, 27, 28, 28, 29, 27, 28, 28,
      29, 27, 28, 28,
    ],
    pre_receive_fi_lens: [
      0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 7, 7, 7, 7, 13, 13, 13, 13, 19, 19, 19,
      19, 25, 25, 25, 25, 31, 31, 31, 26, 32, 31, 31, 26, 32, 31, 31, 26, 32,
      31, 31, 26, 32, 31, 31, 26, 32, 31, 31, 26, 32, 31, 31, 26, 32, 31, 31,
      26, 32, 31,
    ],
    post_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23, 24, 25, 26, 27, 28, 26, 27, 27, 28, 26, 27, 27, 28, 26, 27,
      27, 28, 26, 27, 27, 28, 26, 27, 27, 28, 26, 27, 27, 28, 26, 27, 27, 28,
      26, 27, 27, 28,
    ],
    post_receive_fi_lens: [
      0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 7, 7, 7, 7, 13, 13, 13, 13, 19, 19, 19,
      19, 25, 25, 25, 25, 31, 31, 26, 26, 31, 31, 26, 26, 31, 31, 26, 26, 31,
      31, 26, 26, 31, 31, 26, 26, 31, 31, 26, 26, 31, 31, 26, 26, 31, 31, 26,
      26, 31, 31,
    ],
  };
  lookahead_size_properly_bounded(LOOKAHEAD_SIZE, false, &EXPECTATIONS);
}

#[test]
fn lookahead_size_properly_bounded_lowlatency_8() {
  const LOOKAHEAD_SIZE: usize = 8;
  const EXPECTATIONS: LookaheadTestExpectations = LookaheadTestExpectations {
    pre_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 13, 13, 13, 13, 13, 13, 13,
      13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
      13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
      13, 13, 13, 13,
    ],
    pre_receive_fi_lens: [
      0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10,
      10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
      10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
      10, 10, 10,
    ],
    post_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12,
      12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
      12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
      12, 12, 12, 12,
    ],
    post_receive_fi_lens: [
      0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
      9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ],
  };
  lookahead_size_properly_bounded(LOOKAHEAD_SIZE, true, &EXPECTATIONS);
}

#[test]
fn lookahead_size_properly_bounded_lowlatency_1() {
  const LOOKAHEAD_SIZE: usize = 1;
  const EXPECTATIONS: LookaheadTestExpectations = LookaheadTestExpectations {
    pre_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
      6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
      6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    pre_receive_fi_lens: [
      0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
      3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
      3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    ],
    post_receive_frame_q_lens: [
      1, 2, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
      5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
      5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ],
    post_receive_fi_lens: [
      0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
      2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    ],
  };
  lookahead_size_properly_bounded(LOOKAHEAD_SIZE, true, &EXPECTATIONS);
}

fn lookahead_size_properly_bounded(
  rdo_lookahead: usize, low_latency: bool,
  expectations: &LookaheadTestExpectations,
) {
  // Test that lookahead reads in the proper number of frames at once

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    100,
    8,
    ChromaSampling::Cs420,
    0,
    100,
    0,
    low_latency,
    0,
    true,
    rdo_lookahead,
    None,
  );

  const LIMIT: usize = 60;

  let mut pre_receive_frame_q_lens = [0; LIMIT];
  let mut pre_receive_fi_lens = [0; LIMIT];
  let mut post_receive_frame_q_lens = [0; LIMIT];
  let mut post_receive_fi_lens = [0; LIMIT];

  for i in 0..LIMIT {
    let input = ctx.new_frame();
    let _ = ctx.send_frame(input);
    pre_receive_frame_q_lens[i] = ctx.inner.frame_q.len();
    pre_receive_fi_lens[i] = ctx.inner.frame_data.len();
    while ctx.receive_packet().is_ok() {
      // Receive packets until lookahead consumed, due to pyramids receiving frames in groups
    }
    post_receive_frame_q_lens[i] = ctx.inner.frame_q.len();
    post_receive_fi_lens[i] = ctx.inner.frame_data.len();
  }

  assert_eq!(
    &pre_receive_frame_q_lens[..],
    &expectations.pre_receive_frame_q_lens[..]
  );
  assert_eq!(&pre_receive_fi_lens[..], &expectations.pre_receive_fi_lens[..]);
  assert_eq!(
    &post_receive_frame_q_lens[..],
    &expectations.post_receive_frame_q_lens[..]
  );
  assert_eq!(
    &post_receive_fi_lens[..],
    &expectations.post_receive_fi_lens[..]
  );

  ctx.flush();
  let end = ctx.inner.frame_q.get(&(LIMIT as u64));
  assert!(end.is_some());
  assert!(end.unwrap().is_none());

  while let Ok(_) | Err(EncoderStatus::Encoded) = ctx.receive_packet() {
    // Receive packets until all frames consumed
  }
  assert_eq!(ctx.inner.frames_processed, LIMIT as u64);
}

#[test]
fn zero_frames() {
  let config = Config::default();
  let mut ctx: Context<u8> = config.new_context().unwrap();
  ctx.flush();
  assert_eq!(ctx.receive_packet(), Err(EncoderStatus::LimitReached));
}

#[test]
fn tile_cols_overflow() {
  let enc = EncoderConfig { tile_cols: usize::MAX, ..Default::default() };
  let config = Config::new().with_encoder_config(enc);
  let _: Result<Context<u8>, _> = config.new_context();
}

#[test]
fn max_key_frame_interval_overflow() {
  let enc = EncoderConfig {
    max_key_frame_interval: i32::MAX as u64,
    reservoir_frame_delay: None,
    ..Default::default()
  };
  let config = Config::new().with_encoder_config(enc);
  let _: Result<Context<u8>, _> = config.new_context();
}

#[test]
fn target_bitrate_overflow() {
  let enc = EncoderConfig {
    bitrate: i32::MAX,
    time_base: Rational::new(i64::MAX as u64, 1),
    ..Default::default()
  };
  let config = Config::new().with_encoder_config(enc);
  let _: Result<Context<u8>, _> = config.new_context();
}

#[test]
fn time_base_den_divide_by_zero() {
  let enc =
    EncoderConfig { time_base: Rational::new(1, 0), ..Default::default() };
  let config = Config::new().with_encoder_config(enc);
  let _: Result<Context<u8>, _> = config.new_context();
}

#[test]
fn large_width_assert() {
  let enc = EncoderConfig { width: u32::MAX as usize, ..Default::default() };
  let config = Config::new().with_encoder_config(enc);
  let _: Result<Context<u8>, _> = config.new_context();
}

#[test]
fn reservoir_max_overflow() {
  let enc = EncoderConfig {
    reservoir_frame_delay: Some(i32::MAX),
    bitrate: i32::MAX,
    time_base: Rational::new(i32::MAX as u64 * 2, 1),
    ..Default::default()
  };
  let config = Config::new().with_encoder_config(enc);
  let _: Result<Context<u8>, _> = config.new_context();
}

#[test]
fn zero_width() {
  let enc = EncoderConfig { width: 0, ..Default::default() };
  let config = Config::new().with_encoder_config(enc);
  let res: Result<Context<u8>, _> = config.new_context();
  assert!(res.is_err());
}

#[test]
fn rdo_lookahead_frames_overflow() {
  let enc = EncoderConfig {
    speed_settings: SpeedSettings {
      rdo_lookahead_frames: usize::MAX,
      ..Default::default()
    },
    ..Default::default()
  };
  let config = Config::new().with_encoder_config(enc);
  let res: Result<Context<u8>, _> = config.new_context();
  assert!(res.is_err());
}

#[test]
fn log_q_exp_overflow() {
  let enc = EncoderConfig {
    width: 16,
    height: 16,
    sample_aspect_ratio: Rational::new(1, 1),
    bit_depth: 8,
    chroma_sampling: ChromaSampling::Cs420,
    chroma_sample_position: ChromaSamplePosition::Unknown,
    pixel_range: PixelRange::Limited,
    color_description: None,
    mastering_display: None,
    content_light: None,
    level_idx: Some(31),
    enable_timing_info: false,
    still_picture: false,
    error_resilient: false,
    switch_frame_interval: 0,
    time_base: Rational { num: 1, den: 25 },
    min_key_frame_interval: 12,
    max_key_frame_interval: 240,
    reservoir_frame_delay: None,
    low_latency: false,
    quantizer: 100,
    min_quantizer: 64,
    bitrate: 1,
    tune: Tune::Psychovisual,
    film_grain_params: None,
    tile_cols: 0,
    tile_rows: 0,
    tiles: 0,
    speed_settings: SpeedSettings {
      multiref: false,
      fast_deblock: true,
      rdo_lookahead_frames: 40,
      scene_detection_mode: SceneDetectionSpeed::None,
      cdef: true,
      lrf: true,
      partition: PartitionSpeedSettings {
        partition_range: PartitionRange::new(
          BlockSize::BLOCK_64X64,
          BlockSize::BLOCK_64X64,
        ),
        encode_bottomup: false,
        non_square_partition_max_threshold: BlockSize::BLOCK_4X4,
      },
      transform: TransformSpeedSettings {
        reduced_tx_set: true,
        tx_domain_distortion: true,
        tx_domain_rate: false,
        rdo_tx_decision: false,
        ..Default::default()
      },
      prediction: PredictionSpeedSettings {
        prediction_modes: PredictionModesSetting::Simple,
        ..Default::default()
      },
      motion: MotionSpeedSettings {
        include_near_mvs: false,
        use_satd_subpel: false,
        ..Default::default()
      },
      ..Default::default()
    },
  };
  let config = Config::new().with_encoder_config(enc).with_threads(1);

  let mut ctx: Context<u8> = config.new_context().unwrap();
  for _ in 0..2 {
    ctx.send_frame(ctx.new_frame()).unwrap();
  }
  ctx.flush();

  ctx.receive_packet().unwrap();
  let _ = ctx.receive_packet();
}

#[test]
fn guess_frame_subtypes_assert() {
  let enc = EncoderConfig {
    width: 16,
    height: 16,
    sample_aspect_ratio: Rational::new(1, 1),
    bit_depth: 8,
    chroma_sampling: ChromaSampling::Cs420,
    chroma_sample_position: ChromaSamplePosition::Unknown,
    pixel_range: PixelRange::Limited,
    color_description: None,
    mastering_display: None,
    content_light: None,
    level_idx: Some(31),
    enable_timing_info: false,
    still_picture: false,
    error_resilient: false,
    switch_frame_interval: 0,
    time_base: Rational { num: 1, den: 25 },
    min_key_frame_interval: 0,
    max_key_frame_interval: 1,
    reservoir_frame_delay: None,
    low_latency: false,
    quantizer: 100,
    min_quantizer: 0,
    bitrate: 16384,
    tune: Tune::Psychovisual,
    film_grain_params: None,
    tile_cols: 0,
    tile_rows: 0,
    tiles: 0,
    speed_settings: SpeedSettings {
      multiref: false,
      fast_deblock: true,
      rdo_lookahead_frames: 40,
      scene_detection_mode: SceneDetectionSpeed::None,
      cdef: true,
      lrf: true,
      partition: PartitionSpeedSettings {
        partition_range: PartitionRange::new(
          BlockSize::BLOCK_64X64,
          BlockSize::BLOCK_64X64,
        ),
        encode_bottomup: false,
        non_square_partition_max_threshold: BlockSize::BLOCK_4X4,
      },
      transform: TransformSpeedSettings {
        reduced_tx_set: true,
        tx_domain_distortion: true,
        tx_domain_rate: false,
        rdo_tx_decision: false,
        ..Default::default()
      },
      prediction: PredictionSpeedSettings {
        prediction_modes: PredictionModesSetting::Simple,
        ..Default::default()
      },
      motion: MotionSpeedSettings {
        include_near_mvs: false,
        use_satd_subpel: false,
        ..Default::default()
      },
      ..Default::default()
    },
  };
  let config = Config::new().with_encoder_config(enc).with_threads(1);

  let mut ctx: Context<u8> = config.new_context().unwrap();
  ctx.send_frame(ctx.new_frame()).unwrap();
  ctx.flush();

  ctx.receive_packet().unwrap();
}

#[test]
fn min_quantizer_bounds_correctly() {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    255,
    8,
    ChromaSampling::Cs420,
    25,
    25,
    25000,
    true,
    0,
    true,
    1,
    Some(100),
  );

  let limit = 25;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  for i in 0..limit {
    ctx.inner.encode_packet(i).unwrap();
    let frame_data = ctx.inner.frame_data.get(&i).unwrap().as_ref().unwrap();
    if i == 0 {
      assert_eq!(68, frame_data.fi.base_q_idx);
    } else {
      assert_eq!(96, frame_data.fi.base_q_idx);
    }
  }

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    255,
    8,
    ChromaSampling::Cs420,
    25,
    25,
    2000,
    true,
    0,
    true,
    1,
    Some(100),
  );

  let limit = 25;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  for i in 0..limit {
    ctx.inner.encode_packet(i).unwrap();
    let frame_data = ctx.inner.frame_data.get(&i).unwrap().as_ref().unwrap();
    if i == 0 {
      assert!(frame_data.fi.base_q_idx > 68);
    } else {
      assert!(frame_data.fi.base_q_idx > 96);
    }
  }
}

#[test]
fn max_quantizer_bounds_correctly() {
  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    120,
    8,
    ChromaSampling::Cs420,
    25,
    25,
    2000,
    true,
    0,
    true,
    1,
    None,
  );

  let limit = 25;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  for i in 0..limit {
    ctx.inner.encode_packet(i).unwrap();
    let frame_data = ctx.inner.frame_data.get(&i).unwrap().as_ref().unwrap();
    if i == 0 {
      assert_eq!(95, frame_data.fi.base_q_idx);
    } else {
      assert_eq!(115, frame_data.fi.base_q_idx);
    }
  }

  let mut ctx = setup_encoder::<u8>(
    64,
    80,
    10,
    120,
    8,
    ChromaSampling::Cs420,
    25,
    25,
    20000,
    true,
    0,
    true,
    1,
    None,
  );

  let limit = 25;
  send_frames(&mut ctx, limit, 0);
  ctx.flush();

  for i in 0..limit {
    ctx.inner.encode_packet(i).unwrap();
    let frame_data = ctx.inner.frame_data.get(&i).unwrap().as_ref().unwrap();
    if i == 0 {
      assert!(frame_data.fi.base_q_idx < 95);
    } else {
      assert!(frame_data.fi.base_q_idx < 115);
    }
  }
}
