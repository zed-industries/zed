/*
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UTF16_TEXT_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UTF16_TEXT_ITERATOR_H_

#include <unicode/utf16.h>

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PLATFORM_EXPORT UTF16TextIterator {
  STACK_ALLOCATED();

 public:
  // The passed in UChar pointer starts at 'offset'. The iterator operates on
  // the range [offset, endOffset].
  // 'length' denotes the maximum length of the UChar array, which might exceed
  // 'endOffset'.
  explicit UTF16TextIterator(base::span<const UChar> characters)
      : characters_(characters.data()),
        characters_end_(characters.data() + characters.size()),
        size_(base::checked_cast<wtf_size_t>(characters.size())) {}

  UTF16TextIterator(const UTF16TextIterator&) = delete;
  UTF16TextIterator& operator=(const UTF16TextIterator&) = delete;

  inline bool Consume(UChar32& character) {
    if (offset_ >= size_) {
      return false;
    }

    character = *characters_;
    current_glyph_length_ = 1;
    if (!U16_IS_SURROGATE(character))
      return true;

    return ConsumeSurrogatePair(character);
  }

  void Advance() {
    characters_ += current_glyph_length_;
    offset_ += current_glyph_length_;
  }

  unsigned Offset() const { return offset_; }
  unsigned Size() const { return size_; }
  const UChar* Characters() const { return characters_; }
  const UChar* GlyphEnd() const { return characters_ + current_glyph_length_; }

 private:
  bool IsValidSurrogatePair(UChar32&);
  bool ConsumeSurrogatePair(UChar32&);
  void ConsumeMultipleUChar();

  const UChar* characters_;
  const UChar* const characters_end_;
  unsigned offset_ = 0;
  const unsigned size_;
  unsigned current_glyph_length_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UTF16_TEXT_ITERATOR_H_
