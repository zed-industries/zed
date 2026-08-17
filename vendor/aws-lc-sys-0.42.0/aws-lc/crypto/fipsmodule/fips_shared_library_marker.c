// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// The FIPS build on macOS/iOS/Windows is different than the build on Linux.
// Apple's and Windows' linkers don't support linker scripts so we have to build
// the module in a different way. This file is compiled twice:
//    - with AWSLC_FIPS_SHARED_START flag to generate the start marker object file
//    - with AWSLC_FIPS_SHARED_END flag to generate the end marker object file
// The two generated files are used to link with the module bcm.o such that
// the final module object has start and end markers for  __text and __const
// sections that are used for the integrity check.

#include <stdio.h>
#include <stdint.h>

#if defined(AWSLC_FIPS_SHARED_START)
#if defined(_MSC_VER)
#pragma code_seg(".fipstx$a")
#pragma data_seg(".fipsda$a")
#pragma const_seg(".fipsco$a")
#pragma bss_seg(".fipsbs$a")
// Declare the FIPS rodata section so that __declspec(allocate()) can reference
// it. This provides an explicit fallback in case #pragma const_seg is not fully
// supported by the compiler (e.g. clang-cl on ARM64).
#pragma section(".fipsco$a", read)
#endif

// Dummy but not empty function and array to avoid the compiler completely
// optimizing out the symbols.
const uint8_t *BORINGSSL_bcm_text_start(void) {
  return NULL;
}
#if defined(_MSC_VER)
__declspec(allocate(".fipsco$a"))
#endif
const uint8_t BORINGSSL_bcm_rodata_start[16] =
              {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15};

#elif defined(AWSLC_FIPS_SHARED_END)
#if defined(_MSC_VER)
#pragma code_seg(".fipstx$z")
#pragma data_seg(".fipsda$z")
#pragma const_seg(".fipsco$z")
#pragma bss_seg(".fipsbs$z")
// Declare the FIPS rodata section so that __declspec(allocate()) can reference
// it. This provides an explicit fallback in case #pragma const_seg is not fully
// supported by the compiler (e.g. clang-cl on ARM64).
#pragma section(".fipsco$z", read)
#endif

// Dummy but not empty function and array to avoid the compiler completely
// optimizing out the symbols.
const uint8_t *BORINGSSL_bcm_text_end(void){
  return NULL;
}
#if defined(_MSC_VER)
__declspec(allocate(".fipsco$z"))
#endif
const uint8_t BORINGSSL_bcm_rodata_end[16] =
              {16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31};

#else

#error "This file should be compiled only as part of the Shared FIPS build on macOS/iOS/Windows."

#endif
