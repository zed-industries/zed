// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_TRANSFORMED_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_TRANSFORMED_STRING_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/text_offset_map.h"

namespace blink {

// Represents a text transformed in LayoutText and information on how it was
// collapsed or expanded.  This class is necessary to generate OffsetMappings
// correctly even if text-transform or -webkit-text-security changed the text
// length.
//
// Instances are copyable, and immutable.
class TransformedString {
  STACK_ALLOCATED();

 public:
  explicit TransformedString(StringView view) : view_(view) {}
  TransformedString(StringView view,
                    base::span<const TextOffsetMap::Length> map)
      : view_(view), length_map_(map) {}

  const StringView& View() const { return view_; }
  bool HasLengthMap() const { return !length_map_.empty(); }
  const base::span<const TextOffsetMap::Length>& LengthMap() const {
    return length_map_;
  }

  TransformedString Substring(unsigned start, unsigned length) const;
  TransformedString Substring(unsigned start) const {
    return Substring(start, view_.length() - start);
  }

 private:
  const StringView view_;

  const base::span<const TextOffsetMap::Length> length_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_TRANSFORMED_STRING_H_
