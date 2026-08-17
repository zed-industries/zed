// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/cipher.h>

#include <assert.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "internal.h"
#include "../internal.h"


static const struct {
  int nid;
  const char *name;
  const EVP_CIPHER *(*func)(void);
} kCiphers[] = {
    {NID_aes_128_cbc, "aes-128-cbc", EVP_aes_128_cbc},
    {NID_aes_128_cfb128, "aes-128-cfb", EVP_aes_128_cfb},
    {NID_aes_128_ctr, "aes-128-ctr", EVP_aes_128_ctr},
    {NID_aes_128_ecb, "aes-128-ecb", EVP_aes_128_ecb},
    {NID_aes_128_gcm, "aes-128-gcm", EVP_aes_128_gcm},
    {NID_aes_128_ofb128, "aes-128-ofb", EVP_aes_128_ofb},
    {NID_aes_192_cbc, "aes-192-cbc", EVP_aes_192_cbc},
    {NID_aes_192_cfb128, "aes-192-cfb", EVP_aes_192_cfb},
    {NID_aes_192_ctr, "aes-192-ctr", EVP_aes_192_ctr},
    {NID_aes_192_ecb, "aes-192-ecb", EVP_aes_192_ecb},
    {NID_aes_192_gcm, "aes-192-gcm", EVP_aes_192_gcm},
    {NID_aes_192_ofb128, "aes-192-ofb", EVP_aes_192_ofb},
    {NID_aes_256_cbc, "aes-256-cbc", EVP_aes_256_cbc},
    {NID_aes_256_cfb128, "aes-256-cfb", EVP_aes_256_cfb},
    {NID_aes_256_ctr, "aes-256-ctr", EVP_aes_256_ctr},
    {NID_aes_256_ecb, "aes-256-ecb", EVP_aes_256_ecb},
    {NID_aes_256_gcm, "aes-256-gcm", EVP_aes_256_gcm},
    {NID_aes_256_ofb128, "aes-256-ofb", EVP_aes_256_ofb},
    {NID_aes_256_xts, "aes-256-xts", EVP_aes_256_xts},
    {NID_chacha20_poly1305, "chacha20-poly1305", EVP_chacha20_poly1305},
    {NID_des_cbc, "des-cbc", EVP_des_cbc},
    {NID_des_ecb, "des-ecb", EVP_des_ecb},
    {NID_des_ede_cbc, "des-ede-cbc", EVP_des_ede_cbc},
    {NID_des_ede_ecb, "des-ede", EVP_des_ede},
    {NID_des_ede3_cbc, "des-ede3-cbc", EVP_des_ede3_cbc},
    {NID_rc2_cbc, "rc2-cbc", EVP_rc2_cbc},
    {NID_rc4, "rc4", EVP_rc4},
    {NID_bf_cbc, "bf-cbc", EVP_bf_cbc},
    {NID_bf_cfb64, "bf-cfb", EVP_bf_cfb},
    {NID_bf_ecb, "bf-ecb", EVP_bf_ecb},
};

static const struct {
  const char* alias;
  const char* name;
} kCipherAliases[] = {
    {"3des", "des-ede3-cbc"},
    {"DES", "des-cbc"},
    {"aes256", "aes-256-cbc"},
    {"aes128", "aes-128-cbc"},
    {"id-aes128-gcm", "aes-128-gcm"},
    {"id-aes192-gcm", "aes-192-gcm"},
    {"id-aes256-gcm", "aes-256-gcm"}
};

const EVP_CIPHER *EVP_get_cipherbynid(int nid) {
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kCiphers); i++) {
    if (kCiphers[i].nid == nid) {
      return kCiphers[i].func();
    }
  }
  return NULL;
}

static const EVP_CIPHER *get_cipherbyname(const char* name) {
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kCiphers); i++) {
    if (OPENSSL_strcasecmp(kCiphers[i].name, name) == 0) {
      return kCiphers[i].func();
    }
  }

  return NULL;
}

const EVP_CIPHER *EVP_get_cipherbyname(const char *name) {
  if (name == NULL) {
    return NULL;
  }

  const EVP_CIPHER * ec = get_cipherbyname(name);
  if (ec != NULL) {
    return ec;
  }

  // These are not names used by OpenSSL, but tcpdump registers it with
  // |EVP_add_cipher_alias|. Our |EVP_add_cipher_alias| is a no-op, so we
  // support the name here.
  for(size_t i = 0; i < OPENSSL_ARRAY_SIZE(kCipherAliases); i++) {
    if (OPENSSL_strcasecmp(name, kCipherAliases[i].alias) == 0) {
      name = kCipherAliases[i].name;
      const EVP_CIPHER * cipher = get_cipherbyname(name);
      assert(cipher != NULL);
      return cipher;
    }
  }

  return NULL;
}
