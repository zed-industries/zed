// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SYMBOLS_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SYMBOLS_ITERATOR_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/fonts/font_fallback_priority.h"
#include "third_party/blink/renderer/platform/fonts/utf16_ragel_iterator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// SymbolsIterator is used to segment text based on emoji presentations (text or
// emoji) and emoji variation selectors (U+FE0E and U+FE0F) presence. All
// characters in each of the returned segments should represent only one of 4
// scenarios:
//
// 1) All characters in the segment are emoji characters in emoji presentation
// without emoji variation selectors (vs).
//
// 2) All characters in the segment are pairs of emoji base character +
// variation selector 16 (U+FE0F).
//
// 3) All characters in the segment are text characters, including emoji
// characters in text presentation without variation selector 15. This includes
// pairs of non emoji base character + variation selector 15.
//
// 4) All characters in the segment are pairs of emoji base character +
// variation selector 15 (U+FE0E).
//
// We segment into 4 segmentation categories (distinguishing VS and non-VS runs)
// because font-variant-emoji expects variation selectors in the source text to
// have precedence, but for each shaping run we can only specify one
// FontFallbackPriority. This means, segments with overriding variation
// selectors in the source text need to be split out as separate runs. Without
// the precedence requirement in font-variant-emoji, we could coalesce into
// fewer runs, and drop the *withVS FontFallbackPriority enum values.
class PLATFORM_EXPORT SymbolsIterator {
  STACK_ALLOCATED();

 public:
  explicit SymbolsIterator(base::span<const UChar> buffer);
  SymbolsIterator(const SymbolsIterator&) = delete;
  SymbolsIterator& operator=(const SymbolsIterator&) = delete;

  bool Consume(unsigned* symbols_limit, FontFallbackPriority*);

 private:
  UTF16RagelIterator buffer_iterator_;
  unsigned cursor_ = 0;

  unsigned next_token_end_ = 0;
  bool next_token_emoji_ = false;
  bool next_token_has_vs_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SYMBOLS_ITERATOR_H_
