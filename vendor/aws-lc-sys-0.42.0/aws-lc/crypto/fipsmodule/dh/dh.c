// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/dh.h>
#include <limits.h>

#include <string.h>

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/digest.h>
#include <openssl/mem.h>
#include <openssl/thread.h>

#include "internal.h"
#include "../../internal.h"
#include "../bn/internal.h"


DH *DH_new(void) {
  DH *dh = OPENSSL_zalloc(sizeof(DH));
  if (dh == NULL) {
    return NULL;
  }

  CRYPTO_MUTEX_init(&dh->method_mont_p_lock);
  dh->references = 1;
  return dh;
}

DH *DH_new_by_nid(int nid) {
  switch (nid) {
    case NID_ffdhe2048:
      return DH_get_rfc7919_2048();
    case NID_ffdhe3072:
      return DH_get_rfc7919_3072();
    case NID_ffdhe4096:
      return DH_get_rfc7919_4096();
    case NID_ffdhe8192:
      return DH_get_rfc7919_8192();
    default:
      OPENSSL_PUT_ERROR(DH, DH_R_INVALID_NID);
      return NULL;
  }
}

void DH_free(DH *dh) {
  SET_DIT_AUTO_RESET;
  if (dh == NULL) {
    return;
  }

  if (!CRYPTO_refcount_dec_and_test_zero(&dh->references)) {
    return;
  }

  BN_MONT_CTX_free(dh->method_mont_p);
  BN_clear_free(dh->p);
  BN_clear_free(dh->g);
  BN_clear_free(dh->q);
  BN_clear_free(dh->pub_key);
  BN_clear_free(dh->priv_key);
  CRYPTO_MUTEX_cleanup(&dh->method_mont_p_lock);

  OPENSSL_free(dh);
}

unsigned DH_bits(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return BN_num_bits(dh->p);
}

const BIGNUM *DH_get0_pub_key(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return dh->pub_key;;
}

const BIGNUM *DH_get0_priv_key(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return dh->priv_key;
}

const BIGNUM *DH_get0_p(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return dh->p;
}

const BIGNUM *DH_get0_q(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return dh->q;
}

const BIGNUM *DH_get0_g(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return dh->g;
}

void DH_get0_key(const DH *dh, const BIGNUM **out_pub_key,
                 const BIGNUM **out_priv_key) {
  SET_DIT_AUTO_RESET;
  if (out_pub_key != NULL) {
    *out_pub_key = dh->pub_key;
  }
  if (out_priv_key != NULL) {
    *out_priv_key = dh->priv_key;
  }
}

void DH_clear_flags(DH *dh, int flags) {
  SET_DIT_AUTO_RESET;
  (void) dh;
  (void) flags;
}

int DH_set0_key(DH *dh, BIGNUM *pub_key, BIGNUM *priv_key) {
  SET_DIT_AUTO_RESET;
  if (pub_key != NULL) {
    BN_free(dh->pub_key);
    dh->pub_key = pub_key;
  }

  if (priv_key != NULL) {
    BN_free(dh->priv_key);
    dh->priv_key = priv_key;
  }

  return 1;
}

void DH_get0_pqg(const DH *dh, const BIGNUM **out_p, const BIGNUM **out_q,
                 const BIGNUM **out_g) {
  SET_DIT_AUTO_RESET;
  if (out_p != NULL) {
    *out_p = dh->p;
  }
  if (out_q != NULL) {
    *out_q = dh->q;
  }
  if (out_g != NULL) {
    *out_g = dh->g;
  }
}

int DH_set0_pqg(DH *dh, BIGNUM *p, BIGNUM *q, BIGNUM *g) {
  SET_DIT_AUTO_RESET;
  if ((dh->p == NULL && p == NULL) ||
      (dh->g == NULL && g == NULL)) {
    return 0;
  }

  if (p != NULL) {
    BN_free(dh->p);
    dh->p = p;
  }

  if (q != NULL) {
    BN_free(dh->q);
    dh->q = q;
  }

  if (g != NULL) {
    BN_free(dh->g);
    dh->g = g;
  }

  // Invalidate the cached Montgomery parameters.
  BN_MONT_CTX_free(dh->method_mont_p);
  dh->method_mont_p = NULL;
  return 1;
}

