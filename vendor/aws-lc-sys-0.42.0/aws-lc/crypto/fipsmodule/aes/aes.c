// Copyright (c) 2002-2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0
 
#include <openssl/aes.h>

#include <assert.h>

#include "internal.h"
#include "../modes/internal.h"


// Be aware that different sets of AES functions use incompatible key
// representations, varying in format of the key schedule, the |AES_KEY.rounds|
// value, or both. Therefore they cannot mix. Also, on AArch64, the plain-C
// code, above, is incompatible with the |aes_hw_*| functions.

void AES_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key) {
  SET_DIT_AUTO_RESET;
  if (hwaes_capable()) {
    aes_hw_encrypt(in, out, key);
  } else if (vpaes_capable()) {
    vpaes_encrypt(in, out, key);
  } else {
    aes_nohw_encrypt(in, out, key);
  }
}

void AES_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key) {
  SET_DIT_AUTO_RESET;
  if (hwaes_capable()) {
    aes_hw_decrypt(in, out, key);
  } else if (vpaes_capable()) {
    vpaes_decrypt(in, out, key);
  } else {
    aes_nohw_decrypt(in, out, key);
  }
}

int AES_set_encrypt_key(const uint8_t *key, unsigned bits, AES_KEY *aeskey) {
  SET_DIT_AUTO_RESET;
  if (bits != 128 && bits != 192 && bits != 256) {
    return -2;
  }
  if (hwaes_capable()) {
    return aes_hw_set_encrypt_key(key, bits, aeskey);
  } else if (vpaes_capable()) {
    return vpaes_set_encrypt_key(key, bits, aeskey);
  } else {
    return aes_nohw_set_encrypt_key(key, bits, aeskey);
  }
}

int AES_set_decrypt_key(const uint8_t *key, unsigned bits, AES_KEY *aeskey) {
  SET_DIT_AUTO_RESET;
  if (bits != 128 && bits != 192 && bits != 256) {
    return -2;
  }
  if (hwaes_capable()) {
    return aes_hw_set_decrypt_key(key, bits, aeskey);
  } else if (vpaes_capable()) {
    return vpaes_set_decrypt_key(key, bits, aeskey);
  } else {
    return aes_nohw_set_decrypt_key(key, bits, aeskey);
  }
}
