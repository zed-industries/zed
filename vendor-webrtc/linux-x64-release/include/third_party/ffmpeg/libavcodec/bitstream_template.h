/*
 * Copyright (c) 2016 Alexandra Hájková
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

#ifdef BITSTREAM_TEMPLATE_LE
#   define BS_SUFFIX_LOWER _le
#   define BS_SUFFIX_UPPER LE
#else
#   define BS_SUFFIX_LOWER _be
#   define BS_SUFFIX_UPPER BE
#endif

#define BS_JOIN(x, y, z) x ## y ## z
#define BS_JOIN3(x, y, z) BS_JOIN(x, y, z)
#define BS_FUNC(x) BS_JOIN3(bits_, x, BS_SUFFIX_LOWER)

#define BSCTX BS_JOIN3(Bitstream, Context, BS_SUFFIX_UPPER)

typedef struct BSCTX {
    uint64_t bits;       // stores bits read from the buffer
    const uint8_t *buffer, *buffer_end;
    const uint8_t *ptr;  // pointer to the position inside a buffer
    unsigned bits_valid; // number of bits left in bits field
    unsigned size_in_bits;
} BSCTX;

/**
 * @return
 * - 0 on successful refill
 * - a negative number when bitstream end is hit
 *
 * Always succeeds when UNCHECKED_BITSTREAM_READER is enabled.
 */
static inline int BS_FUNC(priv_refill_64)(BSCTX *bc)
{
#if !UNCHECKED_BITSTREAM_READER
    if (bc->ptr >= bc->buffer_end)
        return -1;
#endif

#ifdef BITSTREAM_TEMPLATE_LE
    bc->bits       = AV_RL64(bc->ptr);
#else
    bc->bits       = AV_RB64(bc->ptr);
#endif
    bc->ptr       += 8;
    bc->bits_valid = 64;

    return 0;
}

/**
 * @return
 * - 0 on successful refill
 * - a negative number when bitstream end is hit
 *
 * Always succeeds when UNCHECKED_BITSTREAM_READER is enabled.
 */
static inline int BS_FUNC(priv_refill_32)(BSCTX *bc)
{
#if !UNCHECKED_BITSTREAM_READER
    if (bc->ptr >= bc->buffer_end)
        return -1;
#endif

#ifdef BITSTREAM_TEMPLATE_LE
    bc->bits      |= (uint64_t)AV_RL32(bc->ptr) << bc->bits_valid;
#else
    bc->bits      |= (uint64_t)AV_RB32(bc->ptr) << (32 - bc->bits_valid);
#endif
    bc->ptr        += 4;
    bc->bits_valid += 32;

    return 0;
}

/**
 * Initialize BitstreamContext.
 * @param buffer bitstream buffer, must be AV_INPUT_BUFFER_PADDING_SIZE bytes
 *        larger than the actual read bits because some optimized bitstream
 *        readers read 32 or 64 bits at once and could read over the end
 * @param bit_size the size of the buffer in bits
 * @return 0 on success, AVERROR_INVALIDDATA if the buffer_size would overflow.
 */
static inline int BS_FUNC(init)(BSCTX *bc, const uint8_t *buffer,
                                     unsigned int bit_size)
{
    unsigned int buffer_size;

    if (bit_size > INT_MAX - 7 || !buffer) {
        bc->buffer     = NULL;
        bc->ptr        = NULL;
        bc->bits_valid = 0;
        return AVERROR_INVALIDDATA;
    }

    buffer_size = (bit_size + 7) >> 3;

    bc->buffer       = buffer;
    bc->buffer_end   = buffer + buffer_size;
    bc->ptr          = bc->buffer;
    bc->size_in_bits = bit_size;
    bc->bits_valid   = 0;
    bc->bits         = 0;

    BS_FUNC(priv_refill_64)(bc);

    return 0;
}

/**
 * Initialize BitstreamContext.
 * @param buffer bitstream buffer, must be AV_INPUT_BUFFER_PADDING_SIZE bytes
 *        larger than the actual read bits because some optimized bitstream
 *        readers read 32 or 64 bits at once and could read over the end
 * @param byte_size the size of the buffer in bytes
 * @return 0 on success, AVERROR_INVALIDDATA if the buffer_size would overflow
 */
static inline int BS_FUNC(init8)(BSCTX *bc, const uint8_t *buffer,
                                      unsigned int byte_size)
{
    if (byte_size > INT_MAX / 8)
        return AVERROR_INVALIDDATA;
    return BS_FUNC(init)(bc, buffer, byte_size * 8);
}

