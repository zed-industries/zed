/*
 * Copyright (c) 2013 Paul B Mahol
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

#ifndef AVFILTER_BLEND_INIT_H
#define AVFILTER_BLEND_INIT_H

#include "config.h"
#include "libavutil/attributes.h"
#include "libavutil/imgutils.h"
#include "blend.h"

#define DEPTH 8
#include "blend_modes.c"

#undef DEPTH
#define DEPTH 9
#include "blend_modes.c"

#undef DEPTH
#define DEPTH 10
#include "blend_modes.c"

#undef DEPTH
#define DEPTH 12
#include "blend_modes.c"

#undef DEPTH
#define DEPTH 14
#include "blend_modes.c"

#undef DEPTH
#define DEPTH 16
#include "blend_modes.c"

#undef DEPTH
#define DEPTH 32
#include "blend_modes.c"

#define COPY(src, depth)                                                            \
static void blend_copy ## src##_##depth(const uint8_t *top, ptrdiff_t top_linesize,    \
                            const uint8_t *bottom, ptrdiff_t bottom_linesize,\
                            uint8_t *dst, ptrdiff_t dst_linesize,            \
                            ptrdiff_t width, ptrdiff_t height,               \
                            FilterParams *param, SliceParams *sliceparam)    \
{                                                                            \
    av_image_copy_plane(dst, dst_linesize, src, src ## _linesize,            \
                        width * depth / 8, height);                          \
}

COPY(top, 8)
COPY(bottom, 8)

COPY(top, 16)
COPY(bottom, 16)

COPY(top, 32)
COPY(bottom, 32)

#undef COPY

#define BLEND_NORMAL(name, type)                                                  \
static void blend_normal_##name(const uint8_t *_top, ptrdiff_t top_linesize,      \
                                const uint8_t *_bottom, ptrdiff_t bottom_linesize,\
                                uint8_t *_dst, ptrdiff_t dst_linesize,            \
                                ptrdiff_t width, ptrdiff_t height,                \
                                FilterParams *param, SliceParams *sliceparam)     \
{                                                                                 \
    const type *top = (const type*)_top;                                          \
    const type *bottom = (const type*)_bottom;                                    \
    type *dst = (type*)_dst;                                                      \
    const float opacity = param->opacity;                                         \
                                                                                  \
    dst_linesize /= sizeof(type);                                                 \
    top_linesize /= sizeof(type);                                                 \
    bottom_linesize /= sizeof(type);                                              \
                                                                                  \
    for (int i = 0; i < height; i++) {                                            \
        for (int j = 0; j < width; j++) {                                         \
            dst[j] = top[j] * opacity + bottom[j] * (1.f - opacity);              \
        }                                                                         \
        dst    += dst_linesize;                                                   \
        top    += top_linesize;                                                   \
        bottom += bottom_linesize;                                                \
    }                                                                             \
}

BLEND_NORMAL(8bit,  uint8_t)
BLEND_NORMAL(16bit, uint16_t)
BLEND_NORMAL(32bit, float)

#define DEFINE_INIT_BLEND_FUNC(depth, nbits)                                          \
static av_cold void init_blend_func_##depth##_##nbits##bit(FilterParams *param)       \
{                                                                                     \
    switch (param->mode) {                                                            \
    case BLEND_ADDITION:     param->blend = blend_addition_##depth##bit;     break;   \
    case BLEND_GRAINMERGE:   param->blend = blend_grainmerge_##depth##bit;   break;   \
    case BLEND_AND:          param->blend = blend_and_##depth##bit;          break;   \
    case BLEND_AVERAGE:      param->blend = blend_average_##depth##bit;      break;   \
    case BLEND_BURN:         param->blend = blend_burn_##depth##bit;         break;   \
    case BLEND_DARKEN:       param->blend = blend_darken_##depth##bit;       break;   \
    case BLEND_DIFFERENCE:   param->blend = blend_difference_##depth##bit;   break;   \
    case BLEND_GRAINEXTRACT: param->blend = blend_grainextract_##depth##bit; break;   \
    case BLEND_DIVIDE:       param->blend = blend_divide_##depth##bit;       break;   \
    case BLEND_DODGE:        param->blend = blend_dodge_##depth##bit;        break;   \
    case BLEND_EXCLUSION:    param->blend = blend_exclusion_##depth##bit;    break;   \
    case BLEND_EXTREMITY:    param->blend = blend_extremity_##depth##bit;    break;   \
    case BLEND_FREEZE:       param->blend = blend_freeze_##depth##bit;       break;   \
    case BLEND_GLOW:         param->blend = blend_glow_##depth##bit;         break;   \
    case BLEND_HARDLIGHT:    param->blend = blend_hardlight_##depth##bit;    break;   \
    case BLEND_HARDMIX:      param->blend = blend_hardmix_##depth##bit;      break;   \
    case BLEND_HEAT:         param->blend = blend_heat_##depth##bit;         break;   \
    case BLEND_LIGHTEN:      param->blend = blend_lighten_##depth##bit;      break;   \
    case BLEND_LINEARLIGHT:  param->blend = blend_linearlight_##depth##bit;  break;   \
    case BLEND_MULTIPLY:     param->blend = blend_multiply_##depth##bit;     break;   \
    case BLEND_MULTIPLY128:  param->blend = blend_multiply128_##depth##bit;  break;   \
    case BLEND_NEGATION:     param->blend = blend_negation_##depth##bit;     break;   \
    case BLEND_NORMAL:       param->blend = blend_normal_##nbits##bit;       break;   \
    case BLEND_OR:           param->blend = blend_or_##depth##bit;           break;   \
    case BLEND_OVERLAY:      param->blend = blend_overlay_##depth##bit;      break;   \
    case BLEND_PHOENIX:      param->blend = blend_phoenix_##depth##bit;      break;   \
    case BLEND_PINLIGHT:     param->blend = blend_pinlight_##depth##bit;     break;   \
    case BLEND_REFLECT:      param->blend = blend_reflect_##depth##bit;      break;   \
    case BLEND_SCREEN:       param->blend = blend_screen_##depth##bit;       break;   \
    case BLEND_SOFTLIGHT:    param->blend = blend_softlight_##depth##bit;    break;   \
    case BLEND_SUBTRACT:     param->blend = blend_subtract_##depth##bit;     break;   \
    case BLEND_VIVIDLIGHT:   param->blend = blend_vividlight_##depth##bit;   break;   \
    case BLEND_XOR:          param->blend = blend_xor_##depth##bit;          break;   \
    case BLEND_SOFTDIFFERENCE:param->blend=blend_softdifference_##depth##bit;break;   \
    case BLEND_GEOMETRIC:    param->blend = blend_geometric_##depth##bit;    break;   \
    case BLEND_HARMONIC:     param->blend = blend_harmonic_##depth##bit;     break;   \
    case BLEND_BLEACH:       param->blend = blend_bleach_##depth##bit;       break;   \
    case BLEND_STAIN:        param->blend = blend_stain_##depth##bit;        break;   \
    case BLEND_INTERPOLATE:  param->blend = blend_interpolate_##depth##bit;  break;   \
    case BLEND_HARDOVERLAY:  param->blend = blend_hardoverlay_##depth##bit;  break;   \
    }                                                                                 \
}
DEFINE_INIT_BLEND_FUNC(8, 8)
DEFINE_INIT_BLEND_FUNC(9, 16)
DEFINE_INIT_BLEND_FUNC(10, 16)
DEFINE_INIT_BLEND_FUNC(12, 16)
DEFINE_INIT_BLEND_FUNC(14, 16)
DEFINE_INIT_BLEND_FUNC(16, 16)
DEFINE_INIT_BLEND_FUNC(32, 32)

static av_unused void ff_blend_init(FilterParams *param, int depth)
{
    switch (depth) {
    case 8:
        init_blend_func_8_8bit(param);
        break;
    case 9:
        init_blend_func_9_16bit(param);
        break;
    case 10:
        init_blend_func_10_16bit(param);
        break;
    case 12:
        init_blend_func_12_16bit(param);
        break;
    case 14:
        init_blend_func_14_16bit(param);
        break;
    case 16:
        init_blend_func_16_16bit(param);
        break;
    case 32:
        init_blend_func_32_32bit(param);
        break;
    }

    if (param->opacity == 0 && param->mode != BLEND_NORMAL) {
        param->blend = depth > 8 ? depth > 16 ? blend_copytop_32 : blend_copytop_16 : blend_copytop_8;
    } else if (param->mode == BLEND_NORMAL) {
        if (param->opacity == 1)
            param->blend = depth > 8 ? depth > 16 ? blend_copytop_32 : blend_copytop_16 : blend_copytop_8;
        else if (param->opacity == 0)
            param->blend = depth > 8 ? depth > 16 ? blend_copybottom_32 : blend_copybottom_16 : blend_copybottom_8;
    }

#if ARCH_X86
    ff_blend_init_x86(param, depth);
#endif
}

#endif /* AVFILTER_BLEND_INIT_H */
