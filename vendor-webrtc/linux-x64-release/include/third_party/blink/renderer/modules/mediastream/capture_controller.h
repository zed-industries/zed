// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_CAPTURE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_CAPTURE_CONTROLLER_H_

#include <optional>

#include "base/functional/callback_helpers.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_capture_start_focus_behavior.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_captured_wheel_action.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;
class HTMLElement;

class MODULES_EXPORT CaptureController final
    : public EventTarget,
      public ExecutionContextClient,
      public MediaStreamSource::Observer {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static Vector<int> getSupportedZoomLevelsForTabs();

  static CaptureController* Create(ExecutionContext*);

  explicit CaptureController(ExecutionContext*);

  // IDL interface
  // https://w3c.github.io/mediacapture-screen-share/#dom-capturecontroller-setfocusbehavior
  void setFocusBehavior(V8CaptureStartFocusBehavior, ExceptionState&);

  // Captured Surface Control IDL interface - scrolling
  ScriptPromise<IDLUndefined> forwardWheel(ScriptState* script_state,
                                           HTMLElement* element);

  // Captured Surface Control IDL interface - zoom controls.
  Vector<int> getSupportedZoomLevels(ExceptionState& exception_state);
  std::optional<int> zoomLevel() const;
  ScriptPromise<IDLUndefined> increaseZoomLevel(ScriptState* script_state);
  ScriptPromise<IDLUndefined> decreaseZoomLevel(ScriptState* script_state);
  ScriptPromise<IDLUndefined> resetZoomLevel(ScriptState* script_state);

  void SetIsBound(bool value) { is_bound_ = value; }
  bool IsBound() const { return is_bound_; }

  // When getDisplayMedia() is resolved, `video_track_` and `descriptor_id_` are
  // set.
  void SetVideoTrack(MediaStreamTrack* video_track, std::string descriptor_id);

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // https://screen-share.github.io/mouse-events/#capture-controller-extensions
  DEFINE_ATTRIBUTE_EVENT_LISTENER(capturedmousechange, kCapturedmousechange)

  // https://w3c.github.io/mediacapture-surface-control/#dom-capturecontroller-onzoomlevelchange
  DEFINE_ATTRIBUTE_EVENT_LISTENER(zoomlevelchange, kZoomlevelchange)

  // Close the window of opportunity to make the focus decision.
  // Further calls to setFocusBehavior() will raise an exception.
  // https://w3c.github.io/mediacapture-screen-share/#dfn-finalize-focus-decision-algorithm
  void FinalizeFocusDecision();

  // MediaStreamSource::Observer
  void SourceChangedState() override {}
  void SourceChangedCaptureConfiguration() override {}
  void SourceChangedCaptureHandle() override {}
#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
  void SourceChangedZoomLevel(int) override;

  void SetMediaStreamDispatcherHostForTesting(
      mojo::PendingRemote<mojom::blink::MediaStreamDispatcherHost>);
#endif
  void Trace(Visitor* visitor) const override;

 private:
  struct ValidationResult {
    ValidationResult(DOMExceptionCode code, String message);

    DOMExceptionCode code;
    String message;
  };

  ValidationResult ValidateCapturedSurfaceControlCall() const;

  ScriptPromise<IDLUndefined> UpdateZoomLevel(
      ScriptState* script_state,
      mojom::blink::ZoomLevelAction action);

#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
  // Deliver a wheel event on the captured tab.
  //
  // `relative_x` is a value from [0, 1). It denotes the relative position
  // in the coordinate space of the captured surface, which is unknown to the
  // capturer. A value of 0 denotes the leftmost pixel; increasing values denote
  // values further to the right. The sender of the message scales from its own
  // coordinate space down to the relative values, and the receiver scales
  // back up to its own coordinates.
  //
  // `relative_y` is defined analogously to `relative_x`.
  //
  // `wheel_delta_x` and `wheel_delta_y` represent the scroll deltas in pixels.
  void SendWheel(double relative_x,
                 double relative_y,
                 int32_t wheel_delta_x,
                 int32_t wheel_delta_y);
  class WheelEventListener;

  mojom::blink::MediaStreamDispatcherHost* GetMediaStreamDispatcherHost();
  void OnForwardWheelPermissionResult(
      ScriptPromiseResolver<IDLUndefined>*,
      HTMLElement*,
      mojom::blink::CapturedSurfaceControlResult);
  bool DoForwardWheel(ScriptState*, HTMLElement*);
#endif

  // Whether this CaptureController has been passed to a getDisplayMedia() call.
  // This helps enforce the requirement that any CaptureController may only
  // be used with a single getDisplayMedia() call.
  // Checking `video_track_` for nullness and/or `descriptor_id_` for emptiness
  // would not have sufficed, as `is_bound_` is flipped even if the promise
  // ends up being rejected.
  bool is_bound_ = false;

  // The video track returned by getDisplayMedia(). It is saved to check whether
  // it's live and capturing a tab or a window.
  WeakMember<MediaStreamTrack> video_track_;

  // Identity browser-side the device request to focus or not the captured
  // surface.
  std::string descriptor_id_;

  // Set by setFocusBehavior(). Indicates whether the app wishes the
  // captured surface to be focused or not once getDisplayMedia() is
  // resolved. If left unset, the default behavior will be used.
  // The default behavior is not specified, and may change.
  // Presently, the default is to focus.
  std::optional<V8CaptureStartFocusBehavior> focus_behavior_;

  // Track whether the window of opportunity to call setFocusBehavior() is still
  // open. Once set to true, this never changes.
  bool focus_decision_finalized_ = false;

  // The last known zoom level of the captured surface.
  // Set to a concrete value when capture starts.
  // Never changes back to nullopt.
  // Always stays at `std::nullopt` for window- and screen-capture.
  std::optional<int> zoom_level_;

#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
  Member<WheelEventListener> wheel_listener_;
  HeapMojoRemote<mojom::blink::MediaStreamDispatcherHost>
      media_stream_dispatcher_host_;

#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_CAPTURE_CONTROLLER_H_
