// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEW_H_

#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/xr/xr_graphics_binding.h"
#include "third_party/blink/renderer/modules/xr/xr_rigid_transform.h"
#include "third_party/blink/renderer/modules/xr/xr_viewport.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/point3_f.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class V8XREye;
class XRCamera;
class XRCPUDepthInformation;
class XRDepthManager;
class XRFrame;
class XRSession;
class XRViewData;
class XRWebGLDepthInformation;

class MODULES_EXPORT XRView final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRView(XRFrame* frame,
         XRViewData* view_data,
         const gfx::Transform& ref_space_from_mojo);

  V8XREye eye() const;
  device::mojom::blink::XREye EyeValue() const { return eye_; }
  XRViewData* ViewData() const { return view_data_.Get(); }
  XRViewport* Viewport(double scale);

  XRFrame* frame() const;
  XRSession* session() const;
  NotShared<DOMFloat32Array> projectionMatrix() const;
  XRRigidTransform* refSpaceFromView() const;
  XRCamera* camera() const;

  // isFirstPersonObserver is only true for views that composed with a video
  // feed that is not directly displayed on the viewer device. Primarily this is
  // used for video streams from optically transparent AR headsets.
  bool isFirstPersonObserver() const;

  std::optional<double> recommendedViewportScale() const;
  void requestViewportScale(std::optional<double> scale);

  void Trace(Visitor*) const override;

  XRCPUDepthInformation* GetCpuDepthInformation(
      ExceptionState& exception_state) const;

  XRWebGLDepthInformation* GetWebGLDepthInformation(
      ExceptionState& exception_state) const;

 private:
  device::mojom::blink::XREye eye_;
  Member<XRFrame> frame_;
  Member<XRViewData> view_data_;
  // The transform from the view to the reference space requested by
  // XRFrame::getViewerPose.
  Member<XRRigidTransform> ref_space_from_view_;
  NotShared<DOMFloat32Array> projection_matrix_;
  Member<XRViewport> viewport_;
};

class MODULES_EXPORT XRViewData final : public GarbageCollected<XRViewData> {
 public:
  explicit XRViewData(wtf_size_t index,
                      device::mojom::blink::XREye eye,
                      gfx::Rect viewport,
                      XRGraphicsBinding::Api graphics_api)
      : index_(index),
        eye_(eye),
        graphics_api_(graphics_api),
        viewport_(viewport) {}
  XRViewData(
      wtf_size_t index,
      device::mojom::blink::XRViewPtr view,
      double depth_near,
      double depth_far,
      const device::mojom::blink::XRSessionDeviceConfig& device_config,
      const HashSet<device::mojom::XRSessionFeature>& enabled_feature_set,
      XRGraphicsBinding::Api graphics_api);

  void UpdateView(device::mojom::blink::XRViewPtr view,
                  double depth_near,
                  double depth_far);

  void UpdateProjectionMatrixFromFoV(float up_rad,
                                     float down_rad,
                                     float left_rad,
                                     float right_rad,
                                     float near_depth,
                                     float far_depth);
  void UpdateProjectionMatrixFromAspect(float fovy,
                                        float aspect,
                                        float near_depth,
                                        float far_depth);

  gfx::Transform UnprojectPointer(double x,
                                  double y,
                                  double canvas_width,
                                  double canvas_height);

  void SetMojoFromView(const gfx::Transform& mojo_from_view);

  wtf_size_t index() const { return index_; }
  device::mojom::blink::XREye Eye() const { return eye_; }
  const gfx::Transform& MojoFromView() const { return mojo_from_view_; }
  const gfx::Transform& ProjectionMatrix() const { return projection_matrix_; }
  const gfx::Rect& Viewport() const { return viewport_; }
  bool IsFirstPersonObserver() const { return is_first_person_observer_; }

  XRCPUDepthInformation* GetCpuDepthInformation(
      const XRFrame* xr_frame,
      ExceptionState& exception_state) const;

  XRWebGLDepthInformation* GetWebGLDepthInformation(
      const XRFrame* xr_frame,
      ExceptionState& exception_state) const;

  std::optional<double> recommendedViewportScale() const;
  void SetRecommendedViewportScale(std::optional<double> scale) {
    recommended_viewport_scale_ = scale;
  }

  void requestViewportScale(std::optional<double> scale);

  bool ViewportModifiable() const { return viewport_modifiable_; }
  void SetViewportModifiable(bool modifiable) {
    viewport_modifiable_ = modifiable;
  }
  double CurrentViewportScale() const { return current_viewport_scale_; }
  void SetCurrentViewportScale(double scale) {
    current_viewport_scale_ = scale;
  }
  double RequestedViewportScale() const { return requested_viewport_scale_; }

  // Returns true if the viewport scale actually changed.
  bool ApplyViewportScaleForFrame();

  void Trace(Visitor*) const;

 private:
  const wtf_size_t index_;
  const device::mojom::blink::XREye eye_;
  const XRGraphicsBinding::Api graphics_api_;
  gfx::Transform mojo_from_view_;
  gfx::Transform projection_matrix_;
  gfx::Transform inv_projection_;
  bool inv_projection_dirty_ = true;
  gfx::Rect viewport_;
  bool is_first_person_observer_ = false;
  std::optional<double> recommended_viewport_scale_ = std::nullopt;
  double requested_viewport_scale_ = 1.0;
  double current_viewport_scale_ = 1.0;
  bool viewport_modifiable_ = false;
  Member<XRDepthManager> depth_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_VIEW_H_
