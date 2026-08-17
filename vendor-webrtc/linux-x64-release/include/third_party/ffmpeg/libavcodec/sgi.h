/*
 * SGI image encoder
 * Xiaohui Sun <tjnksxh@hotmail.com>
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

#ifndef AVCODEC_SGI_H
#define AVCODEC_SGI_H

/**
 * SGI image file signature
 */
#define SGI_MAGIC 474

#define SGI_HEADER_SIZE 512

#define SGI_GRAYSCALE 1
#define SGI_RGB 3
#define SGI_RGBA 4

#endif /* AVCODEC_SGI_H */
