// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCE_H_

#include <memory>
#include <optional>

#include "base/time/time.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/modules/gamepad/gamepad.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/transform.h"

namespace device {
template <class T>
class GamepadImpl;
using Gamepad = GamepadImpl<void>;
}

namespace blink {

class Element;
class V8XRHandedness;
class V8XRTargetRayMode;
class XRGripSpace;
class XRHand;
class XRInputSourceEvent;
class XRSession;
class XRSpace;
class XRTargetRaySpace;

template <typename IDLType>
class FrozenArray;

class XRInputSource : public ScriptWrappable, public Gamepad::Client {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XRInputSource* CreateOrUpdateFrom(
      XRInputSource* other /* may be null, input */,
      XRSession* session,
      const device::mojom::blink::XRInputSourceStatePtr& state);

  XRInputSource(XRSession*,
                uint32_t source_id,
                device::mojom::XRTargetRayMode target_ray_mode =
                    device::mojom::XRTargetRayMode::GAZING);
  XRInputSource(const XRInputSource& other);
  ~XRInputSource() override = default;

  int16_t activeFrameId() const { return state_.active_frame_id; }
  void setActiveFrameId(int16_t id) { state_.active_frame_id = id; }

  XRSession* session() const { return session_.Get(); }

  device::mojom::XRHandedness xr_handedness() const {
    return state_.handedness;
  }

  V8XRHandedness handedness() const;
  V8XRTargetRayMode targetRayMode() const;
  bool emulatedPosition() const { return state_.emulated_position; }
  XRSpace* targetRaySpace() const;
  XRSpace* gripSpace() const;
  Gamepad* gamepad() const { return gamepad_.Get(); }
  XRHand* hand() const { return hand_.Get(); }
  const FrozenArray<IDLString>& profiles() const { return *profiles_.Get(); }

  uint32_t source_id() const { return state_.source_id; }

  void SetInputFromPointer(const gfx::Transform*);
  void SetGamepadConnected(bool state);

  // Gamepad::Client
  GamepadHapticActuator* GetVibrationActuatorForGamepad(
      const Gamepad&) override {
    // TODO(https://crbug.com/955097): XRInputSource implementation of
    // Gamepad::Client must manage vibration actuator state in a similar way to
    // NavigatorGamepad.
    return nullptr;
  }

  device::mojom::XRTargetRayMode TargetRayMode() const {
    return state_.target_ray_mode;
  }

  std::optional<gfx::Transform> MojoFromInput() const;

  std::optional<gfx::Transform> InputFromPointer() const;

  void OnSelectStart();
  void OnSelectEnd();
  void OnSelect();

  void OnSqueezeStart();
  void OnSqueezeEnd();
  void OnSqueeze();

  void UpdateButtonStates(
      const device::mojom::blink::XRInputSourceStatePtr& state);
  void OnRemoved();

  // Check which element within the DOM overlay is hit by the input source's
  // pointer ray, and update primary input state based on that, including
  // suppressing event data for cross-origin iframes. For background, see
  // https://immersive-web.github.io/dom-overlays/#cross-origin-content-events
  void ProcessOverlayHitTest(
      Element* overlay_element,
      const device::mojom::blink::XRInputSourceStatePtr& state);
  bool IsVisible() const { return state_.is_visible; }

  void Trace(Visitor*) const override;

 private:
  // In order to ease copying, any new member variables that can be trivially
  // copied (except for Member<T> variables), should go here
  struct InternalState {
    int16_t active_frame_id = -1;
    bool primary_input_pressed = false;
    bool selection_cancelled = false;
    bool primary_squeeze_pressed = false;
    bool squeezing_cancelled = false;
    // Input sources have two separate states, visible/invisible and select
    // events active/suppressed. All input sources, including auxiliary, should
    // use DOM overlay hit test (the ProcessOverlayHitTest() method) to check if
    // they intersect cross-origin content. If that's the case, the input source
    // is set as invisible, and must not return poses or hit test results. This
    // also automatically suppresses select events (this matches the "poses are
    // limited" conditional in the main WebXR spec). If the hit test doesn't
    // intersect cross-origin content, and if this is the first touch, it fires
    // a beforexrselect event and suppresses select events if that's been
    // preventDefault()ed. For auxiliary input sources, the event does not need
    // to be fired - per spec, their select events need to be suppressed anyway.
    bool xr_select_events_suppressed = false;
    bool is_visible = true;
    const uint32_t source_id;
    device::mojom::XRHandedness handedness = device::mojom::XRHandedness::NONE;
    device::mojom::XRTargetRayMode target_ray_mode;
    bool emulated_position = false;
    base::TimeTicks base_timestamp;

    InternalState(uint32_t source_id,
                  device::mojom::XRTargetRayMode,
                  base::TimeTicks base_timestamp);
    InternalState(const InternalState& other);
    ~InternalState();
  };

  // Use to check if the updates that would/should be made by a given
  // XRInputSourceState would invalidate any SameObject properties guaranteed
  // by the idl, and thus require the xr_input_source to be recreated.
  bool InvalidatesSameObject(
      const device::mojom::blink::XRInputSourceStatePtr& state);

  // Note that UpdateGamepad should only be called after a check/recreation
  // from InvalidatesSameObject
  void UpdateGamepad(const std::optional<device::Gamepad>& gamepad);

  void UpdateHand(
      const device::mojom::blink::XRHandTrackingData* hand_joint_data);

  XRInputSourceEvent* CreateInputSourceEvent(const AtomicString& type);

  // These member variables all require special behavior when being copied or
  // are Member<T> type variables.  When adding another one, be sure to keep the
  // deep-copy constructor updated, when adding any new variable.
  InternalState state_;
  const Member<XRSession> session_;
  Member<XRTargetRaySpace> target_ray_space_;
  Member<XRGripSpace> grip_space_;
  Member<Gamepad> gamepad_;
  Member<XRHand> hand_;
  Member<FrozenArray<IDLString>> profiles_;

  // Input device pose in mojo space. This is the grip pose for
  // tracked controllers, and the viewer pose for screen input.
  std::unique_ptr<gfx::Transform> mojo_from_input_;

  // Pointer pose in input device space, this is the transform to apply to
  // mojo_from_input_ to get the pointer matrix. In most cases it should be
  // static.
  std::unique_ptr<gfx::Transform> input_from_pointer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCE_H_
