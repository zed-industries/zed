/*
 * ATRAC3+ compatible decoder
 *
 * Copyright (c) 2010-2013 Maxim Poliakovski
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

/**
 * @file
 * Global structures, constants and data for ATRAC3+ decoder.
 */

#ifndef AVCODEC_ATRAC3PLUS_H
#define AVCODEC_ATRAC3PLUS_H

#include <stdint.h>

#include "libavutil/float_dsp.h"
#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#include "atrac.h"
#include "avcodec.h"
#include "get_bits.h"

/** Global unit sizes */
#define ATRAC3P_SUBBANDS        16  ///< number of PQF subbands
#define ATRAC3P_SUBBAND_SAMPLES 128 ///< number of samples per subband
#define ATRAC3P_FRAME_SAMPLES   (ATRAC3P_SUBBAND_SAMPLES * ATRAC3P_SUBBANDS)

#define ATRAC3P_PQF_FIR_LEN     12  ///< length of the prototype FIR of the PQF

/** Global constants */
#define ATRAC3P_POWER_COMP_OFF  15  ///< disable power compensation

/** ATRAC3+ channel unit types */
enum Atrac3pChannelUnitTypes {
    CH_UNIT_MONO       = 0, ///< unit containing one coded channel
    CH_UNIT_STEREO     = 1, ///< unit containing two jointly-coded channels
    CH_UNIT_EXTENSION  = 2, ///< unit containing extension information
    CH_UNIT_TERMINATOR = 3  ///< unit sequence terminator
};

/** Per-channel IPQF history */
typedef struct Atrac3pIPQFChannelCtx {
    DECLARE_ALIGNED(32, float, buf1)[ATRAC3P_PQF_FIR_LEN * 2][8];
    DECLARE_ALIGNED(32, float, buf2)[ATRAC3P_PQF_FIR_LEN * 2][8];
    int pos;
} Atrac3pIPQFChannelCtx;

/** Amplitude envelope of a group of sine waves */
typedef struct Atrac3pWaveEnvelope {
    int has_start_point;    ///< indicates start point within the GHA window
    int has_stop_point;     ///< indicates stop point within the GHA window
    int start_pos;          ///< start position expressed in n*4 samples
    int stop_pos;           ///< stop  position expressed in n*4 samples
} Atrac3pWaveEnvelope;

/** Parameters of a group of sine waves */
typedef struct Atrac3pWavesData {
    Atrac3pWaveEnvelope pend_env;   ///< pending envelope from the previous frame
    Atrac3pWaveEnvelope curr_env;   ///< group envelope from the current frame
    int num_wavs;           ///< number of sine waves in the group
    int start_index;        ///< start index into global tones table for that subband
} Atrac3pWavesData;

/** Parameters of a single sine wave */
typedef struct Atrac3pWaveParam {
    int   freq_index;   ///< wave frequency index
    int   amp_sf;       ///< quantized amplitude scale factor
    int   amp_index;    ///< quantized amplitude index
    int   phase_index;  ///< quantized phase index
} Atrac3pWaveParam;

/** Sound channel parameters */
typedef struct Atrac3pChanParams {
    int ch_num;
    int num_coded_vals;         ///< number of transmitted quant unit values
    int fill_mode;
    int split_point;
    int table_type;             ///< table type: 0 - tone?, 1- noise?
    int qu_wordlen[32];         ///< array of word lengths for each quant unit
    int qu_sf_idx[32];          ///< array of scale factor indexes for each quant unit
    int qu_tab_idx[32];         ///< array of code table indexes for each quant unit
    int16_t spectrum[2048];     ///< decoded IMDCT spectrum
    uint8_t power_levs[5];      ///< power compensation levels

    /* imdct window shape history (2 frames) for overlapping. */
    uint8_t wnd_shape_hist[2][ATRAC3P_SUBBANDS];    ///< IMDCT window shape, 0=sine/1=steep
    uint8_t *wnd_shape;         ///< IMDCT window shape for current frame
    uint8_t *wnd_shape_prev;    ///< IMDCT window shape for previous frame

    /* gain control data history (2 frames) for overlapping. */
    AtracGainInfo gain_data_hist[2][ATRAC3P_SUBBANDS];  ///< gain control data for all subbands
    AtracGainInfo *gain_data;       ///< gain control data for next frame
    AtracGainInfo *gain_data_prev;  ///< gain control data for previous frame
    int num_gain_subbands;      ///< number of subbands with gain control data

    /* tones data history (2 frames) for overlapping. */
    Atrac3pWavesData tones_info_hist[2][ATRAC3P_SUBBANDS];
    Atrac3pWavesData *tones_info;
    Atrac3pWavesData *tones_info_prev;
} Atrac3pChanParams;

/* Per-unit sine wave parameters */
typedef struct Atrac3pWaveSynthParams {
    int tones_present;                      ///< 1 - tones info present
    int amplitude_mode;                     ///< 1 - low range, 0 - high range
    int num_tone_bands;                     ///< number of PQF bands with tones
    uint8_t tone_sharing[ATRAC3P_SUBBANDS]; ///< 1 - subband-wise tone sharing flags
    uint8_t tone_master[ATRAC3P_SUBBANDS];  ///< 1 - subband-wise tone channel swapping
    uint8_t invert_phase[ATRAC3P_SUBBANDS]; ///< 1 - subband-wise phase inversion
    int tones_index;                        ///< total sum of tones in this unit
    Atrac3pWaveParam waves[48];
} Atrac3pWaveSynthParams;

