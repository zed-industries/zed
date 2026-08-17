// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_CLEAR_COLLECTION_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_CLEAR_COLLECTION_SCOPE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

// Scope to automatically promptly free the backing for a stack allocated
// collection. Use this to reduce floating garbage. Collections with inline
// capacity are automatically promptly freed so they do not need this. Usage:
//   HeapVector<Foo> foo;
//   ClearCollectionScope<HeapVector<Foo>> clear_scope(&foo);
// TODO(keishi): Limit T to heap collections.
template <typename T>
class ClearCollectionScope final {
  STACK_ALLOCATED();

 public:
  explicit ClearCollectionScope(T* ptr) : ptr_(ptr) {}
  ~ClearCollectionScope() { ptr_->clear(); }

 private:
  T* ptr_;
};

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_CLEAR_COLLECTION_SCOPE_H_
