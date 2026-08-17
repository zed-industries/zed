/*
 * GIF format definitions
 * Copyright (c) 2003 Fabrice Bellard
 * Copyright (c) 2006 Baptiste Coudurier
 * Copyright (c) 2012 Vitaliy E Sugrobov
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
 * GIF format definitions.
 */

#ifndef AVCODEC_GIF_H
#define AVCODEC_GIF_H

#include <stdint.h>

static const uint8_t gif87a_sig[6] = "GIF87a";
static const uint8_t gif89a_sig[6] = "GIF89a";

#define GCE_DISPOSAL_NONE       0
#define GCE_DISPOSAL_INPLACE    1
#define GCE_DISPOSAL_BACKGROUND 2
#define GCE_DISPOSAL_RESTORE    3

#define GIF_TRAILER                 0x3b
#define GIF_EXTENSION_INTRODUCER    0x21
#define GIF_IMAGE_SEPARATOR         0x2c
#define GIF_GCE_EXT_LABEL           0xf9
#define GIF_COM_EXT_LABEL           0xfe
#define GIF_APP_EXT_LABEL           0xff
#define NETSCAPE_EXT_STR            "NETSCAPE2.0"

#endif /* AVCODEC_GIF_H */
