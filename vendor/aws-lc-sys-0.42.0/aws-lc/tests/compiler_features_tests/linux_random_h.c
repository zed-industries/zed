// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// This file checks if <linux/random.h> can be included. Currently, we assume
// that the compiler error is caused by '__u32' not being defined.
// Theory:
// crypto/fipsmodule/rand/urandom.c includes the <linux/random.h> Linux header.
// Some old Linux OS does not define '__u32' causing the following error:
// /usr/include/linux/random.h:38:2: error: unknown type name '__u32'
// __u32 buf[0];
// ^
#if defined(DEFINE_U32)
typedef unsigned int __u32;
#endif
#include <linux/random.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    return EXIT_SUCCESS;
}
