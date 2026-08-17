// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MOCK_CSS_PAINT_IMAGE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MOCK_CSS_PAINT_IMAGE_GENERATOR_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/core/css/css_paint_image_generator.h"
#include "third_party/blink/renderer/platform/graphics/image.h"

using testing::ReturnRef;

namespace blink {

class MockCSSPaintImageGenerator : public CSSPaintImageGenerator {
 public:
  MockCSSPaintImageGenerator() {
    // These methods return references, so setup a default ON_CALL to make them
    // easier to use. They can be overridden by a specific test if desired.
    ON_CALL(*this, NativeInvalidationProperties())
        .WillByDefault(ReturnRef(native_properties_));
    ON_CALL(*this, CustomInvalidationProperties())
        .WillByDefault(ReturnRef(custom_properties_));
    ON_CALL(*this, InputArgumentTypes())
        .WillByDefault(ReturnRef(input_argument_types_));
  }

  MOCK_METHOD3(Paint,
               scoped_refptr<Image>(const ImageResourceObserver&,
                                    const gfx::SizeF& container_size,
                                    const GCedCSSStyleValueVector*));
  MOCK_CONST_METHOD0(NativeInvalidationProperties, Vector<CSSPropertyID>&());
  MOCK_CONST_METHOD0(CustomInvalidationProperties, Vector<AtomicString>&());
  MOCK_CONST_METHOD0(HasAlpha, bool());
  MOCK_CONST_METHOD0(InputArgumentTypes, Vector<CSSSyntaxDefinition>&());
  MOCK_CONST_METHOD0(IsImageGeneratorReady, bool());
  MOCK_CONST_METHOD0(WorkletId, int());

  void AddCustomProperty(const AtomicString& custom_property) {
    custom_properties_.push_back(custom_property);
  }
  void AddNativeProperty() {
    native_properties_.push_back(CSSPropertyID::kBorderImageSource);
  }

 private:
  Vector<CSSPropertyID> native_properties_;
  Vector<AtomicString> custom_properties_;
  Vector<CSSSyntaxDefinition> input_argument_types_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MOCK_CSS_PAINT_IMAGE_GENERATOR_H_
