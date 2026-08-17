// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/trust_token.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/rand.h>
#include <openssl/sha.h>

#include "../ec_extra/internal.h"
#include "../fipsmodule/bn/internal.h"
#include "../fipsmodule/ec/internal.h"

#include "internal.h"


typedef int (*hash_t_func_t)(const EC_GROUP *group, EC_JACOBIAN *out,
                             const uint8_t t[TRUST_TOKEN_NONCE_SIZE]);
typedef int (*hash_s_func_t)(const EC_GROUP *group, EC_JACOBIAN *out,
                             const EC_AFFINE *t,
                             const uint8_t s[TRUST_TOKEN_NONCE_SIZE]);
typedef int (*hash_c_func_t)(const EC_GROUP *group, EC_SCALAR *out,
                             uint8_t *buf, size_t len);
typedef int (*hash_to_scalar_func_t)(const EC_GROUP *group, EC_SCALAR *out,
                                     uint8_t *buf, size_t len);

typedef struct {
  const EC_GROUP *group;
  EC_PRECOMP g_precomp;
  EC_PRECOMP h_precomp;
  EC_JACOBIAN h;
  // hash_t implements the H_t operation in PMBTokens. It returns one on success
  // and zero on error.
  hash_t_func_t hash_t;
  // hash_s implements the H_s operation in PMBTokens. It returns one on success
  // and zero on error.
  hash_s_func_t hash_s;
  // hash_c implements the H_c operation in PMBTokens. It returns one on success
  // and zero on error.
  hash_c_func_t hash_c;
  // hash_to_scalar implements the HashToScalar operation for PMBTokens. It
  // returns one on success and zero on error.
  hash_to_scalar_func_t hash_to_scalar;
  int prefix_point : 1;
} PMBTOKEN_METHOD;

static const uint8_t kDefaultAdditionalData[32] = {0};

static int pmbtoken_init_method(PMBTOKEN_METHOD *method, const EC_GROUP *group,
                                const uint8_t *h_bytes, size_t h_len,
                                hash_t_func_t hash_t, hash_s_func_t hash_s,
                                hash_c_func_t hash_c,
                                hash_to_scalar_func_t hash_to_scalar,
                                int prefix_point) {
  method->group = group;
  method->hash_t = hash_t;
  method->hash_s = hash_s;
  method->hash_c = hash_c;
  method->hash_to_scalar = hash_to_scalar;
  method->prefix_point = prefix_point;

  EC_AFFINE h;
  if (!ec_point_from_uncompressed(method->group, &h, h_bytes, h_len)) {
    return 0;
  }
  ec_affine_to_jacobian(method->group, &method->h, &h);

  if (!ec_init_precomp(method->group, &method->g_precomp,
                       &method->group->generator.raw) ||
      !ec_init_precomp(method->group, &method->h_precomp, &method->h)) {
    return 0;
  }
  return 1;
}

static int derive_scalar_from_secret(const PMBTOKEN_METHOD *method,
                                     EC_SCALAR *out, const uint8_t *secret,
                                     size_t secret_len, uint8_t scalar_id) {
  static const uint8_t kKeygenLabel[] = "TrustTokenPMBTokenKeyGen";

  int ok = 0;
  CBB cbb;
  CBB_zero(&cbb);
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !CBB_add_bytes(&cbb, kKeygenLabel, sizeof(kKeygenLabel)) ||
      !CBB_add_u8(&cbb, scalar_id) ||
      !CBB_add_bytes(&cbb, secret, secret_len) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !method->hash_to_scalar(method->group, out, buf, len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    goto err;
  }

  ok = 1;

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

static int point_to_cbb(CBB *out, const EC_GROUP *group,
                        const EC_AFFINE *point) {
  size_t len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
  if (len == 0) {
    return 0;
  }
  uint8_t *p;
  return CBB_add_space(out, &p, len) &&
         ec_point_to_bytes(group, point, POINT_CONVERSION_UNCOMPRESSED, p,
                           len) == len;
}

static int cbb_add_prefixed_point(CBB *out, const EC_GROUP *group,
                                  const EC_AFFINE *point, int prefix_point) {
  if (prefix_point) {
    CBB child;
    if (!CBB_add_u16_length_prefixed(out, &child) ||
        !point_to_cbb(&child, group, point) ||
        !CBB_flush(out)) {
      return 0;
    }
  } else {
    if (!point_to_cbb(out, group, point) ||
        !CBB_flush(out)) {
      return 0;
    }
  }

  return 1;
}

static int cbs_get_prefixed_point(CBS *cbs, const EC_GROUP *group,
                                  EC_AFFINE *out, int prefix_point) {
  CBS child;
  if (prefix_point) {
    if (!CBS_get_u16_length_prefixed(cbs, &child)) {
      return 0;
    }
  } else {
    size_t plen = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
    if (!CBS_get_bytes(cbs, &child, plen)) {
      return 0;
    }
  }

  if (!ec_point_from_uncompressed(group, out, CBS_data(&child),
                                  CBS_len(&child))) {
    return 0;
  }
  return 1;
}

static int mul_public_3(const EC_GROUP *group, EC_JACOBIAN *out,
                        const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                        const EC_JACOBIAN *p1, const EC_SCALAR *scalar1,
                        const EC_JACOBIAN *p2, const EC_SCALAR *scalar2) {
  EC_JACOBIAN points[3] = {*p0, *p1, *p2};
  EC_SCALAR scalars[3] = {*scalar0, *scalar1, *scalar2};
  return ec_point_mul_scalar_public_batch(group, out, /*g_scalar=*/NULL, points,
                                          scalars, 3);
}

static int pmbtoken_compute_keys(const PMBTOKEN_METHOD *method,
                                 CBB *out_private, CBB *out_public,
                                 const EC_SCALAR *x0, const EC_SCALAR *y0,
                                 const EC_SCALAR *x1, const EC_SCALAR *y1,
                                 const EC_SCALAR *xs, const EC_SCALAR *ys) {
  const EC_GROUP *group = method->group;
  EC_JACOBIAN pub[3];
  if (!ec_point_mul_scalar_precomp(group, &pub[0], &method->g_precomp,
                                   x0, &method->h_precomp, y0, NULL, NULL) ||
      !ec_point_mul_scalar_precomp(group, &pub[1], &method->g_precomp,
                                   x1, &method->h_precomp, y1, NULL, NULL) ||
      !ec_point_mul_scalar_precomp(method->group, &pub[2], &method->g_precomp,
                                   xs, &method->h_precomp, ys, NULL, NULL)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    return 0;
  }

  const EC_SCALAR *scalars[] = {x0, y0, x1, y1, xs, ys};
  size_t scalar_len = BN_num_bytes(EC_GROUP_get0_order(group));
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(scalars); i++) {
    uint8_t *buf;
    if (!CBB_add_space(out_private, &buf, scalar_len)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
      return 0;
    }
    ec_scalar_to_bytes(group, buf, &scalar_len, scalars[i]);
  }

  EC_AFFINE pub_affine[3];
  if (!ec_jacobian_to_affine_batch(group, pub_affine, pub, 3)) {
    return 0;
  }

  if (!cbb_add_prefixed_point(out_public, group, &pub_affine[0],
                              method->prefix_point) ||
      !cbb_add_prefixed_point(out_public, group, &pub_affine[1],
                              method->prefix_point) ||
      !cbb_add_prefixed_point(out_public, group, &pub_affine[2],
                              method->prefix_point)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
    return 0;
  }

  return 1;
}

