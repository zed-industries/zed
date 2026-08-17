/*
 * DCA compatible decoder
 * Copyright (C) 2004 Gildas Bazin
 * Copyright (C) 2004 Benjamin Zores
 * Copyright (C) 2006 Benjamin Larsson
 * Copyright (C) 2007 Konstantin Shishkov
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

#ifndef AVCODEC_DCA_H
#define AVCODEC_DCA_H

#include <stdint.h>

#include "libavutil/common.h"
#include "libavutil/intreadwrite.h"

#include "get_bits.h"

#define DCA_CORE_FRAME_HEADER_SIZE      18

enum DCAParseError {
    DCA_PARSE_ERROR_SYNC_WORD       = -1,
    DCA_PARSE_ERROR_DEFICIT_SAMPLES = -2,
    DCA_PARSE_ERROR_PCM_BLOCKS      = -3,
    DCA_PARSE_ERROR_FRAME_SIZE      = -4,
    DCA_PARSE_ERROR_AMODE           = -5,
    DCA_PARSE_ERROR_SAMPLE_RATE     = -6,
    DCA_PARSE_ERROR_RESERVED_BIT    = -7,
    DCA_PARSE_ERROR_LFE_FLAG        = -8,
    DCA_PARSE_ERROR_PCM_RES         = -9,
};

typedef struct DCACoreFrameHeader {
    uint8_t     normal_frame;       ///< Frame type
    uint8_t     deficit_samples;    ///< Deficit sample count
    uint8_t     crc_present;        ///< CRC present flag
    uint8_t     npcmblocks;         ///< Number of PCM sample blocks
    uint16_t    frame_size;         ///< Primary frame byte size
    uint8_t     audio_mode;         ///< Audio channel arrangement
    uint8_t     sr_code;            ///< Core audio sampling frequency
    uint8_t     br_code;            ///< Transmission bit rate
    uint8_t     drc_present;        ///< Embedded dynamic range flag
    uint8_t     ts_present;         ///< Embedded time stamp flag
    uint8_t     aux_present;        ///< Auxiliary data flag
    uint8_t     hdcd_master;        ///< HDCD mastering flag
    uint8_t     ext_audio_type;     ///< Extension audio descriptor flag
    uint8_t     ext_audio_present;  ///< Extended coding flag
    uint8_t     sync_ssf;           ///< Audio sync word insertion flag
    uint8_t     lfe_present;        ///< Low frequency effects flag
    uint8_t     predictor_history;  ///< Predictor history flag switch
    uint8_t     filter_perfect;     ///< Multirate interpolator switch
    uint8_t     encoder_rev;        ///< Encoder software revision
    uint8_t     copy_hist;          ///< Copy history
    uint8_t     pcmr_code;          ///< Source PCM resolution
    uint8_t     sumdiff_front;      ///< Front sum/difference flag
    uint8_t     sumdiff_surround;   ///< Surround sum/difference flag
    uint8_t     dn_code;            ///< Dialog normalization / unspecified
} DCACoreFrameHeader;

enum DCASpeaker {
    DCA_SPEAKER_C,    DCA_SPEAKER_L,    DCA_SPEAKER_R,    DCA_SPEAKER_Ls,
    DCA_SPEAKER_Rs,   DCA_SPEAKER_LFE1, DCA_SPEAKER_Cs,   DCA_SPEAKER_Lsr,
    DCA_SPEAKER_Rsr,  DCA_SPEAKER_Lss,  DCA_SPEAKER_Rss,  DCA_SPEAKER_Lc,
    DCA_SPEAKER_Rc,   DCA_SPEAKER_Lh,   DCA_SPEAKER_Ch,   DCA_SPEAKER_Rh,
    DCA_SPEAKER_LFE2, DCA_SPEAKER_Lw,   DCA_SPEAKER_Rw,   DCA_SPEAKER_Oh,
    DCA_SPEAKER_Lhs,  DCA_SPEAKER_Rhs,  DCA_SPEAKER_Chr,  DCA_SPEAKER_Lhr,
    DCA_SPEAKER_Rhr,  DCA_SPEAKER_Cl,   DCA_SPEAKER_Ll,   DCA_SPEAKER_Rl,
    DCA_SPEAKER_RSV1, DCA_SPEAKER_RSV2, DCA_SPEAKER_RSV3, DCA_SPEAKER_RSV4,

    DCA_SPEAKER_COUNT
};

enum DCASpeakerMask {
    DCA_SPEAKER_MASK_C     = 0x00000001,
    DCA_SPEAKER_MASK_L     = 0x00000002,
    DCA_SPEAKER_MASK_R     = 0x00000004,
    DCA_SPEAKER_MASK_Ls    = 0x00000008,
    DCA_SPEAKER_MASK_Rs    = 0x00000010,
    DCA_SPEAKER_MASK_LFE1  = 0x00000020,
    DCA_SPEAKER_MASK_Cs    = 0x00000040,
    DCA_SPEAKER_MASK_Lsr   = 0x00000080,
    DCA_SPEAKER_MASK_Rsr   = 0x00000100,
    DCA_SPEAKER_MASK_Lss   = 0x00000200,
    DCA_SPEAKER_MASK_Rss   = 0x00000400,
    DCA_SPEAKER_MASK_Lc    = 0x00000800,
    DCA_SPEAKER_MASK_Rc    = 0x00001000,
    DCA_SPEAKER_MASK_Lh    = 0x00002000,
    DCA_SPEAKER_MASK_Ch    = 0x00004000,
    DCA_SPEAKER_MASK_Rh    = 0x00008000,
    DCA_SPEAKER_MASK_LFE2  = 0x00010000,
    DCA_SPEAKER_MASK_Lw    = 0x00020000,
    DCA_SPEAKER_MASK_Rw    = 0x00040000,
    DCA_SPEAKER_MASK_Oh    = 0x00080000,
    DCA_SPEAKER_MASK_Lhs   = 0x00100000,
    DCA_SPEAKER_MASK_Rhs   = 0x00200000,
    DCA_SPEAKER_MASK_Chr   = 0x00400000,
    DCA_SPEAKER_MASK_Lhr   = 0x00800000,
    DCA_SPEAKER_MASK_Rhr   = 0x01000000,
    DCA_SPEAKER_MASK_Cl    = 0x02000000,
    DCA_SPEAKER_MASK_Ll    = 0x04000000,
    DCA_SPEAKER_MASK_Rl    = 0x08000000,
};

#define DCA_SPEAKER_LAYOUT_MONO         (DCA_SPEAKER_MASK_C)
#define DCA_SPEAKER_LAYOUT_STEREO       (DCA_SPEAKER_MASK_L | DCA_SPEAKER_MASK_R)
#define DCA_SPEAKER_LAYOUT_2POINT1      (DCA_SPEAKER_LAYOUT_STEREO | DCA_SPEAKER_MASK_LFE1)
#define DCA_SPEAKER_LAYOUT_3_0          (DCA_SPEAKER_LAYOUT_STEREO | DCA_SPEAKER_MASK_C)
#define DCA_SPEAKER_LAYOUT_2_1          (DCA_SPEAKER_LAYOUT_STEREO | DCA_SPEAKER_MASK_Cs)
#define DCA_SPEAKER_LAYOUT_3_1          (DCA_SPEAKER_LAYOUT_3_0 | DCA_SPEAKER_MASK_Cs)
#define DCA_SPEAKER_LAYOUT_2_2          (DCA_SPEAKER_LAYOUT_STEREO | DCA_SPEAKER_MASK_Ls | DCA_SPEAKER_MASK_Rs)
#define DCA_SPEAKER_LAYOUT_5POINT0      (DCA_SPEAKER_LAYOUT_3_0 | DCA_SPEAKER_MASK_Ls | DCA_SPEAKER_MASK_Rs)
#define DCA_SPEAKER_LAYOUT_5POINT1      (DCA_SPEAKER_LAYOUT_5POINT0 | DCA_SPEAKER_MASK_LFE1)
#define DCA_SPEAKER_LAYOUT_7POINT0_WIDE (DCA_SPEAKER_LAYOUT_5POINT0 | DCA_SPEAKER_MASK_Lw | DCA_SPEAKER_MASK_Rw)
#define DCA_SPEAKER_LAYOUT_7POINT1_WIDE (DCA_SPEAKER_LAYOUT_7POINT0_WIDE | DCA_SPEAKER_MASK_LFE1)

#define DCA_HAS_STEREO(mask) \
    ((mask & DCA_SPEAKER_LAYOUT_STEREO) == DCA_SPEAKER_LAYOUT_STEREO)

enum DCASpeakerPair {
    DCA_SPEAKER_PAIR_C      = 0x0001,
    DCA_SPEAKER_PAIR_LR     = 0x0002,
    DCA_SPEAKER_PAIR_LsRs   = 0x0004,
    DCA_SPEAKER_PAIR_LFE1   = 0x0008,
    DCA_SPEAKER_PAIR_Cs     = 0x0010,
    DCA_SPEAKER_PAIR_LhRh   = 0x0020,
    DCA_SPEAKER_PAIR_LsrRsr = 0x0040,
    DCA_SPEAKER_PAIR_Ch     = 0x0080,
    DCA_SPEAKER_PAIR_Oh     = 0x0100,
    DCA_SPEAKER_PAIR_LcRc   = 0x0200,
    DCA_SPEAKER_PAIR_LwRw   = 0x0400,
    DCA_SPEAKER_PAIR_LssRss = 0x0800,
    DCA_SPEAKER_PAIR_LFE2   = 0x1000,
    DCA_SPEAKER_PAIR_LhsRhs = 0x2000,
    DCA_SPEAKER_PAIR_Chr    = 0x4000,
    DCA_SPEAKER_PAIR_LhrRhr = 0x8000
};

/**
 * Return number of individual channels in DCASpeakerPair mask
 */
