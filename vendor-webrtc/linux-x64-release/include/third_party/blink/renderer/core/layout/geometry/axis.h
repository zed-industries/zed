// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_AXIS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_AXIS_H_

#include "base/types/strong_alias.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"

namespace blink {

enum class LogicalAxis : uint8_t { kInline = 0b01, kBlock = 0b10 };
enum class PhysicalAxis : uint8_t { kHorizontal = 0b01, kVertical = 0b10 };

using PhysicalAxes = base::StrongAlias<class PhysicalAxesTag, uint8_t>;
using LogicalAxes = base::StrongAlias<class LogicalAxesTag, uint8_t>;

inline constexpr LogicalAxes operator|(LogicalAxes a, LogicalAxes b) {
  return LogicalAxes(a.value() | b.value());
}

inline constexpr LogicalAxes& operator|=(LogicalAxes& a, LogicalAxes b) {
  a.value() |= b.value();
  return a;
}

inline constexpr LogicalAxes operator&(LogicalAxes a, LogicalAxes b) {
  return LogicalAxes(a.value() & b.value());
}

inline constexpr LogicalAxes operator&=(LogicalAxes& a, LogicalAxes b) {
  a.value() &= b.value();
  return a;
}

inline constexpr PhysicalAxes operator|(PhysicalAxes a, PhysicalAxes b) {
  return PhysicalAxes(a.value() | b.value());
}

inline constexpr PhysicalAxes& operator|=(PhysicalAxes& a, PhysicalAxes b) {
  a.value() |= b.value();
  return a;
}

inline constexpr PhysicalAxes operator&(PhysicalAxes a, PhysicalAxes b) {
  return PhysicalAxes(a.value() & b.value());
}

inline constexpr PhysicalAxes operator&=(PhysicalAxes& a, PhysicalAxes b) {
  a.value() &= b.value();
  return a;
}

inline constexpr PhysicalAxes operator^(PhysicalAxes a, PhysicalAxes b) {
  return PhysicalAxes(a.value() ^ b.value());
}

inline constexpr PhysicalAxes operator^=(PhysicalAxes& a, PhysicalAxes b) {
  a.value() ^= b.value();
  return a;
}

inline constexpr LogicalAxes kLogicalAxesNone = LogicalAxes(0);
inline constexpr LogicalAxes kLogicalAxesInline =
    LogicalAxes(static_cast<uint8_t>(LogicalAxis::kInline));
inline constexpr LogicalAxes kLogicalAxesBlock =
    LogicalAxes(static_cast<uint8_t>(LogicalAxis::kBlock));
inline constexpr LogicalAxes kLogicalAxesBoth =
    kLogicalAxesInline | kLogicalAxesBlock;

inline constexpr PhysicalAxes kPhysicalAxesNone = PhysicalAxes(0);
inline constexpr PhysicalAxes kPhysicalAxesHorizontal =
    PhysicalAxes(static_cast<uint8_t>(PhysicalAxis::kHorizontal));
inline constexpr PhysicalAxes kPhysicalAxesVertical =
    PhysicalAxes(static_cast<uint8_t>(PhysicalAxis::kVertical));
inline constexpr PhysicalAxes kPhysicalAxesBoth =
    kPhysicalAxesHorizontal | kPhysicalAxesVertical;

// ConvertAxes relies on the fact that the underlying values for
// for Inline/Horizontal are the same, and that the underlying values for
// Block/Vertical are the same.
static_assert(kLogicalAxesNone.value() == kPhysicalAxesNone.value());
static_assert(kLogicalAxesInline.value() == kPhysicalAxesHorizontal.value());
static_assert(kLogicalAxesBlock.value() == kPhysicalAxesVertical.value());
static_assert(kLogicalAxesBoth.value() == kPhysicalAxesBoth.value());

template <typename FromType, typename ToType>
inline ToType ConvertAxes(FromType from, WritingMode mode) {
  // Reverse the bits if |mode| is a vertical writing mode.
  int shift = !IsHorizontalWritingMode(mode);
  return ToType(((from.value() >> shift) & 1) | ((from.value() << shift) & 2));
}

inline PhysicalAxes ToPhysicalAxes(LogicalAxes logical, WritingMode mode) {
  return ConvertAxes<LogicalAxes, PhysicalAxes>(logical, mode);
}

inline LogicalAxes ToLogicalAxes(PhysicalAxes physical, WritingMode mode) {
  return ConvertAxes<PhysicalAxes, LogicalAxes>(physical, mode);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_AXIS_H_
