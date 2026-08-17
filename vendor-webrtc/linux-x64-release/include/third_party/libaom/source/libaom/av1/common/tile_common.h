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

#ifndef AOM_AV1_COMMON_TILE_COMMON_H_
#define AOM_AV1_COMMON_TILE_COMMON_H_

#include <stdbool.h>

#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AV1Common;
struct SequenceHeader;
struct CommonTileParams;

#define DEFAULT_MAX_NUM_TG 1

typedef struct TileInfo {
  int mi_row_start, mi_row_end;
  int mi_col_start, mi_col_end;
  int tile_row;
  int tile_col;
} TileInfo;

// initializes 'tile->mi_(row|col)_(start|end)' for (row, col) based on
// 'cm->log2_tile_(rows|cols)' & 'cm->mi_(rows|cols)'
void av1_tile_init(TileInfo *tile, const struct AV1Common *cm, int row,
                   int col);

void av1_tile_set_row(TileInfo *tile, const struct AV1Common *cm, int row);
void av1_tile_set_col(TileInfo *tile, const struct AV1Common *cm, int col);

int av1_get_sb_rows_in_tile(const struct AV1Common *cm, const TileInfo *tile);
int av1_get_sb_cols_in_tile(const struct AV1Common *cm, const TileInfo *tile);

// Define tile maximum width and area
// There is no maximum height since height is limited by area and width limits
// The minimum tile width or height is fixed at one superblock
#define MAX_TILE_WIDTH (4096)        // Max Tile width in pixels
#define MAX_TILE_AREA (4096 * 2304)  // Maximum tile area in pixels
#if CONFIG_CWG_C013
#define MAX_TILE_AREA_LEVEL_7_AND_ABOVE (4096 * 4608)
#endif

// Gets the width and height (in units of MI_SIZE) of the tiles in a tile list.
// Returns true on success, false on failure.
bool av1_get_uniform_tile_size(const struct AV1Common *cm, int *w, int *h);
void av1_get_tile_limits(struct AV1Common *const cm);
void av1_calculate_tile_cols(const struct SequenceHeader *const seq_params,
                             int cm_mi_rows, int cm_mi_cols,
                             struct CommonTileParams *const tiles);
void av1_calculate_tile_rows(const struct SequenceHeader *const seq_params,
                             int cm_mi_rows,
                             struct CommonTileParams *const tiles);

// Checks if the minimum tile_width requirement is satisfied
int av1_is_min_tile_width_satisfied(const struct AV1Common *cm);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_TILE_COMMON_H_
