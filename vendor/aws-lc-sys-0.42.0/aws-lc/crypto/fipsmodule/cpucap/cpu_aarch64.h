// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CPUCAP_CPU_AARCH64_H
#define OPENSSL_HEADER_CPUCAP_CPU_AARCH64_H

#if defined(__cplusplus)
extern "C" {
#endif

#include <inttypes.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(OPENSSL_AARCH64) && !defined(OPENSSL_STATIC_ARMCAP)

// cpu_aarch64 contains common functions used across multiple cpu_aarch64_* files

// handle_cpu_env applies the value from |in| to the CPUID values in |out[0]|.
// See the comment in |OPENSSL_cpuid_setup| about this.
void handle_cpu_env(uint32_t *out, const char *in);

#endif // OPENSSL_AARCH64 && !OPENSSL_STATIC_ARMCAP

#if defined(__cplusplus)
}
#endif

#endif // OPENSSL_HEADER_CPUCAP_CPU_AARCH64_H
