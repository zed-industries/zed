// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LIGHT_PROBE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LIGHT_PROBE_H_

#include <memory>
#include <optional>

#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace gfx {
class Transform;
}

namespace blink {

class XRCubeMap;
class XRLightEstimate;
class XRLightProbeInit;
class XRSession;
class XRSpace;

class XRLightProbe : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRLightProbe(XRSession* session, XRLightProbeInit* options);

  enum XRReflectionFormat {
    kReflectionFormatSRGBA8 = 0,
    kReflectionFormatRGBA16F = 1
  };

  XRSession* session() const { return session_.Get(); }

  XRSpace* probeSpace() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(reflectionchange, kReflectionchange)

  std::optional<gfx::Transform> MojoFromObject() const;

  device::mojom::blink::XRNativeOriginInformationPtr NativeOrigin() const;

  void ProcessLightEstimationData(
      const device::mojom::blink::XRLightEstimationData* data,
      double timestamp);

  XRLightEstimate* getLightEstimate() { return light_estimate_.Get(); }
  XRCubeMap* getReflectionCubeMap() { return cube_map_.get(); }

  XRReflectionFormat ReflectionFormat() const { return reflection_format_; }

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  bool IsStationary() const { return true; }

  void Trace(Visitor* visitor) const override;

 private:
  Member<XRSession> session_;
  mutable Member<XRSpace> probe_space_;
  Member<XRLightEstimate> light_estimate_;

  XRReflectionFormat reflection_format_;
  double last_reflection_change_ = 0.0;
  std::unique_ptr<XRCubeMap> cube_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LIGHT_PROBE_H_
