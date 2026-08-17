/*
 * JPEG 2000 image decoder
 * Copyright (c) 2007 Kamil Nowosad
 * Copyright (c) 2013 Nicolas Bertrand <nicoinattendu@gmail.com>
 * Copyright (c) 2022 Caleb Etemesi <etemesicaleb@gmail.com>
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

#ifndef AVCODEC_JPEG2000DEC_H
#define AVCODEC_JPEG2000DEC_H

#include "bytestream.h"
#include "jpeg2000.h"
#include "jpeg2000dsp.h"


#define MAX_POCS 32

typedef struct Jpeg2000POCEntry {
    uint16_t LYEpoc;
    uint16_t CSpoc;
    uint16_t CEpoc;
    uint8_t RSpoc;
    uint8_t REpoc;
    uint8_t Ppoc;
} Jpeg2000POCEntry;

typedef struct Jpeg2000POC {
    Jpeg2000POCEntry poc[MAX_POCS];
    int nb_poc;
    int is_default;
} Jpeg2000POC;

typedef struct Jpeg2000TilePart {
    uint8_t tile_index;                 // Tile index who refers the tile-part
    const uint8_t *tp_end;
    GetByteContext header_tpg;          // bit stream of header if PPM header is used
    GetByteContext tpg;                 // bit stream in tile-part
} Jpeg2000TilePart;

/* RMK: For JPEG2000 DCINEMA 3 tile-parts in a tile
 * one per component, so tile_part elements have a size of 3 */
typedef struct Jpeg2000Tile {
    Jpeg2000Component   *comp;
    uint8_t             properties[4];
    Jpeg2000CodingStyle codsty[4];
    Jpeg2000QuantStyle  qntsty[4];
    Jpeg2000POC         poc;
    Jpeg2000TilePart    tile_part[32];
    uint8_t             has_ppt;                // whether this tile has a ppt marker
    uint8_t             *packed_headers;        // contains packed headers. Used only along with PPT marker
    int                 packed_headers_size;    // size in bytes of the packed headers
    GetByteContext      packed_headers_stream;  // byte context corresponding to packed headers
    uint16_t tp_idx;                    // Tile-part index
    int coord[2][2];                    // border coordinates {{x0, x1}, {y0, y1}}
} Jpeg2000Tile;

typedef struct Jpeg2000DecoderContext {
    AVClass         *class;
    AVCodecContext  *avctx;
    GetByteContext  g;

    int             width, height;
    int             image_offset_x, image_offset_y;
    int             tile_offset_x, tile_offset_y;
    uint8_t         cbps[4];    // bits per sample in particular components
    uint8_t         sgnd[4];    // if a component is signed
    uint8_t         properties[4];

    uint8_t         has_ppm;
    uint8_t         *packed_headers; // contains packed headers. Used only along with PPM marker
    int             packed_headers_size;
    GetByteContext  packed_headers_stream;

    int             cdx[4], cdy[4];
    int             precision;
    int             ncomponents;
    int             colour_space;
    uint32_t        palette[256];
    int8_t          pal8;
    int             cdef[4];
    int             tile_width, tile_height;
    unsigned        numXtiles, numYtiles;
    int             maxtilelen;
    AVRational      sar;

    Jpeg2000CodingStyle codsty[4];
    Jpeg2000QuantStyle  qntsty[4];
    Jpeg2000POC         poc;
    uint8_t             roi_shift[4];

    int             bit_index;

    int             curtileno;

    Jpeg2000Tile    *tile;
    Jpeg2000DSPContext dsp;

    uint8_t         isHT; // HTJ2K?
    uint8_t         Ccap15_b14_15; // HTONLY(= 0) or HTDECLARED(= 1) or MIXED(= 3) ?
    uint8_t         Ccap15_b12; // RGNFREE(= 0) or RGN(= 1)?
    uint8_t         Ccap15_b11; // HOMOGENEOUS(= 0) or HETEROGENEOUS(= 1) ?
    uint8_t         Ccap15_b05; // HTREV(= 0) or HTIRV(= 1) ?
    uint8_t         HT_B; // The parameter B for MAGBp value (see Table 4 in the Rec. ITU-T T.814 | ISO/IEC 15444-15)

    /*options parameters*/
    int             reduction_factor;
} Jpeg2000DecoderContext;

#endif //AVCODEC_JPEG2000DEC_H
