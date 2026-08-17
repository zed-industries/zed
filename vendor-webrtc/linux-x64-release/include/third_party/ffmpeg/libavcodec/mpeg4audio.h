/*
 * MPEG-4 Audio common header
 * Copyright (c) 2008 Baptiste Coudurier <baptiste.coudurier@free.fr>
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

#ifndef AVCODEC_MPEG4AUDIO_H
#define AVCODEC_MPEG4AUDIO_H

#include <stdint.h>

#include "get_bits.h"

typedef struct MPEG4AudioConfig {
    int object_type;
    int sampling_index;
    int sample_rate;
    int chan_config;
    int sbr; ///< -1 implicit, 1 presence
    int ext_object_type;
    int ext_sampling_index;
    int ext_sample_rate;
    int ext_chan_config;
    int channels;
    int ps;  ///< -1 implicit, 1 presence
    int frame_length_short;
} MPEG4AudioConfig;

extern const int     ff_mpeg4audio_sample_rates[16];
extern const uint8_t ff_mpeg4audio_channels[15];

/**
 * Parse MPEG-4 systems extradata from a potentially unaligned GetBitContext to retrieve audio configuration.
 * @param[in] c        MPEG4AudioConfig structure to fill.
 * @param[in] gb       Extradata from container.
 * @param[in] sync_extension look for a sync extension after config if true.
 * @param[in] logctx opaque struct starting with an AVClass element, used for logging.
 * @return negative AVERROR code on error, on success AudioSpecificConfig bit index in extradata.
 */
int ff_mpeg4audio_get_config_gb(MPEG4AudioConfig *c, GetBitContext *gb,
                                int sync_extension, void *logctx);

/**
 * Parse MPEG-4 systems extradata from a raw buffer to retrieve audio configuration.
 * @param[in] c        MPEG4AudioConfig structure to fill.
 * @param[in] buf      Extradata from container.
 * @param[in] size     Extradata size in bytes.
 * @param[in] sync_extension look for a sync extension after config if true.
 * @param[in] logctx opaque struct starting with an AVClass element, used for logging.
 * @return negative AVERROR code on error, AudioSpecificConfig bit index in extradata on success.
 */
int avpriv_mpeg4audio_get_config2(MPEG4AudioConfig *c, const uint8_t *buf,
                                  int size, int sync_extension, void *logctx);

enum AudioObjectType {
    AOT_NULL = 0,
                               // Support?                Name
    AOT_AAC_MAIN         =  1, ///< Y                       Main
    AOT_AAC_LC           =  2, ///< Y                       Low Complexity
    AOT_AAC_SSR          =  3, ///< N (code in SoC repo)    Scalable Sample Rate
    AOT_AAC_LTP          =  4, ///< Y                       Long Term Prediction
    AOT_SBR              =  5, ///< Y                       Spectral Band Replication
    AOT_AAC_SCALABLE     =  6, ///< N                       Scalable
    AOT_TWINVQ           =  7, ///< N                       Twin Vector Quantizer
    AOT_CELP             =  8, ///< N                       Code Excited Linear Prediction
    AOT_HVXC             =  9, ///< N                       Harmonic Vector eXcitation Coding

    AOT_TTSI             = 12, ///< N                       Text-To-Speech Interface
    AOT_MAINSYNTH        = 13, ///< N                       Main Synthesis
    AOT_WAVESYNTH        = 14, ///< N                       Wavetable Synthesis
    AOT_MIDI             = 15, ///< N                       General MIDI
    AOT_SAFX             = 16, ///< N                       Algorithmic Synthesis and Audio Effects
    AOT_ER_AAC_LC        = 17, ///< N                       Error Resilient Low Complexity

    AOT_ER_AAC_LTP       = 19, ///< N                       Error Resilient Long Term Prediction
    AOT_ER_AAC_SCALABLE  = 20, ///< N                       Error Resilient Scalable
    AOT_ER_TWINVQ        = 21, ///< N                       Error Resilient Twin Vector Quantizer
    AOT_ER_BSAC          = 22, ///< N                       Error Resilient Bit-Sliced Arithmetic Coding
    AOT_ER_AAC_LD        = 23, ///< N                       Error Resilient Low Delay
    AOT_ER_CELP          = 24, ///< N                       Error Resilient Code Excited Linear Prediction
    AOT_ER_HVXC          = 25, ///< N                       Error Resilient Harmonic Vector eXcitation Coding
    AOT_ER_HILN          = 26, ///< N                       Error Resilient Harmonic and Individual Lines plus Noise
    AOT_ER_PARAM         = 27, ///< N                       Error Resilient Parametric
    AOT_SSC              = 28, ///< N                       SinuSoidal Coding
    AOT_PS               = 29, ///< N                       Parametric Stereo
    AOT_SURROUND         = 30, ///< N                       MPEG Surround
    AOT_ESCAPE           = 31, ///< Y                       Escape Value
    AOT_L1               = 32, ///< Y                       Layer 1
    AOT_L2               = 33, ///< Y                       Layer 2
    AOT_L3               = 34, ///< Y                       Layer 3
    AOT_DST              = 35, ///< N                       Direct Stream Transfer
    AOT_ALS              = 36, ///< Y                       Audio LosslesS
    AOT_SLS              = 37, ///< N                       Scalable LosslesS
    AOT_SLS_NON_CORE     = 38, ///< N                       Scalable LosslesS (non core)
    AOT_ER_AAC_ELD       = 39, ///< N                       Error Resilient Enhanced Low Delay
    AOT_SMR_SIMPLE       = 40, ///< N                       Symbolic Music Representation Simple
    AOT_SMR_MAIN         = 41, ///< N                       Symbolic Music Representation Main
    AOT_USAC             = 42, ///< Y                       Unified Speech and Audio Coding
    AOT_SAOC             = 43, ///< N                       Spatial Audio Object Coding
    AOT_LD_SURROUND      = 44, ///< N                       Low Delay MPEG Surround
};

#define MAX_PCE_SIZE 320 ///<Maximum size of a PCE including the 3-bit ID_PCE
                         ///<marker and the comment

#endif /* AVCODEC_MPEG4AUDIO_H */
