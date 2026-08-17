/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_LEVEL_H_
#define AOM_AV1_ENCODER_LEVEL_H_

#include "av1/common/enums.h"

struct AV1_COMP;

// AV1 Level Specifications
typedef struct {
  AV1_LEVEL level;
  int max_picture_size;
  int max_h_size;
  int max_v_size;
  int max_header_rate;
  int max_tile_rate;
  int max_tiles;
  int max_tile_cols;
  int64_t max_display_rate;
  int64_t max_decode_rate;
  double main_mbps;
  double high_mbps;
  double main_cr;
  double high_cr;
} AV1LevelSpec;

typedef struct {
  int64_t ts_start;
  int64_t ts_end;
  size_t encoded_size_in_bytes;
  int pic_size;
  int frame_header_count;
  int tiles;
  int show_frame;
  int show_existing_frame;
} FrameRecord;

// Record frame info. in a rolling window.
#define FRAME_WINDOW_SIZE 256
typedef struct {
  FrameRecord buf[FRAME_WINDOW_SIZE];
  int num;    // Number of FrameRecord stored in the buffer.
  int start;  // Buffer index of the first FrameRecord.
} FrameWindowBuffer;

typedef struct {
  int max_bitrate;  // Max bitrate in any 1-second window, in bps.
  int max_tile_size;
  int max_superres_tile_width;
  int min_cropped_tile_width;
  int min_cropped_tile_height;
  int tile_width_is_valid;
  int min_frame_width;
  int min_frame_height;
  double total_compressed_size;  // In bytes.
  double total_time_encoded;     // In seconds.
  double min_cr;
} AV1LevelStats;

// The following data structures are for the decoder model.
typedef struct {
  int decoder_ref_count;
  int player_ref_count;
  int display_index;
  FRAME_TYPE frame_type;
  double presentation_time;
} FRAME_BUFFER;

// Interval of bits transmission for a DFG(Decodable Frame Group).
typedef struct {
  double first_bit_arrival_time;  // Time when the first bit arrives.
  double last_bit_arrival_time;   // Time when the last bit arrives.
  // Removal time means the time when the bits to be decoded are removed from
  // the smoothing buffer. Removal time is essentially the time when the
  // decoding of the frame starts.
  double removal_time;
} DFG_INTERVAL;

#define DFG_INTERVAL_QUEUE_SIZE 64
typedef struct {
  int head;
  int size;
  double total_interval;
  DFG_INTERVAL buf[DFG_INTERVAL_QUEUE_SIZE];
} DFG_INTERVAL_QUEUE;

enum {
  RESOURCE_MODE = 0,  // Resource availability mode.
  SCHEDULE_MODE       // Decoding schedule mode.
} UENUM1BYTE(DECODER_MODEL_MODE);

enum {
  DECODER_MODEL_OK = 0,
  DECODE_BUFFER_AVAILABLE_LATE,
  DECODE_FRAME_BUF_UNAVAILABLE,
  DECODE_EXISTING_FRAME_BUF_EMPTY,
  DISPLAY_FRAME_LATE,
  SMOOTHING_BUFFER_UNDERFLOW,
  SMOOTHING_BUFFER_OVERFLOW,
  DECODER_MODEL_DISABLED
} UENUM1BYTE(DECODER_MODEL_STATUS);

#define BUFFER_POOL_MAX_SIZE 10
typedef struct {
  DECODER_MODEL_STATUS status;
  DECODER_MODEL_MODE mode;
  bool is_low_delay_mode;
  AV1_LEVEL level;
  int encoder_buffer_delay;  // In units of 1/90000 seconds.
  int decoder_buffer_delay;  // In units of 1/90000 seconds.
  int num_ticks_per_picture;
  int initial_display_delay;  // In units of frames.
  int64_t decode_rate;
  double display_clock_tick;          // In units of seconds.
  double current_time;                // In units of seconds.
  double initial_presentation_delay;  // In units of seconds.
  double bit_rate;                    // Bits per second.

  int num_frame;
  int num_decoded_frame;
  int num_shown_frame;
  int vbi[REF_FRAMES];  // Virtual buffer index.
  FRAME_BUFFER frame_buffer_pool[BUFFER_POOL_MAX_SIZE];
  DFG_INTERVAL_QUEUE dfg_interval_queue;

  // Information for the DFG(Decodable Frame Group) being processed.
  double first_bit_arrival_time;
  double last_bit_arrival_time;
  size_t coded_bits;

  // Information for the frame being processed.
  double removal_time;
  double presentation_time;
  int decode_samples;
  int display_samples;

  double max_display_rate;
  double max_decode_rate;
} DECODER_MODEL;

typedef struct {
  AV1LevelStats level_stats;
  AV1LevelSpec level_spec;
  FrameWindowBuffer frame_window_buffer;
  DECODER_MODEL decoder_models[SEQ_LEVELS];
} AV1LevelInfo;

typedef struct AV1LevelParams {
  // Specifies the level that the coded video sequence conforms to for each
  // operating point.
  AV1_LEVEL target_seq_level_idx[MAX_NUM_OPERATING_POINTS];
  // Bit mask to indicate whether to keep level stats for corresponding
  // operating points.
  uint32_t keep_level_stats;
  // Level information for each operating point.
  AV1LevelInfo *level_info[MAX_NUM_OPERATING_POINTS];
} AV1LevelParams;

static inline int is_in_operating_point(int operating_point,
                                        int temporal_layer_id,
                                        int spatial_layer_id) {
  if (!operating_point) return 1;

  return ((operating_point >> temporal_layer_id) & 1) &&
         ((operating_point >> (spatial_layer_id + 8)) & 1);
}

void av1_init_level_info(struct AV1_COMP *cpi);

void av1_update_level_info(struct AV1_COMP *cpi, size_t size, int64_t ts_start,
                           int64_t ts_end);

// Return sequence level indices in seq_level_idx[MAX_NUM_OPERATING_POINTS].
aom_codec_err_t av1_get_seq_level_idx(const SequenceHeader *seq_params,
                                      const AV1LevelParams *level_params,
                                      int *seq_level_idx);

aom_codec_err_t av1_get_target_seq_level_idx(const SequenceHeader *seq_params,
                                             const AV1LevelParams *level_params,
                                             int *target_seq_level_idx);

// This function uses the decoder model to check whether there could be
// SMOOTHING_BUFFER_UNDERFLOW or SMOOTHING_BUFFER_OVERFLOW. It does not
// update the content of decoder_model, and can be used to target certain
// encoding level in the recode loop.
DECODER_MODEL_STATUS av1_decoder_model_try_smooth_buf(
    const struct AV1_COMP *const cpi, size_t coded_bits,
    const DECODER_MODEL *const decoder_model);

// Return max bitrate(bps) for given level.
double av1_get_max_bitrate_for_level(AV1_LEVEL level_index, int tier,
                                     BITSTREAM_PROFILE profile);

// Get max number of tiles and tile columns for given level.
void av1_get_max_tiles_for_level(AV1_LEVEL level_index, int *const max_tiles,
                                 int *const max_tile_cols);

// Return minimum compression ratio for given level.
double av1_get_min_cr_for_level(AV1_LEVEL level_index, int tier,
                                int is_still_picture);
#endif  // AOM_AV1_ENCODER_LEVEL_H_
