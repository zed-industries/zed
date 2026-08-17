/*
 * Flash Compatible Streaming Format common header.
 * Copyright (c) 2000 Fabrice Bellard
 * Copyright (c) 2003 Tinic Uro
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

#ifndef AVFORMAT_SWF_H
#define AVFORMAT_SWF_H

#include "internal.h"

/* should have a generic way to indicate probable size */
#define DUMMY_FILE_SIZE   (100 * 1024 * 1024)
#define DUMMY_DURATION    600 /* in seconds */

enum {
    TAG_END                          =  0,
    TAG_SHOWFRAME                    =  1,
    TAG_DEFINESHAPE                  =  2,
    TAG_FREECHARACTER                =  3,
    TAG_PLACEOBJECT                  =  4,
    TAG_REMOVEOBJECT                 =  5,
    TAG_DEFINEBITS                   =  6,
    TAG_DEFINEBUTTON                 =  7,
    TAG_JPEGTABLES                   =  8,
    TAG_SETBACKGROUNDCOLOR           =  9,
    TAG_DEFINEFONT                   = 10,
    TAG_DEFINETEXT                   = 11,
    TAG_DOACTION                     = 12,
    TAG_DEFINEFONTINFO               = 13,
    TAG_DEFINESOUND                  = 14,
    TAG_STARTSOUND                   = 15,
    TAG_DEFINEBUTTONSOUND            = 17,
    TAG_STREAMHEAD                   = 18,
    TAG_STREAMBLOCK                  = 19,
    TAG_DEFINEBITSLOSSLESS           = 20,
    TAG_JPEG2                        = 21,
    TAG_DEFINESHAPE2                 = 22,
    TAG_DEFINEBUTTONCXFORM           = 23,
    TAG_PROTECT                      = 24,
    TAG_PLACEOBJECT2                 = 26,
    TAG_REMOVEOBJECT2                = 28,
    TAG_DEFINESHAPE3                 = 32,
    TAG_DEFINETEXT2                  = 33,
    TAG_DEFINEBUTTON2                = 34,
    TAG_DEFINEBITSJPEG3              = 35,
    TAG_DEFINEBITSLOSSLESS2          = 36,
    TAG_DEFINEEDITTEXT               = 37,
    TAG_DEFINESPRITE                 = 39,
    TAG_FRAMELABEL                   = 43,
    TAG_STREAMHEAD2                  = 45,
    TAG_DEFINEMORPHSHAPE             = 46,
    TAG_DEFINEFONT2                  = 48,
    TAG_EXPORTASSETS                 = 56,
    TAG_IMPORTASSETS                 = 57,
    TAG_ENABLEDEBUGGER               = 58,
    TAG_DOINITACTION                 = 59,
    TAG_VIDEOSTREAM                  = 60,
    TAG_VIDEOFRAME                   = 61,
    TAG_DEFINEFONTINFO2              = 62,
    TAG_ENABLEDEBUGGER2              = 64,
    TAG_SCRIPTLIMITS                 = 65,
    TAG_SETTABINDEX                  = 66,
    TAG_FILEATTRIBUTES               = 69,
    TAG_PLACEOBJECT3                 = 70,
    TAG_IMPORTASSETS2                = 71,
    TAG_DEFINEFONTALIGNZONES         = 73,
    TAG_CSMTEXTSETTINGS              = 74,
    TAG_DEFINEFONT3                  = 75,
    TAG_SYMBOLCLASS                  = 76,
    TAG_METADATA                     = 77,
    TAG_DEFINESCALINGGRID            = 78,
    TAG_DOABC                        = 82,
    TAG_DEFINESHAPE4                 = 83,
    TAG_DEFINEMORPHSHAPE2            = 84,
    TAG_DEFINESCENEANDFRAMELABELDATA = 86,
    TAG_DEFINEBINARYDATA             = 87,
    TAG_DEFINEFONTNAME               = 88,
    TAG_STARTSOUND2                  = 89,
    TAG_DEFINEBITSJPEG4              = 90,
    TAG_DEFINEFONT4                  = 91,
};

#define TAG_LONG         0x100

/* flags for shape definition */
#define FLAG_MOVETO      0x01
#define FLAG_SETFILL0    0x02
#define FLAG_SETFILL1    0x04

/* character id used */
#define BITMAP_ID 0
#define VIDEO_ID 0
#define SHAPE_ID  1

extern const AVCodecTag ff_swf_codec_tags[];

#endif /* AVFORMAT_SWF_H */
