// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HIT_TEST_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HIT_TEST_RESULT_H_

#include <optional>

#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class ScriptState;
class XRAnchor;
class XRPose;
class XRSession;
class XRSpace;

class XRHitTestResult : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRHitTestResult(XRSession* session,
                           const device::mojom::blink::XRHitResult& hit_result);

  XRPose* getPose(XRSpace* relative_to, ExceptionState& exception_state);

  ScriptPromise<XRAnchor> createAnchor(ScriptState* script_state,
                                       ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override;

 private:
  Member<XRSession> session_;

  // Hit test results do not have origin-offset so mojo_from_this_ contains
  // mojo_from_this with origin-offset (identity) already applied.
  device::Pose mojo_from_this_;
  std::optional<uint64_t> plane_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HIT_TEST_RESULT_H_
