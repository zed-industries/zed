// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_HREF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_HREF_H_

#include "third_party/blink/renderer/core/svg/svg_animated_string.h"

namespace blink {

class SVGAnimatedPropertyBase;

// This is an "access wrapper" for the 'href' attribute. The object
// itself holds the value for 'href' in the null/default NS and wraps
// one for 'href' in the XLink NS. Both objects are queryable through
// PropertyFromAttribute(), and hence any updates/synchronization/etc
// via the "attribute DOM" (setAttribute and friends) will operate on
// the independent objects, while users of an 'href' value will be
// using this interface (which essentially just selects either itself
// or the wrapped object and forwards the operation to it.)
class SVGAnimatedHref final : public SVGAnimatedString {
 public:
  explicit SVGAnimatedHref(SVGElement* context_element);

  SVGString* CurrentValue();
  const SVGString* CurrentValue() const;

  V8UnionStringOrTrustedScriptURL* baseVal() override;
  void setBaseVal(const V8UnionStringOrTrustedScriptURL* value,
                  ExceptionState& exception_state) override;
  String animVal() override;

  bool IsSpecified() const {
    return SVGAnimatedString::IsSpecified() || xlink_href_->IsSpecified();
  }

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name);

  static bool IsKnownAttribute(const QualifiedName&);

  void Trace(Visitor*) const override;

 private:
  SVGAnimatedString* BackingString();
  const SVGAnimatedString* BackingString() const;
  bool UseXLink() const;

  Member<SVGAnimatedString> xlink_href_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ANIMATED_HREF_H_
