/*
 * Immersive Audio Model and Formats parsing
 * Copyright (c) 2023 James Almer <jamrial@gmail.com>
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

#ifndef AVFORMAT_IAMF_PARSE_H
#define AVFORMAT_IAMF_PARSE_H

#include <stdint.h>

#include "avio.h"
#include "iamf.h"

int ff_iamf_parse_obu_header(const uint8_t *buf, int buf_size,
                             unsigned *obu_size, int *start_pos, enum IAMF_OBU_Type *type,
                             unsigned *skip_samples, unsigned *discard_padding);

int ff_iamfdec_read_descriptors(IAMFContext *c, AVIOContext *pb,
                                int size, void *log_ctx);

#endif /* AVFORMAT_IAMF_PARSE_H */
