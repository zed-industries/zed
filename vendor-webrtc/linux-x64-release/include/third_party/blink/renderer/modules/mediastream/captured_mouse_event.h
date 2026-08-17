// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_CAPTURED_MOUSE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_CAPTURED_MOUSE_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "ui/gfx/geometry/point.h"

namespace blink {

class CapturedMouseEventInit;
class ExceptionState;

class MODULES_EXPORT CapturedMouseEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CapturedMouseEvent* Create(const AtomicString& type,
                                    const CapturedMouseEventInit* initializer,
                                    ExceptionState& exception_state);

  CapturedMouseEvent(const AtomicString& type,
                     const CapturedMouseEventInit* initializer);
  ~CapturedMouseEvent() override = default;

  int surfaceX() const { return surface_coordinates_.x(); }
  int surfaceY() const { return surface_coordinates_.y(); }

  // Event
  const AtomicString& InterfaceName() const final;

 private:
  const gfx::Point surface_coordinates_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_CAPTURED_MOUSE_EVENT_H_
