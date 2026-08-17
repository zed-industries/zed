// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LEADING_FLOATS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LEADING_FLOATS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/floats_utils.h"
#include "third_party/blink/renderer/core/layout/positioned_float.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

//
// Represents already handled leading floats within an inline formatting
// context. See `PositionLeadingFloats`.
//
struct CORE_EXPORT LeadingFloats {
  STACK_ALLOCATED();

 public:
  ~LeadingFloats() { floats.clear(); }

  wtf_size_t handled_index = 0;
  HeapVector<PositionedFloat> floats;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LEADING_FLOATS_H_
