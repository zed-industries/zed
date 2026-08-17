/*
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#ifndef MLD_NATIVE_AARCH64_META_H
#define MLD_NATIVE_AARCH64_META_H

/* Set of primitives that this backend replaces */
#define MLD_USE_NATIVE_NTT
#define MLD_USE_NATIVE_INTT
#define MLD_USE_NATIVE_REJ_UNIFORM
#define MLD_USE_NATIVE_REJ_UNIFORM_ETA2
#define MLD_USE_NATIVE_REJ_UNIFORM_ETA4
#define MLD_USE_NATIVE_POLY_DECOMPOSE_32
#define MLD_USE_NATIVE_POLY_DECOMPOSE_88
#define MLD_USE_NATIVE_POLY_CADDQ
#define MLD_USE_NATIVE_POLY_USE_HINT_32
#define MLD_USE_NATIVE_POLY_USE_HINT_88
#define MLD_USE_NATIVE_POLY_CHKNORM
#define MLD_USE_NATIVE_POLYZ_UNPACK_17
#define MLD_USE_NATIVE_POLYZ_UNPACK_19
#define MLD_USE_NATIVE_POINTWISE_MONTGOMERY
#define MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L4
#define MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L5
#define MLD_USE_NATIVE_POLYVECL_POINTWISE_ACC_MONTGOMERY_L7

/* Identifier for this backend so that source and assembly files
 * in the build can be appropriately guarded. */
#define MLD_ARITH_BACKEND_AARCH64


