/*
 * Wrappers for zlib
 * Copyright (C) 2022 Andreas Rheinhardt
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

#ifndef AVCODEC_ZLIB_WRAPPER_H
#define AVCODEC_ZLIB_WRAPPER_H

#include <zlib.h>

typedef struct FFZStream {
    z_stream zstream;
    int inited;
} FFZStream;

/**
 * Wrapper around inflateInit(). It initializes the fields that zlib
 * requires to be initialized before inflateInit().
 * In case of error it also returns an error message to the provided logctx;
 * in any case, it sets zstream->inited to indicate whether inflateInit()
 * succeeded.
 * @return Returns 0 on success or a negative error code on failure
 */
int ff_inflate_init(FFZStream *zstream, void *logctx);

/**
 * Wrapper around inflateEnd(). It calls inflateEnd() iff
 * zstream->inited is set and resets zstream->inited.
 * It is therefore safe to be called even if
 * ff_inflate_init() has never been called on it (or errored out)
 * provided that the FFZStream (or just FFZStream.inited) has been zeroed.
 */
void ff_inflate_end(FFZStream *zstream);

/**
 * Wrapper around deflateInit(). It works analogously to ff_inflate_init().
 */
int ff_deflate_init(FFZStream *zstream, int level, void *logctx);

/**
 * Wrapper around deflateEnd(). It works analogously to ff_inflate_end().
 */
void ff_deflate_end(FFZStream *zstream);

#endif /* AVCODEC_ZLIB_WRAPPER_H */
