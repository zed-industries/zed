// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_TEXT_OFFSET_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_TEXT_OFFSET_RANGE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

// Represents a range of text, as a |start| and |end| offset pair.
struct CORE_EXPORT TextOffsetRange {
  TextOffsetRange() = default;
  TextOffsetRange(wtf_size_t start, wtf_size_t end) : start(start), end(end) {
    AssertValid();
  }

  wtf_size_t Length() const {
    AssertValid();
    return end - start;
  }

  void AssertValid() const { DCHECK_GE(end, start); }
  void AssertNotEmpty() const { DCHECK_GT(end, start); }

  bool operator==(const TextOffsetRange& other) const {
    return start == other.start && end == other.end;
  }
  bool operator!=(const TextOffsetRange& other) const {
    return !operator==(other);
  }

  wtf_size_t start = 0;
  wtf_size_t end = 0;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const TextOffsetRange&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_TEXT_OFFSET_RANGE_H_
