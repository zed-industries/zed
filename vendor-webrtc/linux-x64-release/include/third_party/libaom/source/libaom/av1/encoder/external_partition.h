/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_EXTERNAL_PARTITION_H_
#define AOM_AV1_ENCODER_EXTERNAL_PARTITION_H_

#include <stdbool.h>

#include "aom/aom_codec.h"
#include "aom/aom_external_partition.h"
#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif
/*!\cond */

typedef struct ExtPartController {
  int ready;
  int test_mode;
  aom_ext_part_config_t config;
  aom_ext_part_model_t model;
  aom_ext_part_funcs_t funcs;
} ExtPartController;

aom_codec_err_t av1_ext_part_create(aom_ext_part_funcs_t funcs,
                                    aom_ext_part_config_t config,
                                    ExtPartController *ext_part_controller);

aom_codec_err_t av1_ext_part_delete(ExtPartController *ext_part_controller);

bool av1_ext_part_get_partition_decision(ExtPartController *ext_part_controller,
                                         aom_partition_decision_t *decision);

bool av1_ext_part_send_features(ExtPartController *ext_part_controller,
                                const aom_partition_features_t *features);

#if CONFIG_PARTITION_SEARCH_ORDER
bool av1_ext_part_send_partition_stats(ExtPartController *ext_part_controller,
                                       const aom_partition_stats_t *stats);

aom_ext_part_decision_mode_t av1_get_ext_part_decision_mode(
    const ExtPartController *ext_part_controller);
#endif  // CONFIG_PARTITION_SEARCH_ORDER

/*!\endcond */
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_EXTERNAL_PARTITION_H_
