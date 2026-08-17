/*
 * AC-3 encoder & E-AC-3 encoder common header
 * Copyright (c) 2000 Fabrice Bellard
 * Copyright (c) 2006-2010 Justin Ruggles <justin.ruggles@gmail.com>
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
 * AC-3 encoder & E-AC-3 encoder common header
 */

#ifndef AVCODEC_AC3ENC_H
#define AVCODEC_AC3ENC_H

#include <stdint.h>

#include "libavutil/mem_internal.h"
#include "libavutil/opt.h"
#include "libavutil/tx.h"

#include "ac3.h"
#include "ac3defs.h"
#include "ac3dsp.h"
#include "avcodec.h"
#include "codec_internal.h"
#include "mathops.h"
#include "me_cmp.h"
#include "audiodsp.h"

#ifndef AC3ENC_FLOAT
#define AC3ENC_FLOAT 0
#endif

#if AC3ENC_FLOAT
#include "libavutil/float_dsp.h"
#define MAC_COEF(d,a,b) ((d)+=(a)*(b))
#define COEF_MIN (-16777215.0/16777216.0)
#define COEF_MAX ( 16777215.0/16777216.0)
#define NEW_CPL_COORD_THRESHOLD 0.03
typedef float SampleType;
typedef float CoefType;
typedef float CoefSumType;
#else
#include "libavutil/fixed_dsp.h"
#define MAC_COEF(d,a,b) MAC64(d,a,b)
#define COEF_MIN -16777215
#define COEF_MAX  16777215
#define NEW_CPL_COORD_THRESHOLD 503317
typedef int32_t SampleType;
typedef int32_t CoefType;
typedef int64_t CoefSumType;
#endif

/* common option values */
#define AC3ENC_OPT_NONE            -1
#define AC3ENC_OPT_AUTO            -1
#define AC3ENC_OPT_OFF              0
#define AC3ENC_OPT_ON               1
#define AC3ENC_OPT_NOT_INDICATED    0
#define AC3ENC_OPT_MODE_ON          2
#define AC3ENC_OPT_MODE_OFF         1
#define AC3ENC_OPT_DSUREX_DPLIIZ    3

/* specific option values */
#define AC3ENC_OPT_LARGE_ROOM       1
#define AC3ENC_OPT_SMALL_ROOM       2
#define AC3ENC_OPT_DOWNMIX_LTRT     1
#define AC3ENC_OPT_DOWNMIX_LORO     2
#define AC3ENC_OPT_DOWNMIX_DPLII    3 // reserved value in A/52, but used by encoders to indicate DPL2
#define AC3ENC_OPT_ADCONV_STANDARD  0
#define AC3ENC_OPT_ADCONV_HDCD      1


/**
 * Encoding Options used by AVOption.
 */
typedef struct AC3EncOptions {
    /* AC-3 metadata options*/
    int dialogue_level;
    int bitstream_mode;
    float center_mix_level;
    float surround_mix_level;
    int dolby_surround_mode;
    int audio_production_info;
    int mixing_level;
    int room_type;
    int copyright;
    int original;
    int extended_bsi_1;
    int preferred_stereo_downmix;
    float ltrt_center_mix_level;
    float ltrt_surround_mix_level;
    float loro_center_mix_level;
    float loro_surround_mix_level;
    int extended_bsi_2;
    int dolby_surround_ex_mode;
    int dolby_headphone_mode;
    int ad_converter_type;
    int eac3_mixing_metadata;
    int eac3_info_metadata;

    /* other encoding options */
    int allow_per_frame_metadata;
    int stereo_rematrixing;
    int channel_coupling;
    int cpl_start;
} AC3EncOptions;

/**
 * Data for a single audio block.
 */
