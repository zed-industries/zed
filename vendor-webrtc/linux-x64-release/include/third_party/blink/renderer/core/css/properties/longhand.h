// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_LONGHAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_LONGHAND_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/css/css_initial_value.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer.h"
#include "third_party/blink/renderer/core/css/properties/css_property.h"
#include "third_party/blink/renderer/core/css/resolver/style_resolver_state.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSValue;
class CSSParserContext;
class CSSParserLocalContext;
class CSSParserTokenStream;

class Longhand : public CSSProperty {
 public:
  // Parses and consumes a longhand property value from the token stream.
  // Returns nullptr if the input is invalid.
  //
  // NOTE: This function must accept arbitrary tokens after the value,
  // without returning error. In particular, it must not check for
  // end-of-stream, since it may be called as part of parsing a shorthand, or
  // there may be “!important” after the value that the caller is responsible
  // the caller is responsible for consuming. End-of-stream is checked
  // by the caller (after potentially consuming “!important”).
  CORE_EXPORT
  virtual const CSSValue* ParseSingleValue(
      CSSParserTokenStream& stream,
      const CSSParserContext& context,
      const CSSParserLocalContext& local_tokenizer) const {
    return nullptr;
  }
  virtual void ApplyInitial(StyleResolverState&) const { NOTREACHED(); }
  virtual void ApplyInherit(StyleResolverState&) const { NOTREACHED(); }
  virtual void ApplyValue(StyleResolverState&,
                          const CSSValue&,
                          ValueMode) const {
    NOTREACHED();
  }
  void ApplyUnset(StyleResolverState& state) const {
    if (state.IsInheritedForUnset(*this)) {
      ApplyInherit(state);
    } else {
      ApplyInitial(state);
    }
  }
  virtual const blink::Color ColorIncludingFallback(
      bool,
      const ComputedStyle&,
      bool* is_current_color = nullptr) const {
    NOTREACHED();
  }
  virtual const CSSValue* InitialValue() const {
    return CSSInitialValue::Create();
  }

  [[nodiscard]] virtual bool IsAffectedByCurrentColor(
      const ComputedStyle& style) const {
    NOTREACHED();
  }

 protected:
  constexpr Longhand(CSSPropertyID id, Flags flags, char repetition_separator)
      : CSSProperty(id, flags | kLonghand, repetition_separator) {}
  // Applies the computed CSSValue of the parent style using ApplyValue.
  // This generally achieves the same as ApplyInherit, but effectively
  // "rezooms" the value.
  //
  // https://github.com/w3c/csswg-drafts/issues/9397
  void ApplyParentValue(StyleResolverState&) const;
  // If our zoom is different from the parent zoom, calls ApplyParentValue
  // and returns true. Otherwise does nothing and returns false.
  bool ApplyParentValueIfZoomChanged(StyleResolverState&) const;
};

template <>
struct DowncastTraits<Longhand> {
  static bool AllowFrom(const CSSProperty& longhand) {
    return longhand.IsLonghand();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_LONGHAND_H_
