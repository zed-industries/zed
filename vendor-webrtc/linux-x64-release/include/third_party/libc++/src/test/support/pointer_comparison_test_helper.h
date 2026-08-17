//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef POINTER_COMPARISON_TEST_HELPER_H
#define POINTER_COMPARISON_TEST_HELPER_H

#include <cstdint>
#include <cassert>

#include "test_macros.h"

template <template <class> class CompareTemplate>
void do_pointer_comparison_test() {
  typedef CompareTemplate<int*> Compare;
  typedef CompareTemplate<std::uintptr_t> UIntCompare;
#if TEST_STD_VER > 11
  typedef CompareTemplate<void> VoidCompare;
#else
  typedef Compare VoidCompare;
#endif

  Compare comp;
  UIntCompare ucomp;
  VoidCompare vcomp;
  struct {
    int a, b;
  } local;
  int* pointers[] = {&local.a, &local.b, nullptr, &local.a + 1};
  for (int* lhs : pointers) {
    for (int* rhs : pointers) {
      std::uintptr_t lhs_uint = reinterpret_cast<std::uintptr_t>(lhs);
      std::uintptr_t rhs_uint = reinterpret_cast<std::uintptr_t>(rhs);
      assert(comp(lhs, rhs) == ucomp(lhs_uint, rhs_uint));
      assert(vcomp(lhs, rhs) == ucomp(lhs_uint, rhs_uint));
    }
  }
}

template <class Comp>
void do_pointer_comparison_test(Comp comp) {
  struct {
    int a, b;
  } local;
  int* pointers[] = {&local.a, &local.b, nullptr, &local.a + 1};
  for (int* lhs : pointers) {
    for (int* rhs : pointers) {
      std::uintptr_t lhs_uint = reinterpret_cast<std::uintptr_t>(lhs);
      std::uintptr_t rhs_uint = reinterpret_cast<std::uintptr_t>(rhs);
      void*          lhs_void = static_cast<void*>(lhs);
      void*          rhs_void = static_cast<void*>(rhs);
      assert(comp(lhs, rhs) == comp(lhs_uint, rhs_uint));
      assert(comp(lhs_void, rhs_void) == comp(lhs_uint, rhs_uint));
    }
  }
}

#endif // POINTER_COMPARISON_TEST_HELPER_H
