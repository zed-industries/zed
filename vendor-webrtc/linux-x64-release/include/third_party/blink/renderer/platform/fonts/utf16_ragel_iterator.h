// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UTF16_RAGEL_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UTF16_RAGEL_ITERATOR_H_

#include <unicode/uchar.h>

#include "base/check_op.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/emoji_segmentation_category.h"
#include "third_party/blink/renderer/platform/text/emoji_segmentation_category_inline_header.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"

namespace blink {

// UTF16RagelIterator is set up on top of a UTF-16 UChar* buffer iterating over
// a Blink internal text string and as such is used as an adapter between Blink
// strings and the Ragel-based emoji scanner. It supports forwarding and
// reversing using arithmetic operators. Dereferencing the iterator means
// retrieving a character class as defined in the Ragel grammar of
// third-party/emoji-segmenter. The dereferenced character category is cached
// since Ragel dereferences multiple times without moving the iterator's cursor.
class PLATFORM_EXPORT UTF16RagelIterator {
  STACK_ALLOCATED();

 public:
  UTF16RagelIterator()
      : cursor_(0),
        cached_category_(EmojiSegmentationCategory::kInvalidCacheEntry) {}

  explicit UTF16RagelIterator(base::span<const UChar> buffer,
                              unsigned cursor = 0)
      : buffer_(buffer),
        cursor_(cursor),
        cached_category_(EmojiSegmentationCategory::kInvalidCacheEntry) {}

  UTF16RagelIterator end() {
    UTF16RagelIterator ret = *this;
    ret.cursor_ = static_cast<unsigned>(buffer_.size());
    return ret;
  }

  size_t size() { return buffer_.size(); }

  UTF16RagelIterator& SetCursor(unsigned new_cursor) {
    DCHECK_GE(new_cursor, 0u);
    DCHECK_LT(new_cursor, buffer_.size());
    cursor_ = new_cursor;
    InvalidateCache();
    return *this;
  }

  unsigned Cursor() { return cursor_; }

  UTF16RagelIterator& operator+=(int v) {
    if (v > 0) {
      U16_FWD_N(buffer_, cursor_, buffer_.size(), v);
    } else if (v < 0) {
      U16_BACK_N(buffer_, 0, cursor_, -v);
    }
    InvalidateCache();
    return *this;
  }

  UTF16RagelIterator& operator-=(int v) { return *this += -v; }

  UTF16RagelIterator operator+(int v) {
    UTF16RagelIterator ret = *this;
    return ret += v;
  }

  UTF16RagelIterator operator-(int v) { return *this + -v; }

  int operator-(const UTF16RagelIterator& other) {
    DCHECK_EQ(buffer_, other.buffer_);
    return cursor_ - other.cursor_;
  }

  UTF16RagelIterator& operator++() {
    DCHECK_LT(cursor_, buffer_.size());
    U16_FWD_1(buffer_, cursor_, buffer_.size());
    InvalidateCache();
    return *this;
  }

  UTF16RagelIterator& operator--() {
    DCHECK_GT(cursor_, 0u);
    U16_BACK_1(buffer_, 0, cursor_);
    InvalidateCache();
    return *this;
  }

  UTF16RagelIterator operator++(int) {
    UTF16RagelIterator ret = *this;
    ++(*this);
    return ret;
  }

  UTF16RagelIterator operator--(int) {
    UTF16RagelIterator ret = *this;
    --(*this);
    return ret;
  }

  UTF16RagelIterator operator=(int v) {
    // We need this integer assignment operator because Ragel has initialization
    // code for assigning 0 to ts, te.
    DCHECK_EQ(v, 0);
    UTF16RagelIterator ret = *this;
    ret.cursor_ = v;
    return ret;
  }

  EmojiSegmentationCategory operator*() {
    DCHECK(!buffer_.empty());
    if (cached_category_ == EmojiSegmentationCategory::kInvalidCacheEntry) {
      UChar32 codepoint;
      U16_GET(buffer_, 0, cursor_, buffer_.size(), codepoint);
      cached_category_ = GetEmojiSegmentationCategory(codepoint);
    }
    return cached_category_;
  }

  inline void InvalidateCache() {
    cached_category_ = EmojiSegmentationCategory::kInvalidCacheEntry;
  }

  bool operator==(const UTF16RagelIterator& other) const {
    return buffer_.data() == other.buffer_.data() &&
           buffer_.size() == other.buffer_.size() && cursor_ == other.cursor_;
  }

  bool operator!=(const UTF16RagelIterator& other) const {
    return !(*this == other);
  }

  // Peeks the next codepoint. Note: Does not peak the
  // `EmojiSegmentationCategory` as does `operator*()`. For performance reasons,
  // this method is simplified to return U+FFFD when the cursor is at the end of
  // the stream, instead of using `std::optional` or similar.
  //
  // TODO(drott): Before moving to ICU UNSAFE functions, check
  // InputMethodControllerTest.DeleteSurroundingTextInCodePointsWithInvalidSurrogatePair
  // and DeleteSurroundingTextInCodePointsWithInvalidSurrogatePair which cause
  // this code to encounter an unmatched lead surrogate as the last character in
  // the buffer. (Potential issue with InputMethodController, or the tests?).
  UChar32 PeekCodepoint() {
    UChar32 output = kReplacementCharacter;
    unsigned temp_cursor = cursor_;
    U16_FWD_1(buffer_, temp_cursor, buffer_.size());
    if (temp_cursor < buffer_.size()) {
      U16_GET(buffer_, 0, temp_cursor, buffer_.size(), output);
    }
    return output;
  }

 private:
  base::span<const UChar> buffer_;
  unsigned cursor_;
  EmojiSegmentationCategory cached_category_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UTF16_RAGEL_ITERATOR_H_
