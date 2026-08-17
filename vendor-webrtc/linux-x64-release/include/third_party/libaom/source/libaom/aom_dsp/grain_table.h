/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

/*!\file
 * \brief A table mapping from time to corresponding film grain parameters.
 *
 * In order to apply grain synthesis in the decoder, the film grain parameters
 * need to be signalled in the encoder. The film grain parameters are time
 * varying, and for two-pass encoding (and denoiser implementation flexibility)
 * it is common to denoise the video and do parameter estimation before encoding
 * the denoised video.
 *
 * The film grain table is used to provide this flexibility and is used as a
 * parameter that is passed to the encoder.
 *
 * Further, if regraining is to be done in say a single pass mode, or in two
 * pass within the encoder (before frames are added to the lookahead buffer),
 * this data structure can be used to keep track of on-the-fly estimated grain
 * parameters, that are then extracted from the table before the encoded frame
 * is written.
 */
#ifndef AOM_AOM_DSP_GRAIN_TABLE_H_
#define AOM_AOM_DSP_GRAIN_TABLE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "aom_dsp/grain_params.h"
#include "aom/internal/aom_codec_internal.h"

typedef struct aom_film_grain_table_entry_t {
  aom_film_grain_t params;
  int64_t start_time;
  int64_t end_time;
  struct aom_film_grain_table_entry_t *next;
} aom_film_grain_table_entry_t;

typedef struct {
  aom_film_grain_table_entry_t *head;
  aom_film_grain_table_entry_t *tail;
} aom_film_grain_table_t;

/*!\brief Add a mapping from [time_stamp, end_time) to the given grain
 * parameters
 *
 * \param[in,out] table      The grain table
 * \param[in]     time_stamp The start time stamp
 * \param[in]     end_stamp  The end time_stamp
 * \param[in]     grain      The grain parameters
 */
void aom_film_grain_table_append(aom_film_grain_table_t *table,
                                 int64_t time_stamp, int64_t end_time,
                                 const aom_film_grain_t *grain);

/*!\brief Look-up (and optionally erase) the grain parameters for the given time
 *
 * \param[in]  table      The grain table
 * \param[in]  time_stamp The start time stamp
 * \param[in]  end_stamp  The end time_stamp
 * \param[in]  erase      Whether the time segment can be deleted
 * \param[out] grain      The output grain parameters
 */
int aom_film_grain_table_lookup(aom_film_grain_table_t *t, int64_t time_stamp,
                                int64_t end_time, int erase,
                                aom_film_grain_t *grain);

/*!\brief Reads the grain table from a file.
 *
 * \param[out]  table       The grain table
 * \param[in]   filename    The file to read from
 * \param[in]   error_info  Error info for tracking errors
 */
aom_codec_err_t aom_film_grain_table_read(
    aom_film_grain_table_t *table, const char *filename,
    struct aom_internal_error_info *error_info);

/*!\brief Writes the grain table from a file.
 *
 * \param[out]  table       The grain table
 * \param[in]   filename    The file to read from
 * \param[in]   error_info  Error info for tracking errors
 */
aom_codec_err_t aom_film_grain_table_write(
    const aom_film_grain_table_t *t, const char *filename,
    struct aom_internal_error_info *error_info);

void aom_film_grain_table_free(aom_film_grain_table_t *t);

#ifdef __cplusplus
}
#endif

#endif  // AOM_AOM_DSP_GRAIN_TABLE_H_