typedef struct AC3Block {
    CoefType *mdct_coef[AC3_MAX_CHANNELS];      ///< MDCT coefficients
    int32_t  *fixed_coef[AC3_MAX_CHANNELS];     ///< fixed-point MDCT coefficients
    uint8_t  *exp[AC3_MAX_CHANNELS];            ///< original exponents
    uint8_t  *grouped_exp[AC3_MAX_CHANNELS];    ///< grouped exponents
    int16_t  *psd[AC3_MAX_CHANNELS];            ///< psd per frequency bin
    int16_t  *band_psd[AC3_MAX_CHANNELS];       ///< psd per critical band
    int16_t  *mask[AC3_MAX_CHANNELS];           ///< masking curve
    uint16_t *qmant[AC3_MAX_CHANNELS];          ///< quantized mantissas
    uint8_t  *cpl_coord_exp[AC3_MAX_CHANNELS];  ///< coupling coord exponents           (cplcoexp)
    uint8_t  *cpl_coord_mant[AC3_MAX_CHANNELS]; ///< coupling coord mantissas           (cplcomant)
    uint8_t  new_rematrixing_strategy;          ///< send new rematrixing flags in this block
    int      num_rematrixing_bands;             ///< number of rematrixing bands
    uint8_t  rematrixing_flags[4];              ///< rematrixing flags
    int      new_cpl_strategy;                  ///< send new coupling strategy
    int      cpl_in_use;                        ///< coupling in use for this block     (cplinu)
    uint8_t  channel_in_cpl[AC3_MAX_CHANNELS];  ///< channel in coupling                (chincpl)
    int      num_cpl_channels;                  ///< number of channels in coupling
    uint8_t  new_cpl_coords[AC3_MAX_CHANNELS];  ///< send new coupling coordinates      (cplcoe)
    uint8_t  cpl_master_exp[AC3_MAX_CHANNELS];  ///< coupling coord master exponents    (mstrcplco)
    int      new_snr_offsets;                   ///< send new SNR offsets
    int      new_cpl_leak;                      ///< send new coupling leak info
    int      end_freq[AC3_MAX_CHANNELS];        ///< end frequency bin                  (endmant)
} AC3Block;

struct PutBitContext;

/**
 * AC-3 encoder private context.
 */
