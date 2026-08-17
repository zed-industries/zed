// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SMALL_CAPS_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SMALL_CAPS_ITERATOR_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/fonts/font_orientation.h"
#include "third_party/blink/renderer/platform/fonts/script_run_iterator.h"
#include "third_party/blink/renderer/platform/fonts/utf16_text_iterator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PLATFORM_EXPORT SmallCapsIterator {
  STACK_ALLOCATED();

 public:
  enum SmallCapsBehavior {
    kSmallCapsSameCase,
    kSmallCapsUppercaseNeeded,
    kSmallCapsInvalid
  };

  explicit SmallCapsIterator(base::span<const UChar> buffer);
  SmallCapsIterator(const SmallCapsIterator&) = delete;
  SmallCapsIterator& operator=(const SmallCapsIterator&) = delete;

  bool Consume(unsigned* caps_limit, SmallCapsBehavior*);

 private:
  UTF16TextIterator utf16_iterator_;
  UChar32 next_u_char32_;
  bool at_end_;

  SmallCapsBehavior current_small_caps_behavior_;
  SmallCapsBehavior previous_small_caps_behavior_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SMALL_CAPS_ITERATOR_H_
