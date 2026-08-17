// Copyright (c) 2016 Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_SPAKE25519_INTERNAL_H
#define OPENSSL_HEADER_SPAKE25519_INTERNAL_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <openssl/base.h>
#include <openssl/curve25519.h>

enum spake2_state_t {
  spake2_state_init = 0,
  spake2_state_msg_generated,
  spake2_state_key_generated,
};

struct spake2_ctx_st {
  uint8_t private_key[32];
  uint8_t my_msg[32];
  uint8_t password_scalar[32];
  uint8_t password_hash[64];
  uint8_t *my_name;
  size_t my_name_len;
  uint8_t *their_name;
  size_t their_name_len;
  enum spake2_role_t my_role;
  enum spake2_state_t state;
  char disable_password_scalar_hack;
};

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_SPAKE25519_INTERNAL_H
