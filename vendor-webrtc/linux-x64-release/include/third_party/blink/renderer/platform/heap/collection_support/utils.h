// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_UTILS_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink::internal {

// Internal only. Specifies whether a collection type should be GarbageCollected
// or DISALLOW_NEW().
enum class HeapCollectionType { kGCed, kDisallowNew };

// Internal only. Base class for DISALLOW_NEW() objects. Used for on-stack and
// field collections.
class DisallowNewBaseForHeapCollections {
  DISALLOW_NEW();
};

template <typename T>
struct CompactionTraits;

}  // namespace blink::internal

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_UTILS_H_
