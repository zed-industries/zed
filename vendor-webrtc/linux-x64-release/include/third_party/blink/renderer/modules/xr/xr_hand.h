// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HAND_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HAND_H_

#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_xr_hand.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class V8XRHandJoint;
class XRInputSource;
class XRJointSpace;

class XRHand : public ScriptWrappable, public PairSyncIterable<XRHand> {
  DEFINE_WRAPPERTYPEINFO();

  static constexpr unsigned kNumJoints =
      static_cast<unsigned>(device::mojom::blink::XRHandJoint::kMaxValue) + 1u;

 public:
  explicit XRHand(const device::mojom::blink::XRHandTrackingData* state,
                  XRInputSource* input_source);
  ~XRHand() override = default;

  size_t size() const { return joints_->size(); }

  XRJointSpace* get(const V8XRHandJoint& key) const;

  void updateFromHandTrackingData(
      const device::mojom::blink::XRHandTrackingData* state,
      XRInputSource* input_source);

  bool hasMissingPoses() const { return has_missing_poses_; }

  void Trace(Visitor*) const override;

 private:
  IterationSource* CreateIterationSource(ScriptState*,
                                         ExceptionState&) override;

  Member<GCedHeapVector<Member<XRJointSpace>>> joints_;
  bool has_missing_poses_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_HAND_H_
