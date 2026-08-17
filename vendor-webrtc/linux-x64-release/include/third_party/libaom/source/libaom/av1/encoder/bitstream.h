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

#ifndef AOM_AV1_ENCODER_BITSTREAM_H_
#define AOM_AV1_ENCODER_BITSTREAM_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"
#include "av1/common/enums.h"
#include "av1/encoder/level.h"
#include "aom_dsp/bitwriter.h"
#include "aom_util/aom_pthread.h"

struct aom_write_bit_buffer;
struct AV1_COMP;
struct ThreadData;

/*!\cond */

// Stores the location and size of a tile's data in the bitstream.  Used for
// later identifying identical tiles
typedef struct {
  uint8_t *data;
  size_t size;
} TileBufferEnc;

typedef struct {
  uint8_t *frame_header;
  size_t obu_header_byte_offset;
  size_t total_length;
} FrameHeaderInfo;

typedef struct {
  struct aom_write_bit_buffer *saved_wb;  // Bit stream buffer writer structure
  TileBufferEnc buf;     // Structure to hold bitstream buffer and size
  uint32_t *total_size;  // Size of the bitstream buffer for the tile in bytes
  uint8_t *dst;          // Base address of tile bitstream buffer
  uint8_t *tile_data_curr;   // Base address of tile-group bitstream buffer
  size_t tile_buf_size;      // Available bitstream buffer for the tile in bytes
  uint8_t obu_extn_header;   // Presence of OBU extension header
  uint32_t obu_header_size;  // Size of the OBU header
  int curr_tg_hdr_size;      // Size of the obu, tg, frame headers
  int tile_size_mi;          // Tile size in mi units
  int tile_row;              // Number of tile rows
  int tile_col;              // Number of tile columns
  int is_last_tile_in_tg;    // Flag to indicate last tile in a tile-group
  int new_tg;                // Flag to indicate starting of a new tile-group
} PackBSParams;

typedef struct {
  uint64_t abs_sum_level;
  uint16_t tile_idx;
} PackBSTileOrder;

// Pack bitstream data for pack bitstream multi-threading.
typedef struct {
#if CONFIG_MULTITHREAD
  // Mutex lock used while dispatching jobs.
  pthread_mutex_t *mutex_;
#endif
  // Tile order structure of pack bitstream multithreading.
  PackBSTileOrder pack_bs_tile_order[MAX_TILES];

  // Index of next job to be processed.
  int next_job_idx;
  // Initialized to false, set to true by the worker thread that encounters an
  // error in order to abort the processing of other worker threads.
  bool pack_bs_mt_exit;
} AV1EncPackBSSync;

/*!\endcond */

// Writes only the OBU Sequence Header payload, and returns the size of the
// payload written to 'dst'. This function does not write the OBU header, the
// optional extension, or the OBU size to 'dst'.
uint32_t av1_write_sequence_header_obu(const SequenceHeader *seq_params,
                                       uint8_t *const dst, size_t dst_size);

// Writes the OBU header byte, and the OBU header extension byte when both
// has_nonzero_operating_point_idc and is_layer_specific_obu are true.
// Returns number of bytes written to 'dst'.
uint32_t av1_write_obu_header(AV1LevelParams *const level_params,
                              int *frame_header_count, OBU_TYPE obu_type,
                              bool has_nonzero_operating_point_idc,
                              bool is_layer_specific_obu, int obu_extension,
                              uint8_t *const dst);

// Encodes obu_payload_size as a leb128 integer and writes it to the dest
// buffer. The output must fill the buffer exactly. Returns AOM_CODEC_OK on
// success, AOM_CODEC_ERROR on failure.
int av1_write_uleb_obu_size(size_t obu_payload_size, uint8_t *dest,
                            size_t dest_size);

// Pack tile data in the bitstream with tile_group, frame
// and OBU header.
void av1_pack_tile_info(struct AV1_COMP *const cpi, struct ThreadData *const td,
                        PackBSParams *const pack_bs_params);

void av1_write_last_tile_info(
    struct AV1_COMP *const cpi, const FrameHeaderInfo *fh_info,
    struct aom_write_bit_buffer *saved_wb, size_t *curr_tg_data_size,
    uint8_t *curr_tg_start, uint32_t *const total_size,
    uint8_t **tile_data_start, int *const largest_tile_id,
    int *const is_first_tg, uint32_t obu_header_size, uint8_t obu_extn_header);

/*!\brief Pack the bitstream for one frame
 *
 * \ingroup high_level_algo
 * \callgraph
 */
int av1_pack_bitstream(struct AV1_COMP *const cpi, uint8_t *dst,
                       size_t dst_size, size_t *size,
                       int *const largest_tile_id);

void av1_write_tx_type(const AV1_COMMON *const cm, const MACROBLOCKD *xd,
                       TX_TYPE tx_type, TX_SIZE tx_size, aom_writer *w);

void av1_reset_pack_bs_thread_data(struct ThreadData *const td);

void av1_accumulate_pack_bs_thread_data(struct AV1_COMP *const cpi,
                                        struct ThreadData const *td);

void av1_write_obu_tg_tile_headers(struct AV1_COMP *const cpi,
                                   MACROBLOCKD *const xd,
                                   PackBSParams *const pack_bs_params,
                                   const int tile_idx);

int av1_neg_interleave(int x, int ref, int max);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_BITSTREAM_H_
