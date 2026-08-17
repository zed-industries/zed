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

#ifndef AVCODEC_DCA_XLL_H
#define AVCODEC_DCA_XLL_H

#include "libavutil/mem_internal.h"

#include "avcodec.h"
#include "get_bits.h"
#include "dca.h"
#include "dcadsp.h"
#include "dca_exss.h"

#define DCA_XLL_CHSETS_MAX              3
#define DCA_XLL_CHANNELS_MAX            8
#define DCA_XLL_BANDS_MAX               2
#define DCA_XLL_ADAPT_PRED_ORDER_MAX    16
#define DCA_XLL_DECI_HISTORY_MAX        8
#define DCA_XLL_DMIX_SCALES_MAX         ((DCA_XLL_CHSETS_MAX - 1) * DCA_XLL_CHANNELS_MAX)
#define DCA_XLL_DMIX_COEFFS_MAX         (DCA_XLL_DMIX_SCALES_MAX * DCA_XLL_CHANNELS_MAX)
#define DCA_XLL_PBR_BUFFER_MAX          (240 << 10)
#define DCA_XLL_SAMPLE_BUFFERS_MAX      3

typedef struct DCAXllBand {
    int     decor_enabled;                          ///< Pairwise channel decorrelation flag
    int     orig_order[DCA_XLL_CHANNELS_MAX];       ///< Original channel order
    int     decor_coeff[DCA_XLL_CHANNELS_MAX / 2];  ///< Pairwise channel coefficients

    int     adapt_pred_order[DCA_XLL_CHANNELS_MAX]; ///< Adaptive predictor order
    int     highest_pred_order;                     ///< Highest adaptive predictor order
    int     fixed_pred_order[DCA_XLL_CHANNELS_MAX]; ///< Fixed predictor order
    int     adapt_refl_coeff[DCA_XLL_CHANNELS_MAX][DCA_XLL_ADAPT_PRED_ORDER_MAX];   ///< Adaptive predictor reflection coefficients

    int     dmix_embedded;  ///< Downmix performed by encoder in frequency band

    int     lsb_section_size;                       ///< Size of LSB section in any segment
    int     nscalablelsbs[DCA_XLL_CHANNELS_MAX];    ///< Number of bits to represent the samples in LSB part
    int     bit_width_adjust[DCA_XLL_CHANNELS_MAX]; ///< Number of bits discarded by authoring

    int32_t *msb_sample_buffer[DCA_XLL_CHANNELS_MAX];   ///< MSB sample buffer pointers
    int32_t *lsb_sample_buffer[DCA_XLL_CHANNELS_MAX];   ///< LSB sample buffer pointers or NULL
} DCAXllBand;

