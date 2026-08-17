// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ENUMERATION_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ENUMERATION_MAP_H_

#include "base/check.h"
#include "base/check_op.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// Helper class for SVG enumerations. Maps between name (string) and value.
//
// It is assumed that enumeration values are contiguous, non-zero and
// starting at 1. This allows us to use the string's location in storage as
// the value.
//
// For enumerations that have had new values added since SVG 1.1, the
// |max_exposed_value| should be set to the last old value. From this also
// follow that the new values should sort last - after the |max_exposed_value|.
// (This is currently always the case in the spec too.)
class CORE_EXPORT SVGEnumerationMap {
 public:
  explicit SVGEnumerationMap(base::span<const char* const> entries)
      : SVGEnumerationMap(entries, entries.size()) {}
  SVGEnumerationMap(base::span<const char* const> entries,
                    size_t max_exposed_value)
      : entries_(entries), max_exposed_value_(max_exposed_value) {}

  const char* NameFromValue(uint16_t value) const {
    CHECK(value);  // We should never store 0 (*_UNKNOWN) in the map.
    return entries_[static_cast<size_t>(value - 1)];
  }
  uint16_t ValueFromName(const WTF::String&) const;

  uint16_t ValueOfLast() const {
    return static_cast<uint16_t>(entries_.size());
  }
  uint16_t MaxExposedValue() const {
    return static_cast<uint16_t>(max_exposed_value_);
  }

 private:
  const base::span<const char* const> entries_;
  const size_t max_exposed_value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ENUMERATION_MAP_H_