/**
 * Return number of bits already read.
 */
static inline int BS_FUNC(tell)(const BSCTX *bc)
{
    return (bc->ptr - bc->buffer) * 8 - bc->bits_valid;
}

/**
 * Return buffer size in bits.
 */
static inline int BS_FUNC(size)(const BSCTX *bc)
{
    return bc->size_in_bits;
}

/**
 * Return the number of the bits left in a buffer.
 */
static inline int BS_FUNC(left)(const BSCTX *bc)
{
    return (bc->buffer - bc->ptr) * 8 + bc->size_in_bits + bc->bits_valid;
}

static inline uint64_t BS_FUNC(priv_val_show)(BSCTX *bc, unsigned int n)
{
    av_assert2(n > 0 && n <= 64);

#ifdef BITSTREAM_TEMPLATE_LE
    return bc->bits & (UINT64_MAX >> (64 - n));
#else
    return bc->bits >> (64 - n);
#endif
}

static inline void BS_FUNC(priv_skip_remaining)(BSCTX *bc, unsigned int n)
{
#ifdef BITSTREAM_TEMPLATE_LE
    bc->bits >>= n;
#else
    bc->bits <<= n;
#endif
    bc->bits_valid -= n;
}

static inline uint64_t BS_FUNC(priv_val_get)(BSCTX *bc, unsigned int n)
{
    uint64_t ret;

    av_assert2(n > 0 && n < 64);

    ret = BS_FUNC(priv_val_show)(bc, n);
    BS_FUNC(priv_skip_remaining)(bc, n);

    return ret;
}

/**
 * Return one bit from the buffer.
 */
static inline unsigned int BS_FUNC(read_bit)(BSCTX *bc)
{
    if (!bc->bits_valid && BS_FUNC(priv_refill_64)(bc) < 0)
        return 0;

    return BS_FUNC(priv_val_get)(bc, 1);
}

/**
 * Return n bits from the buffer, n has to be in the 1-32 range.
 * May be faster than bits_read() when n is not a compile-time constant and is
 * known to be non-zero;
 */
static inline uint32_t BS_FUNC(read_nz)(BSCTX *bc, unsigned int n)
{
    av_assert2(n > 0 && n <= 32);

    if (n > bc->bits_valid) {
        if (BS_FUNC(priv_refill_32)(bc) < 0)
            bc->bits_valid = n;
    }

    return BS_FUNC(priv_val_get)(bc, n);
}

/**
 * Return n bits from the buffer, n has to be in the 0-32  range.
 */
static inline uint32_t BS_FUNC(read)(BSCTX *bc, unsigned int n)
{
    av_assert2(n <= 32);

    if (!n)
        return 0;

    return BS_FUNC(read_nz)(bc, n);
}

/**
 * Return n bits from the buffer, n has to be in the 0-63 range.
 */
static inline uint64_t BS_FUNC(read_63)(BSCTX *bc, unsigned int n)
{
    uint64_t ret = 0;
    unsigned left = 0;

    av_assert2(n <= 63);

    if (!n)
        return 0;

    if (n > bc->bits_valid) {
        left = bc->bits_valid;
        n   -= left;

        if (left)
            ret = BS_FUNC(priv_val_get)(bc, left);

        if (BS_FUNC(priv_refill_64)(bc) < 0)
            bc->bits_valid = n;

    }

#ifdef BITSTREAM_TEMPLATE_LE
    ret = BS_FUNC(priv_val_get)(bc, n) << left | ret;
#else
    ret = BS_FUNC(priv_val_get)(bc, n) | ret << n;
#endif

    return ret;
}

/**
 * Return n bits from the buffer, n has to be in the 0-64 range.
 */
static inline uint64_t BS_FUNC(read_64)(BSCTX *bc, unsigned int n)
{
    av_assert2(n <= 64);

    if (n == 64) {
        uint64_t ret = BS_FUNC(read_63)(bc, 63);
#ifdef BITSTREAM_TEMPLATE_LE
        return ret | ((uint64_t)BS_FUNC(read_bit)(bc) << 63);
#else
        return (ret << 1) | (uint64_t)BS_FUNC(read_bit)(bc);
#endif
    }
    return BS_FUNC(read_63)(bc, n);
}

/**
 * Return n bits from the buffer as a signed integer, n has to be in the 1-32
 * range. May be faster than bits_read_signed() when n is not a compile-time
 * constant and is known to be non-zero;
 */
