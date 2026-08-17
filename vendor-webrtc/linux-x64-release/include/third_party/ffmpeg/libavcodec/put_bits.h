/*
 * copyright (c) 2004 Michael Niedermayer <michaelni@gmx.at>
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
 * bitstream writer API
 */

#ifndef AVCODEC_PUT_BITS_H
#define AVCODEC_PUT_BITS_H

#include <stdint.h>
#include <stddef.h>

#include "config.h"
#include "libavutil/intreadwrite.h"
#include "libavutil/avassert.h"
#include "libavutil/common.h"

#if ARCH_X86_64
// TODO: Benchmark and optionally enable on other 64-bit architectures.
typedef uint64_t BitBuf;
#define AV_WBBUF AV_WB64
#define AV_WLBUF AV_WL64
#define BUF_BITS 64
#else
typedef uint32_t BitBuf;
#define AV_WBBUF AV_WB32
#define AV_WLBUF AV_WL32
#define BUF_BITS 32
#endif

typedef struct PutBitContext {
    BitBuf bit_buf;
    int bit_left;
    uint8_t *buf, *buf_ptr, *buf_end;
} PutBitContext;

/**
 * Initialize the PutBitContext s.
 *
 * @param buffer the buffer where to put bits
 * @param buffer_size the size in bytes of buffer
 */
static inline void init_put_bits(PutBitContext *s, uint8_t *buffer,
                                 int buffer_size)
{
    if (buffer_size < 0) {
        buffer_size = 0;
        buffer      = NULL;
    }

    s->buf          = buffer;
    s->buf_end      = s->buf + buffer_size;
    s->buf_ptr      = s->buf;
    s->bit_left     = BUF_BITS;
    s->bit_buf      = 0;
}

/**
 * @return the total number of bits written to the bitstream.
 */
static inline int put_bits_count(PutBitContext *s)
{
    return (s->buf_ptr - s->buf) * 8 + BUF_BITS - s->bit_left;
}

/**
 * @return the number of bytes output so far; may only be called
 *         when the PutBitContext is freshly initialized or flushed.
 */
static inline int put_bytes_output(const PutBitContext *s)
{
    av_assert2(s->bit_left == BUF_BITS);
    return s->buf_ptr - s->buf;
}

/**
 * @param  round_up  When set, the number of bits written so far will be
 *                   rounded up to the next byte.
 * @return the number of bytes output so far.
 */
static inline int put_bytes_count(const PutBitContext *s, int round_up)
{
    return s->buf_ptr - s->buf + ((BUF_BITS - s->bit_left + (round_up ? 7 : 0)) >> 3);
}

/**
 * Rebase the bit writer onto a reallocated buffer.
 *
 * @param buffer the buffer where to put bits
 * @param buffer_size the size in bytes of buffer,
 *                    must be large enough to hold everything written so far
 */
static inline void rebase_put_bits(PutBitContext *s, uint8_t *buffer,
                                   int buffer_size)
{
    av_assert0(8*buffer_size >= put_bits_count(s));

    s->buf_end = buffer + buffer_size;
    s->buf_ptr = buffer + (s->buf_ptr - s->buf);
    s->buf     = buffer;
}

/**
 * @return the number of bits available in the bitstream.
 */
static inline int put_bits_left(PutBitContext* s)
{
    return (s->buf_end - s->buf_ptr) * 8 - BUF_BITS + s->bit_left;
}

/**
 * @param  round_up  When set, the number of bits written will be
 *                   rounded up to the next byte.
 * @return the number of bytes left.
 */
static inline int put_bytes_left(const PutBitContext *s, int round_up)
{
    return s->buf_end - s->buf_ptr - ((BUF_BITS - s->bit_left + (round_up ? 7 : 0)) >> 3);
}

/**
 * Pad the end of the output stream with zeros.
 */
static inline void flush_put_bits(PutBitContext *s)
{
#ifndef BITSTREAM_WRITER_LE
    if (s->bit_left < BUF_BITS)
        s->bit_buf <<= s->bit_left;
#endif
    while (s->bit_left < BUF_BITS) {
        av_assert0(s->buf_ptr < s->buf_end);
#ifdef BITSTREAM_WRITER_LE
        *s->buf_ptr++ = s->bit_buf;
        s->bit_buf  >>= 8;
#else
        *s->buf_ptr++ = s->bit_buf >> (BUF_BITS - 8);
        s->bit_buf  <<= 8;
#endif
        s->bit_left  += 8;
    }
    s->bit_left = BUF_BITS;
    s->bit_buf  = 0;
}

