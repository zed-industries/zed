// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_H_

#include <memory>

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_dom_matrix.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/xr/xr_joint_pose.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class ExceptionState;
class ScriptState;
class XRAnchor;
class XRAnchorSet;
class XRCPUDepthInformation;
class XRHitTestResult;
class XRHitTestSource;
class XRImageTrackingResult;
class XRInputSource;
class XRJointPose;
class XRLightEstimate;
class XRLightProbe;
class XRJointSpace;
class XRPlaneSet;
class XRPose;
class XRReferenceSpace;
class XRRigidTransform;
class XRSession;
class XRSpace;
class XRTransientInputHitTestResult;
class XRTransientInputHitTestSource;
class XRView;
class XRViewerPose;

template <typename IDLType>
class FrozenArray;

class XRFrame final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static constexpr char kInactiveFrame[] =
      "XRFrame access outside the callback that produced it is invalid.";
  static constexpr char kNonAnimationFrame[] =
      "This method can only be called on XRFrame objects passed to "
      "XRSession.requestAnimationFrame callbacks.";

  explicit XRFrame(XRSession* session, bool is_animation_frame = false);

  XRSession* session() const { return session_.Get(); }

  // Returns basespace_from_viewer.
  XRViewerPose* getViewerPose(XRReferenceSpace* basespace,
                              ExceptionState& exception_state);

  // Return an XRPose that has a transform of basespace_from_space, while
  // accounting for the base pose matrix of this frame. If computing a transform
  // isn't possible, return nullptr.
  XRPose* getPose(XRSpace* space,
                  XRSpace* basespace,
                  ExceptionState& exception_state);

  XRAnchorSet* trackedAnchors() const;
  XRLightEstimate* getLightEstimate(XRLightProbe* light_probe,
                                    ExceptionState& exception_state) const;
  XRCPUDepthInformation* getDepthInformation(
      XRView* view,
      ExceptionState& exception_state) const;
  XRPlaneSet* detectedPlanes(ExceptionState& exception_state) const;

  void Trace(Visitor*) const override;

  void Deactivate();

  bool IsActive() const;

  bool IsAnimationFrame() const { return is_animation_frame_; }

  const FrozenArray<XRHitTestResult>& getHitTestResults(
      XRHitTestSource* hit_test_source,
      ExceptionState& exception_state);

  const FrozenArray<XRTransientInputHitTestResult>&
  getHitTestResultsForTransientInput(
      XRTransientInputHitTestSource* hit_test_source,
      ExceptionState& exception_state);

  ScriptPromise<XRAnchor> createAnchor(ScriptState* script_state,
                                       XRRigidTransform* initial_pose,
                                       XRSpace* space,
                                       ExceptionState& exception_state);

  const FrozenArray<XRImageTrackingResult>& getImageTrackingResults(
      ExceptionState&);

  XRJointPose* getJointPose(XRJointSpace* joint,
                            XRSpace* baseSpace,
                            ExceptionState& exception_state) const;
  bool fillJointRadii(const HeapVector<Member<XRJointSpace>>& jointSpaces,
                      NotShared<DOMFloat32Array> radii,
                      ExceptionState& exception_state) const;
  bool fillPoses(const HeapVector<Member<XRSpace>>& spaces,
                 XRSpace* baseSpace,
                 NotShared<DOMFloat32Array> transforms,
                 ExceptionState& exception_state) const;

 private:
  std::unique_ptr<gfx::Transform> GetAdjustedPoseMatrix(XRSpace*) const;
  XRPose* GetTargetRayPose(XRInputSource*, XRSpace*) const;
  XRPose* GetGripPose(XRInputSource*, XRSpace*) const;
  // Helper that creates an anchor with the assumption that the conversion from
  // passed in space to a stationary space is required.
  // |native_origin_from_anchor| is a transform from |space|'s native origin to
  // the desired anchor position (i.e. the origin-offset of the |space| is
  // already taken into account).
  ScriptPromise<XRAnchor> CreateAnchorFromNonStationarySpace(
      ScriptState* script_state,
      const gfx::Transform& native_origin_from_anchor,
      XRSpace* space,
      std::optional<uint64_t> maybe_plane_id,
      ExceptionState& exception_state);
  // Helper for checking if space and frame have the same session.
  // Sets kInvalidStateError exception state if sessions are different.
  bool IsSameSession(XRSession* space_session,
                     ExceptionState& exception_state) const;

  const Member<XRSession> session_;

  // Frames are only active during callbacks. getPose and getViewerPose should
  // only be called from JS on active frames.
  bool is_active_ = true;

  // Only frames created by XRSession.requestAnimationFrame callbacks are
  // animation frames. getViewerPose should only be called from JS on active
  // animation frames.
  bool is_animation_frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_H_
