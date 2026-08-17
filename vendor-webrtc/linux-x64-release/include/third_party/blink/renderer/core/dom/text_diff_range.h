// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_DIFF_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_DIFF_RANGE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

//
// This represents where in a text is changed how.
//
struct TextDiffRange {
  STACK_ALLOCATED();

 public:
  static TextDiffRange Delete(wtf_size_t offset, wtf_size_t old_size) {
    return TextDiffRange{.offset = offset, .old_size = old_size};
  }
  static TextDiffRange Insert(wtf_size_t offset, wtf_size_t new_size) {
    return TextDiffRange{.offset = offset, .new_size = new_size};
  }
  static TextDiffRange Replace(wtf_size_t offset,
                               wtf_size_t old_size,
                               wtf_size_t new_size) {
    return TextDiffRange{offset, old_size, new_size};
  }

  inline wtf_size_t OldEndOffset() const { return offset + old_size; }
  inline wtf_size_t NewEndOffset() const { return offset + new_size; }

  // Check if text outside of the diff are not changed.
  void CheckValid(const WTF::String& old_text,
                  const WTF::String& new_text) const;

  wtf_size_t offset = 0;
  // Indicates a deletion of `old_size` characters at `offset`.
  wtf_size_t old_size = 0;
  // Indicates an insertion of `new_size` characters at `offset`.
  wtf_size_t new_size = 0;
};

#if !EXPENSIVE_DCHECKS_ARE_ON()
inline void TextDiffRange::CheckValid(const WTF::String& old_text,
                                      const WTF::String& new_text) const {}
#endif  // EXPENSIVE_DCHECKS_ARE_ON()

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_DIFF_RANGE_H_
