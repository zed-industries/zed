/*
 * Common code between the AC-3 and E-AC-3 decoders
 * Copyright (c) 2007 Bartlomiej Wolowiec <bartek.wolowiec@gmail.com>
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
 * Common code between the AC-3 and E-AC-3 decoders.
 *
 * Summary of MDCT Coefficient Grouping:
 * The individual MDCT coefficient indices are often referred to in the
 * (E-)AC-3 specification as frequency bins.  These bins are grouped together
 * into subbands of 12 coefficients each.  The subbands are grouped together
 * into bands as defined in the bitstream by the band structures, which
 * determine the number of bands and the size of each band.  The full spectrum
 * of 256 frequency bins is divided into 1 DC bin + 21 subbands = 253 bins.
 * This system of grouping coefficients is used for channel bandwidth, stereo
 * rematrixing, channel coupling, enhanced coupling, and spectral extension.
 *
 * +-+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+-+
 * |1|  |12|  |  [12|12|12|12]  |  |  |  |  |  |  |  |  |  |  |  |  |3|
 * +-+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+-+
 * ~~~  ~~~~     ~~~~~~~~~~~~~                                      ~~~
 *  |     |            |                                             |
 *  |     |            |                    3 unused frequency bins--+
 *  |     |            |
 *  |     |            +--1 band containing 4 subbands
 *  |     |
 *  |     +--1 subband of 12 frequency bins
 *  |
 *  +--DC frequency bin
 */

#ifndef AVCODEC_AC3DEC_H
#define AVCODEC_AC3DEC_H

#include "libavutil/tx.h"
#include "libavutil/float_dsp.h"
#include "libavutil/fixed_dsp.h"
#include "libavutil/lfg.h"
#include "libavutil/mem_internal.h"

#include "ac3.h"
#include "ac3dsp.h"
#include "avcodec.h"
#include "bswapdsp.h"
#include "get_bits.h"
#include "fmtconvert.h"

#define AC3_OUTPUT_LFEON  8

#define SPX_MAX_BANDS    17

/** Large enough for maximum possible frame size when the specification limit is ignored */
#define AC3_FRAME_BUFFER_SIZE 32768

