// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_ORDERED_NAMED_GRID_LINES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_ORDERED_NAMED_GRID_LINES_H_

#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct NamedGridLine {
  explicit NamedGridLine(const AtomicString& line_name,
                         bool is_in_repeat = false,
                         bool is_first_repeat = false)
      : line_name(line_name),
        is_in_repeat(is_in_repeat),
        is_first_repeat(is_first_repeat) {}

  bool operator==(const NamedGridLine& other) const {
    return line_name == other.line_name && is_in_repeat == other.is_in_repeat &&
           is_first_repeat == other.is_first_repeat;
  }

  AtomicString line_name;
  bool is_in_repeat : 1;
  bool is_first_repeat : 1;
};

using OrderedNamedGridLines =
    HashMap<size_t, Vector<NamedGridLine>, IntWithZeroKeyHashTraits<size_t>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_ORDERED_NAMED_GRID_LINES_H_