static int pmbtoken_generate_key(const PMBTOKEN_METHOD *method,
                                 CBB *out_private, CBB *out_public) {
  EC_SCALAR x0, y0, x1, y1, xs, ys;
  if (!ec_random_nonzero_scalar(method->group, &x0, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(method->group, &y0, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(method->group, &x1, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(method->group, &y1, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(method->group, &xs, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(method->group, &ys, kDefaultAdditionalData)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    return 0;
  }

  return pmbtoken_compute_keys(method, out_private, out_public, &x0, &y0, &x1,
                               &y1, &xs, &ys);
}

static int pmbtoken_derive_key_from_secret(const PMBTOKEN_METHOD *method,
                                           CBB *out_private, CBB *out_public,
                                           const uint8_t *secret,
                                           size_t secret_len) {
  EC_SCALAR x0, y0, x1, y1, xs, ys;
  if (!derive_scalar_from_secret(method, &x0, secret, secret_len, 0) ||
      !derive_scalar_from_secret(method, &y0, secret, secret_len, 1) ||
      !derive_scalar_from_secret(method, &x1, secret, secret_len, 2) ||
      !derive_scalar_from_secret(method, &y1, secret, secret_len, 3) ||
      !derive_scalar_from_secret(method, &xs, secret, secret_len, 4) ||
      !derive_scalar_from_secret(method, &ys, secret, secret_len, 5)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    return 0;
  }

  return pmbtoken_compute_keys(method, out_private, out_public, &x0, &y0, &x1,
                               &y1, &xs, &ys);
}

static int pmbtoken_client_key_from_bytes(const PMBTOKEN_METHOD *method,
                                          TRUST_TOKEN_CLIENT_KEY *key,
                                          const uint8_t *in, size_t len) {
  CBS cbs;
  CBS_init(&cbs, in, len);
  if (!cbs_get_prefixed_point(&cbs, method->group, &key->pub0,
                              method->prefix_point) ||
      !cbs_get_prefixed_point(&cbs, method->group, &key->pub1,
                              method->prefix_point) ||
      !cbs_get_prefixed_point(&cbs, method->group, &key->pubs,
                              method->prefix_point) ||
      CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  return 1;
}

static int pmbtoken_issuer_key_from_bytes(const PMBTOKEN_METHOD *method,
                                          TRUST_TOKEN_ISSUER_KEY *key,
                                          const uint8_t *in, size_t len) {
  const EC_GROUP *group = method->group;
  CBS cbs, tmp;
  CBS_init(&cbs, in, len);
  size_t scalar_len = BN_num_bytes(EC_GROUP_get0_order(group));
  EC_SCALAR *scalars[] = {&key->x0, &key->y0, &key->x1,
                          &key->y1, &key->xs, &key->ys};
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(scalars); i++) {
    if (!CBS_get_bytes(&cbs, &tmp, scalar_len) ||
        !ec_scalar_from_bytes(group, scalars[i], CBS_data(&tmp),
                              CBS_len(&tmp))) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      return 0;
    }
  }

  // Recompute the public key.
  EC_JACOBIAN pub[3];
  EC_AFFINE pub_affine[3];
  if (!ec_point_mul_scalar_precomp(group, &pub[0], &method->g_precomp, &key->x0,
                                   &method->h_precomp, &key->y0, NULL, NULL) ||
      !ec_init_precomp(group, &key->pub0_precomp, &pub[0]) ||
      !ec_point_mul_scalar_precomp(group, &pub[1], &method->g_precomp, &key->x1,
                                   &method->h_precomp, &key->y1, NULL, NULL) ||
      !ec_init_precomp(group, &key->pub1_precomp, &pub[1]) ||
      !ec_point_mul_scalar_precomp(group, &pub[2], &method->g_precomp, &key->xs,
                                   &method->h_precomp, &key->ys, NULL, NULL) ||
      !ec_init_precomp(group, &key->pubs_precomp, &pub[2]) ||
      !ec_jacobian_to_affine_batch(group, pub_affine, pub, 3)) {
    return 0;
  }

  key->pub0 = pub_affine[0];
  key->pub1 = pub_affine[1];
  key->pubs = pub_affine[2];
  return 1;
}

static STACK_OF(TRUST_TOKEN_PRETOKEN) *pmbtoken_blind(
    const PMBTOKEN_METHOD *method, CBB *cbb, size_t count, int include_message,
    const uint8_t *msg, size_t msg_len) {
  SHA512_CTX hash_ctx;

  const EC_GROUP *group = method->group;
  STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens = sk_TRUST_TOKEN_PRETOKEN_new_null();
  if (pretokens == NULL) {
    goto err;
  }

  for (size_t i = 0; i < count; i++) {
    // Insert |pretoken| into |pretokens| early to simplify error-handling.
    TRUST_TOKEN_PRETOKEN *pretoken = OPENSSL_malloc(sizeof(TRUST_TOKEN_PRETOKEN));
    if (pretoken == NULL ||
        !sk_TRUST_TOKEN_PRETOKEN_push(pretokens, pretoken)) {
      TRUST_TOKEN_PRETOKEN_free(pretoken);
      goto err;
    }

    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(pretoken->salt, sizeof(pretoken->salt)));
    if (include_message) {
      assert(SHA512_DIGEST_LENGTH == TRUST_TOKEN_NONCE_SIZE);
      SHA512_Init(&hash_ctx);
      SHA512_Update(&hash_ctx, pretoken->salt, sizeof(pretoken->salt));
      SHA512_Update(&hash_ctx, msg, msg_len);
      SHA512_Final(pretoken->t, &hash_ctx);
    } else {
      OPENSSL_memcpy(pretoken->t, pretoken->salt, TRUST_TOKEN_NONCE_SIZE);
    }

    // We sample |pretoken->r| in Montgomery form to simplify inverting.
    if (!ec_random_nonzero_scalar(group, &pretoken->r,
                                  kDefaultAdditionalData)) {
      goto err;
    }

    EC_SCALAR rinv;
    ec_scalar_inv0_montgomery(group, &rinv, &pretoken->r);
    // Convert both out of Montgomery form.
    ec_scalar_from_montgomery(group, &pretoken->r, &pretoken->r);
    ec_scalar_from_montgomery(group, &rinv, &rinv);

    EC_JACOBIAN T, Tp;
    if (!method->hash_t(group, &T, pretoken->t) ||
        !ec_point_mul_scalar(group, &Tp, &T, &rinv) ||
        !ec_jacobian_to_affine(group, &pretoken->Tp, &Tp)) {
      goto err;
    }

    if (!cbb_add_prefixed_point(cbb, group, &pretoken->Tp,
                                method->prefix_point)) {
      goto err;
    }
  }

  return pretokens;

err:
  sk_TRUST_TOKEN_PRETOKEN_pop_free(pretokens, TRUST_TOKEN_PRETOKEN_free);
  return NULL;
}

static int scalar_to_cbb(CBB *out, const EC_GROUP *group,
                         const EC_SCALAR *scalar) {
  uint8_t *buf;
  size_t scalar_len = BN_num_bytes(EC_GROUP_get0_order(group));
  if (!CBB_add_space(out, &buf, scalar_len)) {
    return 0;
  }
  ec_scalar_to_bytes(group, buf, &scalar_len, scalar);
  return 1;
}

static int scalar_from_cbs(CBS *cbs, const EC_GROUP *group, EC_SCALAR *out) {
  size_t scalar_len = BN_num_bytes(EC_GROUP_get0_order(group));
  CBS tmp;
  if (!CBS_get_bytes(cbs, &tmp, scalar_len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  ec_scalar_from_bytes(group, out, CBS_data(&tmp), CBS_len(&tmp));
  return 1;
}

static int hash_c_dleq(const PMBTOKEN_METHOD *method, EC_SCALAR *out,
                       const EC_AFFINE *X, const EC_AFFINE *T,
                       const EC_AFFINE *S, const EC_AFFINE *W,
                       const EC_AFFINE *K0, const EC_AFFINE *K1) {
  static const uint8_t kDLEQ2Label[] = "DLEQ2";

  int ok = 0;
  CBB cbb;
  CBB_zero(&cbb);
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !CBB_add_bytes(&cbb, kDLEQ2Label, sizeof(kDLEQ2Label)) ||
      !point_to_cbb(&cbb, method->group, X) ||
      !point_to_cbb(&cbb, method->group, T) ||
      !point_to_cbb(&cbb, method->group, S) ||
      !point_to_cbb(&cbb, method->group, W) ||
      !point_to_cbb(&cbb, method->group, K0) ||
      !point_to_cbb(&cbb, method->group, K1) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !method->hash_c(method->group, out, buf, len)) {
    goto err;
  }

  ok = 1;

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

static int hash_c_dleqor(const PMBTOKEN_METHOD *method, EC_SCALAR *out,
                         const EC_AFFINE *X0, const EC_AFFINE *X1,
                         const EC_AFFINE *T, const EC_AFFINE *S,
                         const EC_AFFINE *W, const EC_AFFINE *K00,
                         const EC_AFFINE *K01, const EC_AFFINE *K10,
                         const EC_AFFINE *K11) {
  static const uint8_t kDLEQOR2Label[] = "DLEQOR2";

  int ok = 0;
  CBB cbb;
  CBB_zero(&cbb);
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !CBB_add_bytes(&cbb, kDLEQOR2Label, sizeof(kDLEQOR2Label)) ||
      !point_to_cbb(&cbb, method->group, X0) ||
      !point_to_cbb(&cbb, method->group, X1) ||
      !point_to_cbb(&cbb, method->group, T) ||
      !point_to_cbb(&cbb, method->group, S) ||
      !point_to_cbb(&cbb, method->group, W) ||
      !point_to_cbb(&cbb, method->group, K00) ||
      !point_to_cbb(&cbb, method->group, K01) ||
      !point_to_cbb(&cbb, method->group, K10) ||
      !point_to_cbb(&cbb, method->group, K11) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !method->hash_c(method->group, out, buf, len)) {
    goto err;
  }

  ok = 1;

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

static int hash_c_batch(const PMBTOKEN_METHOD *method, EC_SCALAR *out,
                        const CBB *points, size_t index) {
  static const uint8_t kDLEQBatchLabel[] = "DLEQ BATCH";
  if (index > 0xffff) {
    // The protocol supports only two-byte batches.
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_OVERFLOW);
    return 0;
  }

  int ok = 0;
  CBB cbb;
  CBB_zero(&cbb);
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !CBB_add_bytes(&cbb, kDLEQBatchLabel, sizeof(kDLEQBatchLabel)) ||
      !CBB_add_bytes(&cbb, CBB_data(points), CBB_len(points)) ||
      !CBB_add_u16(&cbb, (uint16_t)index) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !method->hash_c(method->group, out, buf, len)) {
    goto err;
  }

  ok = 1;

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

// The DLEQ2 and DLEQOR2 constructions are described in appendix B of
// https://eprint.iacr.org/2020/072/20200324:214215. DLEQ2 is an instance of
// DLEQOR2 with only one value (n=1).

static int dleq_generate(const PMBTOKEN_METHOD *method, CBB *cbb,
                         const TRUST_TOKEN_ISSUER_KEY *priv,
                         const EC_JACOBIAN *T, const EC_JACOBIAN *S,
                         const EC_JACOBIAN *W, const EC_JACOBIAN *Ws,
                         uint8_t private_metadata) {
  const EC_GROUP *group = method->group;

  // We generate a DLEQ proof for the validity token and a DLEQOR2 proof for the
  // private metadata token. To allow amortizing Jacobian-to-affine conversions,
  // we compute Ki for both proofs first.
  enum {
    idx_T,
    idx_S,
    idx_W,
    idx_Ws,
    idx_Ks0,
    idx_Ks1,
    idx_Kb0,
    idx_Kb1,
    idx_Ko0,
    idx_Ko1,
    num_idx,
  };
  EC_JACOBIAN jacobians[num_idx];

  // Setup the DLEQ proof.
  EC_SCALAR ks0, ks1;
  if (// ks0, ks1 <- Zp
      !ec_random_nonzero_scalar(group, &ks0, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(group, &ks1, kDefaultAdditionalData) ||
      // Ks = ks0*(G;T) + ks1*(H;S)
      !ec_point_mul_scalar_precomp(group, &jacobians[idx_Ks0],
                                   &method->g_precomp, &ks0, &method->h_precomp,
                                   &ks1, NULL, NULL) ||
      !ec_point_mul_scalar_batch(group, &jacobians[idx_Ks1], T, &ks0, S, &ks1,
                                 NULL, NULL)) {
    return 0;
  }

  // Setup the DLEQOR proof. First, select values of xb, yb (keys corresponding
  // to the private metadata value) and pubo (public key corresponding to the
  // other value) in constant time.
  BN_ULONG mask = ((BN_ULONG)0) - (private_metadata & 1);
  EC_PRECOMP pubo_precomp;
  EC_SCALAR xb, yb;
  ec_scalar_select(group, &xb, mask, &priv->x1, &priv->x0);
  ec_scalar_select(group, &yb, mask, &priv->y1, &priv->y0);
  ec_precomp_select(group, &pubo_precomp, mask, &priv->pub0_precomp,
                    &priv->pub1_precomp);

  EC_SCALAR k0, k1, minus_co, uo, vo;
  if (// k0, k1 <- Zp
      !ec_random_nonzero_scalar(group, &k0, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(group, &k1, kDefaultAdditionalData) ||
      // Kb = k0*(G;T) + k1*(H;S)
      !ec_point_mul_scalar_precomp(group, &jacobians[idx_Kb0],
                                   &method->g_precomp, &k0, &method->h_precomp,
                                   &k1, NULL, NULL) ||
      !ec_point_mul_scalar_batch(group, &jacobians[idx_Kb1], T, &k0, S, &k1,
                                 NULL, NULL) ||
      // co, uo, vo <- Zp
      !ec_random_nonzero_scalar(group, &minus_co, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(group, &uo, kDefaultAdditionalData) ||
      !ec_random_nonzero_scalar(group, &vo, kDefaultAdditionalData) ||
      // Ko = uo*(G;T) + vo*(H;S) - co*(pubo;W)
      !ec_point_mul_scalar_precomp(group, &jacobians[idx_Ko0],
                                   &method->g_precomp, &uo, &method->h_precomp,
                                   &vo, &pubo_precomp, &minus_co) ||
      !ec_point_mul_scalar_batch(group, &jacobians[idx_Ko1], T, &uo, S, &vo, W,
                                 &minus_co)) {
    return 0;
  }

  EC_AFFINE affines[num_idx];
  jacobians[idx_T] = *T;
  jacobians[idx_S] = *S;
  jacobians[idx_W] = *W;
  jacobians[idx_Ws] = *Ws;
  if (!ec_jacobian_to_affine_batch(group, affines, jacobians, num_idx)) {
    return 0;
  }

  // Select the K corresponding to K0 and K1 in constant-time.
  EC_AFFINE K00, K01, K10, K11;
  ec_affine_select(group, &K00, mask, &affines[idx_Ko0], &affines[idx_Kb0]);
  ec_affine_select(group, &K01, mask, &affines[idx_Ko1], &affines[idx_Kb1]);
  ec_affine_select(group, &K10, mask, &affines[idx_Kb0], &affines[idx_Ko0]);
  ec_affine_select(group, &K11, mask, &affines[idx_Kb1], &affines[idx_Ko1]);

  // Compute c = Hc(...) for the two proofs.
  EC_SCALAR cs, c;
  if (!hash_c_dleq(method, &cs, &priv->pubs, &affines[idx_T], &affines[idx_S],
                   &affines[idx_Ws], &affines[idx_Ks0], &affines[idx_Ks1]) ||
      !hash_c_dleqor(method, &c, &priv->pub0, &priv->pub1, &affines[idx_T],
                     &affines[idx_S], &affines[idx_W], &K00, &K01, &K10,
                     &K11)) {
    return 0;
  }

  // Compute cb, ub, and ub for the two proofs. In each of these products, only
  // one operand is in Montgomery form, so the product does not need to be
  // converted.

  EC_SCALAR cs_mont;
  ec_scalar_to_montgomery(group, &cs_mont, &cs);

  // us = ks0 + cs*xs
  EC_SCALAR us, vs;
  ec_scalar_mul_montgomery(group, &us, &priv->xs, &cs_mont);
  ec_scalar_add(group, &us, &ks0, &us);

  // vs = ks1 + cs*ys
  ec_scalar_mul_montgomery(group, &vs, &priv->ys, &cs_mont);
  ec_scalar_add(group, &vs, &ks1, &vs);

  // Store DLEQ2 proof in transcript.
  if (!scalar_to_cbb(cbb, group, &cs) ||
      !scalar_to_cbb(cbb, group, &us) ||
      !scalar_to_cbb(cbb, group, &vs)) {
    return 0;
  }

  // cb = c - co
  EC_SCALAR cb, ub, vb;
  ec_scalar_add(group, &cb, &c, &minus_co);

  EC_SCALAR cb_mont;
  ec_scalar_to_montgomery(group, &cb_mont, &cb);

  // ub = k0 + cb*xb
  ec_scalar_mul_montgomery(group, &ub, &xb, &cb_mont);
  ec_scalar_add(group, &ub, &k0, &ub);

  // vb = k1 + cb*yb
  ec_scalar_mul_montgomery(group, &vb, &yb, &cb_mont);
  ec_scalar_add(group, &vb, &k1, &vb);

  // Select c, u, v in constant-time.
  EC_SCALAR co, c0, c1, u0, u1, v0, v1;
  ec_scalar_neg(group, &co, &minus_co);
  ec_scalar_select(group, &c0, mask, &co, &cb);
  ec_scalar_select(group, &u0, mask, &uo, &ub);
  ec_scalar_select(group, &v0, mask, &vo, &vb);
  ec_scalar_select(group, &c1, mask, &cb, &co);
  ec_scalar_select(group, &u1, mask, &ub, &uo);
  ec_scalar_select(group, &v1, mask, &vb, &vo);

  // Store DLEQOR2 proof in transcript.
  if (!scalar_to_cbb(cbb, group, &c0) ||
      !scalar_to_cbb(cbb, group, &c1) ||
      !scalar_to_cbb(cbb, group, &u0) ||
      !scalar_to_cbb(cbb, group, &u1) ||
      !scalar_to_cbb(cbb, group, &v0) ||
      !scalar_to_cbb(cbb, group, &v1)) {
    return 0;
  }

  return 1;
}

static int dleq_verify(const PMBTOKEN_METHOD *method, CBS *cbs,
                       const TRUST_TOKEN_CLIENT_KEY *pub, const EC_JACOBIAN *T,
                       const EC_JACOBIAN *S, const EC_JACOBIAN *W,
                       const EC_JACOBIAN *Ws) {
  const EC_GROUP *group = method->group;
  const EC_JACOBIAN *g = &group->generator.raw;

  // We verify a DLEQ proof for the validity token and a DLEQOR2 proof for the
  // private metadata token. To allow amortizing Jacobian-to-affine conversions,
  // we compute Ki for both proofs first. Additionally, all inputs to this
  // function are public, so we can use the faster variable-time
  // multiplications.
  enum {
    idx_T,
    idx_S,
    idx_W,
    idx_Ws,
    idx_Ks0,
    idx_Ks1,
    idx_K00,
    idx_K01,
    idx_K10,
    idx_K11,
    num_idx,
  };
  EC_JACOBIAN jacobians[num_idx];

  // Decode the DLEQ proof.
  EC_SCALAR cs, us, vs;
  if (!scalar_from_cbs(cbs, group, &cs) ||
      !scalar_from_cbs(cbs, group, &us) ||
      !scalar_from_cbs(cbs, group, &vs)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  // Ks = us*(G;T) + vs*(H;S) - cs*(pubs;Ws)
  EC_JACOBIAN pubs;
  ec_affine_to_jacobian(group, &pubs, &pub->pubs);
  EC_SCALAR minus_cs;
  ec_scalar_neg(group, &minus_cs, &cs);
  if (!mul_public_3(group, &jacobians[idx_Ks0], g, &us, &method->h, &vs, &pubs,
                    &minus_cs) ||
      !mul_public_3(group, &jacobians[idx_Ks1], T, &us, S, &vs, Ws,
                    &minus_cs)) {
    return 0;
  }

  // Decode the DLEQOR proof.
  EC_SCALAR c0, c1, u0, u1, v0, v1;
  if (!scalar_from_cbs(cbs, group, &c0) ||
      !scalar_from_cbs(cbs, group, &c1) ||
      !scalar_from_cbs(cbs, group, &u0) ||
      !scalar_from_cbs(cbs, group, &u1) ||
      !scalar_from_cbs(cbs, group, &v0) ||
      !scalar_from_cbs(cbs, group, &v1)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  EC_JACOBIAN pub0, pub1;
  ec_affine_to_jacobian(group, &pub0, &pub->pub0);
  ec_affine_to_jacobian(group, &pub1, &pub->pub1);
  EC_SCALAR minus_c0, minus_c1;
  ec_scalar_neg(group, &minus_c0, &c0);
  ec_scalar_neg(group, &minus_c1, &c1);
  if (// K0 = u0*(G;T) + v0*(H;S) - c0*(pub0;W)
      !mul_public_3(group, &jacobians[idx_K00], g, &u0, &method->h, &v0, &pub0,
                    &minus_c0) ||
      !mul_public_3(group, &jacobians[idx_K01], T, &u0, S, &v0, W, &minus_c0) ||
      // K1 = u1*(G;T) + v1*(H;S) - c1*(pub1;W)
      !mul_public_3(group, &jacobians[idx_K10], g, &u1, &method->h, &v1, &pub1,
                    &minus_c1) ||
      !mul_public_3(group, &jacobians[idx_K11], T, &u1, S, &v1, W, &minus_c1)) {
    return 0;
  }

  EC_AFFINE affines[num_idx];
  jacobians[idx_T] = *T;
  jacobians[idx_S] = *S;
  jacobians[idx_W] = *W;
  jacobians[idx_Ws] = *Ws;
  if (!ec_jacobian_to_affine_batch(group, affines, jacobians, num_idx)) {
    return 0;
  }

  // Check the DLEQ proof.
  EC_SCALAR calculated;
  if (!hash_c_dleq(method, &calculated, &pub->pubs, &affines[idx_T],
                   &affines[idx_S], &affines[idx_Ws], &affines[idx_Ks0],
                   &affines[idx_Ks1])) {
    return 0;
  }

  // cs == calculated
  if (!ec_scalar_equal_vartime(group, &cs, &calculated)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_PROOF);
    return 0;
  }

  // Check the DLEQOR proof.
  if (!hash_c_dleqor(method, &calculated, &pub->pub0, &pub->pub1,
                     &affines[idx_T], &affines[idx_S], &affines[idx_W],
                     &affines[idx_K00], &affines[idx_K01], &affines[idx_K10],
                     &affines[idx_K11])) {
    return 0;
  }

  // c0 + c1 == calculated
  EC_SCALAR c;
  ec_scalar_add(group, &c, &c0, &c1);
  if (!ec_scalar_equal_vartime(group, &c, &calculated)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_PROOF);
    return 0;
  }

  return 1;
}

static int pmbtoken_sign(const PMBTOKEN_METHOD *method,
                         const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                         size_t num_requested, size_t num_to_issue,
                         uint8_t private_metadata) {
  const EC_GROUP *group = method->group;
  if (num_requested < num_to_issue) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  int ret = 0;
  EC_JACOBIAN *Tps = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Sps = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Wps = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Wsps = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_SCALAR *es = OPENSSL_calloc(num_to_issue, sizeof(EC_SCALAR));
  CBB batch_cbb;
  CBB_zero(&batch_cbb);
  if (!Tps ||
      !Sps ||
      !Wps ||
      !Wsps ||
      !es ||
      !CBB_init(&batch_cbb, 0) ||
      !point_to_cbb(&batch_cbb, method->group, &key->pubs) ||
      !point_to_cbb(&batch_cbb, method->group, &key->pub0) ||
      !point_to_cbb(&batch_cbb, method->group, &key->pub1)) {
    goto err;
  }

  for (size_t i = 0; i < num_to_issue; i++) {
    EC_AFFINE Tp_affine;
    EC_JACOBIAN Tp;
    if (!cbs_get_prefixed_point(cbs, group, &Tp_affine, method->prefix_point)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      goto err;
    }
    ec_affine_to_jacobian(group, &Tp, &Tp_affine);

    EC_SCALAR xb, yb;
    BN_ULONG mask = ((BN_ULONG)0) - (private_metadata & 1);
    ec_scalar_select(group, &xb, mask, &key->x1, &key->x0);
    ec_scalar_select(group, &yb, mask, &key->y1, &key->y0);

    uint8_t s[TRUST_TOKEN_NONCE_SIZE];
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(s, TRUST_TOKEN_NONCE_SIZE));
    // The |jacobians| and |affines| contain Sp, Wp, and Wsp.
    EC_JACOBIAN jacobians[3];
    EC_AFFINE affines[3];
    if (!method->hash_s(group, &jacobians[0], &Tp_affine, s) ||
        !ec_point_mul_scalar_batch(group, &jacobians[1], &Tp, &xb,
                                   &jacobians[0], &yb, NULL, NULL) ||
        !ec_point_mul_scalar_batch(group, &jacobians[2], &Tp, &key->xs,
                                   &jacobians[0], &key->ys, NULL, NULL) ||
        !ec_jacobian_to_affine_batch(group, affines, jacobians, 3) ||
        !CBB_add_bytes(cbb, s, TRUST_TOKEN_NONCE_SIZE) ||
        !cbb_add_prefixed_point(cbb, group, &affines[1],
                                method->prefix_point) ||
        !cbb_add_prefixed_point(cbb, group, &affines[2],
                                method->prefix_point)) {
      goto err;
    }

    if (!point_to_cbb(&batch_cbb, group, &Tp_affine) ||
        !point_to_cbb(&batch_cbb, group, &affines[0]) ||
        !point_to_cbb(&batch_cbb, group, &affines[1]) ||
        !point_to_cbb(&batch_cbb, group, &affines[2])) {
      goto err;
    }
    Tps[i] = Tp;
    Sps[i] = jacobians[0];
    Wps[i] = jacobians[1];
    Wsps[i] = jacobians[2];

    if (!CBB_flush(cbb)) {
      goto err;
    }
  }

  // The DLEQ batching construction is described in appendix B of
  // https://eprint.iacr.org/2020/072/20200324:214215. Note the additional
  // computations all act on public inputs.
  for (size_t i = 0; i < num_to_issue; i++) {
    if (!hash_c_batch(method, &es[i], &batch_cbb, i)) {
      goto err;
    }
  }

  EC_JACOBIAN Tp_batch, Sp_batch, Wp_batch, Wsp_batch;
  if (!ec_point_mul_scalar_public_batch(group, &Tp_batch,
                                        /*g_scalar=*/NULL, Tps, es,
                                        num_to_issue) ||
      !ec_point_mul_scalar_public_batch(group, &Sp_batch,
                                        /*g_scalar=*/NULL, Sps, es,
                                        num_to_issue) ||
      !ec_point_mul_scalar_public_batch(group, &Wp_batch,
                                        /*g_scalar=*/NULL, Wps, es,
                                        num_to_issue) ||
      !ec_point_mul_scalar_public_batch(group, &Wsp_batch,
                                        /*g_scalar=*/NULL, Wsps, es,
                                        num_to_issue)) {
    goto err;
  }

  CBB proof;
  if (!CBB_add_u16_length_prefixed(cbb, &proof) ||
      !dleq_generate(method, &proof, key, &Tp_batch, &Sp_batch, &Wp_batch,
                     &Wsp_batch, private_metadata) ||
      !CBB_flush(cbb)) {
    goto err;
  }

  // Skip over any unused requests.
  size_t point_len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
  size_t token_len = point_len;
  if (method->prefix_point) {
    token_len += 2;
  }
  if (!CBS_skip(cbs, token_len * (num_requested - num_to_issue))) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(Tps);
  OPENSSL_free(Sps);
  OPENSSL_free(Wps);
  OPENSSL_free(Wsps);
  OPENSSL_free(es);
  CBB_cleanup(&batch_cbb);
  return ret;
}

static STACK_OF(TRUST_TOKEN) *pmbtoken_unblind(
    const PMBTOKEN_METHOD *method, const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  const EC_GROUP *group = method->group;
  if (count > sk_TRUST_TOKEN_PRETOKEN_num(pretokens)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return NULL;
  }

  int ok = 0;
  STACK_OF(TRUST_TOKEN) *ret = sk_TRUST_TOKEN_new_null();
  EC_JACOBIAN *Tps = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Sps = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Wps = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Wsps = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_SCALAR *es = OPENSSL_calloc(count, sizeof(EC_SCALAR));
  CBB batch_cbb;
  CBB_zero(&batch_cbb);
  if (ret == NULL ||
      Tps == NULL ||
      Sps == NULL ||
      Wps == NULL ||
      Wsps == NULL ||
      es == NULL ||
      !CBB_init(&batch_cbb, 0) ||
      !point_to_cbb(&batch_cbb, method->group, &key->pubs) ||
      !point_to_cbb(&batch_cbb, method->group, &key->pub0) ||
      !point_to_cbb(&batch_cbb, method->group, &key->pub1)) {
    goto err;
  }

  for (size_t i = 0; i < count; i++) {
    const TRUST_TOKEN_PRETOKEN *pretoken =
        sk_TRUST_TOKEN_PRETOKEN_value(pretokens, i);

    uint8_t s[TRUST_TOKEN_NONCE_SIZE];
    EC_AFFINE Wp_affine, Wsp_affine;
    if (!CBS_copy_bytes(cbs, s, TRUST_TOKEN_NONCE_SIZE) ||
        !cbs_get_prefixed_point(cbs, group, &Wp_affine, method->prefix_point) ||
        !cbs_get_prefixed_point(cbs, group, &Wsp_affine,
                                method->prefix_point)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      goto err;
    }

    ec_affine_to_jacobian(group, &Tps[i], &pretoken->Tp);
    ec_affine_to_jacobian(group, &Wps[i], &Wp_affine);
    ec_affine_to_jacobian(group, &Wsps[i], &Wsp_affine);
    if (!method->hash_s(group, &Sps[i], &pretoken->Tp, s)) {
      goto err;
    }

    EC_AFFINE Sp_affine;
    if (!point_to_cbb(&batch_cbb, group, &pretoken->Tp) ||
        !ec_jacobian_to_affine(group, &Sp_affine, &Sps[i]) ||
        !point_to_cbb(&batch_cbb, group, &Sp_affine) ||
        !point_to_cbb(&batch_cbb, group, &Wp_affine) ||
        !point_to_cbb(&batch_cbb, group, &Wsp_affine)) {
      goto err;
    }

    // Unblind the token.
    EC_JACOBIAN jacobians[3];
    EC_AFFINE affines[3];
    if (!ec_point_mul_scalar(group, &jacobians[0], &Sps[i], &pretoken->r) ||
        !ec_point_mul_scalar(group, &jacobians[1], &Wps[i], &pretoken->r) ||
        !ec_point_mul_scalar(group, &jacobians[2], &Wsps[i], &pretoken->r) ||
        !ec_jacobian_to_affine_batch(group, affines, jacobians, 3)) {
      goto err;
    }

    // Serialize the token. Include |key_id| to avoid an extra copy in the layer
    // above.
    CBB token_cbb;
    size_t point_len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
    if (!CBB_init(&token_cbb,
                  4 + TRUST_TOKEN_NONCE_SIZE + 3 * (2 + point_len)) ||
        !CBB_add_u32(&token_cbb, key_id) ||
        !CBB_add_bytes(&token_cbb, pretoken->salt, TRUST_TOKEN_NONCE_SIZE) ||
        !cbb_add_prefixed_point(&token_cbb, group, &affines[0],
                                method->prefix_point) ||
        !cbb_add_prefixed_point(&token_cbb, group, &affines[1],
                                method->prefix_point) ||
        !cbb_add_prefixed_point(&token_cbb, group, &affines[2],
                                method->prefix_point) ||
        !CBB_flush(&token_cbb)) {
      CBB_cleanup(&token_cbb);
      goto err;
    }

    TRUST_TOKEN *token =
        TRUST_TOKEN_new(CBB_data(&token_cbb), CBB_len(&token_cbb));
    CBB_cleanup(&token_cbb);
    if (token == NULL ||
        !sk_TRUST_TOKEN_push(ret, token)) {
      TRUST_TOKEN_free(token);
      goto err;
    }
  }

  // The DLEQ batching construction is described in appendix B of
  // https://eprint.iacr.org/2020/072/20200324:214215. Note the additional
  // computations all act on public inputs.
  for (size_t i = 0; i < count; i++) {
    if (!hash_c_batch(method, &es[i], &batch_cbb, i)) {
      goto err;
    }
  }

  EC_JACOBIAN Tp_batch, Sp_batch, Wp_batch, Wsp_batch;
  if (!ec_point_mul_scalar_public_batch(group, &Tp_batch,
                                        /*g_scalar=*/NULL, Tps, es, count) ||
      !ec_point_mul_scalar_public_batch(group, &Sp_batch,
                                        /*g_scalar=*/NULL, Sps, es, count) ||
      !ec_point_mul_scalar_public_batch(group, &Wp_batch,
                                        /*g_scalar=*/NULL, Wps, es, count) ||
      !ec_point_mul_scalar_public_batch(group, &Wsp_batch,
                                        /*g_scalar=*/NULL, Wsps, es, count)) {
    goto err;
  }

  CBS proof;
  if (!CBS_get_u16_length_prefixed(cbs, &proof) ||
      !dleq_verify(method, &proof, key, &Tp_batch, &Sp_batch, &Wp_batch,
                   &Wsp_batch) ||
      CBS_len(&proof) != 0) {
    goto err;
  }

  ok = 1;

err:
  OPENSSL_free(Tps);
  OPENSSL_free(Sps);
  OPENSSL_free(Wps);
  OPENSSL_free(Wsps);
  OPENSSL_free(es);
  CBB_cleanup(&batch_cbb);
  if (!ok) {
    sk_TRUST_TOKEN_pop_free(ret, TRUST_TOKEN_free);
    ret = NULL;
  }
  return ret;
}

static int pmbtoken_read(const PMBTOKEN_METHOD *method,
                         const TRUST_TOKEN_ISSUER_KEY *key,
                         uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                         uint8_t *out_private_metadata, const uint8_t *token,
                         size_t token_len, int include_message,
                         const uint8_t *msg, size_t msg_len) {
  const EC_GROUP *group = method->group;
  CBS cbs, salt;
  CBS_init(&cbs, token, token_len);
  EC_AFFINE S, W, Ws;
  if (!CBS_get_bytes(&cbs, &salt, TRUST_TOKEN_NONCE_SIZE) ||
      !cbs_get_prefixed_point(&cbs, group, &S, method->prefix_point) ||
      !cbs_get_prefixed_point(&cbs, group, &W, method->prefix_point) ||
      !cbs_get_prefixed_point(&cbs, group, &Ws, method->prefix_point) ||
      CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_TOKEN);
    return 0;
  }

  if (include_message) {
    SHA512_CTX hash_ctx;
    assert(SHA512_DIGEST_LENGTH == TRUST_TOKEN_NONCE_SIZE);
    SHA512_Init(&hash_ctx);
    SHA512_Update(&hash_ctx, CBS_data(&salt), CBS_len(&salt));
    SHA512_Update(&hash_ctx, msg, msg_len);
    SHA512_Final(out_nonce, &hash_ctx);
  } else {
    OPENSSL_memcpy(out_nonce, CBS_data(&salt), CBS_len(&salt));
  }

  EC_JACOBIAN T;
  if (!method->hash_t(group, &T, out_nonce)) {
    return 0;
  }

  // We perform three multiplications with S and T. This is enough that it is
  // worth using |ec_point_mul_scalar_precomp|.
  EC_JACOBIAN S_jacobian;
  EC_PRECOMP S_precomp, T_precomp;
  ec_affine_to_jacobian(group, &S_jacobian, &S);
  if (!ec_init_precomp(group, &S_precomp, &S_jacobian) ||
      !ec_init_precomp(group, &T_precomp, &T)) {
    return 0;
  }

  EC_JACOBIAN Ws_calculated;
  // Check the validity of the token.
  if (!ec_point_mul_scalar_precomp(group, &Ws_calculated, &T_precomp, &key->xs,
                                   &S_precomp, &key->ys, NULL, NULL) ||
      !ec_affine_jacobian_equal(group, &Ws, &Ws_calculated)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BAD_VALIDITY_CHECK);
    return 0;
  }

  EC_JACOBIAN W0, W1;
  if (!ec_point_mul_scalar_precomp(group, &W0, &T_precomp, &key->x0, &S_precomp,
                                   &key->y0, NULL, NULL) ||
      !ec_point_mul_scalar_precomp(group, &W1, &T_precomp, &key->x1, &S_precomp,
                                   &key->y1, NULL, NULL)) {
    return 0;
  }

  const int is_W0 = ec_affine_jacobian_equal(group, &W, &W0);
  const int is_W1 = ec_affine_jacobian_equal(group, &W, &W1);
  const int is_valid = is_W0 ^ is_W1;
  if (!is_valid) {
    // Invalid tokens will fail the validity check above.
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  *out_private_metadata = is_W1;
  return 1;
}


// PMBTokens experiment v1.

static int pmbtoken_exp1_hash_t(const EC_GROUP *group, EC_JACOBIAN *out,
                                const uint8_t t[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashTLabel[] = "PMBTokens Experiment V1 HashT";
  return ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
      group, out, kHashTLabel, sizeof(kHashTLabel), t, TRUST_TOKEN_NONCE_SIZE);
}

static int pmbtoken_exp1_hash_s(const EC_GROUP *group, EC_JACOBIAN *out,
                                const EC_AFFINE *t,
                                const uint8_t s[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashSLabel[] = "PMBTokens Experiment V1 HashS";
  int ret = 0;
  CBB cbb;
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !point_to_cbb(&cbb, group, t) ||
      !CBB_add_bytes(&cbb, s, TRUST_TOKEN_NONCE_SIZE) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
          group, out, kHashSLabel, sizeof(kHashSLabel), buf, len)) {
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(buf);
  CBB_cleanup(&cbb);
  return ret;
}

static int pmbtoken_exp1_hash_c(const EC_GROUP *group, EC_SCALAR *out,
                                uint8_t *buf, size_t len) {
  const uint8_t kHashCLabel[] = "PMBTokens Experiment V1 HashC";
  return ec_hash_to_scalar_p384_xmd_sha512_draft07(
      group, out, kHashCLabel, sizeof(kHashCLabel), buf, len);
}

static int pmbtoken_exp1_hash_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                        uint8_t *buf, size_t len) {
  const uint8_t kHashLabel[] = "PMBTokens Experiment V1 HashToScalar";
  return ec_hash_to_scalar_p384_xmd_sha512_draft07(
      group, out, kHashLabel, sizeof(kHashLabel), buf, len);
}

static int pmbtoken_exp1_ok = 0;
static PMBTOKEN_METHOD pmbtoken_exp1_method;
static CRYPTO_once_t pmbtoken_exp1_method_once = CRYPTO_ONCE_INIT;

static void pmbtoken_exp1_init_method_impl(void) {
  // This is the output of |ec_hash_to_scalar_p384_xmd_sha512_draft07| with DST
  // "PMBTokens Experiment V1 HashH" and message "generator".
  static const uint8_t kH[] = {
      0x04, 0x82, 0xd5, 0x68, 0xf5, 0x39, 0xf6, 0x08, 0x19, 0xa1, 0x75,
      0x9f, 0x98, 0xb5, 0x10, 0xf5, 0x0b, 0x9d, 0x2b, 0xe1, 0x64, 0x4d,
      0x02, 0x76, 0x18, 0x11, 0xf8, 0x2f, 0xd3, 0x33, 0x25, 0x1f, 0x2c,
      0xb8, 0xf6, 0xf1, 0x9e, 0x93, 0x85, 0x79, 0xb3, 0xb7, 0x81, 0xa3,
      0xe6, 0x23, 0xc3, 0x1c, 0xff, 0x03, 0xd9, 0x40, 0x6c, 0xec, 0xe0,
      0x4d, 0xea, 0xdf, 0x9d, 0x94, 0xd1, 0x87, 0xab, 0x27, 0xf7, 0x4f,
      0x53, 0xea, 0xa3, 0x18, 0x72, 0xb9, 0xd1, 0x56, 0xa0, 0x4e, 0x81,
      0xaa, 0xeb, 0x1c, 0x22, 0x6d, 0x39, 0x1c, 0x5e, 0xb1, 0x27, 0xfc,
      0x87, 0xc3, 0x95, 0xd0, 0x13, 0xb7, 0x0b, 0x5c, 0xc7,
  };

  pmbtoken_exp1_ok = pmbtoken_init_method(
      &pmbtoken_exp1_method, EC_group_p384(), kH, sizeof(kH),
      pmbtoken_exp1_hash_t, pmbtoken_exp1_hash_s, pmbtoken_exp1_hash_c,
      pmbtoken_exp1_hash_to_scalar, 1);
}

static int pmbtoken_exp1_init_method(void) {
  CRYPTO_once(&pmbtoken_exp1_method_once, pmbtoken_exp1_init_method_impl);
  if (!pmbtoken_exp1_ok) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }
  return 1;
}

int pmbtoken_exp1_generate_key(CBB *out_private, CBB *out_public) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }

  return pmbtoken_generate_key(&pmbtoken_exp1_method, out_private, out_public);
}

int pmbtoken_exp1_derive_key_from_secret(CBB *out_private, CBB *out_public,
                                         const uint8_t *secret,
                                         size_t secret_len) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }

  return pmbtoken_derive_key_from_secret(&pmbtoken_exp1_method, out_private,
                                         out_public, secret, secret_len);
}

