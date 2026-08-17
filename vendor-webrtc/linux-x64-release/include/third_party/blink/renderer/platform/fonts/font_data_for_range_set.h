/*
 * Copyright (C) 2008, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_DATA_FOR_RANGE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_DATA_FOR_RANGE_SET_H_

#include "third_party/blink/renderer/platform/fonts/font_data.h"
#include "third_party/blink/renderer/platform/fonts/simple_font_data.h"
#include "third_party/blink/renderer/platform/fonts/unicode_range_set.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"

namespace blink {

class SimpleFontData;

class PLATFORM_EXPORT FontDataForRangeSet
    : public GarbageCollected<FontDataForRangeSet> {
 public:
  explicit FontDataForRangeSet(const SimpleFontData* font_data = nullptr,
                               const UnicodeRangeSet* range_set = nullptr)
      : font_data_(font_data), range_set_(range_set) {}

  FontDataForRangeSet(const FontDataForRangeSet& other);

  virtual ~FontDataForRangeSet() = default;

  void Trace(Visitor* visitor) const {
    visitor->Trace(font_data_);
    visitor->Trace(range_set_);
  }

  bool Contains(UChar32 test_char) const {
    return !range_set_ || range_set_->Contains(test_char);
  }
  bool IsEntireRange() const {
    return !range_set_ || range_set_->IsEntireRange();
  }
  const UnicodeRangeSet* Ranges() const { return range_set_.Get(); }
  bool HasFontData() const { return font_data_; }
  const SimpleFontData* FontData() const { return font_data_.Get(); }

  // TODO(xiaochengh): |FontData::IsLoadingFallback()| returns true if the
  // FontData is a pending custom font. We should rename it for better clarity.
  bool IsPendingCustomFont() const {
    return font_data_ && font_data_->IsLoadingFallback();
  }

  bool IsPendingDataUrlCustomFont() const {
    return font_data_ && font_data_->IsPendingDataUrlCustomFont();
  }

 protected:
  Member<const SimpleFontData> font_data_;
  Member<const UnicodeRangeSet> range_set_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_DATA_FOR_RANGE_SET_H_
