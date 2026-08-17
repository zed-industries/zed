// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEWER_POSE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEWER_POSE_H_

#include "third_party/blink/renderer/modules/xr/xr_pose.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"

namespace blink {

class XRFrame;
class XRView;

template <typename IDLType>
class FrozenArray;

class XRViewerPose final : public XRPose {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRViewerPose(XRFrame*,
                        const gfx::Transform& ref_space_from_mojo,
                        const gfx::Transform& ref_space_from_viewer,
                        bool emulated_position);
  ~XRViewerPose() override = default;

  const FrozenArray<XRView>& views() const { return *views_.Get(); }

  void Trace(Visitor*) const override;

 private:
  Member<FrozenArray<XRView>> views_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEWER_POSE_H_