int DH_set_length(DH *dh, unsigned priv_length) {
  SET_DIT_AUTO_RESET;
  dh->priv_length = priv_length;
  return 1;
}

int DH_generate_key(DH *dh) {
  SET_DIT_AUTO_RESET;
  boringssl_ensure_ffdh_self_test();

  if (!dh_check_params_fast(dh)) {
    return 0;
  }

  int ok = 0;
  int generate_new_key = 0;
  BN_CTX *ctx = NULL;
  BIGNUM *pub_key = NULL, *priv_key = NULL, *priv_key_limit = NULL;

  ctx = BN_CTX_new();
  if (ctx == NULL) {
    goto err;
  }

  if (dh->priv_key == NULL) {
    priv_key = BN_new();
    if (priv_key == NULL) {
      goto err;
    }
    generate_new_key = 1;
  } else {
    priv_key = dh->priv_key;
  }

  if (dh->pub_key == NULL) {
    pub_key = BN_new();
    if (pub_key == NULL) {
      goto err;
    }
  } else {
    pub_key = dh->pub_key;
  }

  if (!BN_MONT_CTX_set_locked(&dh->method_mont_p, &dh->method_mont_p_lock,
                              dh->p, ctx)) {
    goto err;
  }

  if (generate_new_key) {
    if (dh->q) {
      // Section 5.6.1.1.4 of SP 800-56A Rev3 generates a private key uniformly
      // from [1, min(2^N-1, q-1)].
      //
      // Although SP 800-56A Rev3 now permits a private key length N,
      // |dh->priv_length| historically was ignored when q is available. We
      // continue to ignore it and interpret such a configuration as N = len(q).
      if (!BN_rand_range_ex(priv_key, 1, dh->q)) {
        goto err;
      }
    } else {
      // If q is unspecified, we expect p to be a safe prime, with g generating
      // the (p-1)/2 subgroup. So, we use q = (p-1)/2. (If g generates a smaller
      // prime-order subgroup, q will still divide (p-1)/2.)
      //
      // We set N from |dh->priv_length|. Section 5.6.1.1.4 of SP 800-56A Rev3
      // says to reject N > len(q), or N > num_bits(p) - 1. However, this logic
      // originally aligned with PKCS#3, which allows num_bits(p). Instead, we
      // clamp |dh->priv_length| before invoking the algorithm.

      // Compute M = min(2^N, q).
      priv_key_limit = BN_new();
      if (priv_key_limit == NULL) {
        goto err;
      }
      if (dh->priv_length == 0 || dh->priv_length >= BN_num_bits(dh->p) - 1) {
        // M = q = (p - 1) / 2.
        if (!BN_rshift1(priv_key_limit, dh->p)) {
          goto err;
        }
      } else {
        // M = 2^N.
        if (!BN_set_bit(priv_key_limit, dh->priv_length)) {
          goto err;
        }
      }

      // Choose a private key uniformly from [1, M-1].
      if (!BN_rand_range_ex(priv_key, 1, priv_key_limit)) {
        goto err;
      }
    }
  }

  if (!BN_mod_exp_mont_consttime(pub_key, dh->g, priv_key, dh->p, ctx,
                                 dh->method_mont_p)) {
    goto err;
  }

  dh->pub_key = pub_key;
  dh->priv_key = priv_key;
  ok = 1;

err:
  if (ok != 1) {
    OPENSSL_PUT_ERROR(DH, ERR_R_BN_LIB);
  }

  if (dh->pub_key == NULL) {
    BN_free(pub_key);
  }
  if (dh->priv_key == NULL) {
    BN_free(priv_key);
  }
  BN_free(priv_key_limit);
  BN_CTX_free(ctx);
  return ok;
}

