/*
 * Copyright (C) 2001-2003 Michael Niedermayer (michaelni@gmx.at)
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef POSTPROC_POSTPROCESS_H
#define POSTPROC_POSTPROCESS_H

/**
 * @file
 * @ingroup lpp
 * external API header
 */

/**
 * @defgroup lpp libpostproc
 * Video postprocessing library.
 *
 * @{
 */

#include "libpostproc/version_major.h"
#ifndef HAVE_AV_CONFIG_H
/* When included as part of the ffmpeg build, only include the major version
 * to avoid unnecessary rebuilds. When included externally, keep including
 * the full version information. */
#include "libpostproc/version.h"
#endif

/**
 * Return the LIBPOSTPROC_VERSION_INT constant.
 */
unsigned postproc_version(void);

/**
 * Return the libpostproc build-time configuration.
 */
const char *postproc_configuration(void);

/**
 * Return the libpostproc license.
 */
const char *postproc_license(void);

#define PP_QUALITY_MAX 6

#include <inttypes.h>

typedef void pp_context;
typedef void pp_mode;

extern const char pp_help[]; ///< a simple help text

void  pp_postprocess(const uint8_t * src[3], const int srcStride[3],
                     uint8_t * dst[3], const int dstStride[3],
                     int horizontalSize, int verticalSize,
                     const int8_t *QP_store,  int QP_stride,
                     pp_mode *mode, pp_context *ppContext, int pict_type);


/**
 * Return a pp_mode or NULL if an error occurred.
 *
 * @param name    the string after "-pp" on the command line
 * @param quality a number from 0 to PP_QUALITY_MAX
 */
pp_mode *pp_get_mode_by_name_and_quality(const char *name, int quality);
void pp_free_mode(pp_mode *mode);

pp_context *pp_get_context(int width, int height, int flags);
void pp_free_context(pp_context *ppContext);

#define PP_CPU_CAPS_MMX   0x80000000
#define PP_CPU_CAPS_MMX2  0x20000000
#define PP_CPU_CAPS_3DNOW 0x40000000
#define PP_CPU_CAPS_ALTIVEC 0x10000000
#define PP_CPU_CAPS_AUTO  0x00080000

#define PP_FORMAT         0x00000008
#define PP_FORMAT_420    (0x00000011|PP_FORMAT)
#define PP_FORMAT_422    (0x00000001|PP_FORMAT)
#define PP_FORMAT_411    (0x00000002|PP_FORMAT)
#define PP_FORMAT_444    (0x00000000|PP_FORMAT)
#define PP_FORMAT_440    (0x00000010|PP_FORMAT)

#define PP_PICT_TYPE_QP2  0x00000010 ///< MPEG2 style QScale

/**
 * @}
 */

#endif /* POSTPROC_POSTPROCESS_H */
