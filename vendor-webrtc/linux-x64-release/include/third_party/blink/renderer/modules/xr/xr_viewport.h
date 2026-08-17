// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEWPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEWPORT_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class XRViewport final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRViewport(int x, int y, int width, int height)
      : x_(x), y_(y), width_(width), height_(height) {}

  int x() const { return x_; }
  int y() const { return y_; }
  int width() const { return width_; }
  int height() const { return height_; }

 private:
  int x_;
  int y_;
  int width_;
  int height_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEWPORT_H_
