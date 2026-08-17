// Copyright 2005-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/rsa.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "../service_indicator/internal.h"
#include "internal.h"


int RSA_padding_add_PKCS1_type_1(uint8_t *to, size_t to_len,
                                 const uint8_t *from, size_t from_len) {
  // See RFC 8017, section 9.2.
  if (to_len < RSA_PKCS1_PADDING_SIZE) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  if (from_len > to_len - RSA_PKCS1_PADDING_SIZE) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DIGEST_TOO_BIG_FOR_RSA_KEY);
    return 0;
  }

  to[0] = 0;
  to[1] = 1;
  OPENSSL_memset(to + 2, 0xff, to_len - 3 - from_len);
  to[to_len - from_len - 1] = 0;
  OPENSSL_memcpy(to + to_len - from_len, from, from_len);
  return 1;
}

int RSA_padding_check_PKCS1_type_1(uint8_t *out, size_t *out_len,
                                   size_t max_out, const uint8_t *from,
                                   size_t from_len) {
  // See RFC 8017, section 9.2. This is part of signature verification and thus
  // does not need to run in constant-time.
  if (from_len < 2) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_SMALL);
    return 0;
  }

  // Check the header.
  if (from[0] != 0 || from[1] != 1) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BLOCK_TYPE_IS_NOT_01);
    return 0;
  }

  // Scan over padded data, looking for the 00.
  size_t pad;
  for (pad = 2 /* header */; pad < from_len; pad++) {
    if (from[pad] == 0x00) {
      break;
    }

    if (from[pad] != 0xff) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_FIXED_HEADER_DECRYPT);
      return 0;
    }
  }

  if (pad == from_len) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_NULL_BEFORE_BLOCK_MISSING);
    return 0;
  }

  if (pad < 2 /* header */ + 8) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_PAD_BYTE_COUNT);
    return 0;
  }

  // Skip over the 00.
  pad++;

  if (from_len - pad > max_out) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE);
    return 0;
  }

  OPENSSL_memcpy(out, from + pad, from_len - pad);
  *out_len = from_len - pad;
  return 1;
}

int RSA_padding_add_none(uint8_t *to, size_t to_len, const uint8_t *from,
                         size_t from_len) {
  if (from_len > to_len) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_KEY_SIZE);
    return 0;
  }

  if (from_len < to_len) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_SMALL);
    return 0;
  }

  OPENSSL_memcpy(to, from, from_len);
  return 1;
}

int PKCS1_MGF1(uint8_t *out, size_t len, const uint8_t *seed, size_t seed_len,
               const EVP_MD *md) {
  int ret = 0;
  bssl::ScopedEVP_MD_CTX ctx;
  FIPS_service_indicator_lock_state();

  size_t md_len = EVP_MD_size(md);

  for (uint32_t i = 0; len > 0; i++) {
    uint8_t counter[4];
    counter[0] = (uint8_t)(i >> 24);
    counter[1] = (uint8_t)(i >> 16);
    counter[2] = (uint8_t)(i >> 8);
    counter[3] = (uint8_t)i;
    if (!EVP_DigestInit_ex(ctx.get(), md, nullptr) ||
        !EVP_DigestUpdate(ctx.get(), seed, seed_len) ||
        !EVP_DigestUpdate(ctx.get(), counter, sizeof(counter))) {
      goto err;
    }

    if (md_len <= len) {
      if (!EVP_DigestFinal_ex(ctx.get(), out, nullptr)) {
        goto err;
      }
      out += md_len;
      len -= md_len;
    } else {
      uint8_t digest[EVP_MAX_MD_SIZE];
      if (!EVP_DigestFinal_ex(ctx.get(), digest, nullptr)) {
        goto err;
      }
      OPENSSL_memcpy(out, digest, len);
      len = 0;
    }
  }

  ret = 1;

err:
  FIPS_service_indicator_unlock_state();
  return ret;
}

static const uint8_t kPSSZeroes[] = {0, 0, 0, 0, 0, 0, 0, 0};

