// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_SPANIFY_TESTS_THIRD_PARTY_DO_NOT_REWRITE_THIRD_PARTY_API_H_
#define TOOLS_CLANG_SPANIFY_TESTS_THIRD_PARTY_DO_NOT_REWRITE_THIRD_PARTY_API_H_

extern int* GetThirdPartyBuffer();

// Implemented in first party.
class ThirdPartyInterface {
 public:
  virtual void ToBeImplemented(int arg[3]) = 0;
};

#endif  // TOOLS_CLANG_SPANIFY_TESTS_THIRD_PARTY_DO_NOT_REWRITE_THIRD_PARTY_API_H_
