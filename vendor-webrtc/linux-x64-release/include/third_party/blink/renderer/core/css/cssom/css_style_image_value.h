// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_IMAGE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_IMAGE_VALUE_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_resource_value.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_image_source.h"

namespace blink {

// CSSStyleImageValue is the base class for Typed OM representations of images.
// The corresponding idl file is CSSImageValue.idl.
class CORE_EXPORT CSSStyleImageValue : public CSSResourceValue,
                                       public CanvasImageSource {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSStyleImageValue(const CSSStyleImageValue&) = delete;
  CSSStyleImageValue& operator=(const CSSStyleImageValue&) = delete;
  ~CSSStyleImageValue() override = default;

  // IDL
  double intrinsicWidth(bool& is_null) const;
  double intrinsicHeight(bool& is_null) const;
  double intrinsicRatio(bool& is_null) const;

  // CanvasImageSource
  bool IsCSSImageValue() const final { return true; }
  bool WouldTaintOrigin() const final { return true; }
  gfx::SizeF ElementSize(const gfx::SizeF& default_object_size,
                         const RespectImageOrientationEnum) const final;

 protected:
  CSSStyleImageValue() = default;

  virtual std::optional<gfx::Size> IntrinsicSize() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_IMAGE_VALUE_H_
