/*
 * AAC decoder
 * Copyright (c) 2005-2006 Oded Shimon ( ods15 ods15 dyndns org )
 * Copyright (c) 2006-2007 Maxim Gavrilov ( maxim.gavrilov gmail com )
 * Copyright (c) 2008-2013 Alex Converse <alex.converse@gmail.com>
 *
 * AAC LATM decoder
 * Copyright (c) 2008-2010 Paul Kendall <paul@kcbbs.gen.nz>
 * Copyright (c) 2010      Janne Grunau <janne-libav@jannau.net>
 *
 * AAC decoder fixed-point implementation
 * Copyright (c) 2013
 *      MIPS Technologies, Inc., California.
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_AAC_AACDEC_FIXED_COUPLING_H
#define AVCODEC_AAC_AACDEC_FIXED_COUPLING_H

#include "aacdec.h"

/**
 * Apply dependent channel coupling (applied before IMDCT).
 *
 * @param   index   index into coupling gain array
 */
static void AAC_RENAME(apply_dependent_coupling)(AACDecContext *ac,
                                                 SingleChannelElement *target,
                                                 ChannelElement *cce, int index)
{
    IndividualChannelStream *ics = &cce->ch[0].ics;
    const uint16_t *offsets = ics->swb_offset;
    int *dest = target->coeffs_fixed;
    const int *src = cce->ch[0].coeffs_fixed;
    int g, i, group, k, idx = 0;
    if (ac->oc[1].m4ac.object_type == AOT_AAC_LTP) {
        av_log(ac->avctx, AV_LOG_ERROR,
               "Dependent coupling is not supported together with LTP\n");
        return;
    }
    for (g = 0; g < ics->num_window_groups; g++) {
        for (i = 0; i < ics->max_sfb; i++, idx++) {
            if (cce->ch[0].band_type[idx] != ZERO_BT) {
                const int gain = cce->coup.gain[index][idx];
                int shift, round, c, tmp;

                if (gain < 0) {
                    c = -cce_scale_fixed[-gain & 7];
                    shift = (-gain-1024) >> 3;
                }
                else {
                    c = cce_scale_fixed[gain & 7];
                    shift = (gain-1024) >> 3;
                }

                if (shift < -31) {
                    // Nothing to do
                } else if (shift < 0) {
                    shift = -shift;
                    round = 1 << (shift - 1);

                    for (group = 0; group < ics->group_len[g]; group++) {
                        for (k = offsets[i]; k < offsets[i + 1]; k++) {
                            tmp = (int)(((int64_t)src[group * 128 + k] * c + \
                                       (int64_t)0x1000000000) >> 37);
                            dest[group * 128 + k] += (tmp + (int64_t)round) >> shift;
                        }
                    }
                }
                else {
                    for (group = 0; group < ics->group_len[g]; group++) {
                        for (k = offsets[i]; k < offsets[i + 1]; k++) {
                            tmp = (int)(((int64_t)src[group * 128 + k] * c + \
                                        (int64_t)0x1000000000) >> 37);
                            dest[group * 128 + k] += tmp * (1U << shift);
                        }
                    }
                }
            }
        }
        dest += ics->group_len[g] * 128;
        src  += ics->group_len[g] * 128;
    }
}

/**
 * Apply independent channel coupling (applied after IMDCT).
 *
 * @param   index   index into coupling gain array
 */
static void AAC_RENAME(apply_independent_coupling)(AACDecContext *ac,
                                                   SingleChannelElement *target,
                                                   ChannelElement *cce, int index)
{
    int i, c, shift, round, tmp;
    const int gain = cce->coup.gain[index][0];
    const int *src = cce->ch[0].output_fixed;
    unsigned int *dest = target->output_fixed;
    const int len = 1024 << (ac->oc[1].m4ac.sbr == 1);

    c = cce_scale_fixed[gain & 7];
    shift = (gain-1024) >> 3;
    if (shift < -31) {
        return;
    } else if (shift < 0) {
        shift = -shift;
        round = 1 << (shift - 1);

        for (i = 0; i < len; i++) {
            tmp = (int)(((int64_t)src[i] * c + (int64_t)0x1000000000) >> 37);
            dest[i] += (tmp + round) >> shift;
        }
    }
    else {
      for (i = 0; i < len; i++) {
          tmp = (int)(((int64_t)src[i] * c + (int64_t)0x1000000000) >> 37);
          dest[i] += tmp * (1U << shift);
      }
    }
}

#endif /* AVCODEC_AAC_AACDEC_FIXED_COUPLING_H */
