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
#include "../fipsmodule/ec/internal.h"

#include "internal.h"


typedef int (*hash_to_group_func_t)(const EC_GROUP *group, EC_JACOBIAN *out,
                                    const uint8_t t[TRUST_TOKEN_NONCE_SIZE]);
typedef int (*hash_to_scalar_func_t)(const EC_GROUP *group, EC_SCALAR *out,
                                     uint8_t *buf, size_t len);

typedef struct {
  const EC_GROUP *(*group_func)(void);

  // hash_to_group implements the HashToGroup operation for VOPRFs. It returns
  // one on success and zero on error.
  hash_to_group_func_t hash_to_group;
  // hash_to_scalar implements the HashToScalar operation for VOPRFs. It returns
  // one on success and zero on error.
  hash_to_scalar_func_t hash_to_scalar;
} VOPRF_METHOD;

static const uint8_t kDefaultAdditionalData[32] = {0};

static int cbb_add_point(CBB *out, const EC_GROUP *group,
                         const EC_AFFINE *point) {
  uint8_t *p;
  size_t len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
  return CBB_add_space(out, &p, len) &&
         ec_point_to_bytes(group, point, POINT_CONVERSION_UNCOMPRESSED, p,
                           len) == len &&
         CBB_flush(out);
}

static int cbb_serialize_point(CBB *out, const EC_GROUP *group,
                               const EC_AFFINE *point) {
  uint8_t *p;
  size_t len = ec_point_byte_len(group, POINT_CONVERSION_COMPRESSED);
  return CBB_add_u16(out, len) && CBB_add_space(out, &p, len) &&
         ec_point_to_bytes(group, point, POINT_CONVERSION_COMPRESSED, p, len) ==
             len &&
         CBB_flush(out);
}

