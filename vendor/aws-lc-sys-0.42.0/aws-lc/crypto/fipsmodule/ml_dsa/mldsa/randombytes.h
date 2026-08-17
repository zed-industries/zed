/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_RANDOMBYTES_H
#define MLD_RANDOMBYTES_H

#include <stddef.h>

#include "cbmc.h"
#include "common.h"

#if !defined(MLD_CONFIG_NO_RANDOMIZED_API)
#if !defined(MLD_CONFIG_CUSTOM_RANDOMBYTES)
MLD_MUST_CHECK_RETURN_VALUE
int randombytes(uint8_t *out, size_t outlen);

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int mld_randombytes(uint8_t *out, size_t outlen)
__contract__(
  requires(memory_no_alias(out, outlen))
  assigns(memory_slice(out, outlen))
) { return randombytes(out, outlen); }
#endif /* !MLD_CONFIG_CUSTOM_RANDOMBYTES */
#endif /* !MLD_CONFIG_NO_RANDOMIZED_API */
#endif /* !MLD_RANDOMBYTES_H */
