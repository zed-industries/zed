// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_ORIENTATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_ORIENTATION_CONTROLLER_H_

#include <memory>

#include "services/device/public/mojom/screen_orientation.mojom-blink.h"
#include "services/device/public/mojom/screen_orientation_lock_types.mojom-shared.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/screen_orientation/web_lock_orientation_callback.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "ui/display/mojom/screen_orientation.mojom-blink.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class ScreenOrientation;

using device::mojom::blink::ScreenOrientationLockResult;

class MODULES_EXPORT ScreenOrientationController final
    : public GarbageCollected<ScreenOrientationController>,
      public ExecutionContextLifecycleObserver,
      public PageVisibilityObserver,
      public Supplement<LocalDOMWindow> {
 public:
  explicit ScreenOrientationController(LocalDOMWindow&);

  ScreenOrientationController(const ScreenOrientationController&) = delete;
  ScreenOrientationController& operator=(const ScreenOrientationController&) =
      delete;

  ~ScreenOrientationController() override;

  void SetOrientation(ScreenOrientation*);
  void NotifyOrientationChanged();

  void lock(device::mojom::blink::ScreenOrientationLockType,
            std::unique_ptr<WebLockOrientationCallback>);
  void unlock();
  bool MaybeHasActiveLock() const;

  static const char kSupplementName[];
  static ScreenOrientationController* From(LocalDOMWindow&);
  static ScreenOrientationController* FromIfExists(LocalDOMWindow&);

  void SetScreenOrientationAssociatedRemoteForTests(
      HeapMojoAssociatedRemote<device::mojom::blink::ScreenOrientation>);

  void Trace(Visitor*) const override;

 private:
  friend class MediaControlsOrientationLockAndRotateToFullscreenDelegateTest;
  friend class ScreenOrientationControllerTest;

  static display::mojom::blink::ScreenOrientation ComputeOrientation(
      const gfx::Rect&,
      uint16_t);
  void NotifyOrientationChangedInternal();

  // Inherited from ExecutionContextLifecycleObserver and
  // PageVisibilityObserver.
  void ContextDestroyed() override;
  void PageVisibilityChanged() override;

  void UpdateOrientation();

  // Sends binding requests to the browser for
  // device::mojom::blink::ScreenOrientation. This method can be called when:
  // * `this` is created in non-prerendering context.
  // * The corresponding prerendering context is activated.
  void BuildMojoConnection();

  bool IsActiveAndVisible() const;

  void OnLockOrientationResult(int, ScreenOrientationLockResult);
  void CancelPendingLocks();
  int GetRequestIdForTests();

  void LockOrientationInternal(
      device::mojom::blink::ScreenOrientationLockType orientation,
      std::unique_ptr<WebLockOrientationCallback> callback);
  void UnlockOrientationInternal();

  Member<ScreenOrientation> orientation_;
  bool active_lock_ = false;
  HeapMojoAssociatedRemote<device::mojom::blink::ScreenOrientation>
      screen_orientation_service_;
  std::unique_ptr<WebLockOrientationCallback> pending_callback_;
  int request_id_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_ORIENTATION_CONTROLLER_H_