static int dh_compute_key(DH *dh, BIGNUM *out_shared_key,
                          const BIGNUM *peers_key, BN_CTX *ctx) {
  if (!dh_check_params_fast(dh)) {
    return 0;
  }

  if (dh->priv_key == NULL) {
    OPENSSL_PUT_ERROR(DH, DH_R_NO_PRIVATE_VALUE);
    return 0;
  }

  int check_result;
  if (!DH_check_pub_key(dh, peers_key, &check_result) || check_result) {
    OPENSSL_PUT_ERROR(DH, DH_R_INVALID_PUBKEY);
    return 0;
  }

  int ret = 0;
  BN_CTX_start(ctx);
  BIGNUM *p_minus_1 = BN_CTX_get(ctx);

  if (!p_minus_1 ||
      !BN_MONT_CTX_set_locked(&dh->method_mont_p, &dh->method_mont_p_lock,
                              dh->p, ctx)) {
    goto err;
  }

  if (!BN_mod_exp_mont_consttime(out_shared_key, peers_key, dh->priv_key, dh->p,
                                 ctx, dh->method_mont_p) ||
      !BN_copy(p_minus_1, dh->p) ||
      !BN_sub_word(p_minus_1, 1)) {
    OPENSSL_PUT_ERROR(DH, ERR_R_BN_LIB);
    goto err;
  }

  // This performs the check required by SP 800-56Ar3 section 5.7.1.1 step two.
  if (BN_cmp_word(out_shared_key, 1) <= 0 ||
      BN_cmp(out_shared_key, p_minus_1) == 0) {
    OPENSSL_PUT_ERROR(DH, DH_R_INVALID_PUBKEY);
    goto err;
  }

  ret = 1;

 err:
  BN_CTX_end(ctx);
  return ret;
}

int dh_compute_key_padded_no_self_test(unsigned char *out,
                                       const BIGNUM *peers_key, DH *dh) {
  BN_CTX *ctx = BN_CTX_new();
  if (ctx == NULL) {
    return -1;
  }
  BN_CTX_start(ctx);

  int dh_size = DH_size(dh);
  int ret = -1;
  BIGNUM *shared_key = BN_CTX_get(ctx);
  if (shared_key &&
      dh_compute_key(dh, shared_key, peers_key, ctx) &&
      BN_bn2bin_padded(out, dh_size, shared_key)) {
    ret = dh_size;
  }

  BN_CTX_end(ctx);
  BN_CTX_free(ctx);
  return ret;
}

int DH_compute_key_padded(unsigned char *out, const BIGNUM *peers_key, DH *dh) {
  boringssl_ensure_ffdh_self_test();
  SET_DIT_AUTO_RESET;

  return dh_compute_key_padded_no_self_test(out, peers_key, dh);
}

int DH_compute_key(unsigned char *out, const BIGNUM *peers_key, DH *dh) {
  boringssl_ensure_ffdh_self_test();
  SET_DIT_AUTO_RESET;

  BN_CTX *ctx = BN_CTX_new();
  if (ctx == NULL) {
    return -1;
  }
  BN_CTX_start(ctx);

  int ret = -1;
  BIGNUM *shared_key = BN_CTX_get(ctx);
  if (shared_key && dh_compute_key(dh, shared_key, peers_key, ctx)) {
    // A |BIGNUM|'s byte count fits in |int|.
    ret = (int)BN_bn2bin(shared_key, out);
  }

  BN_CTX_end(ctx);
  BN_CTX_free(ctx);
  return ret;
}

int DH_compute_key_hashed(DH *dh, uint8_t *out, size_t *out_len,
                          size_t max_out_len, const BIGNUM *peers_key,
                          const EVP_MD *digest) {
  SET_DIT_AUTO_RESET;

  *out_len = SIZE_MAX;

  const size_t digest_len = EVP_MD_size(digest);
  if ((EVP_MD_flags(digest) & EVP_MD_FLAG_XOF) || digest_len > max_out_len
#if SIZE_MAX > UINT_MAX
      || digest_len > UINT_MAX
#endif
  ) {
    return 0;
  }

  // We have to avoid the underlying |EVP_Digest| services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  int ret = 0;
  const size_t dh_len = DH_size(dh);
  uint8_t *shared_bytes = OPENSSL_malloc(dh_len);
  unsigned out_len_unsigned = (unsigned)digest_len;
  if (!shared_bytes ||
      // SP 800-56A is ambiguous about whether the output should be padded prior
      // to revision three. But revision three, section C.1, awkwardly specifies
      // padding to the length of p.
      //
      // Also, padded output avoids side-channels, so is always strongly
      // advisable.
      DH_compute_key_padded(shared_bytes, peers_key, dh) != (int)dh_len ||
      !EVP_Digest(shared_bytes, dh_len, out, &out_len_unsigned, digest, NULL) ||
      out_len_unsigned != digest_len) {
    goto err;
  }

  *out_len = digest_len;
  ret = 1;

 err:
  FIPS_service_indicator_unlock_state();
  OPENSSL_free(shared_bytes);
  return ret;
}

