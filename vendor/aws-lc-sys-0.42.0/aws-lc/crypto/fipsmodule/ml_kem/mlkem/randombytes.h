/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLK_RANDOMBYTES_H
#define MLK_RANDOMBYTES_H


#include "cbmc.h"
#include "common.h"

#if !defined(MLK_CONFIG_NO_RANDOMIZED_API)
#if !defined(MLK_CONFIG_CUSTOM_RANDOMBYTES)
/*************************************************
 * Name:        randombytes
 *
 * Description: Fill a buffer with cryptographically secure random bytes.
 *
 *              mlkem-native does not provide an implementation of this
 *              function. It must be provided by the consumer.
 *
 *              To use a custom random byte source with a different name
 *              or signature, set MLK_CONFIG_CUSTOM_RANDOMBYTES and define
 *              mlk_randombytes directly.
 *
 * Arguments:   - uint8_t *out: pointer to output buffer
 *              - size_t outlen: number of random bytes to write
 *
 * Returns:     0 on success, non-zero on failure.
 *              On failure, top-level APIs return MLK_ERR_RNG_FAIL.
 *
 **************************************************/
int randombytes(uint8_t *out, size_t outlen);

/*************************************************
 * Name:        mlk_randombytes
 *
 * Description: Internal wrapper around randombytes().
 *
 *              Fill a buffer with cryptographically secure random bytes.
 *
 *              This function can be replaced by setting
 *              MLK_CONFIG_CUSTOM_RANDOMBYTES and defining mlk_randombytes
 *              directly.
 *
 * Arguments:   - uint8_t *out: pointer to output buffer
 *              - size_t outlen: number of random bytes to write
 *
 * Returns:     0 on success, non-zero on failure.
 *              On failure, top-level APIs return MLK_ERR_RNG_FAIL.
 *
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_randombytes(uint8_t *out, size_t outlen)
__contract__(
  requires(memory_no_alias(out, outlen))
  assigns(memory_slice(out, outlen))) { return randombytes(out, outlen); }
#endif /* !MLK_CONFIG_CUSTOM_RANDOMBYTES */
#endif /* !MLK_CONFIG_NO_RANDOMIZED_API */
#endif /* !MLK_RANDOMBYTES_H */
