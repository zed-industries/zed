/*
 * Generic DCT based hybrid video encoder
 * Copyright (c) 2000, 2001, 2002 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer
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
 * mpegvideo header.
 */

#ifndef AVCODEC_MPEGVIDEOENC_H
#define AVCODEC_MPEGVIDEOENC_H

#include <float.h>

#include "libavutil/opt.h"
#include "mpegvideo.h"

#define MAX_FCODE        7
#define UNI_AC_ENC_INDEX(run,level) ((run)*128 + (level))
#define INPLACE_OFFSET 16

/* MB types for encoding */
#define CANDIDATE_MB_TYPE_INTRA      (1 <<  0)
#define CANDIDATE_MB_TYPE_INTER      (1 <<  1)
#define CANDIDATE_MB_TYPE_INTER4V    (1 <<  2)
#define CANDIDATE_MB_TYPE_SKIPPED    (1 <<  3)

#define CANDIDATE_MB_TYPE_DIRECT     (1 <<  4)
#define CANDIDATE_MB_TYPE_FORWARD    (1 <<  5)
#define CANDIDATE_MB_TYPE_BACKWARD   (1 <<  6)
#define CANDIDATE_MB_TYPE_BIDIR      (1 <<  7)

#define CANDIDATE_MB_TYPE_INTER_I    (1 <<  8)
#define CANDIDATE_MB_TYPE_FORWARD_I  (1 <<  9)
#define CANDIDATE_MB_TYPE_BACKWARD_I (1 << 10)
#define CANDIDATE_MB_TYPE_BIDIR_I    (1 << 11)

#define CANDIDATE_MB_TYPE_DIRECT0    (1 << 12)

/* mpegvideo_enc common options */
#define FF_MPV_FLAG_SKIP_RD      0x0001
#define FF_MPV_FLAG_STRICT_GOP   0x0002
#define FF_MPV_FLAG_QP_RD        0x0004
#define FF_MPV_FLAG_CBP_RD       0x0008
#define FF_MPV_FLAG_NAQ          0x0010
#define FF_MPV_FLAG_MV0          0x0020

#define FF_MPV_OPT_CMP_FUNC \
{ "sad",    "Sum of absolute differences, fast", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_SAD }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "sse",    "Sum of squared errors", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_SSE }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "satd",   "Sum of absolute Hadamard transformed differences", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_SATD }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "dct",    "Sum of absolute DCT transformed differences", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_DCT }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "psnr",   "Sum of squared quantization errors, low quality", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_PSNR }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "bit",    "Number of bits needed for the block", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_BIT }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "rd",     "Rate distortion optimal, slow", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_RD }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "zero",   "Zero", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_ZERO }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "vsad",   "Sum of absolute vertical differences", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_VSAD }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "vsse",   "Sum of squared vertical differences", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_VSSE }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "nsse",   "Noise preserving sum of squared differences", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_NSSE }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "dct264", NULL, 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_DCT264 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "dctmax", NULL, 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_DCTMAX }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "chroma", NULL, 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_CHROMA }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{ "msad",   "Sum of absolute differences, median predicted", 0, AV_OPT_TYPE_CONST, {.i64 = FF_CMP_MEDIAN_SAD }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }

#define FF_MPV_OFFSET(x) offsetof(MpegEncContext, x)
#define FF_RC_OFFSET(x)  offsetof(MpegEncContext, rc_context.x)
#define FF_MPV_OPT_FLAGS (AV_OPT_FLAG_VIDEO_PARAM | AV_OPT_FLAG_ENCODING_PARAM)
#define FF_MPV_COMMON_OPTS \
FF_MPV_OPT_CMP_FUNC, \
{ "mpv_flags",      "Flags common for all mpegvideo-based encoders.", FF_MPV_OFFSET(mpv_flags), AV_OPT_TYPE_FLAGS, { .i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "skip_rd",        "RD optimal MB level residual skipping", 0, AV_OPT_TYPE_CONST, { .i64 = FF_MPV_FLAG_SKIP_RD },    0, 0, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "strict_gop",     "Strictly enforce gop size",             0, AV_OPT_TYPE_CONST, { .i64 = FF_MPV_FLAG_STRICT_GOP }, 0, 0, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "qp_rd",          "Use rate distortion optimization for qp selection", 0, AV_OPT_TYPE_CONST, { .i64 = FF_MPV_FLAG_QP_RD },  0, 0, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "cbp_rd",         "use rate distortion optimization for CBP",          0, AV_OPT_TYPE_CONST, { .i64 = FF_MPV_FLAG_CBP_RD }, 0, 0, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "naq",            "normalize adaptive quantization",                   0, AV_OPT_TYPE_CONST, { .i64 = FF_MPV_FLAG_NAQ },    0, 0, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "mv0",            "always try a mb with mv=<0,0>",                     0, AV_OPT_TYPE_CONST, { .i64 = FF_MPV_FLAG_MV0 },    0, 0, FF_MPV_OPT_FLAGS, .unit = "mpv_flags" },\
{ "luma_elim_threshold",   "single coefficient elimination threshold for luminance (negative values also consider dc coefficient)",\
                                                                      FF_MPV_OFFSET(luma_elim_threshold), AV_OPT_TYPE_INT, { .i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS },\
{ "chroma_elim_threshold", "single coefficient elimination threshold for chrominance (negative values also consider dc coefficient)",\
                                                                      FF_MPV_OFFSET(chroma_elim_threshold), AV_OPT_TYPE_INT, { .i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS },\
{ "quantizer_noise_shaping", NULL,                                  FF_MPV_OFFSET(quantizer_noise_shaping), AV_OPT_TYPE_INT, { .i64 = 0 },       0, INT_MAX, FF_MPV_OPT_FLAGS },\
{ "error_rate", "Simulate errors in the bitstream to test error concealment.",                                                                                                  \
                                                                    FF_MPV_OFFSET(error_rate),              AV_OPT_TYPE_INT, { .i64 = 0 },       0, INT_MAX, FF_MPV_OPT_FLAGS },\
{"qsquish", "how to keep quantizer between qmin and qmax (0 = clip, 1 = use differentiable function)",                                                                          \
                                                                    FF_RC_OFFSET(qsquish), AV_OPT_TYPE_FLOAT, {.dbl = 0 }, 0, 99, FF_MPV_OPT_FLAGS},                        \
{"rc_qmod_amp", "experimental quantizer modulation",                FF_RC_OFFSET(qmod_amp), AV_OPT_TYPE_FLOAT, {.dbl = 0 }, -FLT_MAX, FLT_MAX, FF_MPV_OPT_FLAGS},           \
{"rc_qmod_freq", "experimental quantizer modulation",               FF_RC_OFFSET(qmod_freq), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS},             \
{"rc_eq", "Set rate control equation. When computing the expression, besides the standard functions "                                                                           \
          "defined in the section 'Expression Evaluation', the following functions are available: "                                                                             \
          "bits2qp(bits), qp2bits(qp). Also the following constants are available: iTex pTex tex mv "                                                                           \
          "fCode iCount mcVar var isI isP isB avgQP qComp avgIITex avgPITex avgPPTex avgBPTex avgTex.",                                                                         \
                                                                    FF_RC_OFFSET(rc_eq), AV_OPT_TYPE_STRING,                           .flags = FF_MPV_OPT_FLAGS },            \
{"rc_init_cplx", "initial complexity for 1-pass encoding",          FF_RC_OFFSET(initial_cplx), AV_OPT_TYPE_FLOAT, {.dbl = 0 }, -FLT_MAX, FLT_MAX, FF_MPV_OPT_FLAGS},       \
{"rc_buf_aggressivity", "currently useless",                        FF_RC_OFFSET(buffer_aggressivity), AV_OPT_TYPE_FLOAT, {.dbl = 1.0 }, -FLT_MAX, FLT_MAX, FF_MPV_OPT_FLAGS}, \
{"border_mask", "increase the quantizer for macroblocks close to borders", FF_MPV_OFFSET(border_masking), AV_OPT_TYPE_FLOAT, {.dbl = 0 }, -FLT_MAX, FLT_MAX, FF_MPV_OPT_FLAGS},    \
{"lmin", "minimum Lagrange factor (VBR)",                           FF_MPV_OFFSET(lmin), AV_OPT_TYPE_INT, {.i64 =  2*FF_QP2LAMBDA }, 0, INT_MAX, FF_MPV_OPT_FLAGS },            \
{"lmax", "maximum Lagrange factor (VBR)",                           FF_MPV_OFFSET(lmax), AV_OPT_TYPE_INT, {.i64 = 31*FF_QP2LAMBDA }, 0, INT_MAX, FF_MPV_OPT_FLAGS },            \
{"skip_threshold", "Frame skip threshold",                          FF_MPV_OFFSET(frame_skip_threshold), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"skip_factor", "Frame skip factor",                                FF_MPV_OFFSET(frame_skip_factor), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"skip_exp", "Frame skip exponent",                                 FF_MPV_OFFSET(frame_skip_exp), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"skip_cmp", "Frame skip compare function",                         FF_MPV_OFFSET(frame_skip_cmp), AV_OPT_TYPE_INT, {.i64 = FF_CMP_DCTMAX }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS, .unit = "cmp_func" }, \
{"sc_threshold", "Scene change threshold",                          FF_MPV_OFFSET(scenechange_threshold), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"noise_reduction", "Noise reduction",                              FF_MPV_OFFSET(noise_reduction), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"ps", "RTP payload size in bytes",                             FF_MPV_OFFSET(rtp_payload_size), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \

#define FF_MPV_COMMON_BFRAME_OPTS \
{"b_strategy", "Strategy to choose between I/P/B-frames",      FF_MPV_OFFSET(b_frame_strategy), AV_OPT_TYPE_INT, {.i64 = 0 }, 0, 2, FF_MPV_OPT_FLAGS }, \
{"b_sensitivity", "Adjust sensitivity of b_frame_strategy 1",  FF_MPV_OFFSET(b_sensitivity), AV_OPT_TYPE_INT, {.i64 = 40 }, 1, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"brd_scale", "Downscale frames for dynamic B-frame decision", FF_MPV_OFFSET(brd_scale), AV_OPT_TYPE_INT, {.i64 = 0 }, 0, 3, FF_MPV_OPT_FLAGS },

#define FF_MPV_COMMON_MOTION_EST_OPTS \
{"motion_est", "motion estimation algorithm",                       FF_MPV_OFFSET(motion_est), AV_OPT_TYPE_INT, {.i64 = FF_ME_EPZS }, FF_ME_ZERO, FF_ME_XONE, FF_MPV_OPT_FLAGS, .unit = "motion_est" },   \
{ "zero", NULL, 0, AV_OPT_TYPE_CONST, { .i64 = FF_ME_ZERO }, 0, 0, FF_MPV_OPT_FLAGS, .unit = "motion_est" }, \
{ "epzs", NULL, 0, AV_OPT_TYPE_CONST, { .i64 = FF_ME_EPZS }, 0, 0, FF_MPV_OPT_FLAGS, .unit = "motion_est" }, \
{ "xone", NULL, 0, AV_OPT_TYPE_CONST, { .i64 = FF_ME_XONE }, 0, 0, FF_MPV_OPT_FLAGS, .unit = "motion_est" }, \
{"mepc", "Motion estimation bitrate penalty compensation (1.0 = 256)", FF_MPV_OFFSET(me_penalty_compensation), AV_OPT_TYPE_INT, {.i64 = 256 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"mepre", "pre motion estimation", FF_MPV_OFFSET(me_pre), AV_OPT_TYPE_INT, {.i64 = 0 }, INT_MIN, INT_MAX, FF_MPV_OPT_FLAGS }, \
{"intra_penalty", "Penalty for intra blocks in block decision", FF_MPV_OFFSET(intra_penalty), AV_OPT_TYPE_INT, {.i64 = 0 }, 0, INT_MAX/2, FF_MPV_OPT_FLAGS }, \

extern const AVClass ff_mpv_enc_class;

int ff_mpv_encode_init(AVCodecContext *avctx);
void ff_mpv_encode_init_x86(MpegEncContext *s);

int ff_mpv_encode_end(AVCodecContext *avctx);
int ff_mpv_encode_picture(AVCodecContext *avctx, AVPacket *pkt,
                          const AVFrame *frame, int *got_packet);
int ff_mpv_reallocate_putbitbuffer(MpegEncContext *s, size_t threshold, size_t size_increase);

void ff_write_quant_matrix(PutBitContext *pb, uint16_t *matrix);

void ff_dct_encode_init(MpegEncContext *s);
void ff_mpvenc_dct_init_mips(MpegEncContext *s);
void ff_dct_encode_init_x86(MpegEncContext *s);

void ff_convert_matrix(MpegEncContext *s, int (*qmat)[64], uint16_t (*qmat16)[2][64],
                       const uint16_t *quant_matrix, int bias, int qmin, int qmax, int intra);

void ff_block_permute(int16_t *block, const uint8_t *permutation,
                      const uint8_t *scantable, int last);

static inline int get_bits_diff(MpegEncContext *s)
{
    const int bits = put_bits_count(&s->pb);
    const int last = s->last_bits;

    s->last_bits = bits;

    return bits - last;
}

#endif