int pmbtoken_exp1_client_key_from_bytes(TRUST_TOKEN_CLIENT_KEY *key,
                                        const uint8_t *in, size_t len) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }
  return pmbtoken_client_key_from_bytes(&pmbtoken_exp1_method, key, in, len);
}

int pmbtoken_exp1_issuer_key_from_bytes(TRUST_TOKEN_ISSUER_KEY *key,
                                        const uint8_t *in, size_t len) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }
  return pmbtoken_issuer_key_from_bytes(&pmbtoken_exp1_method, key, in, len);
}

STACK_OF(TRUST_TOKEN_PRETOKEN) *pmbtoken_exp1_blind(CBB *cbb, size_t count,
                                                    int include_message,
                                                    const uint8_t *msg,
                                                    size_t msg_len) {
  if (!pmbtoken_exp1_init_method()) {
    return NULL;
  }
  return pmbtoken_blind(&pmbtoken_exp1_method, cbb, count, include_message, msg,
                        msg_len);
}

int pmbtoken_exp1_sign(const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                       size_t num_requested, size_t num_to_issue,
                       uint8_t private_metadata) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }
  return pmbtoken_sign(&pmbtoken_exp1_method, key, cbb, cbs, num_requested,
                       num_to_issue, private_metadata);
}

STACK_OF(TRUST_TOKEN) *pmbtoken_exp1_unblind(
    const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  if (!pmbtoken_exp1_init_method()) {
    return NULL;
  }
  return pmbtoken_unblind(&pmbtoken_exp1_method, key, pretokens, cbs, count,
                          key_id);
}

