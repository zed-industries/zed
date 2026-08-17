// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Build-time FIPS probe for the system-library path.
//
// The exit code below must stay in sync with FIPS_MODE_OFF_EXIT_CODE in
// builder/fips_probe.rs.

#include <openssl/crypto.h>

// Not declared in public headers; exported only by FIPS builds.
extern int BORINGSSL_integrity_test(void);

int main(void) {
    (void)BORINGSSL_integrity_test();
    return FIPS_mode() ? 0 : 42;
}
