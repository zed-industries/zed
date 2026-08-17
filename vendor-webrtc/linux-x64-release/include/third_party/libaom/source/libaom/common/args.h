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

#ifndef AOM_COMMON_ARGS_H_
#define AOM_COMMON_ARGS_H_
#include <stdio.h>

#include "aom/aom_codec.h"
#include "aom/aom_encoder.h"
#include "common/args_helper.h"

#ifdef __cplusplus
extern "C" {
#endif

int arg_match(struct arg *arg_, const struct arg_def *def, char **argv);
int parse_cfg(const char *file, cfg_options_t *config);
const char *arg_next(struct arg *arg);
void arg_show_usage(FILE *fp, const struct arg_def *const *defs);
char **argv_dup(int argc, const char **argv);

unsigned int arg_parse_uint(const struct arg *arg);
int arg_parse_int(const struct arg *arg);
struct aom_rational arg_parse_rational(const struct arg *arg);
int arg_parse_enum(const struct arg *arg);
int arg_parse_enum_or_int(const struct arg *arg);
int arg_parse_list(const struct arg *arg, int *list, int n);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_COMMON_ARGS_H_
