/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_AV1_TEMPORAL_DENOISER_H_
#define AOM_AV1_ENCODER_AV1_TEMPORAL_DENOISER_H_

#include "av1/encoder/block.h"
#include "aom_scale/yv12config.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MOTION_MAGNITUDE_THRESHOLD (8 * 3)

// Denoiser is used in non svc real-time mode which does not use alt-ref, so no
// need to allocate for it, and hence we need MAX_REF_FRAME - 1
#define NONSVC_REF_FRAMES REF_FRAMES - 1

// Number of frame buffers when SVC is used. [0] for current denoised buffer and
// [1..8] for REF_FRAMES
#define SVC_REF_FRAMES 9

typedef enum av1_denoiser_decision {
  COPY_BLOCK,
  FILTER_BLOCK,
  FILTER_ZEROMV_BLOCK
} AV1_DENOISER_DECISION;

typedef enum av1_denoiser_level {
  kDenLowLow,
  kDenLow,
  kDenMedium,
  kDenHigh
} AV1_DENOISER_LEVEL;

typedef struct av1_denoiser {
  YV12_BUFFER_CONFIG *running_avg_y;
  YV12_BUFFER_CONFIG *mc_running_avg_y;
  YV12_BUFFER_CONFIG last_source;
  int frame_buffer_initialized;
  int reset;
  int num_ref_frames;
  int num_layers;
  unsigned int current_denoiser_frame;
  AV1_DENOISER_LEVEL denoising_level;
  AV1_DENOISER_LEVEL prev_denoising_level;
} AV1_DENOISER;

typedef struct {
  int64_t zero_last_cost_orig;
  unsigned int *ref_frame_cost;
  int_mv (*frame_mv)[REF_FRAMES];
  int reuse_inter_pred;
  TX_SIZE best_tx_size;
  PREDICTION_MODE best_mode;
  MV_REFERENCE_FRAME best_ref_frame;
  int_interpfilters best_pred_filter;
  uint8_t best_mode_skip_txfm;
} AV1_PICKMODE_CTX_DEN;

struct AV1_COMP;
struct SVC;
struct RTC_REF;

void av1_denoiser_update_frame_info(
    AV1_DENOISER *denoiser, YV12_BUFFER_CONFIG src, struct RTC_REF *rtc_ref,
    struct SVC *svc, FRAME_TYPE frame_type, int refresh_alt_ref_frame,
    int refresh_golden_frame, int refresh_last_frame, int alt_fb_idx,
    int gld_fb_idx, int lst_fb_idx, int resized,
    int svc_refresh_denoiser_buffers, int second_spatial_layer);

void av1_denoiser_denoise(struct AV1_COMP *cpi, MACROBLOCK *mb, int mi_row,
                          int mi_col, BLOCK_SIZE bs, PICK_MODE_CONTEXT *ctx,
                          AV1_DENOISER_DECISION *denoiser_decision,
                          int use_gf_temporal_ref);

void av1_denoiser_reset_frame_stats(PICK_MODE_CONTEXT *ctx);

void av1_denoiser_update_frame_stats(MB_MODE_INFO *mi, int64_t sse,
                                     PREDICTION_MODE mode,
                                     PICK_MODE_CONTEXT *ctx);

int av1_denoiser_realloc_svc(AV1_COMMON *cm, AV1_DENOISER *denoiser,
                             struct RTC_REF *rtc, struct SVC *svc,
                             int svc_buf_shift, int refresh_alt,
                             int refresh_gld, int refresh_lst, int alt_fb_idx,
                             int gld_fb_idx, int lst_fb_idx);

int av1_denoiser_alloc(AV1_COMMON *cm, struct SVC *svc, AV1_DENOISER *denoiser,
                       int use_svc, int noise_sen, int width, int height,
                       int ssx, int ssy, int use_highbitdepth, int border);

#if CONFIG_AV1_TEMPORAL_DENOISING
// This function is used by both c and sse2 denoiser implementations.
// Define it as a static function within the scope where av1_denoiser.h
// is referenced.
static inline int total_adj_strong_thresh(BLOCK_SIZE bs,
                                          int increase_denoising) {
  return (1 << num_pels_log2_lookup[bs]) * (increase_denoising ? 3 : 2);
}
#endif

void av1_denoiser_free(AV1_DENOISER *denoiser);

void av1_denoiser_set_noise_level(struct AV1_COMP *const cpi, int noise_level);

void av1_denoiser_reset_on_first_frame(struct AV1_COMP *const cpi);

int64_t av1_scale_part_thresh(int64_t threshold, AV1_DENOISER_LEVEL noise_level,
                              CONTENT_STATE_SB content_state,
                              int temporal_layer_id);

int64_t av1_scale_acskip_thresh(int64_t threshold,
                                AV1_DENOISER_LEVEL noise_level, int abs_sumdiff,
                                int temporal_layer_id);

void av1_denoiser_update_ref_frame(struct AV1_COMP *const cpi);

void aom_write_yuv_frame(FILE *yuv_file, YV12_BUFFER_CONFIG *s);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_AV1_TEMPORAL_DENOISER_H_
