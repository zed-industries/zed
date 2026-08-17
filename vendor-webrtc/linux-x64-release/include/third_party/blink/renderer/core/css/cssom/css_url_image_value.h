// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_URL_IMAGE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_URL_IMAGE_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_image_value.h"

namespace blink {

class CSSImageValue;

class CORE_EXPORT CSSURLImageValue final : public CSSStyleImageValue {
 public:
  explicit CSSURLImageValue(const CSSImageValue& value) : value_(value) {}
  CSSURLImageValue(const CSSURLImageValue&) = delete;
  CSSURLImageValue& operator=(const CSSURLImageValue&) = delete;

  const String& url() const;

  // CSSStyleImageValue
  std::optional<gfx::Size> IntrinsicSize() const final;

  // CanvasImageSource
  ResourceStatus Status() const final;
  scoped_refptr<Image> GetSourceImageForCanvas(FlushReason,
                                               SourceImageStatus*,
                                               const gfx::SizeF&) final;
  bool IsAccelerated() const final;

  // CSSStyleValue
  StyleValueType GetType() const final { return kURLImageType; }
  const CSSValue* ToCSSValue() const final;

  void Trace(Visitor*) const override;

 private:
  scoped_refptr<Image> GetImage() const;

  Member<const CSSImageValue> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_URL_IMAGE_VALUE_H_
