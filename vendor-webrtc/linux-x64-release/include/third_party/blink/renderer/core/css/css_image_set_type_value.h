// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IMAGE_SET_TYPE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IMAGE_SET_TYPE_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class represents the CSS type() function as specified in:
// https://w3c.github.io/csswg-drafts/css-images-4/#funcdef-image-set-type
// type(<string>) function, specifying the image's MIME type in the <string>.
class CSSImageSetTypeValue : public CSSValue {
 public:
  explicit CSSImageSetTypeValue(const String& type);

  ~CSSImageSetTypeValue();

  // Returns true if the image type is supported
  bool IsSupported() const;

  String CustomCSSText() const;

  bool Equals(const CSSImageSetTypeValue& other) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

 private:
  String type_;
};

template <>
struct DowncastTraits<CSSImageSetTypeValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsImageSetTypeValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IMAGE_SET_TYPE_VALUE_H_
