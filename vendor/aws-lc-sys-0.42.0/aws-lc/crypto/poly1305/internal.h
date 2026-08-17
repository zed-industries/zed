// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_POLY1305_INTERNAL_H
#define OPENSSL_HEADER_POLY1305_INTERNAL_H

#include <openssl/base.h>
#include <openssl/poly1305.h>

#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

static inline struct poly1305_state_st *poly1305_aligned_state(
    poly1305_state *state) {
  return align_pointer(state, 64);
}

#if defined(OPENSSL_ARM) && !defined(OPENSSL_NO_ASM) && !defined(OPENSSL_APPLE)
#define OPENSSL_POLY1305_NEON

void CRYPTO_poly1305_init_neon(poly1305_state *state, const uint8_t key[32]);

void CRYPTO_poly1305_update_neon(poly1305_state *state, const uint8_t *in,
                                 size_t in_len);

void CRYPTO_poly1305_finish_neon(poly1305_state *state, uint8_t mac[16]);
#endif


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_POLY1305_INTERNAL_H
