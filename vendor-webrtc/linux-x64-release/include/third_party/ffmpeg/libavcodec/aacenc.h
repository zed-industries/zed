/*
 * AAC encoder
 * Copyright (C) 2008 Konstantin Shishkov
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

#ifndef AVCODEC_AACENC_H
#define AVCODEC_AACENC_H

#include <stdint.h>

#include "libavutil/channel_layout.h"
#include "libavutil/float_dsp.h"
#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#include "avcodec.h"
#include "put_bits.h"

#include "aac.h"
#include "aacencdsp.h"
#include "audio_frame_queue.h"
#include "psymodel.h"

#include "lpc.h"

#define CLIP_AVOIDANCE_FACTOR 0.95f

typedef enum AACCoder {
    AAC_CODER_TWOLOOP,
    AAC_CODER_FAST,

    AAC_CODER_NB,
}AACCoder;

/**
 * Predictor State
 */
typedef struct PredictorState {
    float cor0;
    float cor1;
    float var0;
    float var1;
    float r0;
    float r1;
    float k1;
    float x_est;
} PredictorState;

typedef struct AACEncOptions {
    int coder;
    int pns;
    int tns;
    int pce;
    int mid_side;
    int intensity_stereo;
} AACEncOptions;

/**
 * Individual Channel Stream
 */
typedef struct IndividualChannelStream {
    uint8_t max_sfb;            ///< number of scalefactor bands per group
    enum WindowSequence window_sequence[2];
    uint8_t use_kb_window[2];   ///< If set, use Kaiser-Bessel window, otherwise use a sine window.
    uint8_t group_len[8];
    const uint16_t *swb_offset; ///< table of offsets to the lowest spectral coefficient of a scalefactor band, sfb, for a particular window
    const uint8_t *swb_sizes;   ///< table of scalefactor band sizes for a particular window
    int num_swb;                ///< number of scalefactor window bands
    int num_windows;
    int tns_max_bands;
    uint8_t window_clipping[8]; ///< set if a certain window is near clipping
    float clip_avoidance_factor; ///< set if any window is near clipping to the necessary atennuation factor to avoid it
} IndividualChannelStream;

/**
 * Temporal Noise Shaping
 */
typedef struct TemporalNoiseShaping {
    int present;
    int n_filt[8];
    int length[8][4];
    int direction[8][4];
    int order[8][4];
    int coef_idx[8][4][TNS_MAX_ORDER];
    float coef[8][4][TNS_MAX_ORDER];
} TemporalNoiseShaping;

/**
 * Single Channel Element - used for both SCE and LFE elements.
 */
typedef struct SingleChannelElement {
    IndividualChannelStream ics;
    TemporalNoiseShaping tns;
    Pulse pulse;
    enum BandType band_type[128];                   ///< band types
    enum BandType band_alt[128];                    ///< alternative band type
    int sf_idx[128];                                ///< scalefactor indices
    uint8_t zeroes[128];                            ///< band is not coded
    uint8_t can_pns[128];                           ///< band is allowed to PNS (informative)
    float  is_ener[128];                            ///< Intensity stereo pos
    float pns_ener[128];                            ///< Noise energy values
    DECLARE_ALIGNED(32, float, pcoeffs)[1024];      ///< coefficients for IMDCT, pristine
    DECLARE_ALIGNED(32, float, coeffs)[1024];       ///< coefficients for IMDCT, maybe processed
    DECLARE_ALIGNED(32, float, ret_buf)[2048];      ///< PCM output buffer
    PredictorState predictor_state[MAX_PREDICTORS];
} SingleChannelElement;

/**
 * channel element - generic struct for SCE/CPE/CCE/LFE
 */
typedef struct ChannelElement {
    // CPE specific
    int common_window;        ///< Set if channels share a common 'IndividualChannelStream' in bitstream.
    int     ms_mode;          ///< Signals mid/side stereo flags coding mode
    uint8_t is_mode;          ///< Set if any bands have been encoded using intensity stereo
    uint8_t ms_mask[128];     ///< Set if mid/side stereo is used for each scalefactor window band
    uint8_t is_mask[128];     ///< Set if intensity stereo is used
    // shared
    SingleChannelElement ch[2];
} ChannelElement;

struct AACEncContext;

