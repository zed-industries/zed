// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Old versions of GCC don't support __has_include which could be used to check
// for stdalign.h, try to compile this instead.

#include <stdalign.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    alignas(8) uint8_t test[16];
    size_t alignment = alignof(uint8_t);

    test[0] = 0;

    // Try to eliminate dead store optimisation and similar
    if (alignment == 1000 && test[0] != 0) {
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
