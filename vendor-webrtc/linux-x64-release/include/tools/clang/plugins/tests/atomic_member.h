// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_ATOMIC_MEMBER_H_
#define TOOLS_CLANG_PLUGINS_TESTS_ATOMIC_MEMBER_H_

#include <atomic>
#include <memory>

// The standard says that std::atomic<Integral> (where Integral is a built-in
// integral type, including raw pointers) is a standard-layout struct, and has a
// trivial destructor:
// https://eel.is/c++draft/atomics.types.generic#atomics.types.int-2 Because of
// that, we classify std::atomic<Integral> as a trivial template.

struct NineAtomicIntAliasesOk {
  std::atomic_int one;
  std::atomic_int two;
  std::atomic_int three;
  std::atomic_int four;
  std::atomic_int five;
  std::atomic_int six;
  std::atomic_int seven;
  std::atomic_int eight;
  std::atomic_int nine;
};

struct TenAtomicIntAliasesWarns {
  std::atomic_int one;
  std::atomic_int two;
  std::atomic_int three;
  std::atomic_int four;
  std::atomic_int five;
  std::atomic_int six;
  std::atomic_int seven;
  std::atomic_int eight;
  std::atomic_int nine;
  std::atomic_int ten;
};

struct NineAtomicIntTemplatesOk {
  std::atomic<int> one;
  std::atomic<int> two;
  std::atomic<int> three;
  std::atomic<int> four;
  std::atomic<int> five;
  std::atomic<int> six;
  std::atomic<int> seven;
  std::atomic<int> eight;
  std::atomic<int> nine;
};

struct TenAtomicIntTemplatesWarns {
  std::atomic<int> one;
  std::atomic<int> two;
  std::atomic<int> three;
  std::atomic<int> four;
  std::atomic<int> five;
  std::atomic<int> six;
  std::atomic<int> seven;
  std::atomic<int> eight;
  std::atomic<int> nine;
  std::atomic<int> ten;
};

struct NineAtomicPtrsOk {
  std::atomic<int*> one;
  std::atomic<int*> two;
  std::atomic<int*> three;
  std::atomic<int*> four;
  std::atomic<int*> five;
  std::atomic<int*> six;
  std::atomic<int*> seven;
  std::atomic<int*> eight;
  std::atomic<int*> nine;
};

struct TenAtomicPtrsWarns {
  std::atomic<int*> one;
  std::atomic<int*> two;
  std::atomic<int*> three;
  std::atomic<int*> four;
  std::atomic<int*> five;
  std::atomic<int*> six;
  std::atomic<int*> seven;
  std::atomic<int*> eight;
  std::atomic<int*> nine;
  std::atomic<int*> ten;
};

struct OneAtomicSharedPtrWarns {
  std::atomic<std::shared_ptr<int>> one;
};

#endif  // TOOLS_CLANG_PLUGINS_TESTS_ATOMIC_MEMBER_H_