int pmbtoken_exp1_read(const TRUST_TOKEN_ISSUER_KEY *key,
                       uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                       uint8_t *out_private_metadata, const uint8_t *token,
                       size_t token_len, int include_message,
                       const uint8_t *msg, size_t msg_len) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }
  return pmbtoken_read(&pmbtoken_exp1_method, key, out_nonce,
                       out_private_metadata, token, token_len, include_message,
                       msg, msg_len);
}

int pmbtoken_exp1_get_h_for_testing(uint8_t out[97]) {
  if (!pmbtoken_exp1_init_method()) {
    return 0;
  }
  EC_AFFINE h;
  return ec_jacobian_to_affine(pmbtoken_exp1_method.group, &h,
                               &pmbtoken_exp1_method.h) &&
         ec_point_to_bytes(pmbtoken_exp1_method.group, &h,
                           POINT_CONVERSION_UNCOMPRESSED, out, 97) == 97;
}

// PMBTokens experiment v2.

static int pmbtoken_exp2_hash_t(const EC_GROUP *group, EC_JACOBIAN *out,
                                const uint8_t t[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashTLabel[] = "PMBTokens Experiment V2 HashT";
  return ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
      group, out, kHashTLabel, sizeof(kHashTLabel), t, TRUST_TOKEN_NONCE_SIZE);
}

