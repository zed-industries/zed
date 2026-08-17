// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ANCHOR_H_

#include <optional>

#include "device/vr/public/mojom/pose.h"
#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class ExceptionState;
class XRSession;
class XRSpace;

class XRAnchor : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRAnchor(uint64_t id,
           XRSession* session,
           const device::mojom::blink::XRAnchorData& anchor_data);

  uint64_t id() const;

  XRSpace* anchorSpace(ExceptionState& exception_state) const;

  std::optional<gfx::Transform> MojoFromObject() const;

  device::mojom::blink::XRNativeOriginInformationPtr NativeOrigin() const;

  void Delete();

  void Update(const device::mojom::blink::XRAnchorData& anchor_data);

  bool IsStationary() const { return true; }

  void Trace(Visitor* visitor) const override;

 private:
  const uint64_t id_;

  bool is_deleted_;

  Member<XRSession> session_;

  // Anchor's pose in device (mojo) space. Nullopt if the pose of the anchor is
  // unknown in the current frame.
  std::optional<device::Pose> mojo_from_anchor_;

  // Cached anchor space - it will be created by `anchorSpace()` if it's not
  // set.
  mutable Member<XRSpace> anchor_space_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ANCHOR_H_
