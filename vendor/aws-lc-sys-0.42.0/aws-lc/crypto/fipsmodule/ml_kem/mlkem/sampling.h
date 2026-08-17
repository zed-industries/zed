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
 */

#ifndef MLK_SAMPLING_H
#define MLK_SAMPLING_H

#include "cbmc.h"
#include "common.h"
#include "poly.h"

#define mlk_poly_cbd2 MLK_NAMESPACE(poly_cbd2)
/*************************************************
 * Name:        mlk_poly_cbd2
 *
 * Description: Given an array of uniformly random bytes, compute
 *              polynomial with coefficients distributed according to
 *              a centered binomial distribution with parameter eta=2
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *buf: pointer to input byte array
 *
 * Specification: Implements @[FIPS203, Algorithm 8, SamplePolyCBD_2]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_cbd2(mlk_poly *r, const uint8_t buf[2 * MLKEM_N / 4]);

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_ETA1 == 3
#define mlk_poly_cbd3 MLK_NAMESPACE(poly_cbd3)
/*************************************************
 * Name:        mlk_poly_cbd3
 *
 * Description: Given an array of uniformly random bytes, compute
 *              polynomial with coefficients distributed according to
 *              a centered binomial distribution with parameter eta=3.
 *              This function is only needed for ML-KEM-512
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *buf: pointer to input byte array
 *
 * Specification: Implements @[FIPS203, Algorithm 8, SamplePolyCBD_3]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_cbd3(mlk_poly *r, const uint8_t buf[3 * MLKEM_N / 4]);
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_ETA1 == 3 */

#if !defined(MLK_CONFIG_SERIAL_FIPS202_ONLY)
#define mlk_poly_rej_uniform_x4 MLK_NAMESPACE(poly_rej_uniform_x4)
/*************************************************
 * Name:        mlk_poly_rej_uniform_x4
 *
 * Description: Generate four polynomials using rejection sampling
 *              on (pseudo-)uniformly random bytes sampled from a seed.
 *
 * Arguments:   - mlk_poly *vec0, *vec1, *vec2, *vec3:
 *                Pointers to 4 polynomials to be sampled.
 *              - uint8_t seed[4][MLK_ALIGN_UP(MLKEM_SYMBYTES + 2)]:
 *                Pointer consecutive array of seed buffers of size
 *                MLKEM_SYMBYTES + 2 each, plus padding for alignment.
 *
 * Specification: Implements @[FIPS203, Algorithm 7, SampleNTT]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_rej_uniform_x4(mlk_poly *vec0, mlk_poly *vec1, mlk_poly *vec2,
                             mlk_poly *vec3,
                             uint8_t seed[4][MLK_ALIGN_UP(MLKEM_SYMBYTES + 2)])
__contract__(
  requires(memory_no_alias(vec0, sizeof(mlk_poly)))
  requires(memory_no_alias(vec1, sizeof(mlk_poly)))
  requires(memory_no_alias(vec2, sizeof(mlk_poly)))
  requires(memory_no_alias(vec3, sizeof(mlk_poly)))
  requires(memory_no_alias(seed, 4 * MLK_ALIGN_UP(MLKEM_SYMBYTES + 2)))
  assigns(memory_slice(vec0, sizeof(mlk_poly)))
  assigns(memory_slice(vec1, sizeof(mlk_poly)))
  assigns(memory_slice(vec2, sizeof(mlk_poly)))
  assigns(memory_slice(vec3, sizeof(mlk_poly)))
  ensures(array_bound(vec0->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  ensures(array_bound(vec1->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  ensures(array_bound(vec2->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  ensures(array_bound(vec3->coeffs, 0, MLKEM_N, 0, MLKEM_Q)));
#endif /* !MLK_CONFIG_SERIAL_FIPS202_ONLY */

#define mlk_poly_rej_uniform MLK_NAMESPACE(poly_rej_uniform)
/*************************************************
 * Name:        mlk_poly_rej_uniform
 *
 * Description: Generate polynomial using rejection sampling
 *              on (pseudo-)uniformly random bytes sampled from a seed.
 *
 * Arguments:   - mlk_poly *vec:           Pointer to polynomial to be sampled.
 *              - uint8_t *seed:       Pointer to seed buffer of size
 *                                     MLKEM_SYMBYTES + 2 each.
 *
 * Specification: Implements @[FIPS203, Algorithm 7, SampleNTT]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_rej_uniform(mlk_poly *entry, uint8_t seed[MLKEM_SYMBYTES + 2])
__contract__(
  requires(memory_no_alias(entry, sizeof(mlk_poly)))
  requires(memory_no_alias(seed, MLKEM_SYMBYTES + 2))
  assigns(memory_slice(entry, sizeof(mlk_poly)))
  ensures(array_bound(entry->coeffs, 0, MLKEM_N, 0, MLKEM_Q)));

#endif /* !MLK_SAMPLING_H */
