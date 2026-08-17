/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [FIPS203]
 *   FIPS 203 Module-Lattice-Based Key-Encapsulation Mechanism Standard
 *   National Institute of Standards and Technology
 *   https://csrc.nist.gov/pubs/fips/203/final
 *
 * - [REF]
 *   CRYSTALS-Kyber C reference implementation
 *   Bos, Ducas, Kiltz, Lepoint, Lyubashevsky, Schanck, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/kyber/tree/main/ref
 */

#ifndef MLK_KEM_H
#define MLK_KEM_H

#include "cbmc.h"
#include "common.h"
#include "sys.h"

#if defined(MLK_CHECK_APIS)
/* Include to ensure consistency between internal kem.h
 * and external mlkem_native.h. */
#include "mlkem_native.h"

#if MLKEM_INDCCA_SECRETKEYBYTES != \
    MLKEM_SECRETKEYBYTES(MLK_CONFIG_PARAMETER_SET)
#error Mismatch for SECRETKEYBYTES between kem.h and mlkem_native.h
#endif

#if MLKEM_INDCCA_PUBLICKEYBYTES != \
    MLKEM_PUBLICKEYBYTES(MLK_CONFIG_PARAMETER_SET)
#error Mismatch for PUBLICKEYBYTES between kem.h and mlkem_native.h
#endif

#if MLKEM_INDCCA_CIPHERTEXTBYTES != \
    MLKEM_CIPHERTEXTBYTES(MLK_CONFIG_PARAMETER_SET)
#error Mismatch for CIPHERTEXTBYTES between kem.h and mlkem_native.h
#endif

#endif /* MLK_CHECK_APIS */

#define mlk_kem_keypair_derand \
  MLK_NAMESPACE_K(keypair_derand) MLK_CONTEXT_PARAMETERS_3
#define mlk_kem_keypair MLK_NAMESPACE_K(keypair) MLK_CONTEXT_PARAMETERS_2
#define mlk_kem_enc_derand MLK_NAMESPACE_K(enc_derand) MLK_CONTEXT_PARAMETERS_4
#define mlk_kem_enc MLK_NAMESPACE_K(enc) MLK_CONTEXT_PARAMETERS_3
#define mlk_kem_dec MLK_NAMESPACE_K(dec) MLK_CONTEXT_PARAMETERS_3
#define mlk_kem_check_pk MLK_NAMESPACE_K(check_pk) MLK_CONTEXT_PARAMETERS_1
#define mlk_kem_check_sk MLK_NAMESPACE_K(check_sk) MLK_CONTEXT_PARAMETERS_1

/*************************************************
 * Name:        mlk_kem_check_pk
 *
 * Description: Implements modulus check mandated by FIPS 203,
 *              i.e., ensures that coefficients are in [0,q-1].
 *
 * Arguments:   - const uint8_t *pk: pointer to input public key
 *                (an already allocated array of MLKEM_INDCCA_PUBLICKEYBYTES
 *                 bytes)
 *
 * Returns: - 0 on success
 *          - MLK_ERR_FAIL: If the modulus check failed.
 *          - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *              used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *
 * Specification: Implements @[FIPS203, Section 7.2, 'modulus check']
 *
 **************************************************/

/* Reference: Not implemented in the reference implementation @[REF]. */
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_check_pk(const uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES],
                     MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY)
);


/*************************************************
 * Name:        mlk_kem_check_sk
 *
 * Description: Implements public key hash check mandated by FIPS 203,
 *              i.e., ensures that
 *              sk[768𝑘+32 ∶ 768𝑘+64] = H(pk)= H(sk[384𝑘 : 768𝑘+32])
 *
 * Arguments:   - const uint8_t *sk: pointer to input private key
 *                (an already allocated array of MLKEM_INDCCA_SECRETKEYBYTES
 *                 bytes)
 *
 * Returns: - 0 on success
 *          - MLK_ERR_FAIL: If the public key hash check failed.
 *          - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *              used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *
 * Specification: Implements @[FIPS203, Section 7.3, 'hash check']
 *
 **************************************************/

/* Reference: Not implemented in the reference implementation @[REF]. */
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_check_sk(const uint8_t sk[MLKEM_INDCCA_SECRETKEYBYTES],
                     MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(sk, MLKEM_INDCCA_SECRETKEYBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY)
);