static inline void flush_put_bits_le(PutBitContext *s)
{
    while (s->bit_left < BUF_BITS) {
        av_assert0(s->buf_ptr < s->buf_end);
        *s->buf_ptr++ = s->bit_buf;
        s->bit_buf  >>= 8;
        s->bit_left  += 8;
    }
    s->bit_left = BUF_BITS;
    s->bit_buf  = 0;
}

#ifdef BITSTREAM_WRITER_LE
#define ff_put_string ff_put_string_unsupported_here
#define ff_copy_bits ff_copy_bits_unsupported_here
#else

/**
 * Put the string string in the bitstream.
 *
 * @param terminate_string 0-terminates the written string if value is 1
 */
void ff_put_string(PutBitContext *pb, const char *string,
                       int terminate_string);

/**
 * Copy the content of src to the bitstream.
 *
 * @param length the number of bits of src to copy
 */
void ff_copy_bits(PutBitContext *pb, const uint8_t *src, int length);
#endif

static inline void put_bits_no_assert(PutBitContext *s, int n, BitBuf value)
{
    BitBuf bit_buf;
    int bit_left;

    bit_buf  = s->bit_buf;
    bit_left = s->bit_left;

    /* XXX: optimize */
#ifdef BITSTREAM_WRITER_LE
    bit_buf |= value << (BUF_BITS - bit_left);
    if (n >= bit_left) {
        if (s->buf_end - s->buf_ptr >= sizeof(BitBuf)) {
            AV_WLBUF(s->buf_ptr, bit_buf);
            s->buf_ptr += sizeof(BitBuf);
        } else {
            av_log(NULL, AV_LOG_ERROR, "Internal error, put_bits buffer too small\n");
            av_assert2(0);
        }
        bit_buf     = value >> bit_left;
        bit_left   += BUF_BITS;
    }
    bit_left -= n;
#else
    if (n < bit_left) {
        bit_buf     = (bit_buf << n) | value;
        bit_left   -= n;
    } else {
        bit_buf   <<= bit_left;
        bit_buf    |= value >> (n - bit_left);
        if (s->buf_end - s->buf_ptr >= sizeof(BitBuf)) {
            AV_WBBUF(s->buf_ptr, bit_buf);
            s->buf_ptr += sizeof(BitBuf);
        } else {
            av_log(NULL, AV_LOG_ERROR, "Internal error, put_bits buffer too small\n");
            av_assert2(0);
        }
        bit_left   += BUF_BITS - n;
        bit_buf     = value;
    }
#endif

    s->bit_buf  = bit_buf;
    s->bit_left = bit_left;
}

/**
 * Write up to 31 bits into a bitstream.
 * Use put_bits32 to write 32 bits.
 */
static inline void put_bits(PutBitContext *s, int n, BitBuf value)
{
    av_assert2(n <= 31 && value < (1UL << n));
    put_bits_no_assert(s, n, value);
}

static inline void put_bits_le(PutBitContext *s, int n, BitBuf value)
{
    BitBuf bit_buf;
    int bit_left;

    av_assert2(n <= 31 && value < (1UL << n));

    bit_buf  = s->bit_buf;
    bit_left = s->bit_left;

    bit_buf |= value << (BUF_BITS - bit_left);
    if (n >= bit_left) {
        if (s->buf_end - s->buf_ptr >= sizeof(BitBuf)) {
            AV_WLBUF(s->buf_ptr, bit_buf);
            s->buf_ptr += sizeof(BitBuf);
        } else {
            av_log(NULL, AV_LOG_ERROR, "Internal error, put_bits buffer too small\n");
            av_assert2(0);
        }
        bit_buf     = value >> bit_left;
        bit_left   += BUF_BITS;
    }
    bit_left -= n;

    s->bit_buf  = bit_buf;
    s->bit_left = bit_left;
}

static inline void put_sbits(PutBitContext *pb, int n, int32_t value)
{
    av_assert2(n >= 0 && n <= 31);

    put_bits(pb, n, av_zero_extend(value, n));
}

/**
 * Write exactly 32 bits into a bitstream.
 */
