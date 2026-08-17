// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUSGROUP_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUSGROUP_FLAGS_H_

#include "base/types/cxx23_to_underlying.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Element;

namespace focusgroup {

enum FocusgroupFlags : uint8_t {
  kNone = 0,
  kExtend = 1 << 0,
  kInline = 1 << 1,
  kBlock = 1 << 2,
  kGrid = 1 << 3,
  kWrapInline = 1 << 4,
  kWrapBlock = 1 << 5,
  kRowFlow = 1 << 6,
  kColFlow = 1 << 7,
};

inline constexpr FocusgroupFlags operator&(FocusgroupFlags a,
                                           FocusgroupFlags b) {
  return static_cast<FocusgroupFlags>(base::to_underlying(a) &
                                      base::to_underlying(b));
}

inline constexpr FocusgroupFlags operator|(FocusgroupFlags a,
                                           FocusgroupFlags b) {
  return static_cast<FocusgroupFlags>(base::to_underlying(a) |
                                      base::to_underlying(b));
}

inline FocusgroupFlags& operator|=(FocusgroupFlags& a, FocusgroupFlags b) {
  return a = a | b;
}

inline FocusgroupFlags& operator&=(FocusgroupFlags& a, FocusgroupFlags b) {
  return a = a & b;
}

inline constexpr FocusgroupFlags operator~(FocusgroupFlags flags) {
  return static_cast<FocusgroupFlags>(~base::to_underlying(flags));
}

FocusgroupFlags FindNearestFocusgroupAncestorFlags(const Element* element);
// Implemented based on this explainer:
// https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/main/Focusgroup/explainer.md
FocusgroupFlags ParseFocusgroup(const Element* element,
                                const AtomicString& input);

}  // namespace focusgroup

// The "::blink" prefix is to avoid false-positive of audit_non_blink_usages.py.
using FocusgroupFlags = ::blink::focusgroup::FocusgroupFlags;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUSGROUP_FLAGS_H_