static int pmbtoken_exp2_hash_s(const EC_GROUP *group, EC_JACOBIAN *out,
                                const EC_AFFINE *t,
                                const uint8_t s[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashSLabel[] = "PMBTokens Experiment V2 HashS";
  int ret = 0;
  CBB cbb;
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !point_to_cbb(&cbb, group, t) ||
      !CBB_add_bytes(&cbb, s, TRUST_TOKEN_NONCE_SIZE) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
          group, out, kHashSLabel, sizeof(kHashSLabel), buf, len)) {
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(buf);
  CBB_cleanup(&cbb);
  return ret;
}

static int pmbtoken_exp2_hash_c(const EC_GROUP *group, EC_SCALAR *out,
                                uint8_t *buf, size_t len) {
  const uint8_t kHashCLabel[] = "PMBTokens Experiment V2 HashC";
  return ec_hash_to_scalar_p384_xmd_sha512_draft07(
      group, out, kHashCLabel, sizeof(kHashCLabel), buf, len);
}

static int pmbtoken_exp2_hash_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                        uint8_t *buf, size_t len) {
  const uint8_t kHashLabel[] = "PMBTokens Experiment V2 HashToScalar";
  return ec_hash_to_scalar_p384_xmd_sha512_draft07(
      group, out, kHashLabel, sizeof(kHashLabel), buf, len);
}

