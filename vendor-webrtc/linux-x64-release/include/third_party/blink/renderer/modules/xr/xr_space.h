// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SPACE_H_

#include <memory>
#include <optional>
#include <string>

#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/transform.h"

namespace gfx {
class Transform;
}

namespace blink {

class XRInputSource;
class XRPose;
class XRSession;

class XRSpace : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 protected:
  explicit XRSpace(XRSession* session);

 public:
  ~XRSpace() override;

  // Gets the pose of this space's native origin in mojo space. This transform
  // maps from this space's native origin to mojo space (aka device space).
  // Unless noted otherwise, all data returned over vr_service.mojom interfaces
  // is expressed in mojo space coordinates.
  // Returns nullopt if computing a transform is not possible.
  virtual std::optional<gfx::Transform> MojoFromNative() const = 0;

  // Convenience method to try to get the inverse of the above. This will return
  // the pose of the mojo origin in this space's native origin.
  // Returns nullopt if computing a transform is not possible.
  std::optional<gfx::Transform> NativeFromMojo() const;

  // Gets the viewer pose in the native coordinates of this space, corresponding
  // to a transform from viewer coordinates to this space's native coordinates.
  // (The position elements of the transformation matrix are the viewer's
  // location in this space's coordinates.)
  // Prefer this helper method over querying NativeFromMojo and multiplying
  // on the calling side, as this allows the viewer space to return identity
  // instead of something near to, but not quite, identity.
  // Returns nullopt if computing a transform is not possible.
  virtual std::optional<gfx::Transform> NativeFromViewer(
      const std::optional<gfx::Transform>& mojo_from_viewer) const;

  // Convenience method for calling NativeFromViewer with the current
  // MojoFromViewer of the session associated with this space. This also handles
  // the multiplication of OffsetFromNative onto the result of NativeFromViewer.
  // Returns nullopt if computing a transform is not possible.
  std::optional<gfx::Transform> OffsetFromViewer() const;

  // Return origin offset matrix, aka native_origin_from_offset_space.
  virtual gfx::Transform NativeFromOffsetMatrix() const;
  virtual gfx::Transform OffsetFromNativeMatrix() const;

  // Returns transformation from offset space to mojo space. Convenience method,
  // returns MojoFromNative() * NativeFromOffsetMatrix() or nullopt if computing
  // a transform is not possible.
  std::optional<gfx::Transform> MojoFromOffsetMatrix() const;

  // Returns true when invoked on a space that is deemed stationary, false
  // otherwise (this means that the space is considered dynamic). Stationary
  // spaces are the spaces that should remain stable relative to the environment
  // over longer periods (i.e. longer than a single frame). Examples of
  // stationary spaces are for XRReferenceSpaces (with the exception of "viewer"
  // space), anchor spaces, plane spaces. Examples of dynamic spaces are input
  // source spaces and XRReference space of type "viewer".
  virtual bool IsStationary() const = 0;

  // Indicates whether or not the position portion of the native origin of this
  // space is emulated.
  virtual bool EmulatedPosition() const;

  // Gets the pose of this space's origin in |other_space|. This is a transform
  // that maps from this space to the other's space, or in other words:
  // other_from_this.
  virtual XRPose* getPose(const XRSpace* other_space) const;

  XRSession* session() const { return session_.Get(); }

  // ToString() helper, used for debugging.
  virtual std::string ToString() const = 0;

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  virtual device::mojom::blink::XRNativeOriginInformationPtr NativeOrigin()
      const = 0;

  void Trace(Visitor* visitor) const override;

 private:
  const Member<XRSession> session_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SPACE_H_
