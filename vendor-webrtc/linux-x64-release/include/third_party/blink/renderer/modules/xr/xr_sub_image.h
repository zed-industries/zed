// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SUB_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SUB_IMAGE_H_

#include "third_party/blink/renderer/modules/xr/xr_viewport.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class XRSubImage : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRSubImage(const gfx::Rect& viewport)
      : viewport_(MakeGarbageCollected<XRViewport>(viewport.x(),
                                                   viewport.y(),
                                                   viewport.width(),
                                                   viewport.height())) {}

  XRViewport* viewport() const { return viewport_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(viewport_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  Member<XRViewport> viewport_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SUB_IMAGE_H_
