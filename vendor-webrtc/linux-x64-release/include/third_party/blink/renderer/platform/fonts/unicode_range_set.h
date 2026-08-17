/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UNICODE_RANGE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UNICODE_RANGE_SET_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

struct PLATFORM_EXPORT UnicodeRange final {
  DISALLOW_NEW();
  UnicodeRange(UChar32 from, UChar32 to) : from_(from), to_(to) {}

  UChar32 From() const { return from_; }
  UChar32 To() const { return to_; }
  bool Contains(UChar32 c) const { return from_ <= c && c <= to_; }
  bool operator<(const UnicodeRange& other) const {
    return from_ < other.from_;
  }
  bool operator<(UChar32 c) const { return to_ < c; }
  bool operator==(const UnicodeRange& other) const {
    return other.from_ == from_ && other.to_ == to_;
  }

 private:
  UChar32 from_;
  UChar32 to_;
};

class PLATFORM_EXPORT UnicodeRangeSet
    : public GarbageCollected<UnicodeRangeSet> {
 public:
  explicit UnicodeRangeSet(HeapVector<UnicodeRange>&&);
  UnicodeRangeSet() = default;

  void Trace(Visitor* visitor) const { visitor->Trace(ranges_); }

  bool Contains(UChar32) const;
  bool IntersectsWith(const String&) const;
  bool IsEntireRange() const { return ranges_.empty(); }
  wtf_size_t size() const { return ranges_.size(); }
  const UnicodeRange& RangeAt(wtf_size_t i) const { return ranges_[i]; }
  bool operator==(const UnicodeRangeSet& other) const;

 private:
  HeapVector<UnicodeRange>
      ranges_;  // If empty, represents the whole code space.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_UNICODE_RANGE_SET_H_