static int pmbtoken_exp2_ok = 0;
static PMBTOKEN_METHOD pmbtoken_exp2_method;
static CRYPTO_once_t pmbtoken_exp2_method_once = CRYPTO_ONCE_INIT;

static void pmbtoken_exp2_init_method_impl(void) {
  // This is the output of |ec_hash_to_scalar_p384_xmd_sha512_draft07| with DST
  // "PMBTokens Experiment V2 HashH" and message "generator".
  static const uint8_t kH[] = {
      0x04, 0xbc, 0x27, 0x24, 0x99, 0xfa, 0xc9, 0xa4, 0x74, 0x6f, 0xf9,
      0x07, 0x81, 0x55, 0xf8, 0x1f, 0x6f, 0xda, 0x09, 0xe7, 0x8c, 0x5d,
      0x9e, 0x4e, 0x14, 0x7c, 0x53, 0x14, 0xbc, 0x7e, 0x29, 0x57, 0x92,
      0x17, 0x94, 0x6e, 0xd2, 0xdf, 0xa5, 0x31, 0x1b, 0x4e, 0xb7, 0xfc,
      0x93, 0xe3, 0x6e, 0x14, 0x1f, 0x4f, 0x14, 0xf3, 0xe5, 0x47, 0x61,
      0x1c, 0x2c, 0x72, 0x25, 0xf0, 0x4a, 0x45, 0x23, 0x2d, 0x57, 0x93,
      0x0e, 0xb2, 0x55, 0xb8, 0x57, 0x25, 0x4c, 0x1e, 0xdb, 0xfd, 0x58,
      0x70, 0x17, 0x9a, 0xbb, 0x9e, 0x5e, 0x93, 0x9e, 0x92, 0xd3, 0xe8,
      0x25, 0x62, 0xbf, 0x59, 0xb2, 0xd2, 0x3d, 0x71, 0xff
  };

  pmbtoken_exp2_ok = pmbtoken_init_method(
      &pmbtoken_exp2_method, EC_group_p384(), kH, sizeof(kH),
      pmbtoken_exp2_hash_t, pmbtoken_exp2_hash_s, pmbtoken_exp2_hash_c,
      pmbtoken_exp2_hash_to_scalar, 0);
}