static int cbs_get_point(CBS *cbs, const EC_GROUP *group, EC_AFFINE *out) {
  CBS child;
  size_t plen = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
  if (!CBS_get_bytes(cbs, &child, plen) ||
      !ec_point_from_uncompressed(group, out, CBS_data(&child),
                                  CBS_len(&child))) {
    return 0;
  }
  return 1;
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

static int voprf_calculate_key(const VOPRF_METHOD *method, CBB *out_private,
                               CBB *out_public, const EC_SCALAR *priv) {
  const EC_GROUP *group = method->group_func();
  EC_JACOBIAN pub;
  EC_AFFINE pub_affine;
  if (!ec_point_mul_scalar_base(group, &pub, priv) ||
      !ec_jacobian_to_affine(group, &pub_affine, &pub)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    return 0;
  }

  if (!scalar_to_cbb(out_private, group, priv) ||
      !cbb_add_point(out_public, group, &pub_affine)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
    return 0;
  }

  return 1;
}


static int voprf_generate_key(const VOPRF_METHOD *method, CBB *out_private,
                              CBB *out_public) {
  EC_SCALAR priv;
  if (!ec_random_nonzero_scalar(method->group_func(), &priv,
                                kDefaultAdditionalData)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    return 0;
  }
  return voprf_calculate_key(method, out_private, out_public, &priv);
}

static int voprf_derive_key_from_secret(const VOPRF_METHOD *method,
                                        CBB *out_private, CBB *out_public,
                                        const uint8_t *secret,
                                        size_t secret_len) {
  static const uint8_t kKeygenLabel[] = "TrustTokenVOPRFKeyGen";

  EC_SCALAR priv;
  int ok = 0;
  CBB cbb;
  CBB_zero(&cbb);
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !CBB_add_bytes(&cbb, kKeygenLabel, sizeof(kKeygenLabel)) ||
      !CBB_add_bytes(&cbb, secret, secret_len) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !method->hash_to_scalar(method->group_func(), &priv, buf, len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_KEYGEN_FAILURE);
    goto err;
  }

  ok = voprf_calculate_key(method, out_private, out_public, &priv);

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

static int voprf_client_key_from_bytes(const VOPRF_METHOD *method,
                                       TRUST_TOKEN_CLIENT_KEY *key,
                                       const uint8_t *in, size_t len) {
  const EC_GROUP *group = method->group_func();
  if (!ec_point_from_uncompressed(group, &key->pubs, in, len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  return 1;
}

static int voprf_issuer_key_from_bytes(const VOPRF_METHOD *method,
                                       TRUST_TOKEN_ISSUER_KEY *key,
                                       const uint8_t *in, size_t len) {
  const EC_GROUP *group = method->group_func();
  if (!ec_scalar_from_bytes(group, &key->xs, in, len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  // Recompute the public key.
  EC_JACOBIAN pub;
  if (!ec_point_mul_scalar_base(group, &pub, &key->xs) ||
      !ec_jacobian_to_affine(group, &key->pubs, &pub)) {
    return 0;
  }

  return 1;
}

static STACK_OF(TRUST_TOKEN_PRETOKEN) *voprf_blind(const VOPRF_METHOD *method,
                                                   CBB *cbb, size_t count,
                                                   int include_message,
                                                   const uint8_t *msg,
                                                   size_t msg_len) {
  SHA512_CTX hash_ctx;

  const EC_GROUP *group = method->group_func();
  STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens =
      sk_TRUST_TOKEN_PRETOKEN_new_null();
  if (pretokens == NULL) {
    goto err;
  }

  for (size_t i = 0; i < count; i++) {
    // Insert |pretoken| into |pretokens| early to simplify error-handling.
    TRUST_TOKEN_PRETOKEN *pretoken =
        OPENSSL_malloc(sizeof(TRUST_TOKEN_PRETOKEN));
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

    // We sample r in Montgomery form to simplify inverting.
    EC_SCALAR r;
    if (!ec_random_nonzero_scalar(group, &r,
                                  kDefaultAdditionalData)) {
      goto err;
    }

    // pretoken->r is rinv.
    ec_scalar_inv0_montgomery(group, &pretoken->r, &r);
    // Convert both out of Montgomery form.
    ec_scalar_from_montgomery(group, &r, &r);
    ec_scalar_from_montgomery(group, &pretoken->r, &pretoken->r);

    // Tp is the blinded token in the VOPRF protocol.
    EC_JACOBIAN P, Tp;
    if (!method->hash_to_group(group, &P, pretoken->t) ||
        !ec_point_mul_scalar(group, &Tp, &P, &r) ||
        !ec_jacobian_to_affine(group, &pretoken->Tp, &Tp)) {
      goto err;
    }

    if (!cbb_add_point(cbb, group, &pretoken->Tp)) {
      goto err;
    }
  }

  return pretokens;

err:
  sk_TRUST_TOKEN_PRETOKEN_pop_free(pretokens, TRUST_TOKEN_PRETOKEN_free);
  return NULL;
}

static int hash_to_scalar_dleq(const VOPRF_METHOD *method, EC_SCALAR *out,
                               const EC_AFFINE *X, const EC_AFFINE *T,
                               const EC_AFFINE *W, const EC_AFFINE *K0,
                               const EC_AFFINE *K1) {
  static const uint8_t kDLEQLabel[] = "DLEQ";

  const EC_GROUP *group = method->group_func();
  int ok = 0;
  CBB cbb;
  CBB_zero(&cbb);
  uint8_t *buf = NULL;
  size_t len;
  if (!CBB_init(&cbb, 0) ||
      !CBB_add_bytes(&cbb, kDLEQLabel, sizeof(kDLEQLabel)) ||
      !cbb_add_point(&cbb, group, X) ||
      !cbb_add_point(&cbb, group, T) ||
      !cbb_add_point(&cbb, group, W) ||
      !cbb_add_point(&cbb, group, K0) ||
      !cbb_add_point(&cbb, group, K1) ||
      !CBB_finish(&cbb, &buf, &len) ||
      !method->hash_to_scalar(group, out, buf, len)) {
    goto err;
  }

  ok = 1;

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

static int hash_to_scalar_challenge(const VOPRF_METHOD *method, EC_SCALAR *out,
                                    const EC_AFFINE *Bm, const EC_AFFINE *a0,
                                    const EC_AFFINE *a1, const EC_AFFINE *a2,
                                    const EC_AFFINE *a3) {
  static const uint8_t kChallengeLabel[] = "Challenge";

  const EC_GROUP *group = method->group_func();
  CBB cbb;
  uint8_t transcript[5 * EC_MAX_COMPRESSED + 2 + sizeof(kChallengeLabel) - 1];
  size_t len;
  if (!CBB_init_fixed(&cbb, transcript, sizeof(transcript)) ||
      !cbb_serialize_point(&cbb, group, Bm) ||
      !cbb_serialize_point(&cbb, group, a0) ||
      !cbb_serialize_point(&cbb, group, a1) ||
      !cbb_serialize_point(&cbb, group, a2) ||
      !cbb_serialize_point(&cbb, group, a3) ||
      !CBB_add_bytes(&cbb, kChallengeLabel, sizeof(kChallengeLabel) - 1) ||
      !CBB_finish(&cbb, NULL, &len) ||
      !method->hash_to_scalar(group, out, transcript, len)) {
    return 0;
  }

  return 1;
}

static int hash_to_scalar_batch(const VOPRF_METHOD *method, EC_SCALAR *out,
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
      !method->hash_to_scalar(method->group_func(), out, buf, len)) {
    goto err;
  }

  ok = 1;

err:
  CBB_cleanup(&cbb);
  OPENSSL_free(buf);
  return ok;
}

static int dleq_generate(const VOPRF_METHOD *method, CBB *cbb,
                         const TRUST_TOKEN_ISSUER_KEY *priv,
                         const EC_JACOBIAN *T, const EC_JACOBIAN *W) {
  const EC_GROUP *group = method->group_func();

  enum {
    idx_T,
    idx_W,
    idx_k0,
    idx_k1,
    num_idx,
  };
  EC_JACOBIAN jacobians[num_idx];

  // Setup the DLEQ proof.
  EC_SCALAR r;
  if (// r <- Zp
      !ec_random_nonzero_scalar(group, &r, kDefaultAdditionalData) ||
      // k0;k1 = r*(G;T)
      !ec_point_mul_scalar_base(group, &jacobians[idx_k0], &r) ||
      !ec_point_mul_scalar(group, &jacobians[idx_k1], T, &r))  {
    return 0;
  }

  EC_AFFINE affines[num_idx];
  jacobians[idx_T] = *T;
  jacobians[idx_W] = *W;
  if (!ec_jacobian_to_affine_batch(group, affines, jacobians, num_idx)) {
    return 0;
  }

  // Compute c = Hc(...).
  EC_SCALAR c;
  if (!hash_to_scalar_dleq(method, &c, &priv->pubs, &affines[idx_T],
                           &affines[idx_W], &affines[idx_k0],
                           &affines[idx_k1])) {
    return 0;
  }


  EC_SCALAR c_mont;
  ec_scalar_to_montgomery(group, &c_mont, &c);

  // u = r + c*xs
  EC_SCALAR u;
  ec_scalar_mul_montgomery(group, &u, &priv->xs, &c_mont);
  ec_scalar_add(group, &u, &r, &u);

  // Store DLEQ proof in transcript.
  if (!scalar_to_cbb(cbb, group, &c) ||
      !scalar_to_cbb(cbb, group, &u)) {
    return 0;
  }

  return 1;
}

static int mul_public_2(const EC_GROUP *group, EC_JACOBIAN *out,
                        const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                        const EC_JACOBIAN *p1, const EC_SCALAR *scalar1) {
  EC_JACOBIAN points[2] = {*p0, *p1};
  EC_SCALAR scalars[2] = {*scalar0, *scalar1};
  return ec_point_mul_scalar_public_batch(group, out, /*g_scalar=*/NULL, points,
                                          scalars, 2);
}

static int dleq_verify(const VOPRF_METHOD *method, CBS *cbs,
                       const TRUST_TOKEN_CLIENT_KEY *pub, const EC_JACOBIAN *T,
                       const EC_JACOBIAN *W) {
  const EC_GROUP *group = method->group_func();


  enum {
    idx_T,
    idx_W,
    idx_k0,
    idx_k1,
    num_idx,
  };
  EC_JACOBIAN jacobians[num_idx];

  // Decode the DLEQ proof.
  EC_SCALAR c, u;
  if (!scalar_from_cbs(cbs, group, &c) ||
      !scalar_from_cbs(cbs, group, &u)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  // k0;k1 = u*(G;T) - c*(pub;W)
  EC_JACOBIAN pubs;
  ec_affine_to_jacobian(group, &pubs, &pub->pubs);
  EC_SCALAR minus_c;
  ec_scalar_neg(group, &minus_c, &c);
  if (!ec_point_mul_scalar_public(group, &jacobians[idx_k0], &u, &pubs,
                                  &minus_c) ||
      !mul_public_2(group, &jacobians[idx_k1], T, &u, W, &minus_c)) {
    return 0;
  }

  // Check the DLEQ proof.
  EC_AFFINE affines[num_idx];
  jacobians[idx_T] = *T;
  jacobians[idx_W] = *W;
  if (!ec_jacobian_to_affine_batch(group, affines, jacobians, num_idx)) {
    return 0;
  }

  // Compute c = Hc(...).
  EC_SCALAR calculated;
  if (!hash_to_scalar_dleq(method, &calculated, &pub->pubs, &affines[idx_T],
                           &affines[idx_W], &affines[idx_k0],
                           &affines[idx_k1])) {
    return 0;
  }

  // c == calculated
  if (!ec_scalar_equal_vartime(group, &c, &calculated)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_PROOF);
    return 0;
  }

  return 1;
}

static int voprf_sign_tt(const VOPRF_METHOD *method,
                         const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                         size_t num_requested, size_t num_to_issue) {
  const EC_GROUP *group = method->group_func();
  if (num_requested < num_to_issue) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  int ret = 0;
  EC_JACOBIAN *BTs = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Zs = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_SCALAR *es = OPENSSL_calloc(num_to_issue, sizeof(EC_SCALAR));
  CBB batch_cbb;
  CBB_zero(&batch_cbb);
  if (!BTs ||
      !Zs ||
      !es ||
      !CBB_init(&batch_cbb, 0) ||
      !cbb_add_point(&batch_cbb, group, &key->pubs)) {
    goto err;
  }

  for (size_t i = 0; i < num_to_issue; i++) {
    EC_AFFINE BT_affine, Z_affine;
    EC_JACOBIAN BT, Z;
    if (!cbs_get_point(cbs, group, &BT_affine)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      goto err;
    }
    ec_affine_to_jacobian(group, &BT, &BT_affine);
    if (!ec_point_mul_scalar(group, &Z, &BT, &key->xs) ||
        !ec_jacobian_to_affine(group, &Z_affine, &Z) ||
        !cbb_add_point(cbb, group, &Z_affine)) {
      goto err;
    }

    if (!cbb_add_point(&batch_cbb, group, &BT_affine) ||
        !cbb_add_point(&batch_cbb, group, &Z_affine)) {
      goto err;
    }
    BTs[i] = BT;
    Zs[i] = Z;

    if (!CBB_flush(cbb)) {
      goto err;
    }
  }

  // The DLEQ batching construction is described in appendix B of
  // https://eprint.iacr.org/2020/072/20200324:214215. Note the additional
  // computations all act on public inputs.
  for (size_t i = 0; i < num_to_issue; i++) {
    if (!hash_to_scalar_batch(method, &es[i], &batch_cbb, i)) {
      goto err;
    }
  }

  EC_JACOBIAN BT_batch, Z_batch;
  if (!ec_point_mul_scalar_public_batch(group, &BT_batch,
                                        /*g_scalar=*/NULL, BTs, es,
                                        num_to_issue) ||
      !ec_point_mul_scalar_public_batch(group, &Z_batch,
                                        /*g_scalar=*/NULL, Zs, es,
                                        num_to_issue)) {
    goto err;
  }

  CBB proof;
  if (!CBB_add_u16_length_prefixed(cbb, &proof) ||
      !dleq_generate(method, &proof, key, &BT_batch, &Z_batch) ||
      !CBB_flush(cbb)) {
    goto err;
  }

  // Skip over any unused requests.
  size_t point_len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
  if (!CBS_skip(cbs, point_len * (num_requested - num_to_issue))) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(BTs);
  OPENSSL_free(Zs);
  OPENSSL_free(es);
  CBB_cleanup(&batch_cbb);
  return ret;
}

static STACK_OF(TRUST_TOKEN) *voprf_unblind_tt(
    const VOPRF_METHOD *method, const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  const EC_GROUP *group = method->group_func();
  if (count > sk_TRUST_TOKEN_PRETOKEN_num(pretokens)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return NULL;
  }

  int ok = 0;
  STACK_OF(TRUST_TOKEN) *ret = sk_TRUST_TOKEN_new_null();
  EC_JACOBIAN *BTs = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Zs = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_SCALAR *es = OPENSSL_calloc(count, sizeof(EC_SCALAR));
  CBB batch_cbb;
  CBB_zero(&batch_cbb);
  if (ret == NULL ||
      BTs == NULL ||
      Zs == NULL ||
      es == NULL ||
      !CBB_init(&batch_cbb, 0) ||
      !cbb_add_point(&batch_cbb, group, &key->pubs)) {
    goto err;
  }

  for (size_t i = 0; i < count; i++) {
    const TRUST_TOKEN_PRETOKEN *pretoken =
        sk_TRUST_TOKEN_PRETOKEN_value(pretokens, i);

    EC_AFFINE Z_affine;
    if (!cbs_get_point(cbs, group, &Z_affine)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      goto err;
    }

    ec_affine_to_jacobian(group, &BTs[i], &pretoken->Tp);
    ec_affine_to_jacobian(group, &Zs[i], &Z_affine);

    if (!cbb_add_point(&batch_cbb, group, &pretoken->Tp) ||
        !cbb_add_point(&batch_cbb, group, &Z_affine)) {
      goto err;
    }

    // Unblind the token.
    // pretoken->r is rinv.
    EC_JACOBIAN N;
    EC_AFFINE N_affine;
    if (!ec_point_mul_scalar(group, &N, &Zs[i], &pretoken->r) ||
        !ec_jacobian_to_affine(group, &N_affine, &N)) {
      goto err;
    }

    // Serialize the token. Include |key_id| to avoid an extra copy in the layer
    // above.
    CBB token_cbb;
    size_t point_len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
    if (!CBB_init(&token_cbb, 4 + TRUST_TOKEN_NONCE_SIZE + (2 + point_len)) ||
        !CBB_add_u32(&token_cbb, key_id) ||
        !CBB_add_bytes(&token_cbb, pretoken->salt, TRUST_TOKEN_NONCE_SIZE) ||
        !cbb_add_point(&token_cbb, group, &N_affine) ||
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
    if (!hash_to_scalar_batch(method, &es[i], &batch_cbb, i)) {
      goto err;
    }
  }

  EC_JACOBIAN BT_batch, Z_batch;
  if (!ec_point_mul_scalar_public_batch(group, &BT_batch,
                                        /*g_scalar=*/NULL, BTs, es, count) ||
      !ec_point_mul_scalar_public_batch(group, &Z_batch,
                                        /*g_scalar=*/NULL, Zs, es, count)) {
    goto err;
  }

  CBS proof;
  if (!CBS_get_u16_length_prefixed(cbs, &proof) ||
      !dleq_verify(method, &proof, key, &BT_batch, &Z_batch) ||
      CBS_len(&proof) != 0) {
    goto err;
  }

  ok = 1;

err:
  OPENSSL_free(BTs);
  OPENSSL_free(Zs);
  OPENSSL_free(es);
  CBB_cleanup(&batch_cbb);
  if (!ok) {
    sk_TRUST_TOKEN_pop_free(ret, TRUST_TOKEN_free);
    ret = NULL;
  }
  return ret;
}

static void sha384_update_u16(SHA512_CTX *ctx, uint16_t v) {
  uint8_t buf[2] = {v >> 8, v & 0xff};
  SHA384_Update(ctx, buf, 2);
}

static void sha384_update_point_with_length(
     SHA512_CTX *ctx, const EC_GROUP *group, const EC_AFFINE *point) {
  uint8_t buf[EC_MAX_COMPRESSED];
  size_t len = ec_point_to_bytes(group, point, POINT_CONVERSION_COMPRESSED,
                                 buf, sizeof(buf));
  assert(len > 0);
  sha384_update_u16(ctx, (uint16_t)len);
  SHA384_Update(ctx, buf, len);
}

static int compute_composite_seed(const VOPRF_METHOD *method,
                                  uint8_t out[SHA384_DIGEST_LENGTH],
                                  const EC_AFFINE *pub) {
  const EC_GROUP *group = method->group_func();
  static const uint8_t kSeedDST[] = "Seed-OPRFV1-\x01-P384-SHA384";

  SHA512_CTX hash_ctx;
  SHA384_Init(&hash_ctx);
  sha384_update_point_with_length(&hash_ctx, group, pub);
  sha384_update_u16(&hash_ctx, sizeof(kSeedDST) - 1);
  SHA384_Update(&hash_ctx, kSeedDST, sizeof(kSeedDST) - 1);
  SHA384_Final(out, &hash_ctx);

  return 1;
}

static int compute_composite_element(const VOPRF_METHOD *method,
                                     uint8_t seed[SHA384_DIGEST_LENGTH],
                                     EC_SCALAR *di, size_t index,
                                     const EC_AFFINE *C, const EC_AFFINE *D) {
  static const uint8_t kCompositeLabel[] = "Composite";
  const EC_GROUP *group = method->group_func();

  if (index > UINT16_MAX) {
    return 0;
  }

  CBB cbb;
  uint8_t transcript[2 + SHA384_DIGEST_LENGTH + 2 + 2 * EC_MAX_COMPRESSED +
                     sizeof(kCompositeLabel) - 1];
  size_t len;
  if (!CBB_init_fixed(&cbb, transcript, sizeof(transcript)) ||
      !CBB_add_u16(&cbb, SHA384_DIGEST_LENGTH) ||
      !CBB_add_bytes(&cbb, seed, SHA384_DIGEST_LENGTH) ||
      !CBB_add_u16(&cbb, index) ||
      !cbb_serialize_point(&cbb, group, C) ||
      !cbb_serialize_point(&cbb, group, D) ||
      !CBB_add_bytes(&cbb, kCompositeLabel,
                     sizeof(kCompositeLabel) - 1) ||
      !CBB_finish(&cbb, NULL, &len) ||
      !method->hash_to_scalar(group, di, transcript, len)) {
    return 0;
  }

  return 1;
}

static int generate_proof(const VOPRF_METHOD *method, CBB *cbb,
                          const TRUST_TOKEN_ISSUER_KEY *priv,
                          const EC_SCALAR *r, const EC_JACOBIAN *M,
                          const EC_JACOBIAN *Z) {
  const EC_GROUP *group = method->group_func();

  enum {
    idx_M,
    idx_Z,
    idx_t2,
    idx_t3,
    num_idx,
  };
  EC_JACOBIAN jacobians[num_idx];

  if (!ec_point_mul_scalar_base(group, &jacobians[idx_t2], r) ||
      !ec_point_mul_scalar(group, &jacobians[idx_t3], M, r)) {
    return 0;
  }


  EC_AFFINE affines[num_idx];
  jacobians[idx_M] = *M;
  jacobians[idx_Z] = *Z;
  if (!ec_jacobian_to_affine_batch(group, affines, jacobians, num_idx)) {
    return 0;
  }

  EC_SCALAR c;
  if (!hash_to_scalar_challenge(method, &c, &priv->pubs, &affines[idx_M],
                                &affines[idx_Z], &affines[idx_t2],
                                &affines[idx_t3])) {
    return 0;
  }

  EC_SCALAR c_mont;
  ec_scalar_to_montgomery(group, &c_mont, &c);

  // s = r - c*xs
  EC_SCALAR s;
  ec_scalar_mul_montgomery(group, &s, &priv->xs, &c_mont);
  ec_scalar_sub(group, &s, r, &s);

  // Store DLEQ proof in transcript.
  if (!scalar_to_cbb(cbb, group, &c) ||
      !scalar_to_cbb(cbb, group, &s)) {
    return 0;
  }

  return 1;
}

static int verify_proof(const VOPRF_METHOD *method, CBS *cbs,
                        const TRUST_TOKEN_CLIENT_KEY *pub,
                        const EC_JACOBIAN *M, const EC_JACOBIAN *Z) {
  const EC_GROUP *group = method->group_func();

  enum {
    idx_M,
    idx_Z,
    idx_t2,
    idx_t3,
    num_idx,
  };
  EC_JACOBIAN jacobians[num_idx];

  EC_SCALAR c, s;
  if (!scalar_from_cbs(cbs, group, &c) ||
      !scalar_from_cbs(cbs, group, &s)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  EC_JACOBIAN pubs;
  ec_affine_to_jacobian(group, &pubs, &pub->pubs);
  if (!ec_point_mul_scalar_public(group, &jacobians[idx_t2], &s, &pubs,
                                  &c) ||
      !mul_public_2(group, &jacobians[idx_t3], M, &s, Z, &c)) {
    return 0;
  }

  EC_AFFINE affines[num_idx];
  jacobians[idx_M] = *M;
  jacobians[idx_Z] = *Z;
  if (!ec_jacobian_to_affine_batch(group, affines, jacobians, num_idx)) {
    return 0;
  }

  EC_SCALAR expected_c;
  if (!hash_to_scalar_challenge(method, &expected_c, &pub->pubs,
                                &affines[idx_M], &affines[idx_Z],
                                &affines[idx_t2], &affines[idx_t3])) {
    return 0;
  }

  // c == expected_c
  if (!ec_scalar_equal_vartime(group, &c, &expected_c)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_PROOF);
    return 0;
  }

  return 1;
}

static int voprf_sign_impl(const VOPRF_METHOD *method,
                           const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb,
                           CBS *cbs, size_t num_requested, size_t num_to_issue,
                           const EC_SCALAR *proof_scalar) {
  const EC_GROUP *group = method->group_func();
  if (num_requested < num_to_issue) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  int ret = 0;
  EC_JACOBIAN *BTs = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Zs = OPENSSL_calloc(num_to_issue, sizeof(EC_JACOBIAN));
  EC_SCALAR *dis = OPENSSL_calloc(num_to_issue, sizeof(EC_SCALAR));
  if (!BTs || !Zs || !dis) {
    goto err;
  }

  uint8_t seed[SHA384_DIGEST_LENGTH];
  if (!compute_composite_seed(method, seed, &key->pubs)) {
    goto err;
  }

  // This implements the BlindEvaluateBatch as defined in section 4 of
  // draft-robert-privacypass-batched-tokens-01, based on the constructions
  // in draft-irtf-cfrg-voprf-21. To optimize the computation of the proof,
  // the computation of di is done during the token signing and passed into
  // the proof generation.
  for (size_t i = 0; i < num_to_issue; i++) {
    EC_AFFINE BT_affine, Z_affine;
    EC_JACOBIAN BT, Z;
    if (!cbs_get_point(cbs, group, &BT_affine)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      goto err;
    }
    ec_affine_to_jacobian(group, &BT, &BT_affine);
    if (!ec_point_mul_scalar(group, &Z, &BT, &key->xs) ||
        !ec_jacobian_to_affine(group, &Z_affine, &Z) ||
        !cbb_add_point(cbb, group, &Z_affine)) {
      goto err;
    }
    BTs[i] = BT;
    Zs[i] = Z;
    if (!compute_composite_element(method, seed, &dis[i], i, &BT_affine,
                                   &Z_affine)) {
      goto err;
    }

    if (!CBB_flush(cbb)) {
      goto err;
    }
  }

  EC_JACOBIAN M, Z;
  if (!ec_point_mul_scalar_public_batch(group, &M,
                                        /*g_scalar=*/NULL, BTs, dis,
                                        num_to_issue) ||
      !ec_point_mul_scalar(group, &Z, &M, &key->xs)) {
    goto err;
  }

  CBB proof;
  if (!CBB_add_u16_length_prefixed(cbb, &proof) ||
      !generate_proof(method, &proof, key, proof_scalar, &M, &Z) ||
      !CBB_flush(cbb)) {
    goto err;
  }

  // Skip over any unused requests.
  size_t point_len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
  if (!CBS_skip(cbs, point_len * (num_requested - num_to_issue))) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(BTs);
  OPENSSL_free(Zs);
  OPENSSL_free(dis);
  return ret;
}

static int voprf_sign(const VOPRF_METHOD *method,
                      const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                      size_t num_requested, size_t num_to_issue) {
  EC_SCALAR proof_scalar;
  if (!ec_random_nonzero_scalar(method->group_func(), &proof_scalar,
                                kDefaultAdditionalData)) {
    return 0;
  }

  return voprf_sign_impl(method, key, cbb, cbs, num_requested, num_to_issue,
                         &proof_scalar);
}

static int voprf_sign_with_proof_scalar_for_testing(
    const VOPRF_METHOD *method, const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb,
    CBS *cbs, size_t num_requested, size_t num_to_issue,
    const uint8_t *proof_scalar_buf, size_t proof_scalar_len) {
  EC_SCALAR proof_scalar;
  if (!ec_scalar_from_bytes(method->group_func(), &proof_scalar,
                            proof_scalar_buf, proof_scalar_len)) {
    return 0;
  }
  return voprf_sign_impl(method, key, cbb, cbs, num_requested, num_to_issue,
                         &proof_scalar);
}

static STACK_OF(TRUST_TOKEN) *voprf_unblind(
    const VOPRF_METHOD *method, const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  const EC_GROUP *group = method->group_func();
  if (count > sk_TRUST_TOKEN_PRETOKEN_num(pretokens)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return NULL;
  }

  int ok = 0;
  STACK_OF(TRUST_TOKEN) *ret = sk_TRUST_TOKEN_new_null();
  EC_JACOBIAN *BTs = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_JACOBIAN *Zs = OPENSSL_calloc(count, sizeof(EC_JACOBIAN));
  EC_SCALAR *dis = OPENSSL_calloc(count, sizeof(EC_SCALAR));
  if (ret == NULL || !BTs || !Zs || !dis) {
    goto err;
  }

  uint8_t seed[SHA384_DIGEST_LENGTH];
  if (!compute_composite_seed(method, seed, &key->pubs)) {
    goto err;
  }

  for (size_t i = 0; i < count; i++) {
    const TRUST_TOKEN_PRETOKEN *pretoken =
        sk_TRUST_TOKEN_PRETOKEN_value(pretokens, i);

    EC_AFFINE Z_affine;
    if (!cbs_get_point(cbs, group, &Z_affine)) {
      OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
      goto err;
    }

    ec_affine_to_jacobian(group, &BTs[i], &pretoken->Tp);
    ec_affine_to_jacobian(group, &Zs[i], &Z_affine);
    if (!compute_composite_element(method, seed, &dis[i], i, &pretoken->Tp,
                                   &Z_affine)) {
      goto err;
    }

    // Unblind the token.
    // pretoken->r is rinv.
    EC_JACOBIAN N;
    EC_AFFINE N_affine;
    if (!ec_point_mul_scalar(group, &N, &Zs[i], &pretoken->r) ||
        !ec_jacobian_to_affine(group, &N_affine, &N)) {
      goto err;
    }

    // Serialize the token. Include |key_id| to avoid an extra copy in the layer
    // above.
    CBB token_cbb;
    size_t point_len = ec_point_byte_len(group, POINT_CONVERSION_UNCOMPRESSED);
    if (!CBB_init(&token_cbb, 4 + TRUST_TOKEN_NONCE_SIZE + (2 + point_len)) ||
        !CBB_add_u32(&token_cbb, key_id) ||
        !CBB_add_bytes(&token_cbb, pretoken->salt, TRUST_TOKEN_NONCE_SIZE) ||
        !cbb_add_point(&token_cbb, group, &N_affine) ||
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

  EC_JACOBIAN M, Z;
  if (!ec_point_mul_scalar_public_batch(group, &M,
                                        /*g_scalar=*/NULL, BTs, dis,
                                        count) ||
      !ec_point_mul_scalar_public_batch(group, &Z,
                                        /*g_scalar=*/NULL, Zs, dis,
                                        count)) {
    goto err;
  }

  CBS proof;
  if (!CBS_get_u16_length_prefixed(cbs, &proof) ||
      !verify_proof(method, &proof, key, &M, &Z) ||
      CBS_len(&proof) != 0) {
    goto err;
  }

  ok = 1;

err:
  OPENSSL_free(BTs);
  OPENSSL_free(Zs);
  OPENSSL_free(dis);
  if (!ok) {
    sk_TRUST_TOKEN_pop_free(ret, TRUST_TOKEN_free);
    ret = NULL;
  }
  return ret;
}

static int voprf_read(const VOPRF_METHOD *method,
                      const TRUST_TOKEN_ISSUER_KEY *key,
                      uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                      const uint8_t *token, size_t token_len,
                      int include_message, const uint8_t *msg, size_t msg_len) {
  const EC_GROUP *group = method->group_func();
  CBS cbs, salt;
  CBS_init(&cbs, token, token_len);
  EC_AFFINE Ws;
  if (!CBS_get_bytes(&cbs, &salt, TRUST_TOKEN_NONCE_SIZE) ||
      !cbs_get_point(&cbs, group, &Ws) ||
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
  if (!method->hash_to_group(group, &T, out_nonce)) {
    return 0;
  }

  EC_JACOBIAN Ws_calculated;
  if (!ec_point_mul_scalar(group, &Ws_calculated, &T, &key->xs) ||
      !ec_affine_jacobian_equal(group, &Ws, &Ws_calculated)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BAD_VALIDITY_CHECK);
    return 0;
  }

  return 1;
}


// VOPRF experiment v2.

static int voprf_exp2_hash_to_group(const EC_GROUP *group, EC_JACOBIAN *out,
                                    const uint8_t t[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashTLabel[] = "TrustToken VOPRF Experiment V2 HashToGroup";
  return ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
      group, out, kHashTLabel, sizeof(kHashTLabel), t, TRUST_TOKEN_NONCE_SIZE);
}

static int voprf_exp2_hash_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                             uint8_t *buf, size_t len) {
  const uint8_t kHashCLabel[] = "TrustToken VOPRF Experiment V2 HashToScalar";
  return ec_hash_to_scalar_p384_xmd_sha512_draft07(
      group, out, kHashCLabel, sizeof(kHashCLabel), buf, len);
}

static VOPRF_METHOD voprf_exp2_method = {
    EC_group_p384, voprf_exp2_hash_to_group, voprf_exp2_hash_to_scalar};

int voprf_exp2_generate_key(CBB *out_private, CBB *out_public) {
  return voprf_generate_key(&voprf_exp2_method, out_private, out_public);
}

int voprf_exp2_derive_key_from_secret(CBB *out_private, CBB *out_public,
                                      const uint8_t *secret,
                                      size_t secret_len) {
  return voprf_derive_key_from_secret(&voprf_exp2_method, out_private,
                                      out_public, secret, secret_len);
}

int voprf_exp2_client_key_from_bytes(TRUST_TOKEN_CLIENT_KEY *key,
                                     const uint8_t *in, size_t len) {
  return voprf_client_key_from_bytes(&voprf_exp2_method, key, in, len);
}

int voprf_exp2_issuer_key_from_bytes(TRUST_TOKEN_ISSUER_KEY *key,
                                     const uint8_t *in, size_t len) {
  return voprf_issuer_key_from_bytes(&voprf_exp2_method, key, in, len);
}

STACK_OF(TRUST_TOKEN_PRETOKEN) *voprf_exp2_blind(CBB *cbb, size_t count,
                                                 int include_message,
                                                 const uint8_t *msg,
                                                 size_t msg_len) {
  return voprf_blind(&voprf_exp2_method, cbb, count, include_message, msg,
                     msg_len);
}

int voprf_exp2_sign(const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                    size_t num_requested, size_t num_to_issue,
                    uint8_t private_metadata) {
  if (private_metadata != 0) {
    return 0;
  }
  return voprf_sign_tt(&voprf_exp2_method, key, cbb, cbs, num_requested,
                       num_to_issue);
}

STACK_OF(TRUST_TOKEN) *voprf_exp2_unblind(
    const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  return voprf_unblind_tt(&voprf_exp2_method, key, pretokens, cbs, count,
                          key_id);
}

int voprf_exp2_read(const TRUST_TOKEN_ISSUER_KEY *key,
                    uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                    uint8_t *out_private_metadata, const uint8_t *token,
                    size_t token_len, int include_message, const uint8_t *msg,
                    size_t msg_len) {
  return voprf_read(&voprf_exp2_method, key, out_nonce, token, token_len,
                    include_message, msg, msg_len);
}

// VOPRF PST v1.

static int voprf_pst1_hash_to_group(const EC_GROUP *group, EC_JACOBIAN *out,
                                    const uint8_t t[TRUST_TOKEN_NONCE_SIZE]) {
  const uint8_t kHashTLabel[] = "HashToGroup-OPRFV1-\x01-P384-SHA384";
  return ec_hash_to_curve_p384_xmd_sha384_sswu(group, out, kHashTLabel,
                                               sizeof(kHashTLabel) - 1, t,
                                               TRUST_TOKEN_NONCE_SIZE);
}

static int voprf_pst1_hash_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                             uint8_t *buf, size_t len) {
  const uint8_t kHashCLabel[] = "HashToScalar-OPRFV1-\x01-P384-SHA384";
  return ec_hash_to_scalar_p384_xmd_sha384(group, out, kHashCLabel,
                                           sizeof(kHashCLabel) - 1, buf, len);
}

static VOPRF_METHOD voprf_pst1_method = {
    EC_group_p384, voprf_pst1_hash_to_group, voprf_pst1_hash_to_scalar};

int voprf_pst1_generate_key(CBB *out_private, CBB *out_public) {
  return voprf_generate_key(&voprf_pst1_method, out_private, out_public);
}

int voprf_pst1_derive_key_from_secret(CBB *out_private, CBB *out_public,
                                      const uint8_t *secret,
                                      size_t secret_len) {
  return voprf_derive_key_from_secret(&voprf_pst1_method, out_private,
                                      out_public, secret, secret_len);
}

int voprf_pst1_client_key_from_bytes(TRUST_TOKEN_CLIENT_KEY *key,
                                     const uint8_t *in, size_t len) {
  return voprf_client_key_from_bytes(&voprf_pst1_method, key, in, len);
}

int voprf_pst1_issuer_key_from_bytes(TRUST_TOKEN_ISSUER_KEY *key,
                                     const uint8_t *in, size_t len) {
  return voprf_issuer_key_from_bytes(&voprf_pst1_method, key, in, len);
}

STACK_OF(TRUST_TOKEN_PRETOKEN) *voprf_pst1_blind(CBB *cbb, size_t count,
                                                 int include_message,
                                                 const uint8_t *msg,
                                                 size_t msg_len) {
  return voprf_blind(&voprf_pst1_method, cbb, count, include_message, msg,
                     msg_len);
}

int voprf_pst1_sign(const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs,
                    size_t num_requested, size_t num_to_issue,
                    uint8_t private_metadata) {
  if (private_metadata != 0) {
    return 0;
  }
  return voprf_sign(&voprf_pst1_method, key, cbb, cbs, num_requested,
                    num_to_issue);
}


int voprf_pst1_sign_with_proof_scalar_for_testing(
    const TRUST_TOKEN_ISSUER_KEY *key, CBB *cbb, CBS *cbs, size_t num_requested,
    size_t num_to_issue, uint8_t private_metadata,
    const uint8_t *proof_scalar_buf, size_t proof_scalar_len) {
  if (private_metadata != 0) {
    return 0;
  }
  return voprf_sign_with_proof_scalar_for_testing(
      &voprf_pst1_method, key, cbb, cbs, num_requested, num_to_issue,
      proof_scalar_buf, proof_scalar_len);
}

STACK_OF(TRUST_TOKEN) *voprf_pst1_unblind(
    const TRUST_TOKEN_CLIENT_KEY *key,
    const STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens, CBS *cbs, size_t count,
    uint32_t key_id) {
  return voprf_unblind(&voprf_pst1_method, key, pretokens, cbs, count, key_id);
}

int voprf_pst1_read(const TRUST_TOKEN_ISSUER_KEY *key,
                    uint8_t out_nonce[TRUST_TOKEN_NONCE_SIZE],
                    uint8_t *out_private_metadata, const uint8_t *token,
                    size_t token_len, int include_message, const uint8_t *msg,
                    size_t msg_len) {
  return voprf_read(&voprf_pst1_method, key, out_nonce, token, token_len,
                    include_message, msg, msg_len);
}
