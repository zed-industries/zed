/*
 * FFV1 codec for libavcodec
 *
 * Copyright (c) 2003-2012 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_FFV1_H
#define AVCODEC_FFV1_H

/**
 * @file
 * FF Video Codec 1 (a lossless codec)
 */

#include "libavutil/attributes.h"
#include "avcodec.h"
#include "get_bits.h"
#include "mathops.h"
#include "progressframe.h"
#include "put_bits.h"
#include "rangecoder.h"

#ifdef __INTEL_COMPILER
#undef av_flatten
#define av_flatten
#endif

#define MAX_PLANES 4
#define CONTEXT_SIZE 32

#define MAX_QUANT_TABLES 8
#define MAX_QUANT_TABLE_SIZE 256
#define MAX_QUANT_TABLE_MASK (MAX_QUANT_TABLE_SIZE - 1)
#define MAX_CONTEXT_INPUTS 5

#define AC_GOLOMB_RICE          0
#define AC_RANGE_DEFAULT_TAB    1
#define AC_RANGE_CUSTOM_TAB     2
#define AC_RANGE_DEFAULT_TAB_FORCE -2

typedef struct VlcState {
    uint32_t error_sum;
    int16_t drift;
    int8_t bias;
    uint8_t count;
} VlcState;

typedef struct PlaneContext {
    int quant_table_index;
    int context_count;
    uint8_t (*state)[CONTEXT_SIZE];
    VlcState *vlc_state;
} PlaneContext;

#define MAX_SLICES 1024

typedef struct FFV1SliceContext {
    int16_t *sample_buffer;
    int32_t *sample_buffer32;

    int slice_width;
    int slice_height;
    int slice_x;
    int slice_y;
    int sx, sy;

    int run_index;
    int slice_coding_mode;
    int slice_rct_by_coef;
    int slice_rct_ry_coef;
    int remap;

    // RefStruct reference, array of MAX_PLANES elements
    PlaneContext *plane;
    PutBitContext pb;
    RangeCoder c;

    int ac_byte_count;                   ///< number of bytes used for AC coding

    union {
        // decoder-only
        struct {
            int slice_reset_contexts;
            int slice_damaged;
        };

        // encoder-only
        struct {
            uint64_t rc_stat[256][2];
            uint64_t (*rc_stat2[MAX_QUANT_TABLES])[32][2];
        };
    };
    uint16_t   fltmap[4][65536];
} FFV1SliceContext;

typedef struct FFV1Context {
    AVClass *class;
    AVCodecContext *avctx;
    uint64_t rc_stat[256][2];
    uint64_t (*rc_stat2[MAX_QUANT_TABLES])[32][2];
    int version;
    int micro_version;
    int combined_version;
    int width, height;
    int chroma_planes;
    int chroma_h_shift, chroma_v_shift;
    int transparency;
    int flags;
    int64_t picture_number;
    int key_frame;
    ProgressFrame picture, last_picture;
    void *hwaccel_picture_private, *hwaccel_last_picture_private;
    uint32_t crcref;
    enum AVPixelFormat pix_fmt;
    enum AVPixelFormat configured_pix_fmt;

    const AVFrame *cur_enc_frame;
    int plane_count;
    int ac;                              ///< 1=range coder <-> 0=golomb rice
    int16_t quant_tables[MAX_QUANT_TABLES][MAX_CONTEXT_INPUTS][MAX_QUANT_TABLE_SIZE];
    int context_count[MAX_QUANT_TABLES];
    uint8_t state_transition[256];
    uint8_t (*initial_states[MAX_QUANT_TABLES])[32];
    int colorspace;
    int flt;
    int remap_mode;


    int use32bit;

    int ec;
    int intra;
    int key_frame_ok;
    int context_model;
    int qtable;

    int bits_per_raw_sample;
    int packed_at_lsb;

    int gob_count;
    int quant_table_count;

    int slice_count;
    int max_slice_count;
    int num_v_slices;
    int num_h_slices;

    FFV1SliceContext *slices;
    /* RefStruct object, per-slice damage flags shared between frame threads.
     *
     * After a frame thread marks some slice as finished with
     * ff_progress_frame_report(), the corresponding array element must not be
     * accessed by this thread anymore, as from then on it is owned by the next
     * thread.
     */
    uint8_t          *slice_damaged;
    /* Frame damage flag, used to delay announcing progress, since ER is
     * applied after all the slices are decoded.
     * NOT shared between frame threads.
     */
    uint8_t           frame_damaged;
} FFV1Context;

int ff_ffv1_common_init(AVCodecContext *avctx, FFV1Context *s);
int ff_ffv1_init_slice_state(const FFV1Context *f, FFV1SliceContext *sc);
int ff_ffv1_init_slices_state(FFV1Context *f);
int ff_ffv1_init_slice_contexts(FFV1Context *f);
PlaneContext *ff_ffv1_planes_alloc(void);
int ff_ffv1_allocate_initial_states(FFV1Context *f);
void ff_ffv1_clear_slice_state(const FFV1Context *f, FFV1SliceContext *sc);
void ff_ffv1_close(FFV1Context *s);
int ff_need_new_slices(int width, int num_h_slices, int chroma_shift);
int ff_ffv1_parse_header(FFV1Context *f, RangeCoder *c, uint8_t *state);
int ff_ffv1_read_extra_header(FFV1Context *f);
int ff_ffv1_read_quant_tables(RangeCoder *c,
                              int16_t quant_table[MAX_CONTEXT_INPUTS][256]);
int ff_ffv1_get_symbol(RangeCoder *c, uint8_t *state, int is_signed);

/**
 * This is intended for both width and height
 */
int ff_slice_coord(const FFV1Context *f, int width, int sx, int num_h_slices, int chroma_shift);

static av_always_inline int fold(int diff, int bits)
{
    if (bits == 8)
        diff = (int8_t)diff;
    else {
        diff = sign_extend(diff, bits);
    }

    return diff;
}

static inline void update_vlc_state(VlcState *const state, const int v)
{
    int drift = state->drift;
    int count = state->count;
    state->error_sum += FFABS(v);
    drift            += v;

    if (count == 128) { // FIXME: variable
        count            >>= 1;
        drift            >>= 1;
        state->error_sum >>= 1;
    }
    count++;

    if (drift <= -count) {
        state->bias = FFMAX(state->bias - 1, -128);

        drift = FFMAX(drift + count, -count + 1);
    } else if (drift > 0) {
        state->bias = FFMIN(state->bias + 1, 127);

        drift = FFMIN(drift - count, 0);
    }

    state->drift = drift;
    state->count = count;
}


static inline av_flatten int get_symbol_inline(RangeCoder *c, uint8_t *state,
                                               int is_signed)
{
    if (get_rac(c, state + 0))
        return 0;
    else {
        int e;
        unsigned a;
        e = 0;
        while (get_rac(c, state + 1 + FFMIN(e, 9))) { // 1..10
            e++;
            if (e > 31)
                return AVERROR_INVALIDDATA;
        }

        a = 1;
        for (int i = e - 1; i >= 0; i--)
            a += a + get_rac(c, state + 22 + FFMIN(i, 9));  // 22..31

        e = -(is_signed && get_rac(c, state + 11 + FFMIN(e, 10))); // 11..21
        return (a ^ e) - e;
    }
}

#endif /* AVCODEC_FFV1_H */