int DH_size(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return BN_num_bytes(dh->p);
}

unsigned DH_num_bits(const DH *dh) {
  SET_DIT_AUTO_RESET;
  return BN_num_bits(dh->p);
}

int DH_up_ref(DH *dh) {
  SET_DIT_AUTO_RESET;
  CRYPTO_refcount_inc(&dh->references);
  return 1;
}

// All the groups in RFC 7919 are of the form:
// q = (p-1)/2
// g = 2
static DH *calculate_rfc7919_DH_from_p(const BN_ULONG data[], size_t data_len) {
  BIGNUM *const ffdhe_p = BN_new();
  BIGNUM *const ffdhe_q = BN_new();
  BIGNUM *const ffdhe_g = BN_new();
  DH *const dh = DH_new();

  if (!ffdhe_p || !ffdhe_q || !ffdhe_g || !dh) {
    goto err;
  }

  bn_set_static_words(ffdhe_p, data, data_len);

  if (!BN_rshift1(ffdhe_q, ffdhe_p) ||
      !BN_set_word(ffdhe_g, 2) ||
      !DH_set0_pqg(dh, ffdhe_p, ffdhe_q, ffdhe_g)) {
    goto err;
  }

  return dh;

err:
  BN_free(ffdhe_p);
  BN_free(ffdhe_q);
  BN_free(ffdhe_g);
  DH_free(dh);
  return NULL;

}

DH *DH_get_rfc7919_2048(void) {
  // This is the prime from https://tools.ietf.org/html/rfc7919#appendix-A.1,
  // which is specifically approved for FIPS in appendix D of SP 800-56Ar3.
  static const BN_ULONG kFFDHE2048Data[] = {
      TOBN(0xffffffff, 0xffffffff), TOBN(0x886b4238, 0x61285c97),
      TOBN(0xc6f34a26, 0xc1b2effa), TOBN(0xc58ef183, 0x7d1683b2),
      TOBN(0x3bb5fcbc, 0x2ec22005), TOBN(0xc3fe3b1b, 0x4c6fad73),
      TOBN(0x8e4f1232, 0xeef28183), TOBN(0x9172fe9c, 0xe98583ff),
      TOBN(0xc03404cd, 0x28342f61), TOBN(0x9e02fce1, 0xcdf7e2ec),
      TOBN(0x0b07a7c8, 0xee0a6d70), TOBN(0xae56ede7, 0x6372bb19),
      TOBN(0x1d4f42a3, 0xde394df4), TOBN(0xb96adab7, 0x60d7f468),
      TOBN(0xd108a94b, 0xb2c8e3fb), TOBN(0xbc0ab182, 0xb324fb61),
      TOBN(0x30acca4f, 0x483a797a), TOBN(0x1df158a1, 0x36ade735),
      TOBN(0xe2a689da, 0xf3efe872), TOBN(0x984f0c70, 0xe0e68b77),
      TOBN(0xb557135e, 0x7f57c935), TOBN(0x85636555, 0x3ded1af3),
      TOBN(0x2433f51f, 0x5f066ed0), TOBN(0xd3df1ed5, 0xd5fd6561),
      TOBN(0xf681b202, 0xaec4617a), TOBN(0x7d2fe363, 0x630c75d8),
      TOBN(0xcc939dce, 0x249b3ef9), TOBN(0xa9e13641, 0x146433fb),
      TOBN(0xd8b9c583, 0xce2d3695), TOBN(0xafdc5620, 0x273d3cf1),
      TOBN(0xadf85458, 0xa2bb4a9a), TOBN(0xffffffff, 0xffffffff),
  };

  return calculate_rfc7919_DH_from_p(kFFDHE2048Data, OPENSSL_ARRAY_SIZE(kFFDHE2048Data));
}

