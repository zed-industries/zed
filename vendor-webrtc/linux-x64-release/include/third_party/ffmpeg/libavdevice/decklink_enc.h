/*
 * Blackmagic DeckLink output
 * Copyright (c) 2013-2014 Ramiro Polla
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

#ifndef AVDEVICE_DECKLINK_ENC_H
#define AVDEVICE_DECKLINK_ENC_H

#ifdef __cplusplus
extern "C" {
#endif

#include "libavformat/avformat.h"

int ff_decklink_write_header(AVFormatContext *avctx);
int ff_decklink_write_packet(AVFormatContext *avctx, AVPacket *pkt);
int ff_decklink_write_trailer(AVFormatContext *avctx);
int ff_decklink_list_output_devices(AVFormatContext *avctx, struct AVDeviceInfoList *device_list);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* AVDEVICE_DECKLINK_ENC_H */
