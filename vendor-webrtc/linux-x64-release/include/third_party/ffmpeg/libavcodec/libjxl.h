/*
 * JPEG XL de/encoding via libjxl, common support header
 * Copyright (c) 2021 Leo Izen <leo.izen@gmail.com>
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
 * JPEG XL via libjxl common support header
 */

#ifndef AVCODEC_LIBJXL_H
#define AVCODEC_LIBJXL_H

#include <jxl/memory_manager.h>
#include <jxl/version.h>

/**
 * Transform threadcount in ffmpeg to one used by libjxl.
 *
 * @param  threads ffmpeg's threads AVOption
 * @return         thread count for libjxl's parallel runner
 */
size_t ff_libjxl_get_threadcount(int threads);

/**
 * Initialize and populate a JxlMemoryManager
 * with av_malloc() and av_free() so libjxl will use these
 * functions.
 * @param manager a pointer to a JxlMemoryManager struct
 */
void ff_libjxl_init_memory_manager(JxlMemoryManager *manager);

#endif /* AVCODEC_LIBJXL_H */