typedef struct AC3DecodeContext {
    AVClass        *class;                  ///< class for AVOptions
    AVCodecContext *avctx;                  ///< parent context
    GetBitContext gbc;                      ///< bitstream reader

///@name Optimization
    BswapDSPContext bdsp;
#if USE_FIXED
    AVFixedDSPContext *fdsp;
#else
    AVFloatDSPContext *fdsp;
#endif
    AC3DSPContext ac3dsp;
    FmtConvertContext fmt_conv;             ///< optimized conversion functions
///@}

    AVTXContext *tx_128, *tx_256;
    av_tx_fn tx_fn_128, tx_fn_256;

    INTFLOAT *xcfptr[AC3_MAX_CHANNELS];
    INTFLOAT *dlyptr[AC3_MAX_CHANNELS];

    AVChannelLayout downmix_layout;
    SHORTFLOAT *downmix_coeffs[2];              ///< stereo downmix coefficients

// Start of flushable fields.
// frame_type must be the flushable field, or the offset changed in ac3_decode_flush().

///@name Bit stream information
///@{
    int frame_type;                         ///< frame type                             (strmtyp)
    int substreamid;                        ///< substream identification
    int superframe_size;                    ///< current superframe size, in bytes
    int frame_size;                         ///< current frame size, in bytes
    int bit_rate;                           ///< stream bit rate, in bits-per-second
    int sample_rate;                        ///< sample frequency, in Hz
    int num_blocks;                         ///< number of audio blocks
    int bitstream_id;                       ///< bitstream id                           (bsid)
    int bitstream_mode;                     ///< bitstream mode                         (bsmod)
    int channel_mode;                       ///< channel mode                           (acmod)
    int lfe_on;                             ///< lfe channel in use
    int dialog_normalization[2];            ///< dialog level in dBFS                   (dialnorm)
    int compression_exists[2];              ///< compression field is valid for frame   (compre)
    int channel_map;                        ///< custom channel map                     (chanmap)
    int preferred_downmix;                  ///< Preferred 2-channel downmix mode       (dmixmod)
    int center_mix_level;                   ///< Center mix level index
    int center_mix_level_ltrt;              ///< Center mix level index for Lt/Rt       (ltrtcmixlev)
    int surround_mix_level;                 ///< Surround mix level index
    int surround_mix_level_ltrt;            ///< Surround mix level index for Lt/Rt     (ltrtsurmixlev)
    int lfe_mix_level_exists;               ///< indicates if lfemixlevcod is specified (lfemixlevcode)
    int lfe_mix_level;                      ///< LFE mix level index                    (lfemixlevcod)
    int eac3;                               ///< indicates if current frame is E-AC-3
    int eac3_subsbtreamid_found;            ///< bitstream has E-AC-3 additional substream(s)
    int eac3_extension_type_a;              ///< bitstream has E-AC-3 extension type A enabled frame(s)
    int dolby_surround_mode;                ///< dolby surround mode                    (dsurmod)
    int dolby_surround_ex_mode;             ///< dolby surround ex mode                 (dsurexmod)
    int dolby_headphone_mode;               ///< dolby headphone mode                   (dheadphonmod)
///@}

    int preferred_stereo_downmix;
    float ltrt_center_mix_level;
    float ltrt_surround_mix_level;
    float loro_center_mix_level;
    float loro_surround_mix_level;
    int target_level;                       ///< target level in dBFS
    float level_gain[2];

///@name Frame syntax parameters
    int snr_offset_strategy;                ///< SNR offset strategy                    (snroffststr)
    int block_switch_syntax;                ///< block switch syntax enabled            (blkswe)
    int dither_flag_syntax;                 ///< dither flag syntax enabled             (dithflage)
    int bit_allocation_syntax;              ///< bit allocation model syntax enabled    (bamode)
    int fast_gain_syntax;                   ///< fast gain codes enabled                (frmfgaincode)
    int dba_syntax;                         ///< delta bit allocation syntax enabled    (dbaflde)
    int skip_syntax;                        ///< skip field syntax enabled              (skipflde)
 ///@}

///@name Standard coupling
    int cpl_in_use[AC3_MAX_BLOCKS];         ///< coupling in use                        (cplinu)
    int cpl_strategy_exists[AC3_MAX_BLOCKS];///< coupling strategy exists               (cplstre)
    int channel_in_cpl[AC3_MAX_CHANNELS];   ///< channel in coupling                    (chincpl)
    int phase_flags_in_use;                 ///< phase flags in use                     (phsflginu)
    int phase_flags[AC3_MAX_CPL_BANDS];     ///< phase flags                            (phsflg)
    int num_cpl_bands;                      ///< number of coupling bands               (ncplbnd)
    uint8_t cpl_band_struct[AC3_MAX_CPL_BANDS];
    uint8_t cpl_band_sizes[AC3_MAX_CPL_BANDS]; ///< number of coeffs in each coupling band
    int firstchincpl;                       ///< first channel in coupling
    int first_cpl_coords[AC3_MAX_CHANNELS]; ///< first coupling coordinates states      (firstcplcos)
    int cpl_coords[AC3_MAX_CHANNELS][AC3_MAX_CPL_BANDS]; ///< coupling coordinates      (cplco)
///@}

///@name Spectral extension
///@{
    int spx_in_use;                             ///< spectral extension in use              (spxinu)
    uint8_t channel_uses_spx[AC3_MAX_CHANNELS]; ///< channel uses spectral extension        (chinspx)
    int8_t spx_atten_code[AC3_MAX_CHANNELS];    ///< spx attenuation code                   (spxattencod)
    int spx_src_start_freq;                     ///< spx start frequency bin
    int spx_dst_end_freq;                       ///< spx end frequency bin
    int spx_dst_start_freq;                     ///< spx starting frequency bin for copying (copystartmant)
                                                ///< the copy region ends at the start of the spx region.
    int num_spx_bands;                          ///< number of spx bands                    (nspxbnds)
    uint8_t spx_band_struct[SPX_MAX_BANDS];
    uint8_t spx_band_sizes[SPX_MAX_BANDS];      ///< number of bins in each spx band
    uint8_t first_spx_coords[AC3_MAX_CHANNELS]; ///< first spx coordinates states           (firstspxcos)
    INTFLOAT spx_noise_blend[AC3_MAX_CHANNELS][SPX_MAX_BANDS]; ///< spx noise blending factor  (nblendfact)
    INTFLOAT spx_signal_blend[AC3_MAX_CHANNELS][SPX_MAX_BANDS];///< spx signal blending factor (sblendfact)
///@}

///@name Adaptive hybrid transform
    int channel_uses_aht[AC3_MAX_CHANNELS];                         ///< channel AHT in use (chahtinu)
    int pre_mantissa[AC3_MAX_CHANNELS][AC3_MAX_COEFS][AC3_MAX_BLOCKS];  ///< pre-IDCT mantissas
///@}

///@name Channel
    int fbw_channels;                           ///< number of full-bandwidth channels
    int channels;                               ///< number of total channels
    int lfe_ch;                                 ///< index of LFE channel
    int downmixed;                              ///< indicates if coeffs are currently downmixed
    int output_mode;                            ///< output channel configuration
    int prev_output_mode;                       ///< output channel configuration for previous frame
    int out_channels;                           ///< number of output channels
    int prev_bit_rate;                          ///< stream bit rate, in bits-per-second for previous frame
///@}

///@name Dynamic range
    INTFLOAT dynamic_range[2];                 ///< dynamic range
    INTFLOAT drc_scale;                        ///< percentage of dynamic range compression to be applied
    int heavy_compression;                     ///< apply heavy compression
    INTFLOAT heavy_dynamic_range[2];           ///< heavy dynamic range compression
///@}

///@name Bandwidth
    int start_freq[AC3_MAX_CHANNELS];       ///< start frequency bin                    (strtmant)
    int end_freq[AC3_MAX_CHANNELS];         ///< end frequency bin                      (endmant)
///@}

///@name Consistent noise generation
    int consistent_noise_generation;        ///< seed noise generation with AC-3 frame on decode
///@}

///@name Rematrixing
    int num_rematrixing_bands;              ///< number of rematrixing bands            (nrematbnd)
    int rematrixing_flags[4];               ///< rematrixing flags                      (rematflg)
///@}

///@name Exponents
    int num_exp_groups[AC3_MAX_CHANNELS];           ///< Number of exponent groups      (nexpgrp)
    int8_t dexps[AC3_MAX_CHANNELS][AC3_MAX_COEFS];  ///< decoded exponents
    int exp_strategy[AC3_MAX_BLOCKS][AC3_MAX_CHANNELS]; ///< exponent strategies        (expstr)
///@}

///@name Bit allocation
    AC3BitAllocParameters bit_alloc_params;         ///< bit allocation parameters
    int first_cpl_leak;                             ///< first coupling leak state      (firstcplleak)
    int snr_offset[AC3_MAX_CHANNELS];               ///< signal-to-noise ratio offsets  (snroffst)
    int fast_gain[AC3_MAX_CHANNELS];                ///< fast gain values/SMR's         (fgain)
    uint8_t bap[AC3_MAX_CHANNELS][AC3_MAX_COEFS];   ///< bit allocation pointers
    int16_t psd[AC3_MAX_CHANNELS][AC3_MAX_COEFS];   ///< scaled exponents
    int16_t band_psd[AC3_MAX_CHANNELS][AC3_CRITICAL_BANDS]; ///< interpolated exponents
    int16_t mask[AC3_MAX_CHANNELS][AC3_CRITICAL_BANDS];     ///< masking curve values
    int dba_mode[AC3_MAX_CHANNELS];                 ///< delta bit allocation mode
    int dba_nsegs[AC3_MAX_CHANNELS];                ///< number of delta segments
    uint8_t dba_offsets[AC3_MAX_CHANNELS][8];       ///< delta segment offsets
    uint8_t dba_lengths[AC3_MAX_CHANNELS][8];       ///< delta segment lengths
    uint8_t dba_values[AC3_MAX_CHANNELS][8];        ///< delta values for each segment
///@}

///@name Zero-mantissa dithering
    int dither_flag[AC3_MAX_CHANNELS];      ///< dither flags                           (dithflg)
    AVLFG dith_state;                       ///< for dither generation
///@}

///@name IMDCT
    int block_switch[AC3_MAX_CHANNELS];     ///< block switch flags                     (blksw)
///@}

    SHORTFLOAT *outptr[AC3_MAX_CHANNELS];

///@name Aligned arrays
    DECLARE_ALIGNED(16, int,   fixed_coeffs)[AC3_MAX_CHANNELS][AC3_MAX_COEFS];       ///< fixed-point transform coefficients
    DECLARE_ALIGNED(32, INTFLOAT, transform_coeffs)[AC3_MAX_CHANNELS][AC3_MAX_COEFS];   ///< transform coefficients
    DECLARE_ALIGNED(32, INTFLOAT, delay)[EAC3_MAX_CHANNELS][AC3_BLOCK_SIZE];         ///< delay - added to the next block
    DECLARE_ALIGNED(32, INTFLOAT, window)[AC3_BLOCK_SIZE];                              ///< window coefficients
    DECLARE_ALIGNED(32, INTFLOAT, tmp_output)[AC3_BLOCK_SIZE];                          ///< temporary storage for output before windowing
    DECLARE_ALIGNED(32, SHORTFLOAT, output)[EAC3_MAX_CHANNELS][AC3_BLOCK_SIZE];            ///< output after imdct transform and windowing
    DECLARE_ALIGNED(32, uint8_t, input_buffer)[AC3_FRAME_BUFFER_SIZE + AV_INPUT_BUFFER_PADDING_SIZE]; ///< temp buffer to prevent overread
    DECLARE_ALIGNED(32, SHORTFLOAT, output_buffer)[EAC3_MAX_CHANNELS][AC3_BLOCK_SIZE * 6];  ///< final output buffer
///@}
} AC3DecodeContext;

/**
 * Parse the E-AC-3 frame header.
 * This parses both the bit stream info and audio frame header.
 */
static int ff_eac3_parse_header(AC3DecodeContext *s);

/**
 * Decode mantissas in a single channel for the entire frame.
 * This is used when AHT mode is enabled.
 */
static void ff_eac3_decode_transform_coeffs_aht_ch(AC3DecodeContext *s, int ch);

/**
 * Apply spectral extension to each channel by copying lower frequency
 * coefficients to higher frequency bins and applying side information to
 * approximate the original high frequency signal.
 */
static void ff_eac3_apply_spectral_extension(AC3DecodeContext *s);

#if (!USE_FIXED)
extern float ff_ac3_heavy_dynamic_range_tab[256];
#endif

#endif /* AVCODEC_AC3DEC_H */