static void av_unused put_bits32(PutBitContext *s, uint32_t value)
{
    BitBuf bit_buf;
    int bit_left;

    if (BUF_BITS > 32) {
        put_bits_no_assert(s, 32, value);
        return;
    }

    bit_buf  = s->bit_buf;
    bit_left = s->bit_left;

#ifdef BITSTREAM_WRITER_LE
    bit_buf |= (BitBuf)value << (BUF_BITS - bit_left);
    if (s->buf_end - s->buf_ptr >= sizeof(BitBuf)) {
        AV_WLBUF(s->buf_ptr, bit_buf);
        s->buf_ptr += sizeof(BitBuf);
    } else {
        av_log(NULL, AV_LOG_ERROR, "Internal error, put_bits buffer too small\n");
        av_assert2(0);
    }
    bit_buf     = (uint64_t)value >> bit_left;
#else
    bit_buf     = (uint64_t)bit_buf << bit_left;
    bit_buf    |= (BitBuf)value >> (BUF_BITS - bit_left);
    if (s->buf_end - s->buf_ptr >= sizeof(BitBuf)) {
        AV_WBBUF(s->buf_ptr, bit_buf);
        s->buf_ptr += sizeof(BitBuf);
    } else {
        av_log(NULL, AV_LOG_ERROR, "Internal error, put_bits buffer too small\n");
        av_assert2(0);
    }
    bit_buf     = value;
#endif

    s->bit_buf  = bit_buf;
    s->bit_left = bit_left;
}

/**
 * Write up to 63 bits into a bitstream.
 */
static inline void put_bits63(PutBitContext *s, int n, uint64_t value)
{
    av_assert2(n < 64U && value < (UINT64_C(1) << n));

#if BUF_BITS >= 64
    put_bits_no_assert(s, n, value);
#else
    if (n < 32)
        put_bits(s, n, value);
    else if (n == 32)
        put_bits32(s, value);
    else if (n < 64) {
        uint32_t lo = value & 0xffffffff;
        uint32_t hi = value >> 32;
#ifdef BITSTREAM_WRITER_LE
        put_bits32(s, lo);
        put_bits(s, n - 32, hi);
#else
        put_bits(s, n - 32, hi);
        put_bits32(s, lo);
#endif
    }
#endif
}

/**
 * Write up to 64 bits into a bitstream.
 */
static inline void put_bits64(PutBitContext *s, int n, uint64_t value)
{
    av_assert2((n == 64) || (n < 64 && value < (UINT64_C(1) << n)));

    if (n < 64) {
        put_bits63(s, n, value);
    } else {
        uint32_t lo = value & 0xffffffff;
        uint32_t hi = value >> 32;
#ifdef BITSTREAM_WRITER_LE
        put_bits32(s, lo);
        put_bits32(s, hi);
#else
        put_bits32(s, hi);
        put_bits32(s, lo);
#endif
    }
}

static inline void put_sbits63(PutBitContext *pb, int n, int64_t value)
{
    av_assert2(n >= 0 && n < 64);

    put_bits63(pb, n, (uint64_t)(value) & (~(UINT64_MAX << n)));
}

/**
 * Return the pointer to the byte where the bitstream writer will put
 * the next bit.
 */
static inline uint8_t *put_bits_ptr(PutBitContext *s)
{
    return s->buf_ptr;
}

/**
 * Skip the given number of bytes.
 * PutBitContext must be flushed & aligned to a byte boundary before calling this.
 */
static inline void skip_put_bytes(PutBitContext *s, int n)
{
    av_assert2((put_bits_count(s) & 7) == 0);
    av_assert2(s->bit_left == BUF_BITS);
    av_assert0(n <= s->buf_end - s->buf_ptr);
    s->buf_ptr += n;
}

/**
 * Skip the given number of bits.
 * Must only be used if the actual values in the bitstream do not matter.
 * If n is < 0 the behavior is undefined.
 */
static inline void skip_put_bits(PutBitContext *s, int n)
{
    unsigned bits = BUF_BITS - s->bit_left + n;
    s->buf_ptr += sizeof(BitBuf) * (bits / BUF_BITS);
    s->bit_left = BUF_BITS - (bits & (BUF_BITS - 1));
}

/**
 * Change the end of the buffer.
 *
 * @param size the new size in bytes of the buffer where to put bits
 */
static inline void set_put_bits_buffer_size(PutBitContext *s, int size)
{
    av_assert0(size <= INT_MAX/8 - BUF_BITS);
    s->buf_end = s->buf + size;
}

/**
 * Pad the bitstream with zeros up to the next byte boundary.
 */
static inline void align_put_bits(PutBitContext *s)
{
    put_bits(s, s->bit_left & 7, 0);
}

#undef AV_WBBUF
#undef AV_WLBUF

#endif /* AVCODEC_PUT_BITS_H */
