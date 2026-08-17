// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RENDER_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RENDER_STATE_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/frozen_array.h"
#include "third_party/blink/renderer/modules/xr/xr_layer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class HTMLCanvasElement;
class XRWebGLLayer;
class XRLayer;
class XRRenderStateInit;

class XRRenderState : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRRenderState(bool immersive);
  ~XRRenderState() override = default;

  // Near and far depths are used when computing projection matrices for the
  // Session's views.
  double depthNear() const { return depth_near_; }
  double depthFar() const { return depth_far_; }
  std::optional<double> inlineVerticalFieldOfView() const;
  XRWebGLLayer* baseLayer() const { return base_layer_.Get(); }
  const FrozenArray<XRLayer>& layers() const { return *layers_.Get(); }

  // Returns either baseLayer or layers[0], or nullptr if neither is set.
  XRLayer* GetFirstLayer() const;

  HTMLCanvasElement* output_canvas() const;

  void Update(const XRRenderStateInit* init);

  // Only used when removing an outputContext from the session because it was
  // bound to a different session.
  void removeOutputContext();

  void Trace(Visitor*) const override;

 private:
  bool immersive_;
  double depth_near_ = 0.1;
  double depth_far_ = 1000.0;
  Member<XRWebGLLayer> base_layer_;
  Member<FrozenArray<XRLayer>> layers_ =
      MakeGarbageCollected<FrozenArray<XRLayer>>();
  std::optional<double> inline_vertical_fov_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_RENDER_STATE_H_
