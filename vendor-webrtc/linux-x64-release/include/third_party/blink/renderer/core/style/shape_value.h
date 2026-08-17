/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHAPE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHAPE_VALUE_H_

#include "base/check_op.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/values_equivalent.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/style/basic_shapes.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class ShapeValue final : public GarbageCollected<ShapeValue> {
 public:
  enum ShapeValueType {
    // The Auto value is defined by a null ShapeValue*
    kShape,
    kBox,
    kImage
  };

  explicit ShapeValue(ShapeValueType type)
      : type_(type), css_box_(CSSBoxType::kMissing) {}
  explicit ShapeValue(StyleImage* image)
      : type_(kImage), image_(image), css_box_(CSSBoxType::kContent) {}
  explicit ShapeValue(CSSBoxType css_box) : type_(kBox), css_box_(css_box) {}
  ShapeValue(const BasicShape* shape, CSSBoxType css_box)
      : type_(kShape), shape_(shape), css_box_(css_box) {}

  ShapeValueType GetType() const { return type_; }
  const BasicShape* Shape() const { return shape_.Get(); }

  StyleImage* GetImage() const { return image_.Get(); }
  void SetImage(StyleImage* image) {
    DCHECK_EQ(GetType(), kImage);
    if (image_ != image) {
      image_ = image;
    }
  }
  CSSBoxType CssBox() const { return css_box_; }

  bool operator==(const ShapeValue& other) const;

  void Trace(Visitor* visitor) const {
    visitor->Trace(shape_);
    visitor->Trace(image_);
  }

 private:
  ShapeValueType type_;
  Member<const BasicShape> shape_;
  Member<StyleImage> image_;
  CSSBoxType css_box_;
};

inline bool ShapeValue::operator==(const ShapeValue& other) const {
  if (GetType() != other.GetType()) {
    return false;
  }

  switch (GetType()) {
    case kShape:
      return base::ValuesEquivalent(Shape(), other.Shape()) &&
             CssBox() == other.CssBox();
    case kBox:
      return CssBox() == other.CssBox();
    case kImage:
      return base::ValuesEquivalent(GetImage(), other.GetImage());
  }

  NOTREACHED();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHAPE_VALUE_H_
