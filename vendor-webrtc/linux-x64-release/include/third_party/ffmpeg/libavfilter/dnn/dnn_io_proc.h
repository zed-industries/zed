/*
 * Copyright (c) 2020
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
 * DNN input&output process between AVFrame and DNNData.
 */


#ifndef AVFILTER_DNN_DNN_IO_PROC_H
#define AVFILTER_DNN_DNN_IO_PROC_H

#include "../dnn_interface.h"
#include "libavutil/frame.h"

int ff_proc_from_frame_to_dnn(AVFrame *frame, DNNData *input, void *log_ctx);
int ff_proc_from_dnn_to_frame(AVFrame *frame, DNNData *output, void *log_ctx);
int ff_frame_to_dnn_detect(AVFrame *frame, DNNData *input, void *log_ctx);
int ff_frame_to_dnn_classify(AVFrame *frame, DNNData *input, uint32_t bbox_index, void *log_ctx);

#endif
