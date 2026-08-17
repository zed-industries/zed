/*
 * Copyright (C) 2016 foo86
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

#ifndef AVCODEC_DCA_LBR_H
#define AVCODEC_DCA_LBR_H

#include "libavutil/float_dsp.h"
#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#include "avcodec.h"
#include "get_bits.h"
#include "dca.h"
#include "dca_exss.h"
#include "dcadsp.h"

#define DCA_LBR_CHANNELS        6
#define DCA_LBR_CHANNELS_TOTAL  32
#define DCA_LBR_SUBBANDS        32
#define DCA_LBR_TONES           512

#define DCA_LBR_TIME_SAMPLES    128
#define DCA_LBR_TIME_HISTORY    8

enum DCALBRHeader {
    DCA_LBR_HEADER_SYNC_ONLY    = 1,
    DCA_LBR_HEADER_DECODER_INIT = 2
};

typedef struct DCALbrTone {
    uint8_t     x_freq;     ///< Spectral line offset
    uint8_t     f_delt;     ///< Difference between original and center frequency
    uint8_t     ph_rot;     ///< Phase rotation
    uint8_t     pad;        ///< Padding field
    uint8_t     amp[DCA_LBR_CHANNELS];  ///< Per-channel amplitude
    uint8_t     phs[DCA_LBR_CHANNELS];  ///< Per-channel phase
} DCALbrTone;

typedef struct DCALbrDecoder {
    AVCodecContext  *avctx;
    GetBitContext   gb;

    int     sample_rate;        ///< Sample rate of LBR audio
    int     ch_mask;            ///< LBR speaker mask
    int     flags;              ///< Flags for LBR decoder initialization
    int     bit_rate_orig;      ///< Original bit rate
    int     bit_rate_scaled;    ///< Scaled bit rate

    int     nchannels;          ///< Number of fullband channels to decode
    int     nchannels_total;    ///< Total number of fullband channels
    int     freq_range;         ///< Frequency range of LBR audio
    int     band_limit;         ///< Band limit factor
    int     limited_rate;       ///< Band limited sample rate
    int     limited_range;      ///< Band limited frequency range
    int     res_profile;        ///< Resolution profile
    int     nsubbands;          ///< Number of encoded subbands
    int     g3_avg_only_start_sb;   ///< Subband index where grid 3 scale factors end
    int     min_mono_subband;   ///< Subband index where mono encoding starts
    int     max_mono_subband;   ///< Subband index where mono encoding ends

    int     framenum;   ///< Lower 5 bits of current frame number
    int     lbr_rand;   ///< Seed for subband randomization
    int     warned;     ///< Flags for warning suppression

    uint8_t     quant_levels[DCA_LBR_CHANNELS / 2][DCA_LBR_SUBBANDS];   ///< Quantization levels
    uint8_t     sb_indices[DCA_LBR_SUBBANDS];   ///< Subband reordering indices

    uint8_t     sec_ch_sbms[DCA_LBR_CHANNELS / 2][DCA_LBR_SUBBANDS];    ///< Right channel inversion or mid/side decoding flags
    uint8_t     sec_ch_lrms[DCA_LBR_CHANNELS / 2][DCA_LBR_SUBBANDS];    ///< Flags indicating if left/right channel are swapped
    uint32_t    ch_pres[DCA_LBR_CHANNELS];  ///< Subband allocation flags

    uint8_t     grid_1_scf[DCA_LBR_CHANNELS][12][8];    ///< Grid 1 scale factors
    uint8_t     grid_2_scf[DCA_LBR_CHANNELS][3][64];    ///< Grid 2 scale factors

    int8_t      grid_3_avg[DCA_LBR_CHANNELS][DCA_LBR_SUBBANDS - 4];     ///< Grid 3 average values
    int8_t      grid_3_scf[DCA_LBR_CHANNELS][DCA_LBR_SUBBANDS - 4][8];  ///< Grid 3 scale factors
    uint32_t    grid_3_pres[DCA_LBR_CHANNELS];  ///< Grid 3 scale factors presence flags

    uint8_t     high_res_scf[DCA_LBR_CHANNELS][DCA_LBR_SUBBANDS][8];    ///< High-frequency resolution scale factors

    uint8_t     part_stereo[DCA_LBR_CHANNELS][DCA_LBR_SUBBANDS / 4][5]; ///< Partial stereo coefficients
    uint8_t     part_stereo_pres;   ///< Partial stereo coefficients presence flags

    float       lpc_coeff[2][DCA_LBR_CHANNELS][3][2][8];    ///< Predictor coefficients

    float       sb_scf[DCA_LBR_SUBBANDS];   ///< Subband randomization scale factors

    float       *time_samples[DCA_LBR_CHANNELS][DCA_LBR_SUBBANDS]; ///< Time samples

    float           *ts_buffer; ///< Time sample buffer base
    unsigned int    ts_size;    ///< Time sample buffer size

    DECLARE_ALIGNED(32, float, history)[DCA_LBR_CHANNELS][DCA_LBR_SUBBANDS * 4];    ///< IMDCT history
    DECLARE_ALIGNED(32, float, window)[DCA_LBR_SUBBANDS * 4];   ///< Long window for IMDCT

    DECLARE_ALIGNED(32, float, lfe_data)[64];       ///< Decimated LFE samples
    DECLARE_ALIGNED(32, float, lfe_history)[5][2];  ///< LFE IIR filter history
    float lfe_scale;    ///< Scale factor of LFE samples before IIR filter

    uint8_t     tonal_scf[6];           ///< Tonal scale factors
    uint16_t    tonal_bounds[5][32][2]; ///< Per-group per-subframe start/end positions of tones
    DCALbrTone  tones[DCA_LBR_TONES];   ///< Circular buffer of tones
    int         ntones;                 ///< Circular buffer head position

    AVTXContext         *imdct;
    av_tx_fn             imdct_fn;
    AVFloatDSPContext   *fdsp;
    DCADSPContext       *dcadsp;
} DCALbrDecoder;

int ff_dca_lbr_parse(DCALbrDecoder *s, const uint8_t *data, DCAExssAsset *asset);
int ff_dca_lbr_filter_frame(DCALbrDecoder *s, AVFrame *frame);
av_cold void ff_dca_lbr_flush(DCALbrDecoder *s);
av_cold void ff_dca_lbr_init_tables(void);
av_cold int ff_dca_lbr_init(DCALbrDecoder *s);
av_cold void ff_dca_lbr_close(DCALbrDecoder *s);

#endif