static int pmbtoken_exp2_init_method(void) {
  CRYPTO_once(&pmbtoken_exp2_method_once, pmbtoken_exp2_init_method_impl);
  if (!pmbtoken_exp2_ok) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }
  return 1;
}

int pmbtoken_exp2_generate_key(CBB *out_private, CBB *out_public) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }

  return pmbtoken_generate_key(&pmbtoken_exp2_method, out_private, out_public);
}


int pmbtoken_exp2_derive_key_from_secret(CBB *out_private, CBB *out_public,
                                         const uint8_t *secret,
                                         size_t secret_len) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }

  return pmbtoken_derive_key_from_secret(&pmbtoken_exp2_method, out_private,
                                         out_public, secret, secret_len);
}

int pmbtoken_exp2_client_key_from_bytes(TRUST_TOKEN_CLIENT_KEY *key,
                                        const uint8_t *in, size_t len) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }
  return pmbtoken_client_key_from_bytes(&pmbtoken_exp2_method, key, in, len);
}

int pmbtoken_exp2_issuer_key_from_bytes(TRUST_TOKEN_ISSUER_KEY *key,
                                        const uint8_t *in, size_t len) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }
  return pmbtoken_issuer_key_from_bytes(&pmbtoken_exp2_method, key, in, len);
}

STACK_OF(TRUST_TOKEN_PRETOKEN) *pmbtoken_exp2_blind(CBB *cbb, size_t count,
                                                    int include_message,
                                                    const uint8_t *msg,
                                                    size_t msg_len) {
  if (!pmbtoken_exp2_init_method()) {
    return NULL;
  }
  return pmbtoken_blind(&pmbtoken_exp2_method, cbb, count, include_message, msg,
                        msg_len);
}

int pmbtoken_exp2_sign(const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                       size_t num_requested, size_t num_to_issue,
                       uint8_t private_metadata) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }
  return pmbtoken_sign(&pmbtoken_exp2_method, key, cbb, cbs, num_requested,
                       num_to_issue, private_metadata);
}

