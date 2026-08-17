/*
 * Binary text decoder
 * Copyright (c) 2010 Peter Ross (pross@xvid.org)
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
 * Binary text decoder
 */

#ifndef AVCODEC_BINTEXT_H
#define AVCODEC_BINTEXT_H

/* flag values passed between avformat and avcodec;
 * while these are identical to the XBIN flags, they are also used
 * for the BINTEXT and IDF decoders.
 */
#define BINTEXT_PALETTE  0x1
#define BINTEXT_FONT     0x2

#endif /* AVCODEC_BINTEXT_H */
