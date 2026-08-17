// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_STUB_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_STUB_IMAGE_H_

#include "third_party/blink/renderer/platform/graphics/image.h"

namespace blink {

class StubImage : public Image {
 public:
  StubImage() = default;

  bool CurrentFrameKnownToBeOpaque() override { return false; }
  gfx::Size SizeWithConfig(SizeConfig) const override {
    return gfx::Size(10, 10);
  }
  void DestroyDecodedData() override {}
  PaintImage PaintImageForCurrentFrame() override { return PaintImage(); }
  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF& dst_rect,
            const gfx::RectF& src_rect,
            const ImageDrawOptions&) override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_STUB_IMAGE_H_