STACK_OF(TRUST_TOKEN) *pmbtoken_exp2_unblind(
    const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  if (!pmbtoken_exp2_init_method()) {
    return NULL;
  }
  return pmbtoken_unblind(&pmbtoken_exp2_method, key, pretokens, cbs, count,
                          key_id);
}

int pmbtoken_exp2_read(const TRUST_TOKEN_ISSUER_KEY *key,
                       uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                       uint8_t *out_private_metadata, const uint8_t *token,
                       size_t token_len, int include_message,
                       const uint8_t *msg, size_t msg_len) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }
  return pmbtoken_read(&pmbtoken_exp2_method, key, out_nonce,
                       out_private_metadata, token, token_len, include_message,
                       msg, msg_len);
}

int pmbtoken_exp2_get_h_for_testing(uint8_t out[97]) {
  if (!pmbtoken_exp2_init_method()) {
    return 0;
  }
  EC_AFFINE h;
  return ec_jacobian_to_affine(pmbtoken_exp2_method.group, &h,
                               &pmbtoken_exp2_method.h) &&
         ec_point_to_bytes(pmbtoken_exp2_method.group, &h,
                           POINT_CONVERSION_UNCOMPRESSED, out, 97) == 97;
}

// PMBTokens PST v1.

static int pmbtoken_pst1_hash_t(const EC_GROUP *group, EC_JACOBIAN *out,
                                const uint8_t t[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashTLabel[] = "PMBTokens PST V1 HashT";
  return ec_hash_to_curve_p384_xmd_sha384_sswu(
      group, out, kHashTLabel, sizeof(kHashTLabel), t, TRUST_TOKEN_NONCE_SIZE);
}

static int pmbtoken_pst1_hash_s(const EC_GROUP *group, EC_JACOBIAN *out,
                                const EC_AFFINE *t,
                                const uint8_t s[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashSLabel[] = "PMBTokens PST V1 HashS";
  int ret = 0;
  CBB cbb;
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !point_to_cbb(&cbb, group, t) ||
      !CBB_add_bytes(&cbb, s, TRUST_TOKEN_NONCE_SIZE) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !ec_hash_to_curve_p384_xmd_sha384_sswu(
          group, out, kHashSLabel, sizeof(kHashSLabel), buf, len)) {
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(buf);
  CBB_cleanup(&cbb);
  return ret;
}

static int pmbtoken_pst1_hash_c(const EC_GROUP *group, EC_SCALAR *out,
                                uint8_t *buf, size_t len) {
  const uint8_t kHashCLabel[] = "PMBTokens PST V1 HashC";
  return ec_hash_to_scalar_p384_xmd_sha384(
      group, out, kHashCLabel, sizeof(kHashCLabel), buf, len);
}

static int pmbtoken_pst1_hash_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                        uint8_t *buf, size_t len) {
  const uint8_t kHashLabel[] = "PMBTokens PST V1 HashToScalar";
  return ec_hash_to_scalar_p384_xmd_sha384(
      group, out, kHashLabel, sizeof(kHashLabel), buf, len);
}

static int pmbtoken_pst1_ok = 0;
static PMBTOKEN_METHOD pmbtoken_pst1_method;
static CRYPTO_once_t pmbtoken_pst1_method_once = CRYPTO_ONCE_INIT;

static void pmbtoken_pst1_init_method_impl(void) {
  // This is the output of |ec_hash_to_scalar_p384_xmd_sha384| with DST
  // "PMBTokens PST V1 HashH" and message "generator".
  static const uint8_t kH[] = {
      0x04, 0x4c, 0xfa, 0xd4, 0x33, 0x6d, 0x8c, 0x4e, 0x18, 0xce, 0x1a,
      0x82, 0x7b, 0x53, 0x8c, 0xf8, 0x63, 0x18, 0xe5, 0xa3, 0x96, 0x0d,
      0x05, 0xde, 0xf4, 0x83, 0xa7, 0xd8, 0xde, 0x9c, 0x50, 0x81, 0x38,
      0xc9, 0x38, 0x25, 0xa3, 0x70, 0x97, 0xc1, 0x1c, 0x33, 0x2e, 0x83,
      0x68, 0x64, 0x9c, 0x53, 0x73, 0xc3, 0x03, 0xc1, 0xa9, 0xd8, 0x92,
      0xa2, 0x32, 0xf4, 0x22, 0x40, 0x07, 0x2d, 0x9b, 0x6f, 0xab, 0xff,
      0x2a, 0x92, 0x03, 0xb1, 0x73, 0x09, 0x1a, 0x6a, 0x4a, 0xc2, 0x4c,
      0xac, 0x13, 0x59, 0xf4, 0x28, 0x0e, 0x78, 0x69, 0xa5, 0xdf, 0x0d,
      0x74, 0xeb, 0x14, 0xca, 0x8a, 0x32, 0xbb, 0xd3, 0x91
  };

  pmbtoken_pst1_ok = pmbtoken_init_method(
      &pmbtoken_pst1_method, EC_group_p384(), kH, sizeof(kH),
      pmbtoken_pst1_hash_t, pmbtoken_pst1_hash_s, pmbtoken_pst1_hash_c,
      pmbtoken_pst1_hash_to_scalar, 0);
}

static int pmbtoken_pst1_init_method(void) {
  CRYPTO_once(&pmbtoken_pst1_method_once, pmbtoken_pst1_init_method_impl);
  if (!pmbtoken_pst1_ok) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }
  return 1;
}

int pmbtoken_pst1_generate_key(CBB *out_private, CBB *out_public) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }

  return pmbtoken_generate_key(&pmbtoken_pst1_method, out_private, out_public);
}


int pmbtoken_pst1_derive_key_from_secret(CBB *out_private, CBB *out_public,
                                         const uint8_t *secret,
                                         size_t secret_len) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }

  return pmbtoken_derive_key_from_secret(&pmbtoken_pst1_method, out_private,
                                         out_public, secret, secret_len);
}

int pmbtoken_pst1_client_key_from_bytes(TRUST_TOKEN_CLIENT_KEY *key,
                                        const uint8_t *in, size_t len) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }
  return pmbtoken_client_key_from_bytes(&pmbtoken_pst1_method, key, in, len);
}

int pmbtoken_pst1_issuer_key_from_bytes(TRUST_TOKEN_ISSUER_KEY *key,
                                        const uint8_t *in, size_t len) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }
  return pmbtoken_issuer_key_from_bytes(&pmbtoken_pst1_method, key, in, len);
}

STACK_OF(TRUST_TOKEN_PRETOKEN) *pmbtoken_pst1_blind(CBB *cbb, size_t count,
                                                    int include_message,
                                                    const uint8_t *msg,
                                                    size_t msg_len) {
  if (!pmbtoken_pst1_init_method()) {
    return NULL;
  }
  return pmbtoken_blind(&pmbtoken_pst1_method, cbb, count, include_message, msg,
                        msg_len);
}

int pmbtoken_pst1_sign(const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                       size_t num_requested, size_t num_to_issue,
                       uint8_t private_metadata) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }
  return pmbtoken_sign(&pmbtoken_pst1_method, key, cbb, cbs, num_requested,
                       num_to_issue, private_metadata);
}

STACK_OF(TRUST_TOKEN) *pmbtoken_pst1_unblind(
    const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  if (!pmbtoken_pst1_init_method()) {
    return NULL;
  }
  return pmbtoken_unblind(&pmbtoken_pst1_method, key, pretokens, cbs, count,
                          key_id);
}

int pmbtoken_pst1_read(const TRUST_TOKEN_ISSUER_KEY *key,
                       uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                       uint8_t *out_private_metadata, const uint8_t *token,
                       size_t token_len, int include_message,
                       const uint8_t *msg, size_t msg_len) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }
  return pmbtoken_read(&pmbtoken_pst1_method, key, out_nonce,
                       out_private_metadata, token, token_len, include_message,
                       msg, msg_len);
}

int pmbtoken_pst1_get_h_for_testing(uint8_t out[97]) {
  if (!pmbtoken_pst1_init_method()) {
    return 0;
  }
  EC_AFFINE h;
  return ec_jacobian_to_affine(pmbtoken_pst1_method.group, &h,
                               &pmbtoken_pst1_method.h) &&
         ec_point_to_bytes(pmbtoken_pst1_method.group, &h,
                           POINT_CONVERSION_UNCOMPRESSED, out, 97) == 97;
}
