/*
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

#ifndef COMPAT_CUDA_DYNLINK_LOADER_H
#define COMPAT_CUDA_DYNLINK_LOADER_H

#include "libavutil/log.h"
#include "compat/w32dlfcn.h"

#define FFNV_LOAD_FUNC(path) dlopen((path), RTLD_LAZY)
#define FFNV_SYM_FUNC(lib, sym) dlsym((lib), (sym))
#define FFNV_FREE_FUNC(lib) dlclose(lib)
#define FFNV_LOG_FUNC(logctx, msg, ...) av_log(logctx, AV_LOG_ERROR, msg,  __VA_ARGS__)
#define FFNV_DEBUG_LOG_FUNC(logctx, msg, ...) av_log(logctx, AV_LOG_DEBUG, msg,  __VA_ARGS__)

#include <ffnvcodec/dynlink_loader.h>

#endif /* COMPAT_CUDA_DYNLINK_LOADER_H */