DH *DH_get_rfc7919_3072(void) {
  // This is the prime from https://tools.ietf.org/html/rfc7919#appendix-A.2,
  // which is specifically approved for FIPS in appendix D of SP 800-56Ar3.
  static const BN_ULONG kFFDHE3072Data[] = {
    TOBN(0xffffffff, 0xffffffff), TOBN(0x25e41d2b, 0x66c62e37),
    TOBN(0x3c1b20ee, 0x3fd59d7c), TOBN(0x0abcd06b, 0xfa53ddef),
    TOBN(0x1dbf9a42, 0xd5c4484e), TOBN(0xabc52197, 0x9b0deada),
    TOBN(0xe86d2bc5, 0x22363a0d), TOBN(0x5cae82ab, 0x9c9df69e),
    TOBN(0x64f2e21e, 0x71f54bff), TOBN(0xf4fd4452, 0xe2d74dd3),
    TOBN(0xb4130c93, 0xbc437944), TOBN(0xaefe1309, 0x85139270),
    TOBN(0x598cb0fa, 0xc186d91c), TOBN(0x7ad91d26, 0x91f7f7ee),
    TOBN(0x61b46fc9, 0xd6e6c907), TOBN(0xbc34f4de, 0xf99c0238),
    TOBN(0xde355b3b, 0x6519035b), TOBN(0x886b4238, 0x611fcfdc),
    TOBN(0xc6f34a26, 0xc1b2effa), TOBN(0xc58ef183, 0x7d1683b2),
    TOBN(0x3bb5fcbc, 0x2ec22005), TOBN(0xc3fe3b1b, 0x4c6fad73),
    TOBN(0x8e4f1232, 0xeef28183), TOBN(0x9172fe9c, 0xe98583ff),
    TOBN(0xc03404cd, 0x28342f61), TOBN(0x9e02fce1, 0xcdf7e2ec),
    TOBN(0x0b07a7c8, 0xee0a6d70), TOBN(0xae56ede7, 0x6372bb19),
    TOBN(0x1d4f42a3, 0xde394df4), TOBN(0xb96adab7, 0x60d7f468),
    TOBN(0xd108a94b, 0xb2c8e3fb), TOBN(0xbc0ab182, 0xb324fb61),
    TOBN(0x30acca4f, 0x483a797a), TOBN(0x1df158a1, 0x36ade735),
    TOBN(0xe2a689da, 0xf3efe872), TOBN(0x984f0c70, 0xe0e68b77),
    TOBN(0xb557135e, 0x7f57c935), TOBN(0x85636555, 0x3ded1af3),
    TOBN(0x2433f51f, 0x5f066ed0), TOBN(0xd3df1ed5, 0xd5fd6561),
    TOBN(0xf681b202, 0xaec4617a), TOBN(0x7d2fe363, 0x630c75d8),
    TOBN(0xcc939dce, 0x249b3ef9), TOBN(0xa9e13641, 0x146433fb),
    TOBN(0xd8b9c583, 0xce2d3695), TOBN(0xafdc5620, 0x273d3cf1),
    TOBN(0xadf85458, 0xa2bb4a9a), TOBN(0xffffffff, 0xffffffff)};

  return calculate_rfc7919_DH_from_p(kFFDHE3072Data,
                                     OPENSSL_ARRAY_SIZE(kFFDHE3072Data));
}

