// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_RAW_PTR_TO_STACK_ALLOCATED_THIRD_PARTY_TEST_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_RAW_PTR_TO_STACK_ALLOCATED_THIRD_PARTY_TEST_H_

#include "base/memory/raw_ptr.h"

struct StackAllocatedType {
  using IsStackAllocatedTypeMarker [[maybe_unused]] = int;
};
struct StackAllocatedSubType : public StackAllocatedType {};
struct NonStackAllocatedType {};

// typedefs should be checked but excluded here
typedef raw_ptr<StackAllocatedType> ErrTypeA;
typedef raw_ptr<StackAllocatedSubType> ErrTypeB;
typedef raw_ptr<NonStackAllocatedType> OkTypeC;

// fields should be checked but excluded here
struct MyStruct {
  raw_ptr<StackAllocatedType> err_a;
  raw_ptr<StackAllocatedSubType> err_b;
  raw_ptr<NonStackAllocatedType> ok_c;
};

// variables should be checked but excluded here
void MyFunc() {
  raw_ptr<StackAllocatedType> err_a;
  raw_ptr<StackAllocatedSubType> err_b;
  raw_ptr<NonStackAllocatedType> ok_c;
}

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_RAW_PTR_TO_STACK_ALLOCATED_THIRD_PARTY_TEST_H_
