// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GRIP_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GRIP_SPACE_H_

#include <optional>
#include <string>

#include "third_party/blink/renderer/modules/xr/xr_space.h"

namespace blink {

class XRGripSpace : public XRSpace {
 public:
  XRGripSpace(XRSession* session, XRInputSource* input_source);

  std::optional<gfx::Transform> MojoFromNative() const override;
  bool EmulatedPosition() const override;

  device::mojom::blink::XRNativeOriginInformationPtr NativeOrigin()
      const override;

  bool IsStationary() const override;

  std::string ToString() const override;

  void Trace(Visitor*) const override;

 private:
  Member<XRInputSource> input_source_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GRIP_SPACE_H_
