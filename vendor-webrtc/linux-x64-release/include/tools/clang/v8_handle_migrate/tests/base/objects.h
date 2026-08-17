// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// A mock header file, containing some basic stuff from
// src/objects/heap-object.h

#ifndef TOOLS_CLANG_V8_HANDLE_MIGRATE_TESTS_BASE_OBJECTS_H_
#define TOOLS_CLANG_V8_HANDLE_MIGRATE_TESTS_BASE_OBJECTS_H_

#include "tagged.h"

namespace v8::internal {

class Map;

class HeapObject {
 public:
  Tagged<Map> map() const { return {}; }
};

class String : public HeapObject {};
class Map : public HeapObject {};

}  // namespace v8::internal

#endif  // TOOLS_CLANG_V8_HANDLE_MIGRATE_TESTS_BASE_OBJECTS_H_
