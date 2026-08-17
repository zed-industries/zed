// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CAMERA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CAMERA_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class XRFrame;

class XRCamera final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRCamera(XRFrame* frame);

  int width() const { return size_.width(); }
  int height() const { return size_.height(); }

  XRFrame* Frame() const;

  void Trace(Visitor* visitor) const override;

 private:
  Member<XRFrame> frame_;

  const gfx::Size size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CAMERA_H_
