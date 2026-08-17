/*
 * Copyright (c) 2011 Jan Kokemüller
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
 *
 * This file is based on libebur128 which is available at
 * https://github.com/jiixyj/libebur128/
 *
*/

#ifndef AVFILTER_EBUR128_H
#define AVFILTER_EBUR128_H

/** \file ebur128.h
 *  \brief libebur128 - a library for loudness measurement according to
 *         the EBU R128 standard.
 */

#include <stddef.h>             /* for size_t */

/** \enum channel
 *  Use these values when setting the channel map with ebur128_set_channel().
 *  See definitions in ITU R-REC-BS 1770-4
 */
enum channel {
    FF_EBUR128_UNUSED = 0,   /**< unused channel (for example LFE channel) */
    FF_EBUR128_LEFT,
    FF_EBUR128_Mp030 = 1,    /**< itu M+030 */
    FF_EBUR128_RIGHT,
    FF_EBUR128_Mm030 = 2,    /**< itu M-030 */
    FF_EBUR128_CENTER,
    FF_EBUR128_Mp000 = 3,    /**< itu M+000 */
    FF_EBUR128_LEFT_SURROUND,
    FF_EBUR128_Mp110 = 4,    /**< itu M+110 */
    FF_EBUR128_RIGHT_SURROUND,
    FF_EBUR128_Mm110 = 5,    /**< itu M-110 */
    FF_EBUR128_DUAL_MONO,    /**< a channel that is counted twice */
    FF_EBUR128_MpSC,         /**< itu M+SC */
    FF_EBUR128_MmSC,         /**< itu M-SC */
    FF_EBUR128_Mp060,        /**< itu M+060 */
    FF_EBUR128_Mm060,        /**< itu M-060 */
    FF_EBUR128_Mp090,        /**< itu M+090 */
    FF_EBUR128_Mm090,        /**< itu M-090 */
    FF_EBUR128_Mp135,        /**< itu M+135 */
    FF_EBUR128_Mm135,        /**< itu M-135 */
    FF_EBUR128_Mp180,        /**< itu M+180 */
    FF_EBUR128_Up000,        /**< itu U+000 */
    FF_EBUR128_Up030,        /**< itu U+030 */
    FF_EBUR128_Um030,        /**< itu U-030 */
    FF_EBUR128_Up045,        /**< itu U+045 */
    FF_EBUR128_Um045,        /**< itu U-030 */
    FF_EBUR128_Up090,        /**< itu U+090 */
    FF_EBUR128_Um090,        /**< itu U-090 */
    FF_EBUR128_Up110,        /**< itu U+110 */
    FF_EBUR128_Um110,        /**< itu U-110 */
    FF_EBUR128_Up135,        /**< itu U+135 */
    FF_EBUR128_Um135,        /**< itu U-135 */
    FF_EBUR128_Up180,        /**< itu U+180 */
    FF_EBUR128_Tp000,        /**< itu T+000 */
    FF_EBUR128_Bp000,        /**< itu B+000 */
    FF_EBUR128_Bp045,        /**< itu B+045 */
    FF_EBUR128_Bm045         /**< itu B-045 */
};

/** \enum mode
 *  Use these values in ebur128_init (or'ed). Try to use the lowest possible
 *  modes that suit your needs, as performance will be better.
 */
enum mode {
  /** can resurrrect and call ff_ebur128_loudness_momentary */
    FF_EBUR128_MODE_M = (1 << 0),
  /** can call ff_ebur128_loudness_shortterm */
    FF_EBUR128_MODE_S = (1 << 1) | FF_EBUR128_MODE_M,
  /** can call ff_ebur128_loudness_global_* and ff_ebur128_relative_threshold */
    FF_EBUR128_MODE_I = (1 << 2) | FF_EBUR128_MODE_M,
  /** can call ff_ebur128_loudness_range */
    FF_EBUR128_MODE_LRA = (1 << 3) | FF_EBUR128_MODE_S,
  /** can call ff_ebur128_sample_peak */
    FF_EBUR128_MODE_SAMPLE_PEAK = (1 << 4) | FF_EBUR128_MODE_M,
};

/** forward declaration of FFEBUR128StateInternal */
struct FFEBUR128StateInternal;

/** \brief Contains information about the state of a loudness measurement.
 *
 *  You should not need to modify this struct directly.
 */
