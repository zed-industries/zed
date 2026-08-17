/*
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 * Copyright (C) 2013 Apple Inc. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_STRING_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_STRING_SOURCE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/svg_parsing_error.h"
#include "third_party/blink/renderer/core/svg/svg_path_data.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

namespace blink {

class CORE_EXPORT SVGPathStringSource {
  STACK_ALLOCATED();

 public:
  explicit SVGPathStringSource(StringView);
  SVGPathStringSource(const SVGPathStringSource&) = delete;
  SVGPathStringSource& operator=(const SVGPathStringSource&) = delete;

  bool HasMoreData() const {
    if (is_8bit_source_)
      return current_.character8_ < end_.character8_;
    return current_.character16_ < end_.character16_;
  }
  PathSegmentData ParseSegment();

  SVGParsingError ParseError() const { return error_; }

 private:
  void EatWhitespace();
  float ParseNumberWithError();
  bool ParseArcFlagWithError();
  void SetErrorMark(SVGParseStatus);

  bool is_8bit_source_;

  union {
    const LChar* character8_;
    const UChar* character16_;
  } current_;
  union {
    const LChar* character8_;
    const UChar* character16_;
  } end_;

  SVGPathSegType previous_command_;
  SVGParsingError error_;
  StringView source_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_STRING_SOURCE_H_
