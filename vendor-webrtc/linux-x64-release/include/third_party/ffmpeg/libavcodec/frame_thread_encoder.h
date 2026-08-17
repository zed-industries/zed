/*
 * Copyright (c) 2012 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_FRAME_THREAD_ENCODER_H
#define AVCODEC_FRAME_THREAD_ENCODER_H

#include "avcodec.h"

/**
 * Initialize frame thread encoder.
 * @note hardware encoders are not supported
 */
int ff_frame_thread_encoder_init(AVCodecContext *avctx);
void ff_frame_thread_encoder_free(AVCodecContext *avctx);
int ff_thread_video_encode_frame(AVCodecContext *avctx, AVPacket *pkt,
                                 AVFrame *frame, int *got_packet_ptr);

#endif /* AVCODEC_FRAME_THREAD_ENCODER_H */
