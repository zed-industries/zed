/*
 * CGA/EGA/VGA ROM font data
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
 * CGA/EGA/VGA ROM font data
 */

#ifndef AVUTIL_XGA_FONT_DATA_H
#define AVUTIL_XGA_FONT_DATA_H

#include <stdint.h>
#include "internal.h"

extern av_export_avutil const uint8_t avpriv_cga_font[2048];
extern av_export_avutil const uint8_t avpriv_vga16_font[4096];

#endif /* AVUTIL_XGA_FONT_DATA_H */
