// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IMAGE_SET_OPTION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IMAGE_SET_OPTION_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSImageSetTypeValue;
class CSSLengthResolver;
class CSSPrimitiveValue;

// This class represents an image-set-option as specified in:
// https://w3c.github.io/csswg-drafts/css-images-4/#typedef-image-set-option
// <image-set-option> = [ <image> | <string> ] [<resolution> || type(<string>)]
class CSSImageSetOptionValue : public CSSValue {
 public:
  explicit CSSImageSetOptionValue(const CSSValue* image,
                                  const CSSPrimitiveValue* resolution = nullptr,
                                  const CSSImageSetTypeValue* type = nullptr);

  // It is expected that CSSImageSetOptionValue objects should always have
  // non-null image and resolution values.
  CSSImageSetOptionValue() = delete;

  ~CSSImageSetOptionValue();

  // Gets the resolution value in Dots Per Pixel
  double ComputedResolution(const CSSLengthResolver&) const;

  // Returns true if the image-set-option uses an image format that the
  // browser can render.
  bool IsSupported(const CSSLengthResolver&) const;

  CSSValue& GetImage() const;
  const CSSPrimitiveValue& GetResolution() const;
  const CSSImageSetTypeValue* GetType() const;

  String CustomCSSText() const;

  bool Equals(const CSSImageSetOptionValue& other) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

 private:
  Member<const CSSValue> image_;
  Member<const CSSPrimitiveValue> resolution_;
  Member<const CSSImageSetTypeValue> type_;
};

template <>
struct DowncastTraits<CSSImageSetOptionValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsImageSetOptionValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IMAGE_SET_OPTION_VALUE_H_