typedef struct AC3EncodeContext {
    AVClass *av_class;                      ///< AVClass used for AVOption
    AC3EncOptions options;                  ///< encoding options
    AVCodecContext *avctx;                  ///< parent AVCodecContext
    AudioDSPContext adsp;
#if AC3ENC_FLOAT
    AVFloatDSPContext *fdsp;
#else
    AVFixedDSPContext *fdsp;
#endif
    MECmpContext mecc;
    AC3DSPContext ac3dsp;                   ///< AC-3 optimized functions
    AVTXContext *tx;                        ///< FFT context for MDCT calculation
    av_tx_fn tx_fn;

    AC3Block blocks[AC3_MAX_BLOCKS];        ///< per-block info

    int fixed_point;                        ///< indicates if fixed-point encoder is being used
    int eac3;                               ///< indicates if this is E-AC-3 vs. AC-3
    int bitstream_id;                       ///< bitstream id                           (bsid)
    int bitstream_mode;                     ///< bitstream mode                         (bsmod)

    int bit_rate;                           ///< target bit rate, in bits-per-second
    int sample_rate;                        ///< sampling frequency, in Hz

    int num_blks_code;                      ///< number of blocks code                  (numblkscod)
    int num_blocks;                         ///< number of blocks per frame
    int frame_size_min;                     ///< minimum frame size in case rounding is necessary
    int frame_size;                         ///< current frame size in bytes
    int frame_size_code;                    ///< frame size code                        (frmsizecod)
    uint16_t crc_inv[2];
    int64_t bits_written;                   ///< bit count    (used to avg. bitrate)
    int64_t samples_written;                ///< sample count (used to avg. bitrate)

    int fbw_channels;                       ///< number of full-bandwidth channels      (nfchans)
    int channels;                           ///< total number of channels               (nchans)
    int lfe_on;                             ///< indicates if there is an LFE channel   (lfeon)
    int lfe_channel;                        ///< channel index of the LFE channel
    int has_center;                         ///< indicates if there is a center channel
    int has_surround;                       ///< indicates if there are one or more surround channels
    int channel_mode;                       ///< channel mode                           (acmod)
    const uint8_t *channel_map;             ///< channel map used to reorder channels

    int center_mix_level;                   ///< center mix level code
    int surround_mix_level;                 ///< surround mix level code
    int ltrt_center_mix_level;              ///< Lt/Rt center mix level code
    int ltrt_surround_mix_level;            ///< Lt/Rt surround mix level code
    int loro_center_mix_level;              ///< Lo/Ro center mix level code
    int loro_surround_mix_level;            ///< Lo/Ro surround mix level code

    int cutoff;                             ///< user-specified cutoff frequency, in Hz
    int bandwidth_code;                     ///< bandwidth code (0 to 60)               (chbwcod)
    int start_freq[AC3_MAX_CHANNELS];       ///< start frequency bin                    (strtmant)
    int cpl_end_freq;                       ///< coupling channel end frequency bin

    int cpl_on;                             ///< coupling turned on for this frame
    int cpl_enabled;                        ///< coupling enabled for all frames
    int num_cpl_subbands;                   ///< number of coupling subbands            (ncplsubnd)
    int num_cpl_bands;                      ///< number of coupling bands               (ncplbnd)
    uint8_t cpl_band_sizes[AC3_MAX_CPL_BANDS];  ///< number of coeffs in each coupling band

    int rematrixing_enabled;                ///< stereo rematrixing enabled

    /* bitrate allocation control */
    int slow_gain_code;                     ///< slow gain code                         (sgaincod)
    int slow_decay_code;                    ///< slow decay code                        (sdcycod)
    int fast_decay_code;                    ///< fast decay code                        (fdcycod)
    int db_per_bit_code;                    ///< dB/bit code                            (dbpbcod)
    int floor_code;                         ///< floor code                             (floorcod)
    AC3BitAllocParameters bit_alloc;        ///< bit allocation parameters
    int coarse_snr_offset;                  ///< coarse SNR offsets                     (csnroffst)
    int fast_gain_code[AC3_MAX_CHANNELS];   ///< fast gain codes (signal-to-mask ratio) (fgaincod)
    int fine_snr_offset[AC3_MAX_CHANNELS];  ///< fine SNR offsets                       (fsnroffst)
    int frame_bits_fixed;                   ///< number of non-coefficient bits for fixed parameters
    int frame_bits;                         ///< all frame bits except exponents and mantissas
    int exponent_bits;                      ///< number of bits used for exponents

    uint8_t *planar_samples[AC3_MAX_CHANNELS - 1];
    uint8_t *bap_buffer;
    uint8_t *bap1_buffer;
    CoefType *mdct_coef_buffer;
    int32_t *fixed_coef_buffer;
    uint8_t *exp_buffer;
    uint8_t *grouped_exp_buffer;
    int16_t *psd_buffer;
    int16_t *band_psd_buffer;
    int16_t *mask_buffer;
    int16_t *qmant_buffer;
    uint8_t *cpl_coord_buffer;

    uint8_t exp_strategy[AC3_MAX_CHANNELS][AC3_MAX_BLOCKS]; ///< exponent strategies
    uint8_t frame_exp_strategy[AC3_MAX_CHANNELS];           ///< frame exp strategy index
    int use_frame_exp_strategy;                             ///< indicates use of frame exp strategy
    uint8_t exp_ref_block[AC3_MAX_CHANNELS][AC3_MAX_BLOCKS]; ///< reference blocks for EXP_REUSE
    uint8_t *ref_bap     [AC3_MAX_CHANNELS][AC3_MAX_BLOCKS]; ///< bit allocation pointers (bap)
    int ref_bap_set;                                         ///< indicates if ref_bap pointers have been set

    /** fixed vs. float function pointers */
    void (*encode_frame)(struct AC3EncodeContext *s, uint8_t * const *samples);

    /* AC-3 vs. E-AC-3 function pointers */
    void (*output_frame_header)(struct AC3EncodeContext *s, struct PutBitContext *pb);

    union {
        DECLARE_ALIGNED(32, float,   mdct_window_float)[AC3_BLOCK_SIZE];
        DECLARE_ALIGNED(32, int32_t, mdct_window_fixed)[AC3_BLOCK_SIZE];
    };
    union {
        DECLARE_ALIGNED(32, float,   windowed_samples_float)[AC3_WINDOW_SIZE];
        DECLARE_ALIGNED(32, int32_t, windowed_samples_fixed)[AC3_WINDOW_SIZE];
    };
} AC3EncodeContext;

extern const AVChannelLayout ff_ac3_ch_layouts[19];
extern const AVOption ff_ac3_enc_options[];
extern const AVClass ff_ac3enc_class;
extern const FFCodecDefault ff_ac3_enc_defaults[];

int ff_ac3_encode_init(AVCodecContext *avctx);
int ff_ac3_float_encode_init(AVCodecContext *avctx);

int ff_ac3_encode_close(AVCodecContext *avctx);


void ff_ac3_compute_coupling_strategy(AC3EncodeContext *s);

int ff_ac3_encode_frame(AVCodecContext *avctx, AVPacket *avpkt,
                        const AVFrame *frame, int *got_packet_ptr);

#endif /* AVCODEC_AC3ENC_H */
