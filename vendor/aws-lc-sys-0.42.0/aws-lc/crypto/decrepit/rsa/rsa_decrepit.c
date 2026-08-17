// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/rsa.h>

#include <assert.h>

#include <openssl/bn.h>


RSA *RSA_generate_key(int bits, uint64_t e_value, void *callback,
                      void *cb_arg) {
  assert(callback == NULL);
  assert(cb_arg == NULL);

  RSA *rsa = RSA_new();
  BIGNUM *e = BN_new();

  if (rsa == NULL ||
      e == NULL ||
      !BN_set_u64(e, e_value) ||
      !RSA_generate_key_ex(rsa, bits, e, NULL)) {
    goto err;
  }

  BN_free(e);
  return rsa;

err:
  BN_free(e);
  RSA_free(rsa);
  return NULL;
}

int RSA_padding_add_PKCS1_PSS(const RSA *rsa, uint8_t *EM, const uint8_t *mHash,
                              const EVP_MD *Hash, int sLen) {
  return RSA_padding_add_PKCS1_PSS_mgf1(rsa, EM, mHash, Hash, NULL, sLen);
}

int RSA_verify_PKCS1_PSS(const RSA *rsa, const uint8_t *mHash,
                         const EVP_MD *Hash, const uint8_t *EM, int sLen) {
  return RSA_verify_PKCS1_PSS_mgf1(rsa, mHash, Hash, NULL, EM, sLen);
}

int RSA_padding_add_PKCS1_OAEP(uint8_t *to, size_t to_len,
                               const uint8_t *from, size_t from_len,
                               const uint8_t *param, size_t param_len) {
  return RSA_padding_add_PKCS1_OAEP_mgf1(to, to_len, from, from_len, param,
                                         param_len, NULL, NULL);
}
