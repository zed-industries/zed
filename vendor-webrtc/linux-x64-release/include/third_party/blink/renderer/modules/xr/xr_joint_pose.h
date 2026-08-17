// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_JOINT_POSE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_JOINT_POSE_H_

#include "third_party/blink/renderer/modules/xr/xr_pose.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class XRJointPose : public XRPose {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRJointPose(const gfx::Transform& transform, float radius);

  float radius() const { return radius_; }

 private:
  float radius_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_JOINT_POSE_H_
