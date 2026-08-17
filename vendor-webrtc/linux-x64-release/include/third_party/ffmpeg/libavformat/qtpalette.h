/*
 * Default Palettes for Quicktime Files
 *  Automatically generated from a utility derived from XAnim:
 *  http://xanim.va.pubnix.com/home.html
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

#ifndef AVFORMAT_QTPALETTE_H
#define AVFORMAT_QTPALETTE_H

#include <stdint.h>
#include "avio.h"

/**
 * Retrieve the palette (or "color table" in QuickTime terms), either
 * from the video sample description, or from the default Macintosh
 * palette.
 *
 * The file offset of the AVIOContext pointed to by the 'pb' variable
 * should be the start of the video sample description (the sample
 * description size and the data format).
 */
int ff_get_qtpalette(int codec_id, AVIOContext *pb, uint32_t *palette);

#endif /* AVFORMAT_QTPALETTE_H */