/** Channel unit parameters */
typedef struct Atrac3pChanUnitCtx {
    /* channel unit variables */
    int unit_type;                          ///< unit type (mono/stereo)
    int num_quant_units;
    int num_subbands;
    int used_quant_units;                   ///< number of quant units with coded spectrum
    int num_coded_subbands;                 ///< number of subbands with coded spectrum
    int mute_flag;                          ///< mute flag
    int use_full_table;                     ///< 1 - full table list, 0 - restricted one
    int noise_present;                      ///< 1 - global noise info present
    int noise_level_index;                  ///< global noise level index
    int noise_table_index;                  ///< global noise RNG table index
    uint8_t swap_channels[ATRAC3P_SUBBANDS];    ///< 1 - perform subband-wise channel swapping
    uint8_t negate_coeffs[ATRAC3P_SUBBANDS];    ///< 1 - subband-wise IMDCT coefficients negation
    Atrac3pChanParams channels[2];

    /* Variables related to GHA tones */
    Atrac3pWaveSynthParams wave_synth_hist[2];     ///< waves synth history for two frames
    Atrac3pWaveSynthParams *waves_info;
    Atrac3pWaveSynthParams *waves_info_prev;

    Atrac3pIPQFChannelCtx ipqf_ctx[2];
    DECLARE_ALIGNED(32, float, prev_buf)[2][ATRAC3P_FRAME_SAMPLES]; ///< overlapping buffer
} Atrac3pChanUnitCtx;

/**
 * Initialize VLC tables for bitstream parsing.
 */
void ff_atrac3p_init_vlcs(void);

/**
 * Decode bitstream data of a channel unit.
 *
 * @param[in]     gb            the GetBit context
 * @param[in,out] ctx           ptr to the channel unit context
 * @param[in]     num_channels  number of channels to process
 * @param[in]     avctx         ptr to the AVCodecContext
 * @return result code: 0 = OK, otherwise - error code
 */
int ff_atrac3p_decode_channel_unit(GetBitContext *gb, Atrac3pChanUnitCtx *ctx,
                                   int num_channels, AVCodecContext *avctx);

/**
 * Initialize sine waves synthesizer and ff_sine_* tables.
 */
void ff_atrac3p_init_dsp_static(void);

/**
 * Synthesize sine waves for a particular subband.
 *
 * @param[in]   ch_unit   pointer to the channel unit context
 * @param[in]   fdsp      pointer to float DSP context
 * @param[in]   ch_num    which channel to process
 * @param[in]   sb        which subband to process
 * @param[out]  out       receives processed data
 */
void ff_atrac3p_generate_tones(Atrac3pChanUnitCtx *ch_unit, AVFloatDSPContext *fdsp,
                               int ch_num, int sb, float *out);

/**
 * Perform power compensation aka noise dithering.
 *
 * @param[in]      ctx         ptr to the channel context
 * @param[in]      fdsp        pointer to float DSP context
 * @param[in]      ch_index    which channel to process
 * @param[in,out]  sp          ptr to channel spectrum to process
 * @param[in]      rng_index   indicates which RNG table to use
 * @param[in]      sb_num      which subband to process
 */
void ff_atrac3p_power_compensation(Atrac3pChanUnitCtx *ctx, AVFloatDSPContext *fdsp,
                                   int ch_index, float *sp, int rng_index, int sb_num);

/**
 * Regular IMDCT and windowing without overlapping,
 * with spectrum reversal in the odd subbands.
 *
 * @param[in]   fdsp       pointer to float DSP context
 * @param[in]   mdct_ctx   pointer to MDCT transform context
 * @param[in]   pIn        float input
 * @param[out]  pOut       float output
 * @param[in]   wind_id    which MDCT window to apply
 * @param[in]   sb         subband number
 */
void ff_atrac3p_imdct(AVFloatDSPContext *fdsp, AVTXContext *mdct_ctx,
                      av_tx_fn mdct_fn, float *pIn, float *pOut,
                      int wind_id, int sb);

/**
 * Subband synthesis filter based on the polyphase quadrature (pseudo-QMF)
 * filter bank.
 *
 * @param[in]      dct_ctx   ptr to the pre-initialized IDCT context
 * @param[in,out]  hist      ptr to the filter history
 * @param[in]      in        input data to process
 * @param[out]     out       receives processed data
 */
void ff_atrac3p_ipqf(AVTXContext *dct_ctx, av_tx_fn dct_fn,
                     Atrac3pIPQFChannelCtx *hist, const float *in, float *out);

extern const uint16_t ff_atrac3p_qu_to_spec_pos[33];
extern const float ff_atrac3p_sf_tab[64];
extern const float ff_atrac3p_mant_tab[8];

#endif /* AVCODEC_ATRAC3PLUS_H */
