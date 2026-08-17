/*
 * Copyright (C) 2014 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STRING_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STRING_LIST_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/svg/properties/svg_property.h"
#include "third_party/blink/renderer/core/svg/svg_parsing_error.h"

namespace blink {

class SVGStringListTearOff;

// Implementation of SVGStringList spec:
// http://www.w3.org/TR/SVG/single-page.html#types-InterfaceSVGStringList
// See SVGStringListTearOff for actual Javascript interface.
// Unlike other SVG*List implementations, SVGStringList is NOT tied to
// SVGString.
// SVGStringList operates directly on DOMString.
//
// In short:
//   SVGStringList has_a Vector<String>.
//   SVGStringList items are exposed to Javascript as DOMString (not SVGString)
//   as in the spec.
//   SVGString is used only for boxing values for non-list string property
//   SVGAnimatedString,
//   and not used for SVGStringList.
class SVGStringListBase : public SVGPropertyBase {
 public:
  typedef SVGStringListTearOff TearOffType;

  ~SVGStringListBase() override;

  const Vector<String>& Values() const { return values_; }

  uint32_t length() { return values_.size(); }
  void Clear();
  void Insert(uint32_t, const String&);
  void Remove(uint32_t);
  void Append(const String&);
  void Replace(uint32_t, const String&);

  virtual SVGParsingError SetValueAsString(const String&) = 0;

  // SVGPropertyBase:
  void Add(const SVGPropertyBase*, const SVGElement*) override;
  void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from_value,
      const SVGPropertyBase* to_value,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement*) override;
  float CalculateDistance(const SVGPropertyBase* to,
                          const SVGElement*) const override;

  static AnimatedPropertyType ClassType() { return kAnimatedStringList; }
  AnimatedPropertyType GetType() const override { return ClassType(); }

 protected:
  SVGParsingError SetValueAsStringWithDelimiter(const String& data,
                                                char list_delimiter);
  String ValueAsStringWithDelimiter(char list_delimiter) const;

  template <typename CharType>
  void ParseInternal(const CharType* ptr,
                     const CharType* end,
                     char list_delimiter);

  Vector<String> values_;
};

template <char list_delimiter>
class SVGStringList final : public SVGStringListBase {
 public:
  SVGStringList() = default;
  ~SVGStringList() override = default;

  SVGParsingError SetValueAsString(const String& data) override {
    return SVGStringListBase::SetValueAsStringWithDelimiter(data,
                                                            list_delimiter);
  }

  String ValueAsString() const override {
    return SVGStringListBase::ValueAsStringWithDelimiter(list_delimiter);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STRING_LIST_H_
