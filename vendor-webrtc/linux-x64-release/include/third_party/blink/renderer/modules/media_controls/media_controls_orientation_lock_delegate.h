// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_ORIENTATION_LOCK_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_ORIENTATION_LOCK_DELEGATE_H_

#include <optional>

#include "services/device/public/mojom/screen_orientation.mojom-blink.h"
#include "services/device/public/mojom/screen_orientation_lock_types.mojom-shared.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

class DeviceOrientationData;
class DeviceOrientationEvent;
class Document;
class HTMLVideoElement;

// MediaControlsOrientationLockDelegate is implementing the orientation lock
// feature when a <video> is fullscreen. It is meant to be created by
// `MediaControlsImpl` when the feature applies. Once created, it will use
// events to change state.
//
// The behavior depends on whether MediaControlsRotateToFullscreenDelegate is
// enabled. If it is enabled and the user has not locked the screen orientation
// at the OS level, then the orientation lock is only held until the user
// rotates their device to match the orientation of the video; otherwise it is
// held until fullscreen is exited.
//
// The different states of the class are:
// - PendingFullscreen: the object is created and it is either waiting for the
//   associated <video> to go fullscreen in order to apply an orientation lock,
//   or it already went fullscreen then the lock was unlocked since the user
//   rotated their device, and now it is waiting until fullscreen is re-entered;
// - PendingMetadata: the <video> is fullscreen but the metadata have not been
//   downloaded yet. It can happen because of network latency or because the
//   <video> went fullscreen before playback and download started;
// - MaybeLockedFullscreen: the <video> is fullscreen and a screen orientation
//   lock is applied.
//
// The possible state transitions are:
// - PendingFullscreen => PendingMetadata: on fullscreenchange event (entering
//   fullscreen) when metadata are not available;
// - PendingFullscreen => MaybeLockedFullscreen: on fullscreenchange event
//   (entering fullscreen) when metadata are available;
// - PendingMetadata => MaybeLockedFullscreen: on loadedmetadata;
// - PendingMetadata => PendingFullscreen: on fullscreenchange event (exiting
//   fullscreen);
// - MaybeLockedFullscreen => PendingFullscreen: on fullscreenchange event
//   (exiting fullscreen) or on deviceorientation event (rotated to match the
//   orientation of the video).
class MediaControlsOrientationLockDelegate final : public NativeEventListener {
 public:
  explicit MediaControlsOrientationLockDelegate(HTMLVideoElement&);

  // Called by MediaControlsImpl when the HTMLMediaElement is added to a
  // document. All event listeners should be added.
  void Attach();

  // Called by MediaControlsImpl when the HTMLMediaElement is no longer in the
  // document. All event listeners should be removed in order to prepare the
  // object to be garbage collected.
  void Detach();

  // NativeEventListener implementation.
  void Invoke(ExecutionContext*, Event*) override;
  void Trace(Visitor*) const override;

 private:
  friend class MediaControlsOrientationLockDelegateTest;
  friend class MediaControlsOrientationLockAndRotateToFullscreenDelegateTest;

  enum class State {
    kPendingFullscreen,
    kPendingMetadata,
    kMaybeLockedFullscreen,
  };

  enum class DeviceOrientationType {
    kUnknown,
    kFlat,
    kDiagonal,
    kPortrait,
    kLandscape
  };

  HTMLVideoElement& VideoElement() const;
  Document& GetDocument() const;

  // Returns the orientation in which the video should be locked based on its
  // size.
  MODULES_EXPORT device::mojom::blink::ScreenOrientationLockType
  ComputeOrientationLock() const;

  // Locks the screen orientation if the video has metadata information
  // available. Delays locking orientation until metadata are available
  // otherwise.
  void MaybeLockOrientation();

  // Changes a previously locked screen orientation to instead be locked to
  // the "any" orientation that allows accelerometer-based rotation. This is
  // not the same as unlocking (which returns to the "default" orientation,
  // which may in fact be more restrictive).
  void ChangeLockToAnyOrientation();

  // Unlocks the screen orientation if the screen orientation was previously
  // locked.
  void MaybeUnlockOrientation();

  void MaybeListenToDeviceOrientation();
  void GotIsAutoRotateEnabledByUser(bool enabled);

  MODULES_EXPORT DeviceOrientationType
  ComputeDeviceOrientation(DeviceOrientationData*) const;

  void MaybeLockToAnyIfDeviceOrientationMatchesVideo(DeviceOrientationEvent*);

  // Delay before `MaybeLockToAnyIfDeviceOrientationMatchesVideo` changes lock.
  // Emprically, 200ms is too short, but 250ms avoids glitches. 500ms gives us
  // a 2x margin in case the device is running slow, without being noticeable.
  MODULES_EXPORT static constexpr base::TimeDelta kLockToAnyDelay =
      base::Milliseconds(500);

  // Current state of the object. See comment at the top of the file for a
  // detailed description.
  State state_ = State::kPendingFullscreen;

  // Which lock is currently applied by this delegate.
  device::mojom::blink::ScreenOrientationLockType locked_orientation_ =
      device::mojom::blink::ScreenOrientationLockType::DEFAULT /* unlocked */;

  TaskHandle lock_to_any_task_;

  HeapMojoRemote<device::mojom::blink::ScreenOrientationListener> monitor_;

  std::optional<bool> is_auto_rotate_enabled_by_user_override_for_testing_;

  // `video_element_` owns MediaControlsImpl that owns |this|.
  Member<HTMLVideoElement> video_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_ORIENTATION_LOCK_DELEGATE_H_
