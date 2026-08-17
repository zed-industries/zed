/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_APPS_AOMENC_H_
#define AOM_APPS_AOMENC_H_

#include "aom/aom_codec.h"
#include "aom/aom_encoder.h"
#include "av1/arg_defs.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  I420,  // 4:2:0 8+ bit-depth
  I422,  // 4:2:2 8+ bit-depth
  I444,  // 4:4:4 8+ bit-depth
  YV12,  // 4:2:0 with uv flipped, only 8-bit depth
  NV12,  // 4:2:0 with uv interleaved, only 8-bit depth
} ColorInputType;

/* Configuration elements common to all streams. */
struct AvxEncoderConfig {
  aom_codec_iface_t *codec;
  int passes;
  int pass;
  unsigned int usage;
  ColorInputType color_type;
  int quiet;
  int verbose;
  int limit;
  int skip_frames;
  int show_psnr;
  enum TestDecodeFatality test_decode;
  int have_framerate;
  struct aom_rational framerate;
  int debug;
  int show_q_hist_buckets;
  int show_rate_hist_buckets;
  int disable_warnings;
  int disable_warning_prompt;
  int experimental_bitstream;
  aom_chroma_sample_position_t csp;
  cfg_options_t encoder_config;
};

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_APPS_AOMENC_H_