typedef struct AACCoefficientsEncoder {
    void (*search_for_quantizers)(AVCodecContext *avctx, struct AACEncContext *s,
                                  SingleChannelElement *sce, const float lambda);
    void (*encode_window_bands_info)(struct AACEncContext *s, SingleChannelElement *sce,
                                     int win, int group_len, const float lambda);
    void (*quantize_and_encode_band)(struct AACEncContext *s, PutBitContext *pb, const float *in, float *out, int size,
                                     int scale_idx, int cb, const float lambda, int rtz);
    void (*encode_tns_info)(struct AACEncContext *s, SingleChannelElement *sce);
    void (*apply_tns_filt)(struct AACEncContext *s, SingleChannelElement *sce);
    void (*set_special_band_scalefactors)(struct AACEncContext *s, SingleChannelElement *sce);
    void (*search_for_pns)(struct AACEncContext *s, AVCodecContext *avctx, SingleChannelElement *sce);
    void (*mark_pns)(struct AACEncContext *s, AVCodecContext *avctx, SingleChannelElement *sce);
    void (*search_for_tns)(struct AACEncContext *s, SingleChannelElement *sce);
    void (*search_for_ms)(struct AACEncContext *s, ChannelElement *cpe);
    void (*search_for_is)(struct AACEncContext *s, AVCodecContext *avctx, ChannelElement *cpe);
} AACCoefficientsEncoder;

extern const AACCoefficientsEncoder ff_aac_coders[];

typedef struct AACQuantizeBandCostCacheEntry {
    float rd;
    float energy;
    int bits;
    char cb;
    char rtz;
    uint16_t generation;
} AACQuantizeBandCostCacheEntry;

typedef struct AACPCEInfo {
    AVChannelLayout layout;
    int num_ele[4];                              ///< front, side, back, lfe
    int pairing[3][8];                           ///< front, side, back
    int index[4][8];                             ///< front, side, back, lfe
    uint8_t config_map[16];                      ///< configs the encoder's channel specific settings
    uint8_t reorder_map[16];                     ///< maps channels from lavc to aac order
} AACPCEInfo;

/**
 * AAC encoder context
 */
typedef struct AACEncContext {
    AVClass *av_class;
    AACEncOptions options;                       ///< encoding options
    PutBitContext pb;
    AVTXContext *mdct1024;                       ///< long (1024 samples) frame transform context
    av_tx_fn mdct1024_fn;
    AVTXContext *mdct128;                        ///< short (128 samples) frame transform context
    av_tx_fn mdct128_fn;
    AVFloatDSPContext *fdsp;
    AACPCEInfo pce;                              ///< PCE data, if needed
    float *planar_samples[16];                   ///< saved preprocessed input

    int profile;                                 ///< copied from avctx
    int needs_pce;                               ///< flag for non-standard layout
    LPCContext lpc;                              ///< used by TNS
    int samplerate_index;                        ///< MPEG-4 samplerate index
    int channels;                                ///< channel count
    const uint8_t *reorder_map;                  ///< lavc to aac reorder map
    const uint8_t *chan_map;                     ///< channel configuration map

    ChannelElement *cpe;                         ///< channel elements
    FFPsyContext psy;
    struct FFPsyPreprocessContext* psypp;
    const AACCoefficientsEncoder *coder;
    int cur_channel;                             ///< current channel for coder context
    int random_state;
    float lambda;
    int last_frame_pb_count;                     ///< number of bits for the previous frame
    float lambda_sum;                            ///< sum(lambda), for Qvg reporting
    int lambda_count;                            ///< count(lambda), for Qvg reporting
    enum RawDataBlockType cur_type;              ///< channel group type cur_channel belongs to

    AudioFrameQueue afq;
    DECLARE_ALIGNED(32, int,   qcoefs)[96];      ///< quantized coefficients
    DECLARE_ALIGNED(32, float, scoefs)[1024];    ///< scaled coefficients

    uint16_t quantize_band_cost_cache_generation;
    AACQuantizeBandCostCacheEntry quantize_band_cost_cache[256][128]; ///< memoization area for quantize_band_cost

    AACEncDSPContext aacdsp;

    struct {
        float *samples;
    } buffer;
} AACEncContext;

void ff_quantize_band_cost_cache_init(struct AACEncContext *s);


#endif /* AVCODEC_AACENC_H */
