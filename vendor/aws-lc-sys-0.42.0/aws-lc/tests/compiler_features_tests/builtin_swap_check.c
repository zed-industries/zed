// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Builtin swap was added in GCC 4.8 https://gcc.gnu.org/gcc-4.8/changes.html.
// Recent versions of Clang pretend to be 4.2 but they do support the builtin
// swap functions.

#include <stdint.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    uint16_t test16 = 0;
    test16 = __builtin_bswap16(test16);

    uint32_t test32 = 0;
    test32 = __builtin_bswap32(test32);

    uint64_t test64 = 0;
    test64 = __builtin_bswap64(test64);

    return EXIT_SUCCESS;
}