static inline int32_t BS_FUNC(read_signed_nz)(BSCTX *bc, unsigned int n)
{
    av_assert2(n > 0 && n <= 32);
    return sign_extend(BS_FUNC(read_nz)(bc, n), n);
}

/**
 * Return n bits from the buffer as a signed integer.
 * n has to be in the 0-32 range.
 */
static inline int32_t BS_FUNC(read_signed)(BSCTX *bc, unsigned int n)
{
    av_assert2(n <= 32);

    if (!n)
        return 0;

    return BS_FUNC(read_signed_nz)(bc, n);
}

/**
 * Return n bits from the buffer but do not change the buffer state.
 * n has to be in the 1-32 range. May
 */
static inline uint32_t BS_FUNC(peek_nz)(BSCTX *bc, unsigned int n)
{
    av_assert2(n > 0 && n <= 32);

    if (n > bc->bits_valid)
        BS_FUNC(priv_refill_32)(bc);

    return BS_FUNC(priv_val_show)(bc, n);
}

/**
 * Return n bits from the buffer but do not change the buffer state.
 * n has to be in the 0-32 range.
 */
static inline uint32_t BS_FUNC(peek)(BSCTX *bc, unsigned int n)
{
    av_assert2(n <= 32);

    if (!n)
        return 0;

    return BS_FUNC(peek_nz)(bc, n);
}

/**
 * Return n bits from the buffer as a signed integer, do not change the buffer
 * state. n has to be in the 1-32 range. May be faster than bits_peek_signed()
 * when n is not a compile-time constant and is known to be non-zero;
 */
static inline int BS_FUNC(peek_signed_nz)(BSCTX *bc, unsigned int n)
{
    av_assert2(n > 0 && n <= 32);
    return sign_extend(BS_FUNC(peek_nz)(bc, n), n);
}

/**
 * Return n bits from the buffer as a signed integer,
 * do not change the buffer state.
 * n has to be in the 0-32 range.
 */
static inline int BS_FUNC(peek_signed)(BSCTX *bc, unsigned int n)
{
    av_assert2(n <= 32);

    if (!n)
        return 0;

    return BS_FUNC(peek_signed_nz)(bc, n);
}

/**
 * Skip n bits in the buffer.
 */
static inline void BS_FUNC(skip)(BSCTX *bc, unsigned int n)
{
    if (n < bc->bits_valid)
        BS_FUNC(priv_skip_remaining)(bc, n);
    else {
        n -= bc->bits_valid;
        bc->bits       = 0;
        bc->bits_valid = 0;

        if (n >= 64) {
            unsigned int skip = n / 8;

            n -= skip * 8;
            bc->ptr += skip;
        }
        BS_FUNC(priv_refill_64)(bc);
        if (n)
            BS_FUNC(priv_skip_remaining)(bc, n);
    }
}

/**
 * Seek to the given bit position.
 */
static inline void BS_FUNC(seek)(BSCTX *bc, unsigned pos)
{
    bc->ptr        = bc->buffer;
    bc->bits       = 0;
    bc->bits_valid = 0;

    BS_FUNC(skip)(bc, pos);
}

/**
 * Skip bits to a byte boundary.
 */
static inline const uint8_t *BS_FUNC(align)(BSCTX *bc)
{
    unsigned int n = -BS_FUNC(tell)(bc) & 7;
    if (n)
        BS_FUNC(skip)(bc, n);
    return bc->buffer + (BS_FUNC(tell)(bc) >> 3);
}

/**
 * Read MPEG-1 dc-style VLC (sign bit + mantissa with no MSB).
 * If MSB not set it is negative.
 * @param n length in bits
 */
static inline int BS_FUNC(read_xbits)(BSCTX *bc, unsigned int n)
{
    int32_t cache = BS_FUNC(peek)(bc, 32);
    int sign = ~cache >> 31;
    BS_FUNC(priv_skip_remaining)(bc, n);

    return ((((uint32_t)(sign ^ cache)) >> (32 - n)) ^ sign) - sign;
}

/**
 * Return decoded truncated unary code for the values 0, 1, 2.
 */
static inline int BS_FUNC(decode012)(BSCTX *bc)
{
    if (!BS_FUNC(read_bit)(bc))
        return 0;
    else
        return BS_FUNC(read_bit)(bc) + 1;
}

/**
 * Return decoded truncated unary code for the values 2, 1, 0.
 */
static inline int BS_FUNC(decode210)(BSCTX *bc)
{
    if (BS_FUNC(read_bit)(bc))
        return 0;
    else
        return 2 - BS_FUNC(read_bit)(bc);
}