#if !defined(__ASSEMBLER__)
#include "../api.h"
#include "src/arith_native_aarch64.h"

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_ntt_native(int32_t data[MLDSA_N])
{
  mld_ntt_aarch64_asm(data, mld_aarch64_ntt_zetas_layer123456,
                      mld_aarch64_ntt_zetas_layer78);
  return MLD_NATIVE_FUNC_SUCCESS;
}

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_intt_native(int32_t data[MLDSA_N])
{
  mld_intt_aarch64_asm(data, mld_aarch64_intt_zetas_layer78,
                       mld_aarch64_intt_zetas_layer123456);
  return MLD_NATIVE_FUNC_SUCCESS;
}

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_rej_uniform_native(int32_t *r, unsigned len,
                                             const uint8_t *buf,
                                             unsigned buflen)
{
  if (len != MLDSA_N ||
      buflen % 24 != 0) /* NEON support is mandatory for AArch64 */
  {
    return MLD_NATIVE_FUNC_FALLBACK;
  }

  /* Safety: outlen is at most MLDSA_N, hence, this cast is safe. */
  return (int)mld_rej_uniform_aarch64_asm(r, buf, buflen,
                                          mld_rej_uniform_table);
}

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLDSA_ETA == 2
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_rej_uniform_eta2_native(int32_t *r, unsigned len,
                                                  const uint8_t *buf,
                                                  unsigned buflen)
{
  uint64_t outlen;
  /* AArch64 implementation assumes specific buffer lengths */
  if (len != MLDSA_N || buflen != MLD_AARCH64_REJ_UNIFORM_ETA2_BUFLEN)
  {
    return MLD_NATIVE_FUNC_FALLBACK;
  }
  /* Constant time: Inputs and outputs to this function are secret.
   * It is safe to leak which coefficients are accepted/rejected.
   * The assembly implementation must not leak any other information about the
   * accepted coefficients. Constant-time testing cannot cover this, and we
   * hence have to manually verify the assembly.
   * We declassify prior the input data and mark the outputs as secret.
   */
  MLD_CT_TESTING_DECLASSIFY(buf, buflen);
  outlen = mld_rej_uniform_eta2_aarch64_asm(r, buf, buflen,
                                            mld_rej_uniform_eta_table);
  MLD_CT_TESTING_SECRET(r, sizeof(int32_t) * outlen);
  /* Safety: outlen is at most MLDSA_N and, hence, this cast is safe. */
  return (int)outlen;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLDSA_ETA == 2 */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLDSA_ETA == 4
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_rej_uniform_eta4_native(int32_t *r, unsigned len,
                                                  const uint8_t *buf,
                                                  unsigned buflen)
{
  uint64_t outlen;
  /* AArch64 implementation assumes specific buffer lengths */
  if (len != MLDSA_N || buflen != MLD_AARCH64_REJ_UNIFORM_ETA4_BUFLEN)
  {
    return MLD_NATIVE_FUNC_FALLBACK;
  }
  /* Constant time: Inputs and outputs to this function are secret.
   * It is safe to leak which coefficients are accepted/rejected.
   * The assembly implementation must not leak any other information about the
   * accepted coefficients. Constant-time testing cannot cover this, and we
   * hence have to manually verify the assembly.
   * We declassify prior the input data and mark the outputs as secret.
   */
  MLD_CT_TESTING_DECLASSIFY(buf, buflen);
  outlen = mld_rej_uniform_eta4_aarch64_asm(r, buf, buflen,
                                            mld_rej_uniform_eta_table);
  MLD_CT_TESTING_SECRET(r, sizeof(int32_t) * outlen);
  /* Safety: outlen is at most MLDSA_N and, hence, this cast is safe. */
  return (int)outlen;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLDSA_ETA == 4 */
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_decompose_32_native(int32_t *a1, int32_t *a0)
{
  mld_poly_decompose_32_aarch64_asm(a1, a0);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87 */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLD_CONFIG_PARAMETER_SET == 44
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_decompose_88_native(int32_t *a1, int32_t *a0)
{
  mld_poly_decompose_88_aarch64_asm(a1, a0);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 44 \
        */
#endif /* !MLD_CONFIG_NO_SIGN_API */

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_caddq_native(int32_t a[MLDSA_N])
{
  mld_poly_caddq_aarch64_asm(a);
  return MLD_NATIVE_FUNC_SUCCESS;
}

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_use_hint_32_native(int32_t *a, const int32_t *h)
{
  mld_poly_use_hint_32_aarch64_asm(a, h);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87 */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLD_CONFIG_PARAMETER_SET == 44
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_use_hint_88_native(int32_t *a, const int32_t *h)
{
  mld_poly_use_hint_88_aarch64_asm(a, h);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 44 \
        */
#endif /* !MLD_CONFIG_NO_VERIFY_API */

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_chknorm_native(const int32_t *a, int32_t B)
{
  return mld_poly_chknorm_aarch64_asm(a, B);
}

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLD_CONFIG_PARAMETER_SET == 44
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_polyz_unpack_17_native(int32_t *r, const uint8_t *buf)
{
  mld_polyz_unpack_17_aarch64_asm(r, buf, mld_polyz_unpack_17_indices);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 44 \
        */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_polyz_unpack_19_native(int32_t *r, const uint8_t *buf)
{
  mld_polyz_unpack_19_aarch64_asm(r, buf, mld_polyz_unpack_19_indices);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87 */
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API) || \
    defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_poly_pointwise_montgomery_native(
    int32_t a[MLDSA_N], const int32_t b[MLDSA_N])
{
  mld_poly_pointwise_montgomery_aarch64_asm(a, b);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API || \
          MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLDSA_L == 4
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_polyvecl_pointwise_acc_montgomery_l4_native(
    int32_t w[MLDSA_N], const int32_t u[4][MLDSA_N],
    const int32_t v[4][MLDSA_N])
{
  mld_polyvecl_pointwise_acc_montgomery_l4_aarch64_asm(w, u, v);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLDSA_L == 4 */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLDSA_L == 5
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_polyvecl_pointwise_acc_montgomery_l5_native(
    int32_t w[MLDSA_N], const int32_t u[5][MLDSA_N],
    const int32_t v[5][MLDSA_N])
{
  mld_polyvecl_pointwise_acc_montgomery_l5_aarch64_asm(w, u, v);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLDSA_L == 5 */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLDSA_L == 7
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_polyvecl_pointwise_acc_montgomery_l7_native(
    int32_t w[MLDSA_N], const int32_t u[7][MLDSA_N],
    const int32_t v[7][MLDSA_N])
{
  mld_polyvecl_pointwise_acc_montgomery_l7_aarch64_asm(w, u, v);
  return MLD_NATIVE_FUNC_SUCCESS;
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLDSA_L == 7 */

#endif /* !__ASSEMBLER__ */
#endif /* !MLD_NATIVE_AARCH64_META_H */
