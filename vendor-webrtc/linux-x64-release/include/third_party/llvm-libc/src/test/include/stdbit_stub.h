//===-- Utilities for testing stdbit --------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

/*
 * Declare these BEFORE including stdbit-macros.h so that this test may still be
 * run even if a given target doesn't yet have these individual entrypoints
 * enabled.
 */

#include "include/__llvm-libc-common.h"

#include <stdbool.h> // bool in C

#define STDBIT_STUB_FUNCTION(FUNC_NAME, LEADING_VAL)                           \
  unsigned FUNC_NAME##_uc(unsigned char x) { return LEADING_VAL##AU; }         \
  unsigned FUNC_NAME##_us(unsigned short x) { return LEADING_VAL##BU; }        \
  unsigned FUNC_NAME##_ui(unsigned int x) { return LEADING_VAL##CU; }          \
  unsigned FUNC_NAME##_ul(unsigned long x) { return LEADING_VAL##DU; }         \
  unsigned FUNC_NAME##_ull(unsigned long long x) { return LEADING_VAL##EU; }

__BEGIN_C_DECLS

STDBIT_STUB_FUNCTION(stdc_leading_zeros, 0xA)
STDBIT_STUB_FUNCTION(stdc_leading_ones, 0xB)
STDBIT_STUB_FUNCTION(stdc_trailing_zeros, 0xC)
STDBIT_STUB_FUNCTION(stdc_trailing_ones, 0xD)
STDBIT_STUB_FUNCTION(stdc_first_leading_zero, 0xE)
STDBIT_STUB_FUNCTION(stdc_first_leading_one, 0xF)
STDBIT_STUB_FUNCTION(stdc_first_trailing_zero, 0x0)
STDBIT_STUB_FUNCTION(stdc_first_trailing_one, 0x1)
STDBIT_STUB_FUNCTION(stdc_count_zeros, 0x2)
STDBIT_STUB_FUNCTION(stdc_count_ones, 0x3)

bool stdc_has_single_bit_uc(unsigned char x) { return false; }
bool stdc_has_single_bit_us(unsigned short x) { return false; }
bool stdc_has_single_bit_ui(unsigned x) { return false; }
bool stdc_has_single_bit_ul(unsigned long x) { return false; }
bool stdc_has_single_bit_ull(unsigned long long x) { return false; }

STDBIT_STUB_FUNCTION(stdc_bit_width, 0x4)

unsigned char stdc_bit_floor_uc(unsigned char x) { return 0x5AU; }
unsigned short stdc_bit_floor_us(unsigned short x) { return 0x5BU; }
unsigned stdc_bit_floor_ui(unsigned x) { return 0x5CU; }
unsigned long stdc_bit_floor_ul(unsigned long x) { return 0x5DUL; }
unsigned long long stdc_bit_floor_ull(unsigned long long x) { return 0x5EULL; }

unsigned char stdc_bit_ceil_uc(unsigned char x) { return 0x6AU; }
unsigned short stdc_bit_ceil_us(unsigned short x) { return 0x6BU; }
unsigned stdc_bit_ceil_ui(unsigned x) { return 0x6CU; }
unsigned long stdc_bit_ceil_ul(unsigned long x) { return 0x6DUL; }
unsigned long long stdc_bit_ceil_ull(unsigned long long x) { return 0x6EULL; }

__END_C_DECLS
