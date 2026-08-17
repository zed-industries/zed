// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GRAPHICS_BINDING_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GRAPHICS_BINDING_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace gfx {
class Rect;
}  // namespace gfx

namespace blink {

class XRCompositionLayer;
class XRProjectionLayer;
class XRSession;
class XRViewData;

// Base class for XRWebGLBinding and XRGPUBinding, which helps facilitate type
// checking when layers are passed in to get sub images.
class XRGraphicsBinding : public GarbageCollectedMixin {
 public:
  enum class Api { kWebGL, kWebGPU };

  explicit XRGraphicsBinding(XRSession*);
  virtual ~XRGraphicsBinding() = default;

  XRSession* session() const { return session_.Get(); }

  double nativeProjectionScaleFactor() const;

  bool OwnsLayer(XRCompositionLayer*);

  virtual gfx::Rect GetViewportForView(XRProjectionLayer* layer,
                                       XRViewData* view) = 0;

  void Trace(Visitor*) const override;

 private:
  const Member<XRSession> session_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GRAPHICS_BINDING_H_