DH *DH_get_rfc7919_4096(void) {
    // This is the prime from https://tools.ietf.org/html/rfc7919#appendix-A.3,
    // which is specifically approved for FIPS in appendix D of SP 800-56Ar3.
    static const BN_ULONG kFFDHE4096Data[] = {
        TOBN(0xFFFFFFFF, 0xFFFFFFFF),TOBN(0xC68A007E, 0x5E655F6A),
        TOBN(0x4DB5A851, 0xF44182E1),TOBN(0x8EC9B55A, 0x7F88A46B),
        TOBN(0x0A8291CD, 0xCEC97DCF),TOBN(0x2A4ECEA9, 0xF98D0ACC),
        TOBN(0x1A1DB93D, 0x7140003C),TOBN(0x092999A3, 0x33CB8B7A),
        TOBN(0x6DC778F9, 0x71AD0038),TOBN(0xA907600A, 0x918130C4),
        TOBN(0xED6A1E01, 0x2D9E6832),TOBN(0x7135C886, 0xEFB4318A),
        TOBN(0x87F55BA5, 0x7E31CC7A),TOBN(0x7763CF1D, 0x55034004),
        TOBN(0xAC7D5F42, 0xD69F6D18),TOBN(0x7930E9E4, 0xE58857B6),
        TOBN(0x6E6F52C3, 0x164DF4FB),TOBN(0x25E41D2B, 0x669E1EF1),
        TOBN(0x3C1B20EE, 0x3FD59D7C),TOBN(0x0ABCD06B, 0xFA53DDEF),
        TOBN(0x1DBF9A42, 0xD5C4484E),TOBN(0xABC52197, 0x9B0DEADA),
        TOBN(0xE86D2BC5, 0x22363A0D),TOBN(0x5CAE82AB, 0x9C9DF69E),
        TOBN(0x64F2E21E, 0x71F54BFF),TOBN(0xF4FD4452, 0xE2D74DD3),
        TOBN(0xB4130C93, 0xBC437944),TOBN(0xAEFE1309, 0x85139270),
        TOBN(0x598CB0FA, 0xC186D91C),TOBN(0x7AD91D26, 0x91F7F7EE),
        TOBN(0x61B46FC9, 0xD6E6C907),TOBN(0xBC34F4DE, 0xF99C0238),
        TOBN(0xDE355B3B, 0x6519035B),TOBN(0x886B4238, 0x611FCFDC),
        TOBN(0xC6F34A26, 0xC1B2EFFA),TOBN(0xC58EF183, 0x7D1683B2),
        TOBN(0x3BB5FCBC, 0x2EC22005),TOBN(0xC3FE3B1B, 0x4C6FAD73),
        TOBN(0x8E4F1232, 0xEEF28183),TOBN(0x9172FE9C, 0xE98583FF),
        TOBN(0xC03404CD, 0x28342F61),TOBN(0x9E02FCE1, 0xCDF7E2EC),
        TOBN(0x0B07A7C8, 0xEE0A6D70),TOBN(0xAE56EDE7, 0x6372BB19),
        TOBN(0x1D4F42A3, 0xDE394DF4),TOBN(0xB96ADAB7, 0x60D7F468),
        TOBN(0xD108A94B, 0xB2C8E3FB),TOBN(0xBC0AB182, 0xB324FB61),
        TOBN(0x30ACCA4F, 0x483A797A),TOBN(0x1DF158A1, 0x36ADE735),
        TOBN(0xE2A689DA, 0xF3EFE872),TOBN(0x984F0C70, 0xE0E68B77),
        TOBN(0xB557135E, 0x7F57C935),TOBN(0x85636555, 0x3DED1AF3),
        TOBN(0x2433F51F, 0x5F066ED0),TOBN(0xD3DF1ED5, 0xD5FD6561),
        TOBN(0xF681B202, 0xAEC4617A),TOBN(0x7D2FE363, 0x630C75D8),
        TOBN(0xCC939DCE, 0x249B3EF9),TOBN(0xA9E13641, 0x146433FB),
        TOBN(0xD8B9C583, 0xCE2D3695),TOBN(0xAFDC5620, 0x273D3CF1),
        TOBN(0xADF85458, 0xA2BB4A9A),TOBN(0xFFFFFFFF, 0xFFFFFFFF)
    };

    return calculate_rfc7919_DH_from_p(kFFDHE4096Data, OPENSSL_ARRAY_SIZE(kFFDHE4096Data));
}