int RSA_verify_PKCS1_PSS_mgf1(const RSA *rsa, const uint8_t *mHash,
                              const EVP_MD *Hash, const EVP_MD *mgf1Hash,
                              const uint8_t *EM, int sLen) {
  if (mgf1Hash == NULL) {
    mgf1Hash = Hash;
  }

  int ret = 0;
  uint8_t *DB = NULL;
  const uint8_t *H;
  bssl::ScopedEVP_MD_CTX ctx;
  unsigned MSBits;
  size_t emLen, maskedDBLen, salt_start;
  FIPS_service_indicator_lock_state();

  // Negative sLen has special meanings:
  //   -1      sLen == hLen
  //   -2      salt length is autorecovered from signature
  //   -N      reserved
  size_t hLen = EVP_MD_size(Hash);
  if (sLen == -1) {
    sLen = (int)hLen;
  } else if (sLen == -2) {
    sLen = -2;
  } else if (sLen < -2) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_SLEN_CHECK_FAILED);
    goto err;
  }

  MSBits = (BN_num_bits(rsa->n) - 1) & 0x7;
  emLen = RSA_size(rsa);
  if (EM[0] & (0xFF << MSBits)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_FIRST_OCTET_INVALID);
    goto err;
  }
  if (MSBits == 0) {
    EM++;
    emLen--;
  }
  // |sLen| may be -2 for the non-standard salt length recovery mode.
  if (emLen < hLen + 2 || (sLen >= 0 && emLen < hLen + (size_t)sLen + 2)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE);
    goto err;
  }
  if (EM[emLen - 1] != 0xbc) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_LAST_OCTET_INVALID);
    goto err;
  }
  maskedDBLen = emLen - hLen - 1;
  H = EM + maskedDBLen;
  DB = reinterpret_cast<uint8_t *>(OPENSSL_malloc(maskedDBLen));
  if (!DB) {
    goto err;
  }
  if (!PKCS1_MGF1(DB, maskedDBLen, H, hLen, mgf1Hash)) {
    goto err;
  }
  for (size_t i = 0; i < maskedDBLen; i++) {
    DB[i] ^= EM[i];
  }
  if (MSBits) {
    DB[0] &= 0xFF >> (8 - MSBits);
  }
  // This step differs slightly from EMSA-PSS-VERIFY (RFC 8017) step 10 because
  // it accepts a non-standard salt recovery flow. DB should be some number of
  // zeros, a one, then the salt.
  for (salt_start = 0; DB[salt_start] == 0 && salt_start < maskedDBLen - 1;
       salt_start++) {
    ;
  }
  if (DB[salt_start] != 0x1) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_SLEN_RECOVERY_FAILED);
    goto err;
  }
  salt_start++;
  // If a salt length was specified, check it matches.
  if (sLen >= 0 && maskedDBLen - salt_start != (size_t)sLen) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_SLEN_CHECK_FAILED);
    goto err;
  }
  uint8_t H_[EVP_MAX_MD_SIZE];
  if (!EVP_DigestInit_ex(ctx.get(), Hash, NULL) ||
      !EVP_DigestUpdate(ctx.get(), kPSSZeroes, sizeof(kPSSZeroes)) ||
      !EVP_DigestUpdate(ctx.get(), mHash, hLen) ||
      !EVP_DigestUpdate(ctx.get(), DB + salt_start, maskedDBLen - salt_start) ||
      !EVP_DigestFinal_ex(ctx.get(), H_, NULL)) {
    goto err;
  }
  if (OPENSSL_memcmp(H_, H, hLen) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_SIGNATURE);
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(DB);
  FIPS_service_indicator_unlock_state();
  return ret;
}

int RSA_padding_add_PKCS1_PSS_mgf1(const RSA *rsa, unsigned char *EM,
                                   const unsigned char *mHash,
                                   const EVP_MD *Hash, const EVP_MD *mgf1Hash,
                                   int sLenRequested) {
  int ret = 0;
  bssl::ScopedEVP_MD_CTX ctx;
  size_t maskedDBLen, MSBits, emLen;
  size_t hLen;
  unsigned char *H, *salt = NULL, *p;

  if (mgf1Hash == NULL) {
    mgf1Hash = Hash;
  }

  FIPS_service_indicator_lock_state();
  hLen = EVP_MD_size(Hash);

  if (BN_is_zero(rsa->n)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_EMPTY_PUBLIC_KEY);
    goto err;
  }

  MSBits = (BN_num_bits(rsa->n) - 1) & 0x7;
  emLen = RSA_size(rsa);
  if (MSBits == 0) {
    assert(emLen >= 1);
    *EM++ = 0;
    emLen--;
  }

  if (emLen < hLen + 2) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_KEY_SIZE);
    goto err;
  }

  // Negative sLenRequested has special meanings:
  //   -1  sLen == hLen
  //   -2  salt length is maximized
  //   -N  reserved
  size_t sLen;
  if (sLenRequested == -1) {
    sLen = hLen;
  } else if (sLenRequested == -2) {
    sLen = emLen - hLen - 2;
  } else if (sLenRequested < 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_SLEN_CHECK_FAILED);
    goto err;
  } else {
    sLen = (size_t)sLenRequested;
  }

  if (emLen - hLen - 2 < sLen) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_KEY_SIZE);
    goto err;
  }

  if (sLen > 0) {
    salt = reinterpret_cast<uint8_t *>(OPENSSL_malloc(sLen));
    if (!salt) {
      goto err;
    }
    BCM_rand_bytes(salt, sLen);
  }
  maskedDBLen = emLen - hLen - 1;
  H = EM + maskedDBLen;

  if (!EVP_DigestInit_ex(ctx.get(), Hash, NULL) ||
      !EVP_DigestUpdate(ctx.get(), kPSSZeroes, sizeof(kPSSZeroes)) ||
      !EVP_DigestUpdate(ctx.get(), mHash, hLen) ||
      !EVP_DigestUpdate(ctx.get(), salt, sLen) ||
      !EVP_DigestFinal_ex(ctx.get(), H, NULL)) {
    goto err;
  }

  // Generate dbMask in place then perform XOR on it
  if (!PKCS1_MGF1(EM, maskedDBLen, H, hLen, mgf1Hash)) {
    goto err;
  }

  p = EM;
  // Initial PS XORs with all zeroes which is a NOP so just update
  // pointer. Note from a test above this value is guaranteed to
  // be non-negative.
  p += emLen - sLen - hLen - 2;
  *p++ ^= 0x1;
  if (sLen > 0) {
    for (size_t i = 0; i < sLen; i++) {
      *p++ ^= salt[i];
    }
  }
  if (MSBits) {
    EM[0] &= 0xFF >> (8 - MSBits);
  }

  // H is already in place so just set final 0xbc

  EM[emLen - 1] = 0xbc;

  ret = 1;

err:
  OPENSSL_free(salt);
  FIPS_service_indicator_unlock_state();

  return ret;
}
