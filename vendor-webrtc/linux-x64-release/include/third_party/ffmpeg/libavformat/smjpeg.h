/*
 * SMJPEG common code
 * Copyright (c) 2011-2012 Paul B Mahol
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
 * SMJPEG common code
 */

#ifndef AVFORMAT_SMJPEG_H
#define AVFORMAT_SMJPEG_H

#include "internal.h"

#define SMJPEG_MAGIC "\x0\xaSMJPEG"

#define SMJPEG_DONE     MKTAG('D', 'O', 'N', 'E')
#define SMJPEG_HEND     MKTAG('H', 'E', 'N', 'D')
#define SMJPEG_SND      MKTAG('_', 'S', 'N', 'D')
#define SMJPEG_SNDD     MKTAG('s', 'n', 'd', 'D')
#define SMJPEG_TXT      MKTAG('_', 'T', 'X', 'T')
#define SMJPEG_VID      MKTAG('_', 'V', 'I', 'D')
#define SMJPEG_VIDD     MKTAG('v', 'i', 'd', 'D')

extern const AVCodecTag ff_codec_smjpeg_video_tags[];
extern const AVCodecTag ff_codec_smjpeg_audio_tags[];

#endif /* AVFORMAT_SMJPEG_H */