/*************************************************
 * Name:        mlk_kem_keypair_derand
 *
 * Description: Generates public and private key
 *              for CCA-secure ML-KEM key encapsulation mechanism
 *
 * Arguments:   - uint8_t *pk: pointer to output public key
 *                (an already allocated array of MLKEM_INDCCA_PUBLICKEYBYTES
 *                 bytes)
 *              - uint8_t *sk: pointer to output private key
 *                (an already allocated array of MLKEM_INDCCA_SECRETKEYBYTES
 *                 bytes)
 *              - uint8_t *coins: pointer to input randomness
 *                (an already allocated array filled with 2*MLKEM_SYMBYTES
 *                 random bytes)
 *
 * Returns:     - 0: On success
 *              - MLK_ERR_FAIL: If MLK_CONFIG_KEYGEN_PCT is enabled and the
 *                  PCT failed.
 *              - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *                  used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *
 * Specification: Implements @[FIPS203, Algorithm 16, ML-KEM.KeyGen_Internal]
 *
 **************************************************/
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_keypair_derand(uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES],
                           uint8_t sk[MLKEM_INDCCA_SECRETKEYBYTES],
                           const uint8_t coins[2 * MLKEM_SYMBYTES],
                           MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  requires(memory_no_alias(sk, MLKEM_INDCCA_SECRETKEYBYTES))
  requires(memory_no_alias(coins, 2 * MLKEM_SYMBYTES))
  assigns(memory_slice(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  assigns(memory_slice(sk, MLKEM_INDCCA_SECRETKEYBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY ||
          return_value == MLK_ERR_RNG_FAIL)
);

/*************************************************
 * Name:        mlk_kem_keypair
 *
 * Description: Generates public and private key
 *              for CCA-secure ML-KEM key encapsulation mechanism
 *
 * Arguments:   - uint8_t *pk: pointer to output public key
 *                (an already allocated array of MLKEM_INDCCA_PUBLICKEYBYTES
 *                 bytes)
 *              - uint8_t *sk: pointer to output private key
 *                (an already allocated array of MLKEM_INDCCA_SECRETKEYBYTES
 *                 bytes)
 *
 * Returns:     - 0: On success
 *              - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *                  used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *              - MLK_ERR_RNG_FAIL: Random number generation failed.
 *              - MLK_ERR_FAIL: If MLK_CONFIG_KEYGEN_PCT is enabled and the
 *                  PCT failed.
 *
 * Specification: Implements @[FIPS203, Algorithm 19, ML-KEM.KeyGen]
 *
 **************************************************/
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_keypair(uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES],
                    uint8_t sk[MLKEM_INDCCA_SECRETKEYBYTES],
                    MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  requires(memory_no_alias(sk, MLKEM_INDCCA_SECRETKEYBYTES))
  assigns(memory_slice(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  assigns(memory_slice(sk, MLKEM_INDCCA_SECRETKEYBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY ||
          return_value == MLK_ERR_RNG_FAIL)
);

/*************************************************
 * Name:        mlk_kem_enc_derand
 *
 * Description: Generates cipher text and shared
 *              secret for given public key
 *
 * Arguments:   - uint8_t *ct: pointer to output cipher text
 *                (an already allocated array of MLKEM_INDCCA_CIPHERTEXTBYTES
 *                 bytes)
 *              - uint8_t *ss: pointer to output shared secret
 *                (an already allocated array of MLKEM_SSBYTES bytes)
 *              - const uint8_t *pk: pointer to input public key
 *                (an already allocated array of MLKEM_INDCCA_PUBLICKEYBYTES
 *                 bytes)
 *              - const uint8_t *coins: pointer to input randomness
 *                (an already allocated array filled with MLKEM_SYMBYTES random
 *                 bytes)
 *
 * Returns: - 0 on success
 *          - MLK_ERR_FAIL: If the 'modulus check' @[FIPS203, Section 7.2]
 *              for the public key fails.
 *          - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *              used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *
 * Specification: Implements @[FIPS203, Algorithm 17, ML-KEM.Encaps_Internal]
 *
 **************************************************/
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_enc_derand(uint8_t ct[MLKEM_INDCCA_CIPHERTEXTBYTES],
                       uint8_t ss[MLKEM_SSBYTES],
                       const uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES],
                       const uint8_t coins[MLKEM_SYMBYTES],
                       MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(ct, MLKEM_INDCCA_CIPHERTEXTBYTES))
  requires(memory_no_alias(ss, MLKEM_SSBYTES))
  requires(memory_no_alias(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  requires(memory_no_alias(coins, MLKEM_SYMBYTES))
  assigns(memory_slice(ct, MLKEM_INDCCA_CIPHERTEXTBYTES))
  assigns(memory_slice(ss, MLKEM_SSBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY)
);

/*************************************************
 * Name:        mlk_kem_enc
 *
 * Description: Generates cipher text and shared
 *              secret for given public key
 *
 * Arguments:   - uint8_t *ct: pointer to output cipher text
 *                (an already allocated array of MLKEM_INDCCA_CIPHERTEXTBYTES
 *                 bytes)
 *              - uint8_t *ss: pointer to output shared secret
 *                (an already allocated array of MLKEM_SSBYTES bytes)
 *              - const uint8_t *pk: pointer to input public key
 *                (an already allocated array of MLKEM_INDCCA_PUBLICKEYBYTES
 *                 bytes)
 *
 * Returns: - 0 on success
 *          - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *              used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *          - MLK_ERR_RNG_FAIL: Random number generation failed.
 *          - MLK_ERR_FAIL: If the 'modulus check' @[FIPS203, Section 7.2]
 *              for the public key fails.
 *
 * Specification: Implements @[FIPS203, Algorithm 20, ML-KEM.Encaps]
 *
 **************************************************/
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_enc(uint8_t ct[MLKEM_INDCCA_CIPHERTEXTBYTES],
                uint8_t ss[MLKEM_SSBYTES],
                const uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES],
                MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(ct, MLKEM_INDCCA_CIPHERTEXTBYTES))
  requires(memory_no_alias(ss, MLKEM_SSBYTES))
  requires(memory_no_alias(pk, MLKEM_INDCCA_PUBLICKEYBYTES))
  assigns(memory_slice(ct, MLKEM_INDCCA_CIPHERTEXTBYTES))
  assigns(memory_slice(ss, MLKEM_SSBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY ||
          return_value == MLK_ERR_RNG_FAIL)
);

/*************************************************
 * Name:        mlk_kem_dec
 *
 * Description: Generates shared secret for given
 *              cipher text and private key
 *
 * Arguments:   - uint8_t *ss: pointer to output shared secret
 *                (an already allocated array of MLKEM_SSBYTES bytes)
 *              - const uint8_t *ct: pointer to input cipher text
 *                (an already allocated array of MLKEM_INDCCA_CIPHERTEXTBYTES
 *                 bytes)
 *              - const uint8_t *sk: pointer to input private key
 *                (an already allocated array of MLKEM_INDCCA_SECRETKEYBYTES
 *                 bytes)
 *
 * Returns: - 0 on success
 *          - MLK_ERR_FAIL: If the 'hash check' @[FIPS203, Section 7.3]
 *              for the secret key fails.
 *          - MLK_ERR_OUT_OF_MEMORY: If MLK_CONFIG_CUSTOM_ALLOC_FREE is
 *              used and an allocation via MLK_CUSTOM_ALLOC returned NULL.
 *
 * Specification: Implements @[FIPS203, Algorithm 21, ML-KEM.Decaps]
 *
 **************************************************/
MLK_EXTERNAL_API
MLK_MUST_CHECK_RETURN_VALUE
int mlk_kem_dec(uint8_t ss[MLKEM_SSBYTES],
                const uint8_t ct[MLKEM_INDCCA_CIPHERTEXTBYTES],
                const uint8_t sk[MLKEM_INDCCA_SECRETKEYBYTES],
                MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
__contract__(
  requires(memory_no_alias(ss, MLKEM_SSBYTES))
  requires(memory_no_alias(ct, MLKEM_INDCCA_CIPHERTEXTBYTES))
  requires(memory_no_alias(sk, MLKEM_INDCCA_SECRETKEYBYTES))
  assigns(memory_slice(ss, MLKEM_SSBYTES))
  ensures(return_value == 0 || return_value == MLK_ERR_FAIL ||
          return_value == MLK_ERR_OUT_OF_MEMORY)
);

#endif /* !MLK_KEM_H */
