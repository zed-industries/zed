// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

// Ensure that no ambient #pragma const_seg is active. BORINGSSL_bcm_text_hash
// MUST be placed in the default .rdata section, outside the FIPS module rodata
// boundary (.fipsco). If it were placed inside the FIPS rodata boundary, the
// integrity check would hash the expected value itself, creating a circular
// dependency that can never be satisfied.
#if defined(_MSC_VER)
#pragma const_seg()
#endif

#if defined(BORINGSSL_FIPS) && defined(BORINGSSL_SHARED_LIBRARY)
#include <stdint.h>

// BORINGSSL_bcm_text_hash is is default hash value for the FIPS integrity check
// that must be replaced with the real value during the build process. This
// value need only be distinct, i.e. so that we can safely search-and-replace it
// in an object file.
const uint8_t BORINGSSL_bcm_text_hash[32] = {
    0xae, 0x2c, 0xea, 0x2a, 0xbd, 0xa6, 0xf3, 0xec, 0x97, 0x7f, 0x9b,
    0xf6, 0x94, 0x9a, 0xfc, 0x83, 0x68, 0x27, 0xcb, 0xa0, 0xa0, 0x9f,
    0x6b, 0x6f, 0xde, 0x52, 0xcd, 0xe2, 0xcd, 0xff, 0x31, 0x80,
};
#else
// C requires a translation unit to contain at least one declaration. Since
// BORINGSSL_FIPS or BORINGSSL_SHARED_LIBRARY is not defined, this file is
// otherwise empty. This typedef prevents MSVC warning C4206.
typedef int fips_shared_support_dummy;
#endif  // FIPS && SHARED_LIBRARY
