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
#ifndef AOM_COMMON_WEBMENC_H_
#define AOM_COMMON_WEBMENC_H_

#include <stdio.h>
#include <stdlib.h>

#include "tools_common.h"
#include "aom/aom_encoder.h"

#ifdef __cplusplus
extern "C" {
#endif

struct WebmOutputContext {
  int debug;
  FILE *stream;
  int64_t last_pts_ns;
  void *writer;
  void *segment;
};

/* Stereo 3D packed frame format */
enum {
  STEREO_FORMAT_MONO = 0,
  STEREO_FORMAT_LEFT_RIGHT = 1,
  STEREO_FORMAT_BOTTOM_TOP = 2,
  STEREO_FORMAT_TOP_BOTTOM = 3,
  STEREO_FORMAT_RIGHT_LEFT = 11
} UENUM1BYTE(stereo_format_t);

// Simplistic mechanism to extract encoder settings, without having
// to re-invoke the entire flag-parsing logic. It lists the codec version
// and then copies the arguments as-is from argv, but skips the binary name,
// any arguments that match the input filename, and the output flags "-o"
// and "--output" (and the following argument for those flags). The caller
// is responsible for free-ing the returned string. If there is insufficient
// memory, it returns nullptr.
char *extract_encoder_settings(const char *version, const char **argv, int argc,
                               const char *input_fname);

// The following functions wrap libwebm's mkvmuxer. All functions return 0 upon
// success, or -1 upon failure.

int write_webm_file_header(struct WebmOutputContext *webm_ctx,
                           aom_codec_ctx_t *encoder_ctx,
                           const aom_codec_enc_cfg_t *cfg,
                           stereo_format_t stereo_fmt, unsigned int fourcc,
                           const struct AvxRational *par,
                           const char *encoder_settings);

int write_webm_block(struct WebmOutputContext *webm_ctx,
                     const aom_codec_enc_cfg_t *cfg,
                     const aom_codec_cx_pkt_t *pkt);

int write_webm_file_footer(struct WebmOutputContext *webm_ctx);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_COMMON_WEBMENC_H_
