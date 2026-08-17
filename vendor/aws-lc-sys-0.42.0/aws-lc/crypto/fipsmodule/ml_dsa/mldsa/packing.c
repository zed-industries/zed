/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#include <string.h>

#include "common.h"
#include "packing.h"
#include "poly.h"
#include "polyvec.h"
#include "rounding.h"

/* Parameter set namespacing
 * This is to facilitate building multiple instances
 * of mldsa-native (e.g. with varying parameter sets)
 * within a single compilation unit. */
/* End of parameter set namespacing */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_INTERNAL_API
void mld_unpack_pk_t1(mld_poly *t1,
                      const uint8_t pk[MLDSA_CRYPTO_PUBLICKEYBYTES],
                      unsigned int i)
{
  mld_polyt1_unpack(t1, pk + MLDSA_PK_T1_OFFSET + i * MLDSA_POLYT1_PACKEDBYTES);
}
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
MLD_INTERNAL_API
void mld_pack_sk_s1(uint8_t sk[MLDSA_CRYPTO_SECRETKEYBYTES],
                    const mld_polyvecl *s1)
{
  mld_polyvecl_pack_eta(sk + MLDSA_SK_S1_OFFSET, s1);
}

MLD_INTERNAL_API
void mld_pack_sk_rho_key_tr_s2(uint8_t sk[MLDSA_CRYPTO_SECRETKEYBYTES],
                               const uint8_t rho[MLDSA_SEEDBYTES],
                               const uint8_t tr[MLDSA_TRBYTES],
                               const uint8_t key[MLDSA_SEEDBYTES],
                               const mld_polyveck *s2)
{
  mld_memcpy(sk + MLDSA_SK_RHO_OFFSET, rho, MLDSA_SEEDBYTES);
  mld_memcpy(sk + MLDSA_SK_KEY_OFFSET, key, MLDSA_SEEDBYTES);
  mld_memcpy(sk + MLDSA_SK_TR_OFFSET, tr, MLDSA_TRBYTES);
  /* s1 already packed via mld_pack_sk_s1 */
  mld_polyveck_pack_eta(sk + MLDSA_SK_S2_OFFSET, s2);
  /* t0 already packed via mld_compute_pack_t0_t1 */
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
MLD_INTERNAL_API
void mld_unpack_sk(uint8_t rho[MLDSA_SEEDBYTES], uint8_t tr[MLDSA_TRBYTES],
                   uint8_t key[MLDSA_SEEDBYTES], mld_sk_t0hat *t0,
                   mld_sk_s1hat *s1, mld_sk_s2hat *s2,
                   const uint8_t sk[MLDSA_CRYPTO_SECRETKEYBYTES])
{
  mld_memcpy(rho, sk + MLDSA_SK_RHO_OFFSET, MLDSA_SEEDBYTES);
  mld_memcpy(key, sk + MLDSA_SK_KEY_OFFSET, MLDSA_SEEDBYTES);
  mld_memcpy(tr, sk + MLDSA_SK_TR_OFFSET, MLDSA_TRBYTES);
  mld_unpack_sk_s1hat(s1, sk + MLDSA_SK_S1_OFFSET);
  mld_unpack_sk_s2hat(s2, sk + MLDSA_SK_S2_OFFSET);
  mld_unpack_sk_t0hat(t0, sk + MLDSA_SK_T0_OFFSET);
}

MLD_INTERNAL_API
void mld_pack_sig_c(uint8_t sig[MLDSA_CRYPTO_BYTES],
                    const uint8_t c[MLDSA_CTILDEBYTES])
{
  mld_memcpy(sig, c, MLDSA_CTILDEBYTES);
}

MLD_INTERNAL_API
int mld_pack_sig_h(uint8_t sig[MLDSA_CRYPTO_BYTES], const mld_polyveck *w0,
                   const mld_polyveck *w1)
{
  unsigned int j, k, n;

  /* The hint section of sig[] is MLDSA_POLYVECH_PACKEDBYTES long, where
   * MLDSA_POLYVECH_PACKEDBYTES = MLDSA_OMEGA + MLDSA_K.
   *
   * The first OMEGA bytes record the index numbers of the coefficients
   * that are not equal to 0.
   *
   * The final K bytes record a running tally of the number of hints
   * coming from each of the K polynomials. */
  uint8_t *sig_h = sig + MLDSA_SIG_H_OFFSET;

  mld_memset(sig_h, 0, MLDSA_POLYVECH_PACKEDBYTES);
  n = 0;

  /* For each coefficient of each polynomial, compute its hint bit and, if
   * non-zero, record the index in the hint section of sig. If recording the
   * hint would overflow the OMEGA-sized index array, abort early and return
   * MLD_ERR_FAIL. The caller is expected to reject the signature in that case.
   *
   * Constant time: At this point w0/w1 are public (see comment in sign.c
   * before the call), so a data-dependent early return is fine. */
  for (k = 0; k < MLDSA_K; k++)
  __loop__(
    assigns(k, j, n, memory_slice(sig_h, MLDSA_POLYVECH_PACKEDBYTES))
    invariant(k <= MLDSA_K && n <= MLDSA_OMEGA)
    decreases(MLDSA_K - k)
  )
  {
    for (j = 0; j < MLDSA_N; j++)
    __loop__(
      assigns(j, n, memory_slice(sig_h, MLDSA_POLYVECH_PACKEDBYTES))
      invariant(j <= MLDSA_N && n <= MLDSA_OMEGA)
      decreases(MLDSA_N - j)
    )
    {
      const unsigned int hint_bit =
          mld_make_hint(w0->vec[k].coeffs[j], w1->vec[k].coeffs[j]);
      if (hint_bit)
      {
        if (n == MLDSA_OMEGA)
        {
          return MLD_ERR_FAIL;
        }
        /* Safety: branch above ensures n < MLDSA_OMEGA so n is a valid index
         * into the OMEGA-sized index array; j < MLDSA_N <= 256 fits in
         * uint8_t. */
        sig_h[n] = (uint8_t)j;
        n++;
      }
    }
    /* Record the running tally into the correct slot for this polynomial.
     * Safety: k < MLDSA_K, so MLDSA_OMEGA + k is a valid index into the
     * K-byte tally tail; n <= MLDSA_OMEGA fits in uint8_t. */
    sig_h[MLDSA_OMEGA + k] = (uint8_t)n;
  }
  return 0;
}

MLD_INTERNAL_API
void mld_pack_sig_z(uint8_t sig[MLDSA_CRYPTO_BYTES], const mld_poly *zi,
                    unsigned i)
{
  mld_polyz_pack(sig + MLDSA_SIG_Z_OFFSET + i * MLDSA_POLYZ_PACKEDBYTES, zi);
}
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_INTERNAL_API
int mld_sig_unpack_hints(mld_poly *h, const uint8_t sig[MLDSA_CRYPTO_BYTES],
                         unsigned int i)
{
  const uint8_t *packed_hints = sig + MLDSA_SIG_H_OFFSET;
  const unsigned int old_hint_count =
      (i == 0) ? 0 : packed_hints[MLDSA_OMEGA + i - 1];
  const unsigned int new_hint_count = packed_hints[MLDSA_OMEGA + i];
  unsigned int j;

  if (new_hint_count < old_hint_count || new_hint_count > MLDSA_OMEGA)
  {
    return MLD_ERR_FAIL;
  }

  mld_memset(h, 0, sizeof(mld_poly));

  for (j = old_hint_count; j < new_hint_count; ++j)
  __loop__(
    invariant(j >= old_hint_count && j <= new_hint_count &&
              new_hint_count <= MLDSA_OMEGA)
    invariant(array_bound(h->coeffs, 0, MLDSA_N, 0, 2))
    decreases(new_hint_count - j)
  )
  {
    if (j > old_hint_count && packed_hints[j] <= packed_hints[j - 1])
    {
      return MLD_ERR_FAIL;
    }
    /* Safety: packed_hints[j] is uint8_t (<= 255) and MLDSA_N == 256. */
    h->coeffs[packed_hints[j]] = 1;
  }

  /* On the last row, also verify that the trailing index slots are zero. */
  if (i == MLDSA_K - 1)
  {
    for (j = new_hint_count; j < MLDSA_OMEGA; ++j)
    __loop__(
      invariant(j <= MLDSA_OMEGA)
      decreases(MLDSA_OMEGA - j)
    )
    {
      if (packed_hints[j] != 0)
      {
        return MLD_ERR_FAIL;
      }
    }
  }

  return 0;
}
#endif /* !MLD_CONFIG_NO_VERIFY_API */

/* To facilitate single-compilation-unit (SCU) builds, undefine all macros.
 * Don't modify by hand -- this is auto-generated by scripts/autogen. */