/* Read sign bit and flip the sign of the provided value accordingly. */
static inline int BS_FUNC(apply_sign)(BSCTX *bc, int val)
{
    int sign = BS_FUNC(read_signed)(bc, 1);
    return (val ^ sign) - sign;
}

static inline int BS_FUNC(skip_1stop_8data)(BSCTX *s)
{
    if (BS_FUNC(left)(s) <= 0)
        return AVERROR_INVALIDDATA;

    while (BS_FUNC(read_bit)(s)) {
        BS_FUNC(skip)(s, 8);
        if (BS_FUNC(left)(s) <= 0)
            return AVERROR_INVALIDDATA;
    }

    return 0;
}

/**
 * Return the LUT element for the given bitstream configuration.
 */
static inline int BS_FUNC(priv_set_idx)(BSCTX *bc, int code, int *n,
                                             int *nb_bits, const VLCElem *table)
{
    unsigned idx;

    *nb_bits = -*n;
    idx = BS_FUNC(peek)(bc, *nb_bits) + code;
    *n = table[idx].len;

    return table[idx].sym;
}

/**
 * Parse a vlc code.
 * @param bits is the number of bits which will be read at once, must be
 *             identical to nb_bits in vlc_init()
 * @param max_depth is the number of times bits bits must be read to completely
 *                  read the longest vlc code
 *                  = (max_vlc_length + bits - 1) / bits
 * If the vlc code is invalid and max_depth=1, then no bits will be removed.
 * If the vlc code is invalid and max_depth>1, then the number of bits removed
 * is undefined.
 */
static inline int BS_FUNC(read_vlc)(BSCTX *bc, const VLCElem *table,
                                         int bits, int max_depth)
{
    int nb_bits;
    unsigned idx = BS_FUNC(peek)(bc, bits);
    int code     = table[idx].sym;
    int n        = table[idx].len;

    if (max_depth > 1 && n < 0) {
        BS_FUNC(priv_skip_remaining)(bc, bits);
        code = BS_FUNC(priv_set_idx)(bc, code, &n, &nb_bits, table);
        if (max_depth > 2 && n < 0) {
            BS_FUNC(priv_skip_remaining)(bc, nb_bits);
            code = BS_FUNC(priv_set_idx)(bc, code, &n, &nb_bits, table);
        }
    }
    BS_FUNC(priv_skip_remaining)(bc, n);

    return code;
}

/**
 * Parse a vlc / vlc_multi code.
 * @param bits is the number of bits which will be read at once, must be
 *             identical to nb_bits in vlc_init()
 * @param max_depth is the number of times bits bits must be read to completely
 *                  read the longest vlc code
 *                  = (max_vlc_length + bits - 1) / bits
 * @param dst the parsed symbol(s) will be stored here. Up to 8 bytes are written
 * @returns number of symbols parsed
 * If the vlc code is invalid and max_depth=1, then no bits will be removed.
 * If the vlc code is invalid and max_depth>1, then the number of bits removed
 * is undefined.
 */
static inline int BS_FUNC(read_vlc_multi)(BSCTX *bc, uint8_t dst[8],
                                          const VLC_MULTI_ELEM *const Jtable,
                                          const VLCElem *const table,
                                          const int bits, const int max_depth,
                                          const int symbols_size)
{
    unsigned idx = BS_FUNC(peek)(bc, bits);
    int ret, nb_bits, code, n = Jtable[idx].len;
    if (Jtable[idx].num) {
        AV_COPY64U(dst, Jtable[idx].val8);
        ret = Jtable[idx].num;
    } else {
        code = table[idx].sym;
        n = table[idx].len;
        if (max_depth > 1 && n < 0) {
            BS_FUNC(priv_skip_remaining)(bc, bits);
            code = BS_FUNC(priv_set_idx)(bc, code, &n, &nb_bits, table);
            if (max_depth > 2 && n < 0) {
                BS_FUNC(priv_skip_remaining)(bc, nb_bits);
                code = BS_FUNC(priv_set_idx)(bc, code, &n, &nb_bits, table);
            }
        }
        if (symbols_size == 1)
            *dst = code;
        else
            AV_WN16(dst, code);
        ret = n > 0;
    }
    BS_FUNC(priv_skip_remaining)(bc, n);

    return ret;
}

#undef BSCTX
#undef BS_FUNC
#undef BS_JOIN3
#undef BS_JOIN
#undef BS_SUFFIX_UPPER
#undef BS_SUFFIX_LOWER
