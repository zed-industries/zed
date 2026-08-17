// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_TOUCH_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_TOUCH_H_

#include "device/gamepad/public/cpp/gamepad.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class GamepadTouch : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GamepadTouch() = default;
  GamepadTouch(const GamepadTouch&) = delete;
  GamepadTouch& operator=(const GamepadTouch&) = delete;

  uint32_t touchId() const { return touch_id_; }
  void SetTouchId(uint32_t id) { touch_id_ = id; }

  uint8_t surfaceId() const { return surface_id_; }
  void SetSurfaceId(uint8_t id) { surface_id_ = id; }

  NotShared<DOMFloat32Array> position() const { return position_; }
  void SetPosition(float x, float y);

  NotShared<DOMUint32Array> surfaceDimensions() const {
    return surface_dimensions_;
  }
  bool HasSurfaceDimensions() { return has_surface_dimensions_; }
  void SetSurfaceDimensions(uint32_t x, uint32_t y);

  bool IsEqual(const device::GamepadTouch&) const;
  void UpdateValuesFrom(const device::GamepadTouch&, uint32_t);

  void Trace(blink::Visitor*) const override;

 private:
  uint32_t touch_id_ = 0;
  uint8_t surface_id_ = 0;
  bool has_surface_dimensions_ = false;

  NotShared<DOMFloat32Array> position_;
  NotShared<DOMUint32Array> surface_dimensions_;
};

using GamepadTouchVector = HeapVector<Member<GamepadTouch>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_TOUCH_H_
