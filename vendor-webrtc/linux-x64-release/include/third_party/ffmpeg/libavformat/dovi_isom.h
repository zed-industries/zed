/*
 * DOVI ISO Media common code
 * Copyright (c) 2021 quietvoid
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

#ifndef AVFORMAT_DOVI_ISOM_H
#define AVFORMAT_DOVI_ISOM_H

#include "libavutil/dovi_meta.h"

#include "avformat.h"

#define ISOM_DVCC_DVVC_SIZE 24

int ff_isom_parse_dvcc_dvvc(void *logctx, AVStream *st,
                            const uint8_t *buf_ptr, uint64_t size);
void ff_isom_put_dvcc_dvvc(void *logctx, uint8_t out[ISOM_DVCC_DVVC_SIZE],
                           const AVDOVIDecoderConfigurationRecord *dovi);

#endif /* AVFORMAT_DOVI_ISOM_H */