typedef struct FFEBUR128State {
    int mode;                         /**< The current mode. */
    unsigned int channels;            /**< The number of channels. */
    unsigned long samplerate;         /**< The sample rate. */
    struct FFEBUR128StateInternal *d; /**< Internal state. */
} FFEBUR128State;

/** \brief Initialize library state.
 *
 *  @param channels the number of channels.
 *  @param samplerate the sample rate.
 *  @param window set the maximum window size in ms, set to 0 for auto.
 *  @param mode see the mode enum for possible values.
 *  @return an initialized library state.
 */
FFEBUR128State *ff_ebur128_init(unsigned int channels,
                                unsigned long samplerate,
                                unsigned long window, int mode);

/** \brief Destroy library state.
 *
 *  @param st pointer to a library state.
 */
void ff_ebur128_destroy(FFEBUR128State ** st);

/** \brief Set channel type.
 *
 *  The default is:
 *  - 0 -> FF_EBUR128_LEFT
 *  - 1 -> FF_EBUR128_RIGHT
 *  - 2 -> FF_EBUR128_CENTER
 *  - 3 -> FF_EBUR128_UNUSED
 *  - 4 -> FF_EBUR128_LEFT_SURROUND
 *  - 5 -> FF_EBUR128_RIGHT_SURROUND
 *
 *  @param st library state.
 *  @param channel_number zero based channel index.
 *  @param value channel type from the "channel" enum.
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if invalid channel index.
 */
int ff_ebur128_set_channel(FFEBUR128State * st,
                           unsigned int channel_number, int value);

/** \brief Add frames to be processed.
 *
 *  @param st library state.
 *  @param src array of source frames. Channels must be interleaved.
 *  @param frames number of frames. Not number of samples!
 */
void ff_ebur128_add_frames_double(FFEBUR128State * st,
                                  const double *src, size_t frames);

/** \brief Get global integrated loudness in LUFS.
 *
 *  @param st library state.
 *  @param out integrated loudness in LUFS. -HUGE_VAL if result is negative
 *             infinity.
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if mode "FF_EBUR128_MODE_I" has not been set.
 */
int ff_ebur128_loudness_global(FFEBUR128State * st, double *out);

/** \brief Get short-term loudness (last 3s) in LUFS.
 *
 *  @param st library state.
 *  @param out short-term loudness in LUFS. -HUGE_VAL if result is negative
 *             infinity.
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if mode "FF_EBUR128_MODE_S" has not been set.
 */
int ff_ebur128_loudness_shortterm(FFEBUR128State * st, double *out);

/** \brief Get loudness range (LRA) of programme in LU.
 *
 *  Calculates loudness range according to EBU 3342.
 *
 *  @param st library state.
 *  @param out loudness range (LRA) in LU. Will not be changed in case of
 *             error. AVERROR(EINVAL) will be returned in this case.
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if mode "FF_EBUR128_MODE_LRA" has not been set.
 */
int ff_ebur128_loudness_range(FFEBUR128State * st, double *out);
/** \brief Get loudness range (LRA) in LU across multiple instances.
 *
 *  Calculates loudness range according to EBU 3342.
 *
 *  @param sts array of library states.
 *  @param size length of sts
 *  @param out loudness range (LRA) in LU. Will not be changed in case of
 *             error. AVERROR(EINVAL) will be returned in this case.
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if mode "FF_EBUR128_MODE_LRA" has not been set.
 */
int ff_ebur128_loudness_range_multiple(FFEBUR128State ** sts,
                                       size_t size, double *out);

/** \brief Get maximum sample peak of selected channel in float format.
 *
 *  @param st library state
 *  @param channel_number channel to analyse
 *  @param out maximum sample peak in float format (1.0 is 0 dBFS)
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if mode "FF_EBUR128_MODE_SAMPLE_PEAK" has not been set.
 *    - AVERROR(EINVAL) if invalid channel index.
 */
int ff_ebur128_sample_peak(FFEBUR128State * st,
                           unsigned int channel_number, double *out);

/** \brief Get relative threshold in LUFS.
 *
 *  @param st library state
 *  @param out relative threshold in LUFS.
 *  @return
 *    - 0 on success.
 *    - AVERROR(EINVAL) if mode "FF_EBUR128_MODE_I" has not been set.
 */
int ff_ebur128_relative_threshold(FFEBUR128State * st, double *out);

#endif                          /* AVFILTER_EBUR128_H */