DH *DH_get_rfc7919_8192(void) {
  // This is the prime from https://tools.ietf.org/html/rfc7919#appendix-A.4,
  // which is specifically approved for FIPS in appendix D of SP 800-56Ar3.
  static const BN_ULONG kFFDHE8192Data[] = {
      TOBN(0xffffffff, 0xffffffff), TOBN(0xd68c8bb7, 0xc5c6424c),
      TOBN(0x011e2a94, 0x838ff88c), TOBN(0x0822e506, 0xa9f4614e),
      TOBN(0x97d11d49, 0xf7a8443d), TOBN(0xa6bbfde5, 0x30677f0d),
      TOBN(0x2f741ef8, 0xc1fe86fe), TOBN(0xfafabe1c, 0x5d71a87e),
      TOBN(0xded2fbab, 0xfbe58a30), TOBN(0xb6855dfe, 0x72b0a66e),
      TOBN(0x1efc8ce0, 0xba8a4fe8), TOBN(0x83f81d4a, 0x3f2fa457),
      TOBN(0xa1fe3075, 0xa577e231), TOBN(0xd5b80194, 0x88d9c0a0),
      TOBN(0x624816cd, 0xad9a95f9), TOBN(0x99e9e316, 0x50c1217b),
      TOBN(0x51aa691e, 0x0e423cfc), TOBN(0x1c217e6c, 0x3826e52c),
      TOBN(0x51a8a931, 0x09703fee), TOBN(0xbb709987, 0x6a460e74),
      TOBN(0x541fc68c, 0x9c86b022), TOBN(0x59160cc0, 0x46fd8251),
      TOBN(0x2846c0ba, 0x35c35f5c), TOBN(0x54504ac7, 0x8b758282),
      TOBN(0x29388839, 0xd2af05e4), TOBN(0xcb2c0f1c, 0xc01bd702),
      TOBN(0x555b2f74, 0x7c932665), TOBN(0x86b63142, 0xa3ab8829),
      TOBN(0x0b8cc3bd, 0xf64b10ef), TOBN(0x687feb69, 0xedd1cc5e),
      TOBN(0xfdb23fce, 0xc9509d43), TOBN(0x1e425a31, 0xd951ae64),
      TOBN(0x36ad004c, 0xf600c838), TOBN(0xa40e329c, 0xcff46aaa),
      TOBN(0xa41d570d, 0x7938dad4), TOBN(0x62a69526, 0xd43161c1),
      TOBN(0x3fdd4a8e, 0x9adb1e69), TOBN(0x5b3b71f9, 0xdc6b80d6),
      TOBN(0xec9d1810, 0xc6272b04), TOBN(0x8ccf2dd5, 0xcacef403),
      TOBN(0xe49f5235, 0xc95b9117), TOBN(0x505dc82d, 0xb854338a),
      TOBN(0x62292c31, 0x1562a846), TOBN(0xd72b0374, 0x6ae77f5e),
      TOBN(0xf9c9091b, 0x462d538c), TOBN(0x0ae8db58, 0x47a67cbe),
      TOBN(0xb3a739c1, 0x22611682), TOBN(0xeeaac023, 0x2a281bf6),
      TOBN(0x94c6651e, 0x77caf992), TOBN(0x763e4e4b, 0x94b2bbc1),
      TOBN(0x587e38da, 0x0077d9b4), TOBN(0x7fb29f8c, 0x183023c3),
      TOBN(0x0abec1ff, 0xf9e3a26e), TOBN(0xa00ef092, 0x350511e3),
      TOBN(0xb855322e, 0xdb6340d8), TOBN(0xa52471f7, 0xa9a96910),
      TOBN(0x388147fb, 0x4cfdb477), TOBN(0x9b1f5c3e, 0x4e46041f),
      TOBN(0xcdad0657, 0xfccfec71), TOBN(0xb38e8c33, 0x4c701c3a),
      TOBN(0x917bdd64, 0xb1c0fd4c), TOBN(0x3bb45432, 0x9b7624c8),
      TOBN(0x23ba4442, 0xcaf53ea6), TOBN(0x4e677d2c, 0x38532a3a),
      TOBN(0x0bfd64b6, 0x45036c7a), TOBN(0xc68a007e, 0x5e0dd902),
      TOBN(0x4db5a851, 0xf44182e1), TOBN(0x8ec9b55a, 0x7f88a46b),
      TOBN(0x0a8291cd, 0xcec97dcf), TOBN(0x2a4ecea9, 0xf98d0acc),
      TOBN(0x1a1db93d, 0x7140003c), TOBN(0x092999a3, 0x33cb8b7a),
      TOBN(0x6dc778f9, 0x71ad0038), TOBN(0xa907600a, 0x918130c4),
      TOBN(0xed6a1e01, 0x2d9e6832), TOBN(0x7135c886, 0xefb4318a),
      TOBN(0x87f55ba5, 0x7e31cc7a), TOBN(0x7763cf1d, 0x55034004),
      TOBN(0xac7d5f42, 0xd69f6d18), TOBN(0x7930e9e4, 0xe58857b6),
      TOBN(0x6e6f52c3, 0x164df4fb), TOBN(0x25e41d2b, 0x669e1ef1),
      TOBN(0x3c1b20ee, 0x3fd59d7c), TOBN(0x0abcd06b, 0xfa53ddef),
      TOBN(0x1dbf9a42, 0xd5c4484e), TOBN(0xabc52197, 0x9b0deada),
      TOBN(0xe86d2bc5, 0x22363a0d), TOBN(0x5cae82ab, 0x9c9df69e),
      TOBN(0x64f2e21e, 0x71f54bff), TOBN(0xf4fd4452, 0xe2d74dd3),
      TOBN(0xb4130c93, 0xbc437944), TOBN(0xaefe1309, 0x85139270),
      TOBN(0x598cb0fa, 0xc186d91c), TOBN(0x7ad91d26, 0x91f7f7ee),
      TOBN(0x61b46fc9, 0xd6e6c907), TOBN(0xbc34f4de, 0xf99c0238),
      TOBN(0xde355b3b, 0x6519035b), TOBN(0x886b4238, 0x611fcfdc),
      TOBN(0xc6f34a26, 0xc1b2effa), TOBN(0xc58ef183, 0x7d1683b2),
      TOBN(0x3bb5fcbc, 0x2ec22005), TOBN(0xc3fe3b1b, 0x4c6fad73),
      TOBN(0x8e4f1232, 0xeef28183), TOBN(0x9172fe9c, 0xe98583ff),
      TOBN(0xc03404cd, 0x28342f61), TOBN(0x9e02fce1, 0xcdf7e2ec),
      TOBN(0x0b07a7c8, 0xee0a6d70), TOBN(0xae56ede7, 0x6372bb19),
      TOBN(0x1d4f42a3, 0xde394df4), TOBN(0xb96adab7, 0x60d7f468),
      TOBN(0xd108a94b, 0xb2c8e3fb), TOBN(0xbc0ab182, 0xb324fb61),
      TOBN(0x30acca4f, 0x483a797a), TOBN(0x1df158a1, 0x36ade735),
      TOBN(0xe2a689da, 0xf3efe872), TOBN(0x984f0c70, 0xe0e68b77),
      TOBN(0xb557135e, 0x7f57c935), TOBN(0x85636555, 0x3ded1af3),
      TOBN(0x2433f51f, 0x5f066ed0), TOBN(0xd3df1ed5, 0xd5fd6561),
      TOBN(0xf681b202, 0xaec4617a), TOBN(0x7d2fe363, 0x630c75d8),
      TOBN(0xcc939dce, 0x249b3ef9), TOBN(0xa9e13641, 0x146433fb),
      TOBN(0xd8b9c583, 0xce2d3695), TOBN(0xafdc5620, 0x273d3cf1),
      TOBN(0xadf85458, 0xa2bb4a9a), TOBN(0xffffffff, 0xffffffff)};

  return calculate_rfc7919_DH_from_p(kFFDHE8192Data,
                                     OPENSSL_ARRAY_SIZE(kFFDHE8192Data));
}
