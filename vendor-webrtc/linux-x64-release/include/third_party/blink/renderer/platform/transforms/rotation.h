// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_ROTATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_ROTATION_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/vector3d_f.h"

namespace blink {

struct PLATFORM_EXPORT Rotation {
  Rotation() : axis(0, 0, 0), angle(0) {}

  Rotation(const gfx::Vector3dF& axis, double angle)
      : axis(axis), angle(angle) {}

  // If either rotation is effectively "zero" or both rotations share the same
  // normalized axes this function returns true and the "non-zero" axis is
  // returned as resultAxis and the effective angles are returned as
  // resultAngleA and resultAngleB.  Otherwise false is returned.
  static bool GetCommonAxis(const Rotation& /*a*/,
                            const Rotation& /*b*/,
                            gfx::Vector3dF& result_axis,
                            double& result_angle_a,
                            double& result_angle_b);

  // A progress of 0 corresponds to "from" and a progress of 1 corresponds to
  // "to".
  static Rotation Slerp(const Rotation& from,
                        const Rotation& to,
                        double progress);

  // Returns a rotation whose effect is equivalent to applying a followed by b.
  static Rotation Add(const Rotation& /*a*/, const Rotation& /*b*/);

  // No restrictions on the axis vector.
  gfx::Vector3dF axis;

  // Measured in degrees.
  double angle;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_ROTATION_H_
