/*
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

#ifndef AVCODEC_PROFILES_H
#define AVCODEC_PROFILES_H

#include "codec.h"
#include "defs.h"
#include "libavutil/opt.h"

#define FF_AVCTX_PROFILE_OPTION(name, description, type, value) \
    {name, description, 0, AV_OPT_TYPE_CONST, {.i64 = value }, INT_MIN, INT_MAX, AV_OPT_FLAG_ENCODING_PARAM | AV_OPT_FLAG_## type ##_PARAM, .unit = "avctx.profile"},

#define FF_AAC_PROFILE_OPTS \
    FF_AVCTX_PROFILE_OPTION("aac_main",      NULL, AUDIO, AV_PROFILE_AAC_MAIN)\
    FF_AVCTX_PROFILE_OPTION("aac_low",       NULL, AUDIO, AV_PROFILE_AAC_LOW)\
    FF_AVCTX_PROFILE_OPTION("aac_ssr",       NULL, AUDIO, AV_PROFILE_AAC_SSR)\
    FF_AVCTX_PROFILE_OPTION("aac_ltp",       NULL, AUDIO, AV_PROFILE_AAC_LTP)\
    FF_AVCTX_PROFILE_OPTION("aac_he",        NULL, AUDIO, AV_PROFILE_AAC_HE)\
    FF_AVCTX_PROFILE_OPTION("aac_he_v2",     NULL, AUDIO, AV_PROFILE_AAC_HE_V2)\
    FF_AVCTX_PROFILE_OPTION("aac_ld",        NULL, AUDIO, AV_PROFILE_AAC_LD)\
    FF_AVCTX_PROFILE_OPTION("aac_eld",       NULL, AUDIO, AV_PROFILE_AAC_ELD)\
    FF_AVCTX_PROFILE_OPTION("aac_xhe",       NULL, AUDIO, AV_PROFILE_AAC_USAC)\
    FF_AVCTX_PROFILE_OPTION("mpeg2_aac_low", NULL, AUDIO, AV_PROFILE_MPEG2_AAC_LOW)\
    FF_AVCTX_PROFILE_OPTION("mpeg2_aac_he",  NULL, AUDIO, AV_PROFILE_MPEG2_AAC_HE)\

#define FF_MPEG4_PROFILE_OPTS \
    FF_AVCTX_PROFILE_OPTION("mpeg4_sp",      NULL, VIDEO, AV_PROFILE_MPEG4_SIMPLE)\
    FF_AVCTX_PROFILE_OPTION("mpeg4_core",    NULL, VIDEO, AV_PROFILE_MPEG4_CORE)\
    FF_AVCTX_PROFILE_OPTION("mpeg4_main",    NULL, VIDEO, AV_PROFILE_MPEG4_MAIN)\
    FF_AVCTX_PROFILE_OPTION("mpeg4_asp",     NULL, VIDEO, AV_PROFILE_MPEG4_ADVANCED_SIMPLE)\

#define FF_MPEG2_PROFILE_OPTS \
    FF_AVCTX_PROFILE_OPTION("422",           NULL, VIDEO, AV_PROFILE_MPEG2_422)\
    FF_AVCTX_PROFILE_OPTION("high",          NULL, VIDEO, AV_PROFILE_MPEG2_HIGH)\
    FF_AVCTX_PROFILE_OPTION("ss",            NULL, VIDEO, AV_PROFILE_MPEG2_SS)\
    FF_AVCTX_PROFILE_OPTION("snr",           NULL, VIDEO, AV_PROFILE_MPEG2_SNR_SCALABLE)\
    FF_AVCTX_PROFILE_OPTION("main",          NULL, VIDEO, AV_PROFILE_MPEG2_MAIN)\
    FF_AVCTX_PROFILE_OPTION("simple",        NULL, VIDEO, AV_PROFILE_MPEG2_SIMPLE)\

#define FF_AV1_PROFILE_OPTS \
    FF_AVCTX_PROFILE_OPTION("main",          NULL, VIDEO, AV_PROFILE_AV1_MAIN)\
    FF_AVCTX_PROFILE_OPTION("high",          NULL, VIDEO, AV_PROFILE_AV1_HIGH)\
    FF_AVCTX_PROFILE_OPTION("professional",  NULL, VIDEO, AV_PROFILE_AV1_PROFESSIONAL)\

extern const AVProfile ff_aac_profiles[];
extern const AVProfile ff_dca_profiles[];
extern const AVProfile ff_eac3_profiles[];
extern const AVProfile ff_truehd_profiles[];
extern const AVProfile ff_dnxhd_profiles[];
extern const AVProfile ff_h264_profiles[];
extern const AVProfile ff_hevc_profiles[];
extern const AVProfile ff_vvc_profiles[];
extern const AVProfile ff_jpeg2000_profiles[];
extern const AVProfile ff_mpeg2_video_profiles[];
extern const AVProfile ff_mpeg4_video_profiles[];
extern const AVProfile ff_vc1_profiles[];
extern const AVProfile ff_vp9_profiles[];
extern const AVProfile ff_av1_profiles[];
extern const AVProfile ff_sbc_profiles[];
extern const AVProfile ff_prores_profiles[];
extern const AVProfile ff_mjpeg_profiles[];
extern const AVProfile ff_arib_caption_profiles[];
extern const AVProfile ff_evc_profiles[];

#endif /* AVCODEC_PROFILES_H */
