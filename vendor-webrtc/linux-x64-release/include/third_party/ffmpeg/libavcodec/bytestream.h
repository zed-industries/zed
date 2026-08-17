/*
 * Bytestream functions
 * copyright (c) 2006 Baptiste Coudurier <baptiste.coudurier@free.fr>
 * Copyright (c) 2012 Aneesh Dogra (lionaneesh) <lionaneesh@gmail.com>
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

#ifndef AVCODEC_BYTESTREAM_H
#define AVCODEC_BYTESTREAM_H

#include <stdint.h>
#include <string.h>

#include "libavutil/avassert.h"
#include "libavutil/common.h"
#include "libavutil/intreadwrite.h"

typedef struct GetByteContext {
    const uint8_t *buffer, *buffer_end, *buffer_start;
} GetByteContext;

typedef struct PutByteContext {
    uint8_t *buffer, *buffer_end, *buffer_start;
    int eof;
} PutByteContext;

#define DEF(type, name, bytes, read, write)                                  \
static av_always_inline type bytestream_get_ ## name(const uint8_t **b)        \
{                                                                              \
    (*b) += bytes;                                                             \
    return read(*b - bytes);                                                   \
}                                                                              \
static av_always_inline void bytestream_put_ ## name(uint8_t **b,              \
                                                     const type value)         \
{                                                                              \
    write(*b, value);                                                          \
    (*b) += bytes;                                                             \
}                                                                              \
static av_always_inline void bytestream2_put_ ## name ## u(PutByteContext *p,  \
                                                           const type value)   \
{                                                                              \
    bytestream_put_ ## name(&p->buffer, value);                                \
}                                                                              \
static av_always_inline void bytestream2_put_ ## name(PutByteContext *p,       \
                                                      const type value)        \
{                                                                              \
    if (!p->eof && (p->buffer_end - p->buffer >= bytes)) {                     \
        write(p->buffer, value);                                               \
        p->buffer += bytes;                                                    \
    } else                                                                     \
        p->eof = 1;                                                            \
}                                                                              \
static av_always_inline type bytestream2_get_ ## name ## u(GetByteContext *g)  \
{                                                                              \
    return bytestream_get_ ## name(&g->buffer);                                \
}                                                                              \
static av_always_inline type bytestream2_get_ ## name(GetByteContext *g)       \
{                                                                              \
    if (g->buffer_end - g->buffer < bytes) {                                   \
        g->buffer = g->buffer_end;                                             \
        return 0;                                                              \
    }                                                                          \
    return bytestream2_get_ ## name ## u(g);                                   \
}                                                                              \
static av_always_inline type bytestream2_peek_ ## name ## u(GetByteContext *g) \
{                                                                              \
    return read(g->buffer);                                                    \
}                                                                              \
static av_always_inline type bytestream2_peek_ ## name(GetByteContext *g)      \
{                                                                              \
    if (g->buffer_end - g->buffer < bytes)                                     \
        return 0;                                                              \
    return bytestream2_peek_ ## name ## u(g);                                  \
}

DEF(uint64_t,     le64, 8, AV_RL64, AV_WL64)
DEF(unsigned int, le32, 4, AV_RL32, AV_WL32)
DEF(unsigned int, le24, 3, AV_RL24, AV_WL24)
DEF(unsigned int, le16, 2, AV_RL16, AV_WL16)
DEF(uint64_t,     be64, 8, AV_RB64, AV_WB64)
DEF(unsigned int, be32, 4, AV_RB32, AV_WB32)
DEF(unsigned int, be24, 3, AV_RB24, AV_WB24)
DEF(unsigned int, be16, 2, AV_RB16, AV_WB16)
DEF(unsigned int, byte, 1, AV_RB8 , AV_WB8)

#if AV_HAVE_BIGENDIAN
#   define bytestream2_get_ne16  bytestream2_get_be16
#   define bytestream2_get_ne24  bytestream2_get_be24
#   define bytestream2_get_ne32  bytestream2_get_be32
#   define bytestream2_get_ne64  bytestream2_get_be64
#   define bytestream2_get_ne16u bytestream2_get_be16u
#   define bytestream2_get_ne24u bytestream2_get_be24u
#   define bytestream2_get_ne32u bytestream2_get_be32u
#   define bytestream2_get_ne64u bytestream2_get_be64u
#   define bytestream2_put_ne16  bytestream2_put_be16
#   define bytestream2_put_ne24  bytestream2_put_be24
#   define bytestream2_put_ne32  bytestream2_put_be32
#   define bytestream2_put_ne64  bytestream2_put_be64
#   define bytestream2_peek_ne16 bytestream2_peek_be16
#   define bytestream2_peek_ne24 bytestream2_peek_be24
#   define bytestream2_peek_ne32 bytestream2_peek_be32
#   define bytestream2_peek_ne64 bytestream2_peek_be64
#else
#   define bytestream2_get_ne16  bytestream2_get_le16
#   define bytestream2_get_ne24  bytestream2_get_le24
#   define bytestream2_get_ne32  bytestream2_get_le32
#   define bytestream2_get_ne64  bytestream2_get_le64
#   define bytestream2_get_ne16u bytestream2_get_le16u
#   define bytestream2_get_ne24u bytestream2_get_le24u
#   define bytestream2_get_ne32u bytestream2_get_le32u
#   define bytestream2_get_ne64u bytestream2_get_le64u
#   define bytestream2_put_ne16  bytestream2_put_le16
#   define bytestream2_put_ne24  bytestream2_put_le24
#   define bytestream2_put_ne32  bytestream2_put_le32
#   define bytestream2_put_ne64  bytestream2_put_le64
#   define bytestream2_peek_ne16 bytestream2_peek_le16
#   define bytestream2_peek_ne24 bytestream2_peek_le24
#   define bytestream2_peek_ne32 bytestream2_peek_le32
#   define bytestream2_peek_ne64 bytestream2_peek_le64
#endif

static av_always_inline void bytestream2_init(GetByteContext *g,
                                              const uint8_t *buf,
                                              int buf_size)
{
    av_assert0(buf_size >= 0);
    g->buffer       = buf;
    g->buffer_start = buf;
    g->buffer_end   = buf + buf_size;
}

static av_always_inline void bytestream2_init_writer(PutByteContext *p,
                                                     uint8_t *buf,
                                                     int buf_size)
{
    av_assert0(buf_size >= 0);
    p->buffer       = buf;
    p->buffer_start = buf;
    p->buffer_end   = buf + buf_size;
    p->eof          = 0;
}

static av_always_inline int bytestream2_get_bytes_left(GetByteContext *g)
{
    return g->buffer_end - g->buffer;
}

static av_always_inline int bytestream2_get_bytes_left_p(PutByteContext *p)
{
    return p->buffer_end - p->buffer;
}

static av_always_inline void bytestream2_skip(GetByteContext *g,
                                              unsigned int size)
{
    g->buffer += FFMIN(g->buffer_end - g->buffer, size);
}

static av_always_inline void bytestream2_skipu(GetByteContext *g,
                                               unsigned int size)
{
    g->buffer += size;
}

static av_always_inline void bytestream2_skip_p(PutByteContext *p,
                                                unsigned int size)
{
    unsigned int size2;
    if (p->eof)
        return;
    size2 = FFMIN(p->buffer_end - p->buffer, size);
    if (size2 != size)
        p->eof = 1;
    p->buffer += size2;
}

static av_always_inline int bytestream2_tell(GetByteContext *g)
{
    return (int)(g->buffer - g->buffer_start);
}

static av_always_inline int bytestream2_tell_p(PutByteContext *p)
{
    return (int)(p->buffer - p->buffer_start);
}

static av_always_inline int bytestream2_size(GetByteContext *g)
{
    return (int)(g->buffer_end - g->buffer_start);
}

static av_always_inline int bytestream2_size_p(PutByteContext *p)
{
    return (int)(p->buffer_end - p->buffer_start);
}

static av_always_inline int bytestream2_seek(GetByteContext *g,
                                             int offset,
                                             int whence)
{
    switch (whence) {
    case SEEK_CUR:
        offset     = av_clip(offset, -(g->buffer - g->buffer_start),
                             g->buffer_end - g->buffer);
        g->buffer += offset;
        break;
    case SEEK_END:
        offset    = av_clip(offset, -(g->buffer_end - g->buffer_start), 0);
        g->buffer = g->buffer_end + offset;
        break;
    case SEEK_SET:
        offset    = av_clip(offset, 0, g->buffer_end - g->buffer_start);
        g->buffer = g->buffer_start + offset;
        break;
    default:
        return AVERROR(EINVAL);
    }
    return bytestream2_tell(g);
}

static av_always_inline int bytestream2_seek_p(PutByteContext *p,
                                               int offset,
                                               int whence)
{
    p->eof = 0;
    switch (whence) {
    case SEEK_CUR:
        if (p->buffer_end - p->buffer < offset)
            p->eof = 1;
        offset     = av_clip(offset, -(p->buffer - p->buffer_start),
                             p->buffer_end - p->buffer);
        p->buffer += offset;
        break;
    case SEEK_END:
        if (offset > 0)
            p->eof = 1;
        offset    = av_clip(offset, -(p->buffer_end - p->buffer_start), 0);
        p->buffer = p->buffer_end + offset;
        break;
    case SEEK_SET:
        if (p->buffer_end - p->buffer_start < offset)
            p->eof = 1;
        offset    = av_clip(offset, 0, p->buffer_end - p->buffer_start);
        p->buffer = p->buffer_start + offset;
        break;
    default:
        return AVERROR(EINVAL);
    }
    return bytestream2_tell_p(p);
}

static av_always_inline unsigned int bytestream2_get_buffer(GetByteContext *g,
                                                            uint8_t *dst,
                                                            unsigned int size)
{
    unsigned int size2 = FFMIN(g->buffer_end - g->buffer, size);
    memcpy(dst, g->buffer, size2);
    g->buffer += size2;
    return size2;
}

static av_always_inline unsigned int bytestream2_get_bufferu(GetByteContext *g,
                                                             uint8_t *dst,
                                                             unsigned int size)
{
    memcpy(dst, g->buffer, size);
    g->buffer += size;
    return size;
}

static av_always_inline unsigned int bytestream2_put_buffer(PutByteContext *p,
                                                            const uint8_t *src,
                                                            unsigned int size)
{
    unsigned int size2;
    if (p->eof)
        return 0;
    size2 = FFMIN(p->buffer_end - p->buffer, size);
    if (size2 != size)
        p->eof = 1;
    memcpy(p->buffer, src, size2);
    p->buffer += size2;
    return size2;
}

static av_always_inline unsigned int bytestream2_put_bufferu(PutByteContext *p,
                                                             const uint8_t *src,
                                                             unsigned int size)
{
    memcpy(p->buffer, src, size);
    p->buffer += size;
    return size;
}

static av_always_inline void bytestream2_set_buffer(PutByteContext *p,
                                                    const uint8_t c,
                                                    unsigned int size)
{
    unsigned int size2;
    if (p->eof)
        return;
    size2 = FFMIN(p->buffer_end - p->buffer, size);
    if (size2 != size)
        p->eof = 1;
    memset(p->buffer, c, size2);
    p->buffer += size2;
}

static av_always_inline void bytestream2_set_bufferu(PutByteContext *p,
                                                     const uint8_t c,
                                                     unsigned int size)
{
    memset(p->buffer, c, size);
    p->buffer += size;
}

static av_always_inline unsigned int bytestream2_get_eof(PutByteContext *p)
{
    return p->eof;
}

static av_always_inline unsigned int bytestream2_copy_bufferu(PutByteContext *p,
                                                              GetByteContext *g,
                                                              unsigned int size)
{
    memcpy(p->buffer, g->buffer, size);
    p->buffer += size;
    g->buffer += size;
    return size;
}

static av_always_inline unsigned int bytestream2_copy_buffer(PutByteContext *p,
                                                             GetByteContext *g,
                                                             unsigned int size)
{
    unsigned int size2;

    if (p->eof)
        return 0;
    size  = FFMIN(g->buffer_end - g->buffer, size);
    size2 = FFMIN(p->buffer_end - p->buffer, size);
    if (size2 != size)
        p->eof = 1;

    return bytestream2_copy_bufferu(p, g, size2);
}

static av_always_inline unsigned int bytestream_get_buffer(const uint8_t **b,
                                                           uint8_t *dst,
                                                           unsigned int size)
{
    memcpy(dst, *b, size);
    (*b) += size;
    return size;
}

static av_always_inline void bytestream_put_buffer(uint8_t **b,
                                                   const uint8_t *src,
                                                   unsigned int size)
{
    memcpy(*b, src, size);
    (*b) += size;
}

#endif /* AVCODEC_BYTESTREAM_H */
