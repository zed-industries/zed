// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CUSTOM_SPACES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CUSTOM_SPACES_H_

#include <memory>
#include <vector>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "v8/include/cppgc/custom-space.h"

namespace blink {

// The following defines custom spaces that are used to partition Oilpan's heap.
// Each custom space is assigned to a type partition using `cppgc::SpaceTrait`.
// It is expected that `kSpaceIndex` uniquely identifies a space and that the
// indices of all custom spaces form a sequence starting at 0. See
// `cppgc::CustomSpace` for details.

class PLATFORM_EXPORT CompactableHeapVectorBackingSpace
    : public cppgc::CustomSpace<CompactableHeapVectorBackingSpace> {
 public:
  static constexpr cppgc::CustomSpaceIndex kSpaceIndex = 0;
  static constexpr bool kSupportsCompaction = true;
};

class PLATFORM_EXPORT CompactableHeapHashTableBackingSpace
    : public cppgc::CustomSpace<CompactableHeapHashTableBackingSpace> {
 public:
  static constexpr cppgc::CustomSpaceIndex kSpaceIndex = 1;
  static constexpr bool kSupportsCompaction = true;
};

class PLATFORM_EXPORT NodeSpace : public cppgc::CustomSpace<NodeSpace> {
 public:
  static constexpr cppgc::CustomSpaceIndex kSpaceIndex = 2;
};

class PLATFORM_EXPORT CSSValueSpace : public cppgc::CustomSpace<CSSValueSpace> {
 public:
  static constexpr cppgc::CustomSpaceIndex kSpaceIndex = 3;
};

class PLATFORM_EXPORT LayoutObjectSpace
    : public cppgc::CustomSpace<LayoutObjectSpace> {
 public:
  static constexpr cppgc::CustomSpaceIndex kSpaceIndex = 4;
};

struct PLATFORM_EXPORT CustomSpaces final {
  static std::vector<std::unique_ptr<cppgc::CustomSpaceBase>>
  CreateCustomSpaces();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CUSTOM_SPACES_H_
