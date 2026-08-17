/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2006, 2007, 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_RUN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_RUN_H_

#include "base/check_op.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// TextRun instances are immutable.
class PLATFORM_EXPORT TextRun final {
  DISALLOW_NEW();

 public:
  // For all constructors, the contents of the specified string must
  // outlive the created TextRun instance.

  explicit TextRun(base::span<const LChar> c) : TextRun(StringView(c)) {}

  explicit TextRun(base::span<const UChar> c) : TextRun(StringView(c)) {}

  TextRun(const StringView& string)
      : TextRun(string, TextDirection::kLtr, false, false) {}
  TextRun(const StringView& string,
          TextDirection direction,
          bool directional_override = false,
          bool normalize_space = false)
      : text_(string),
        direction_(static_cast<unsigned>(direction)),
        directional_override_(directional_override),
        normalize_space_(normalize_space) {}

  // TextRun supports move construction, but supports neither copy construction,
  // copy assignment, nor move assignment.

  TextRun(const TextRun&) = delete;
  TextRun& operator=(const TextRun&) = delete;
  TextRun(TextRun&&) = default;
  TextRun& operator=(TextRun&&) = delete;

  // direction - An optional TextDirection of the new TextRun. If this is not
  //             specified, the new TextRun inherits the TextDirection of
  //             `this`.
  TextRun SubRun(unsigned start_offset,
                 unsigned length,
                 std::optional<TextDirection> direction = std::nullopt) const {
    return TextRun(StringView(text_, start_offset, length),
                   direction.value_or(Direction()), directional_override_,
                   normalize_space_);
  }

  // Returns the start index of a sub run if it was created by |SubRun|.
  // std::numeric_limits<unsigned>::max() if not a sub run.
  unsigned IndexOfSubRun(const TextRun&) const;

  UChar operator[](unsigned i) const { return text_[i]; }

  base::span<const LChar> Span8() const { return text_.Span8(); }
  base::span<const UChar> Span16() const { return text_.Span16(); }

  const StringView& ToStringView() const { return text_; }

  bool Is8Bit() const { return text_.Is8Bit(); }
  unsigned length() const { return text_.length(); }

  bool NormalizeSpace() const { return normalize_space_; }

  TextDirection Direction() const {
    return static_cast<TextDirection>(direction_);
  }
  bool Rtl() const { return Direction() == TextDirection::kRtl; }
  bool Ltr() const { return Direction() == TextDirection::kLtr; }
  bool DirectionalOverride() const { return directional_override_; }

  // Up-converts to UTF-16 as needed and normalizes spaces and Unicode control
  // characters as per the CSS Text Module Level 3 specification.
  // https://drafts.csswg.org/css-text-3/#white-space-processing
  String NormalizedUTF16() const;

 private:
  const StringView text_;

  const unsigned direction_ : 1;
  // Was this direction set by an override character.
  unsigned directional_override_ : 1;
  const unsigned normalize_space_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_RUN_H_
