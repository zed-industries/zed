// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_FRAGMENT_LINK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_FRAGMENT_LINK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_offset.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class PhysicalFragment;

// Similar to |PhysicalFragmentLink| but with |LogicalOffset| instead of
// |PhysicalOffset|.
struct CORE_EXPORT LogicalFragmentLink {
  DISALLOW_NEW();

 public:
  LogicalFragmentLink() = default;
  LogicalFragmentLink(const PhysicalFragment& fragment, LogicalOffset offset)
      : fragment(&fragment), offset(offset) {}

  const LogicalOffset& Offset() const { return offset; }
  const PhysicalFragment* get() const { return fragment.Get(); }

  explicit operator bool() const { return fragment != nullptr; }
  const PhysicalFragment& operator*() const { return *fragment; }
  const PhysicalFragment* operator->() const { return fragment.Get(); }

  void Trace(Visitor* visitor) const { visitor->Trace(fragment); }

  Member<const PhysicalFragment> fragment;
  LogicalOffset offset;
};

using LogicalFragmentLinkVector = HeapVector<LogicalFragmentLink, 4>;

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::LogicalFragmentLink)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_FRAGMENT_LINK_H_
