// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SPACE_SPLIT_STRING_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SPACE_SPLIT_STRING_WRAPPER_H_

#include "third_party/blink/renderer/core/dom/space_split_string.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

struct SpaceSplitStringWrapper
    : public GarbageCollected<SpaceSplitStringWrapper> {
  SpaceSplitStringWrapper() = default;
  explicit SpaceSplitStringWrapper(const AtomicString& string)
      : value(string) {}

  void Trace(Visitor* visitor) const { visitor->Trace(value); }

  SpaceSplitString value;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SPACE_SPLIT_STRING_WRAPPER_H_
