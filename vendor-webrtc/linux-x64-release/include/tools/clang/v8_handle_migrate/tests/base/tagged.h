// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A mock header file, containing some basic stuff from src/objects/tagged.h

#ifndef TOOLS_CLANG_V8_HANDLE_MIGRATE_TESTS_BASE_TAGGED_H_
#define TOOLS_CLANG_V8_HANDLE_MIGRATE_TESTS_BASE_TAGGED_H_

namespace v8::internal {

template <typename T>
class Tagged {
 public:
  T* operator->() { return nullptr; }
  const T* operator->() const { return nullptr; }
};

}  // namespace v8::internal

#endif  // TOOLS_CLANG_V8_HANDLE_MIGRATE_TESTS_BASE_TAGGED_H_
