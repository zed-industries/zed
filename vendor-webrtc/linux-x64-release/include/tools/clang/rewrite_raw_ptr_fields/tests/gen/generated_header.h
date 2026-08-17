// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_REWRITE_RAW_PTR_FIELDS_TESTS_GEN_GENERATED_HEADER_H_
#define TOOLS_CLANG_REWRITE_RAW_PTR_FIELDS_TESTS_GEN_GENERATED_HEADER_H_

class SomeClass;

// This file simulates a header that was generated during build.  This is
// simulated by virtue of being located inside a directory named "gen".
struct GeneratedStruct {
  // No rewrite expected inside generated code.
  int* ptr_field;
  SomeClass* ptr_field2;

  // No rewrite expected inside generated code.
  int& ref_field;
  SomeClass& ref_field2;
};

#endif  // TOOLS_CLANG_REWRITE_RAW_PTR_FIELDS_TESTS_GEN_GENERATED_HEADER_H_
