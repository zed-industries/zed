/*
 * audio encoder psychoacoustic model
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

#ifndef AVCODEC_PSYMODEL_H
#define AVCODEC_PSYMODEL_H

#include "avcodec.h"

/** maximum possible number of bands */
#define PSY_MAX_BANDS 128
/** maximum number of channels */
#define PSY_MAX_CHANS 20

/* cutoff for VBR is purposely increased, since LP filtering actually
 * hinders VBR performance rather than the opposite
 */
#define AAC_CUTOFF_FROM_BITRATE(bit_rate,channels,sample_rate) (bit_rate ? FFMIN3(FFMIN3( \
    FFMAX(bit_rate/channels/5, bit_rate/channels*15/32 - 5500), \
    3000 + bit_rate/channels/4, \
    12000 + bit_rate/channels/16), \
    22000, \
    sample_rate / 2): (sample_rate / 2))
#define AAC_CUTOFF(s) ( \
    (s->flags & AV_CODEC_FLAG_QSCALE) \
    ? s->sample_rate / 2 \
    : AAC_CUTOFF_FROM_BITRATE(s->bit_rate, s->ch_layout.nb_channels, s->sample_rate) \
)

/**
 * single band psychoacoustic information
 */
typedef struct FFPsyBand {
    int   bits;
    float energy;
    float threshold;
    float spread;    /* Energy spread over the band */
} FFPsyBand;

/**
 * single channel psychoacoustic information
 */
typedef struct FFPsyChannel {
    FFPsyBand psy_bands[PSY_MAX_BANDS]; ///< channel bands information
    float     entropy;                  ///< total PE for this channel
} FFPsyChannel;

/**
 * psychoacoustic information for an arbitrary group of channels
 */
typedef struct FFPsyChannelGroup {
    FFPsyChannel *ch[PSY_MAX_CHANS];  ///< pointers to the individual channels in the group
    uint8_t num_ch;                   ///< number of channels in this group
    uint8_t coupling[PSY_MAX_BANDS];  ///< allow coupling for this band in the group
} FFPsyChannelGroup;

/**
 * windowing related information
 */
typedef struct FFPsyWindowInfo {
    int window_type[3];               ///< window type (short/long/transitional, etc.) - current, previous and next
    int window_shape;                 ///< window shape (sine/KBD/whatever)
    int num_windows;                  ///< number of windows in a frame
    int grouping[8];                  ///< window grouping (for e.g. AAC)
    float clipping[8];                ///< maximum absolute normalized intensity in the given window for clip avoidance
    int *window_sizes;                ///< sequence of window sizes inside one frame (for eg. WMA)
} FFPsyWindowInfo;

/**
 * context used by psychoacoustic model
 */
typedef struct FFPsyContext {
    AVCodecContext *avctx;            ///< encoder context
    const struct FFPsyModel *model;   ///< encoder-specific model functions

    FFPsyChannel      *ch;            ///< single channel information
    FFPsyChannelGroup *group;         ///< channel group information
    int num_groups;                   ///< number of channel groups
    int cutoff;                       ///< lowpass frequency cutoff for analysis

    uint8_t **bands;                  ///< scalefactor band sizes for possible frame sizes
    int     *num_bands;               ///< number of scalefactor bands for possible frame sizes
    int num_lens;                     ///< number of scalefactor band sets

    struct {
        int size;                     ///< size of the bitresevoir in bits
        int bits;                     ///< number of bits used in the bitresevoir
        int alloc;                    ///< number of bits allocated by the psy, or -1 if no allocation was done
    } bitres;

    void* model_priv_data;            ///< psychoacoustic model implementation private data
} FFPsyContext;

/**
 * codec-specific psychoacoustic model implementation
 */
typedef struct FFPsyModel {
    const char *name;
    int  (*init)   (FFPsyContext *apc);

    /**
     * Suggest window sequence for channel.
     *
     * @param ctx       model context
     * @param audio     samples for the current frame
     * @param la        lookahead samples (NULL when unavailable)
     * @param channel   number of channel element to analyze
     * @param prev_type previous window type
     *
     * @return suggested window information in a structure
     */
    FFPsyWindowInfo (*window)(FFPsyContext *ctx, const float *audio, const float *la, int channel, int prev_type);

    /**
     * Perform psychoacoustic analysis and set band info (threshold, energy) for a group of channels.
     *
     * @param ctx      model context
     * @param channel  channel number of the first channel in the group to perform analysis on
     * @param coeffs   array of pointers to the transformed coefficients
     * @param wi       window information for the channels in the group
     */
    void (*analyze)(FFPsyContext *ctx, int channel, const float **coeffs, const FFPsyWindowInfo *wi);

    void (*end)    (FFPsyContext *apc);
} FFPsyModel;

/**
 * Initialize psychoacoustic model.
 *
 * @param ctx        model context
 * @param avctx      codec context
 * @param num_lens   number of possible frame lengths
 * @param bands      scalefactor band lengths for all frame lengths
 * @param num_bands  number of scalefactor bands for all frame lengths
 * @param num_groups number of channel groups
 * @param group_map  array with # of channels in group - 1, for each group
 *
 * @return zero if successful, a negative value if not
 */
int ff_psy_init(FFPsyContext *ctx, AVCodecContext *avctx, int num_lens,
                const uint8_t **bands, const int *num_bands,
                int num_groups, const uint8_t *group_map);

/**
 * Determine what group a channel belongs to.
 *
 * @param ctx     psymodel context
 * @param channel channel to locate the group for
 *
 * @return pointer to the FFPsyChannelGroup this channel belongs to
 */
FFPsyChannelGroup *ff_psy_find_group(FFPsyContext *ctx, int channel);

/**
 * Cleanup model context at the end.
 *
 * @param ctx model context
 */
void ff_psy_end(FFPsyContext *ctx);


/**************************************************************************
 *                       Audio preprocessing stuff.                       *
 *       This should be moved into some audio filter eventually.          *
 **************************************************************************/
struct FFPsyPreprocessContext;

/**
 * psychoacoustic model audio preprocessing initialization
 */
struct FFPsyPreprocessContext *ff_psy_preprocess_init(AVCodecContext *avctx);

/**
 * Preprocess several channel in audio frame in order to compress it better.
 *
 * @param ctx      preprocessing context
 * @param audio    samples to be filtered (in place)
 * @param channels number of channel to preprocess
 */
void ff_psy_preprocess(struct FFPsyPreprocessContext *ctx, float **audio, int channels);

/**
 * Cleanup audio preprocessing module.
 */
void ff_psy_preprocess_end(struct FFPsyPreprocessContext *ctx);

#endif /* AVCODEC_PSYMODEL_H */
