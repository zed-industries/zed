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

#ifndef AOM_STATS_AOMSTATS_H_
#define AOM_STATS_AOMSTATS_H_

#include <stdio.h>

#include "aom/aom_encoder.h"

#ifdef __cplusplus
extern "C" {
#endif

/* This structure is used to abstract the different ways of handling
 * first pass statistics
 */
typedef struct {
  aom_fixed_buf_t buf;
  int pass;
  FILE *file;
  char *buf_ptr;
  size_t buf_alloc_sz;
} stats_io_t;

int stats_open_file(stats_io_t *stats, const char *fpf, int pass);
int stats_open_mem(stats_io_t *stats, int pass);
void stats_close(stats_io_t *stats, int last_pass);
void stats_write(stats_io_t *stats, const void *pkt, size_t len);
aom_fixed_buf_t stats_get(stats_io_t *stats);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_STATS_AOMSTATS_H_