typedef struct DCAXllChSet {
    // Channel set header
    int     nchannels;          ///< Number of channels in the channel set (N)
    int     residual_encode;    ///< Residual encoding mask (0 - residual, 1 - full channel)
    int     pcm_bit_res;        ///< PCM bit resolution (variable)
    int     storage_bit_res;    ///< Storage bit resolution (16 or 24)
    int     freq;               ///< Original sampling frequency (max. 96000 Hz)

    int     primary_chset;          ///< Primary channel set flag
    int     dmix_coeffs_present;    ///< Downmix coefficients present in stream
    int     dmix_embedded;          ///< Downmix already performed by encoder
    int     dmix_type;              ///< Primary channel set downmix type
    int     hier_chset;             ///< Whether the channel set is part of a hierarchy
    int     hier_ofs;               ///< Number of preceding channels in a hierarchy (M)
    int     dmix_coeff[DCA_XLL_DMIX_COEFFS_MAX];       ///< Downmixing coefficients
    int     dmix_scale[DCA_XLL_DMIX_SCALES_MAX];       ///< Downmixing scales
    int     dmix_scale_inv[DCA_XLL_DMIX_SCALES_MAX];   ///< Inverse downmixing scales
    int     ch_mask;                ///< Channel mask for set
    int     ch_remap[DCA_XLL_CHANNELS_MAX];    ///< Channel to speaker map

    int     nfreqbands; ///< Number of frequency bands (1 or 2)
    int     nabits;     ///< Number of bits to read bit allocation coding parameter

    DCAXllBand     bands[DCA_XLL_BANDS_MAX];   ///< Frequency bands

    // Frequency band coding parameters
    int     seg_common;                                     ///< Segment type
    int     rice_code_flag[DCA_XLL_CHANNELS_MAX];           ///< Rice coding flag
    int     bitalloc_hybrid_linear[DCA_XLL_CHANNELS_MAX];   ///< Binary code length for isolated samples
    int     bitalloc_part_a[DCA_XLL_CHANNELS_MAX];          ///< Coding parameter for part A of segment
    int     bitalloc_part_b[DCA_XLL_CHANNELS_MAX];          ///< Coding parameter for part B of segment
    int     nsamples_part_a[DCA_XLL_CHANNELS_MAX];          ///< Number of samples in part A of segment

    // Decimator history
    DECLARE_ALIGNED(32, int32_t, deci_history)[DCA_XLL_CHANNELS_MAX][DCA_XLL_DECI_HISTORY_MAX]; ///< Decimator history for frequency band 1

    // Sample buffers
    unsigned int    sample_size[DCA_XLL_SAMPLE_BUFFERS_MAX];
    int32_t         *sample_buffer[DCA_XLL_SAMPLE_BUFFERS_MAX];
} DCAXllChSet;

typedef struct DCAXllDecoder {
    AVCodecContext  *avctx;
    GetBitContext   gb;

    int     frame_size;             ///< Number of bytes in a lossless frame
    int     nchsets;                ///< Number of channels sets per frame
    int     nframesegs;             ///< Number of segments per frame
    int     nsegsamples_log2;       ///< log2(nsegsamples)
    int     nsegsamples;            ///< Samples in segment per one frequency band
    int     nframesamples_log2;     ///< log2(nframesamples)
    int     nframesamples;          ///< Samples in frame per one frequency band
    int     seg_size_nbits;         ///< Number of bits used to read segment size
    int     band_crc_present;       ///< Presence of CRC16 within each frequency band
    int     scalable_lsbs;          ///< MSB/LSB split flag
    int     ch_mask_nbits;          ///< Number of bits used to read channel mask
    int     fixed_lsb_width;        ///< Fixed LSB width

    DCAXllChSet    chset[DCA_XLL_CHSETS_MAX]; ///< Channel sets

    int             *navi;          ///< NAVI table
    unsigned int    navi_size;

    int     nfreqbands;     ///< Highest number of frequency bands
    int     nchannels;      ///< Total number of channels in a hierarchy
    int     nreschsets;     ///< Number of channel sets that have residual encoded channels
    int     nactivechsets;  ///< Number of active channel sets to decode

    int     hd_stream_id;   ///< Previous DTS-HD stream ID for detecting changes

    uint8_t     *pbr_buffer;        ///< Peak bit rate (PBR) smoothing buffer
    int         pbr_length;         ///< Length in bytes of data currently buffered
    int         pbr_delay;          ///< Delay in frames before decoding buffered data

    DCADSPContext   *dcadsp;

    int    x_syncword_present;        ///< Syncword for extension data at end of frame (DTS:X) is present
    int    x_imax_syncword_present;   ///< Syncword for extension data at end of frame (DTS:X IMAX) is present

    int     output_mask;
    int32_t *output_samples[DCA_SPEAKER_COUNT];
} DCAXllDecoder;

int ff_dca_xll_parse(DCAXllDecoder *s, const uint8_t *data, DCAExssAsset *asset);
int ff_dca_xll_filter_frame(DCAXllDecoder *s, AVFrame *frame);
av_cold void ff_dca_xll_flush(DCAXllDecoder *s);
av_cold void ff_dca_xll_close(DCAXllDecoder *s);

#endif
