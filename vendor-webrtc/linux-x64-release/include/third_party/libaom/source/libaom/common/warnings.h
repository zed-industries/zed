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
#ifndef AOM_COMMON_WARNINGS_H_
#define AOM_COMMON_WARNINGS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct aom_codec_enc_cfg;
struct AvxEncoderConfig;

/*
 * Checks config for improperly used settings. Warns user upon encountering
 * settings that will lead to poor output quality. Prompts user to continue
 * when warnings are issued.
 */
void check_encoder_config(int disable_prompt,
                          const struct AvxEncoderConfig *global_config,
                          const struct aom_codec_enc_cfg *stream_config);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_COMMON_WARNINGS_H_