static inline int ff_dca_count_chs_for_mask(unsigned int mask)
{
    return av_popcount((mask & 0xffff) | ((mask & 0xae66) << 16));
}

enum DCARepresentationType {
    DCA_REPR_TYPE_LtRt = 2,
    DCA_REPR_TYPE_LhRh = 3
};

enum DCAExtensionMask {
    DCA_CSS_CORE   = 0x001,
    DCA_CSS_XXCH   = 0x002,
    DCA_CSS_X96    = 0x004,
    DCA_CSS_XCH    = 0x008,
    DCA_CSS_MASK   = 0x00f,
    DCA_EXSS_CORE  = 0x010,
    DCA_EXSS_XBR   = 0x020,
    DCA_EXSS_XXCH  = 0x040,
    DCA_EXSS_X96   = 0x080,
    DCA_EXSS_LBR   = 0x100,
    DCA_EXSS_XLL   = 0x200,
    DCA_EXSS_RSV1  = 0x400,
    DCA_EXSS_RSV2  = 0x800,
    DCA_EXSS_MASK  = 0xff0,
};

enum DCADownMixType {
    DCA_DMIX_TYPE_1_0,
    DCA_DMIX_TYPE_LoRo,
    DCA_DMIX_TYPE_LtRt,
    DCA_DMIX_TYPE_3_0,
    DCA_DMIX_TYPE_2_1,
    DCA_DMIX_TYPE_2_2,
    DCA_DMIX_TYPE_3_1,

    DCA_DMIX_TYPE_COUNT
};

extern const uint32_t ff_dca_sample_rates[16];
extern const uint32_t ff_dca_sampling_freqs[16];
extern const uint8_t ff_dca_freq_ranges[16];
extern const uint8_t ff_dca_bits_per_sample[8];


/**
 * Convert bitstream to one representation based on sync marker
 */
int avpriv_dca_convert_bitstream(const uint8_t *src, int src_size, uint8_t *dst,
                                 int max_size);

/**
 * Parse and validate core frame header
 * @param[out] h    Pointer to struct where header info is written.
 * @param[in]  buf  Pointer to the data buffer
 * @param[in]  size Size of the data buffer
 * @return 0 on success, negative AVERROR code on failure
 */
int avpriv_dca_parse_core_frame_header(DCACoreFrameHeader *h, const uint8_t *buf, int size);

/**
 * Parse and validate core frame header
 * @param[out] h   Pointer to struct where header info is written.
 * @param[in]  gbc BitContext containing the first 120 bits of the frame.
 * @return 0 on success, negative DCA_PARSE_ERROR_ code on failure
 */
int ff_dca_parse_core_frame_header(DCACoreFrameHeader *h, GetBitContext *gb);

#endif /* AVCODEC_DCA_H */
