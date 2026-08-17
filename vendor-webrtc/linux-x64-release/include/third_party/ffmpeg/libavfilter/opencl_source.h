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

#ifndef AVFILTER_OPENCL_SOURCE_H
#define AVFILTER_OPENCL_SOURCE_H

extern const char *ff_source_avgblur_cl;
extern const char *ff_source_colorkey_cl;
extern const char *ff_source_colorspace_common_cl;
extern const char *ff_source_convolution_cl;
extern const char *ff_source_deshake_cl;
extern const char *ff_source_neighbor_cl;
extern const char *ff_source_nlmeans_cl;
extern const char *ff_source_overlay_cl;
extern const char *ff_source_pad_cl;
extern const char *ff_source_remap_cl;
extern const char *ff_source_tonemap_cl;
extern const char *ff_source_transpose_cl;
extern const char *ff_source_unsharp_cl;
extern const char *ff_source_xfade_cl;

#endif /* AVFILTER_OPENCL_SOURCE_H */
