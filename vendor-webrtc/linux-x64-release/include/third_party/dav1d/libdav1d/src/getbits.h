/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_SRC_GETBITS_H
#define DAV1D_SRC_GETBITS_H

#include <stddef.h>
#include <stdint.h>

typedef struct GetBits {
    uint64_t state;
    int bits_left, error;
    const uint8_t *ptr, *ptr_start, *ptr_end;
} GetBits;

void dav1d_init_get_bits(GetBits *c, const uint8_t *data, size_t sz);
unsigned dav1d_get_bit(GetBits *c);
unsigned dav1d_get_bits(GetBits *c, int n);
int dav1d_get_sbits(GetBits *c, int n);
unsigned dav1d_get_uleb128(GetBits *c);

// Output in range 0..max-1
unsigned dav1d_get_uniform(GetBits *c, unsigned max);
unsigned dav1d_get_vlc(GetBits *c);
int dav1d_get_bits_subexp(GetBits *c, int ref, unsigned n);

// Discard bits from the buffer until we're next byte-aligned.
static inline void dav1d_bytealign_get_bits(GetBits *c) {
    // bits_left is never more than 7, because it is only incremented
    // by refill(), called by dav1d_get_bits and that never reads more
    // than 7 bits more than it needs.
    //
    // If this wasn't true, we would need to work out how many bits to
    // discard (bits_left % 8), subtract that from bits_left and then
    // shift state right by that amount.
    assert(c->bits_left <= 7);

    c->bits_left = 0;
    c->state = 0;
}

// Return the current bit position relative to the start of the buffer.
static inline unsigned dav1d_get_bits_pos(const GetBits *c) {
    return (unsigned) (c->ptr - c->ptr_start) * 8 - c->bits_left;
}

#endif /* DAV1D_SRC_GETBITS_H */
