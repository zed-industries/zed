/*
 * VVC thread logic
 *
 * Copyright (C) 2023 Nuo Mi
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

#ifndef AVCODEC_VVC_THREAD_H
#define AVCODEC_VVC_THREAD_H

#include "dec.h"

struct FFExecutor* ff_vvc_executor_alloc(VVCContext *s, int thread_count);
void ff_vvc_executor_free(struct FFExecutor **e);

int ff_vvc_frame_thread_init(VVCFrameContext *fc);
void ff_vvc_frame_thread_free(VVCFrameContext *fc);
int ff_vvc_frame_submit(VVCContext *s, VVCFrameContext *fc);
int ff_vvc_frame_wait(VVCContext *s, VVCFrameContext *fc);
int ff_vvc_per_frame_init(VVCFrameContext *fc);

#endif // AVCODEC_VVC_THREAD_H
