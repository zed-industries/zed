// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_UTILS_H_

#include <optional>

#include "device/vr/public/mojom/pose.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_hand_joint.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace gfx {
class Transform;
}

namespace blink {

class DOMPointReadOnly;
class ExecutionContext;
class WebGLRenderingContextBase;

NotShared<DOMFloat32Array> transformationMatrixToDOMFloat32Array(
    const gfx::Transform&);

gfx::Transform DOMFloat32ArrayToTransform(NotShared<DOMFloat32Array>);

gfx::Transform WTFFloatVectorToTransform(const Vector<float>&);

DOMPointReadOnly* makeNormalizedQuaternion(double x,
                                           double y,
                                           double z,
                                           double w);

WebGLRenderingContextBase* webglRenderingContextBaseFromUnion(
    const V8XRWebGLRenderingContext* context);

constexpr char kUnableToNormalizeZeroLength[] =
    "Unable to normalize vector of length 0.";

// Conversion method from transformation matrix to device::Pose. The conversion
// may fail if the matrix cannot be decomposed. In case of failure, the method
// will return std::nullopt.
// TODO(crbug.com/1359528): The above comment about failure is not true.
// Remove this function.
std::optional<device::Pose> CreatePose(const gfx::Transform& matrix);

// Hand joint conversion methods
device::mojom::blink::XRHandJoint StringToMojomHandJoint(
    const String& hand_joint_string);
V8XRHandJoint::Enum MojomHandJointToV8Enum(
    device::mojom::blink::XRHandJoint hand_joint);

// Converts the given string to an XRSessionFeature. If the string is
// unrecognized, returns nullopt. Based on the spec:
// https://immersive-web.github.io/webxr/#feature-name
std::optional<device::mojom::XRSessionFeature> StringToXRSessionFeature(
    const String& feature_string);

// Inverse of |StringToXRSessionFeature()|, used for logging to console and for
// |XRSession::enabledFeatures|.
String XRSessionFeatureToString(device::mojom::XRSessionFeature feature);

bool IsFeatureEnabledForContext(device::mojom::XRSessionFeature feature,
                                const ExecutionContext* context);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_UTILS_H_
