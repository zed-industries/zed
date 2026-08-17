/*
 * AAC definitions and structures
 * Copyright (c) 2005-2006 Oded Shimon ( ods15 ods15 dyndns org )
 * Copyright (c) 2006-2007 Maxim Gavrilov ( maxim.gavrilov gmail com )
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
 * AAC definitions and structures
 * @author Oded Shimon  ( ods15 ods15 dyndns org )
 * @author Maxim Gavrilov ( maxim.gavrilov gmail com )
 */

#ifndef AVCODEC_AAC_H
#define AVCODEC_AAC_H

#define MAX_CHANNELS 64
#define MAX_ELEM_ID 64

#define TNS_MAX_ORDER 20
#define MAX_LTP_LONG_SFB 40

enum RawDataBlockType {
    TYPE_SCE,
    TYPE_CPE,
    TYPE_CCE,
    TYPE_LFE,
    TYPE_DSE,
    TYPE_PCE,
    TYPE_FIL,
    TYPE_END,
};

enum ExtensionPayloadID {
    EXT_FILL,
    EXT_FILL_DATA,
    EXT_DATA_ELEMENT,
    EXT_DYNAMIC_RANGE = 0xb,
    EXT_SBR_DATA      = 0xd,
    EXT_SBR_DATA_CRC  = 0xe,
};

enum WindowSequence {
    ONLY_LONG_SEQUENCE,
    LONG_START_SEQUENCE,
    EIGHT_SHORT_SEQUENCE,
    LONG_STOP_SEQUENCE,
};

enum BandType {
    ZERO_BT        = 0,     ///< Scalefactors and spectral data are all zero.
    FIRST_PAIR_BT  = 5,     ///< This and later band types encode two values (rather than four) with one code word.
    ESC_BT         = 11,    ///< Spectral data are coded with an escape sequence.
    RESERVED_BT    = 12,    ///< Band types following are encoded differently from others.
    NOISE_BT       = 13,    ///< Spectral data are scaled white noise not coded in the bitstream.
    INTENSITY_BT2  = 14,    ///< Scalefactor data are intensity stereo positions (out of phase).
    INTENSITY_BT   = 15,    ///< Scalefactor data are intensity stereo positions (in phase).
};

enum ChannelPosition {
    AAC_CHANNEL_OFF   = 0,
    AAC_CHANNEL_FRONT = 1,
    AAC_CHANNEL_SIDE  = 2,
    AAC_CHANNEL_BACK  = 3,
    AAC_CHANNEL_LFE   = 4,
    AAC_CHANNEL_CC    = 5,
};

#define MAX_PREDICTORS 672

#define SCALE_DIV_512    36    ///< scalefactor difference that corresponds to scale difference in 512 times
#define SCALE_ONE_POS   140    ///< scalefactor index that corresponds to scale=1.0
#define SCALE_MAX_POS   255    ///< scalefactor index maximum value
#define SCALE_MAX_DIFF   60    ///< maximum scalefactor difference allowed by standard
#define SCALE_DIFF_ZERO  60    ///< codebook index corresponding to zero scalefactor indices difference

#define POW_SF2_ZERO    200    ///< ff_aac_pow2sf_tab index corresponding to pow(2, 0);

#define NOISE_PRE       256    ///< preamble for NOISE_BT, put in bitstream with the first noise band
#define NOISE_PRE_BITS    9    ///< length of preamble
#define NOISE_OFFSET     90    ///< subtracted from global gain, used as offset for the preamble

typedef struct Pulse {
    int num_pulse;
    int start;
    int pos[4];
    int amp[4];
} Pulse;

static inline int ff_aac_sample_rate_idx(int rate)
{
         if (92017 <= rate) return 0;
    else if (75132 <= rate) return 1;
    else if (55426 <= rate) return 2;
    else if (46009 <= rate) return 3;
    else if (37566 <= rate) return 4;
    else if (27713 <= rate) return 5;
    else if (23004 <= rate) return 6;
    else if (18783 <= rate) return 7;
    else if (13856 <= rate) return 8;
    else if (11502 <= rate) return 9;
    else if (9391  <= rate) return 10;
    else                    return 11;
}

#endif /* AVCODEC_AAC_H */
