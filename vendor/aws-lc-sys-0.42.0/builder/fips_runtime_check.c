// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Runtime FIPS-mode self-check for the system-library path.

#include <openssl/crypto.h>
#include <stdio.h>
#include <stdlib.h>

static void aws_lc_fips_sys_runtime_check(void) {
    if (!FIPS_mode()) {
        fprintf(stderr,
            "aws-lc-fips-sys: linked libcrypto is not running in FIPS mode; aborting.\n");
        abort();
    }
}

#if defined(_MSC_VER)

// MSVC uses `.CRT$XCU` instead of `__attribute__((constructor))`.
#pragma section(".CRT$XCU", read)
__declspec(allocate(".CRT$XCU"))
void (*aws_lc_fips_sys_runtime_check_assert_fips_mode_v1)(void) =
    aws_lc_fips_sys_runtime_check;
#if defined(_M_IX86)
#pragma comment(linker, "/include:_aws_lc_fips_sys_runtime_check_assert_fips_mode_v1")
#else
#pragma comment(linker, "/include:aws_lc_fips_sys_runtime_check_assert_fips_mode_v1")
#endif

#else

// On ELF/Mach-O this runs via `.init_array`. Uses a late priority so that
// library constructors (including libcrypto's own init) run first.
__attribute__((constructor(65535)))
void aws_lc_fips_sys_runtime_check_assert_fips_mode_v1(void) {
    aws_lc_fips_sys_runtime_check();
}

#endif
