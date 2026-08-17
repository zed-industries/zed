// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_DEPTH_ORDERED_LAYOUT_OBJECT_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_DEPTH_ORDERED_LAYOUT_OBJECT_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"

namespace blink {

class LayoutObject;

// Put data inside a forward-declared struct, to avoid including
// layout_object.h.
class DepthOrderedLayoutObjectListData;

struct LayoutObjectWithDepth {
  DISALLOW_NEW();

 public:
  explicit LayoutObjectWithDepth(LayoutObject* in_object)
      : object(in_object), depth(DetermineDepth(in_object)) {}
  LayoutObjectWithDepth() = default;
  void Trace(Visitor*) const;

  Member<LayoutObject> object = nullptr;
  unsigned depth = 0u;

  LayoutObject& operator*() const { return *object; }
  LayoutObject* operator->() const { return object.Get(); }

  bool operator<(const LayoutObjectWithDepth& other) const {
    return depth > other.depth;
  }

 private:
  static unsigned DetermineDepth(LayoutObject*);
};

class DepthOrderedLayoutObjectList {
  DISALLOW_NEW();

 public:
  DepthOrderedLayoutObjectList();
  ~DepthOrderedLayoutObjectList();
  void Trace(Visitor*) const;

  void Add(LayoutObject&);
  void Remove(LayoutObject&);
  void Clear();

  int size() const;
  CORE_EXPORT bool IsEmpty() const;

  const HeapHashSet<Member<LayoutObject>>& Unordered() const;
  const HeapVector<LayoutObjectWithDepth>& Ordered();

 private:
  Member<DepthOrderedLayoutObjectListData> data_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::LayoutObjectWithDepth)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_DEPTH_ORDERED_LAYOUT_OBJECT_LIST_H_
