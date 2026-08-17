// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_VIRTUALKEYBOARD_VIRTUAL_KEYBOARD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_VIRTUALKEYBOARD_VIRTUAL_KEYBOARD_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/frame/virtual_keyboard_overlay_changed_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace gfx {
class Rect;
}

namespace blink {

class DOMRect;
class ExecutionContext;
class Navigator;

// The VirtualKeyboard API provides control of the on-screen keyboard
// to JS authors. The VirtualKeyboard object lives in the Navigator.
// It is exposed to JS via a new attribute virtualKeyboard in the Navigator.
class VirtualKeyboard final : public EventTarget,
                              public Supplement<Navigator>,
                              public VirtualKeyboardOverlayChangedObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static VirtualKeyboard* virtualKeyboard(Navigator&);

  explicit VirtualKeyboard(Navigator& navigator);
  VirtualKeyboard(const VirtualKeyboard&) = delete;
  ~VirtualKeyboard() override;

  VirtualKeyboard& operator=(const VirtualKeyboard&) = delete;

  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(geometrychange, kGeometrychange)

  bool overlaysContent() const;
  void setOverlaysContent(bool overlays_content);
  DOMRect* boundingRect() const;

  void VirtualKeyboardOverlayChanged(const gfx::Rect&) final;

  // Public APIs for controlling the visibility of VirtualKeyboard.
  void show();
  void hide();

  void Trace(Visitor*) const override;

 private:
  Member<DOMRect> bounding_rect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_VIRTUALKEYBOARD_VIRTUAL_KEYBOARD_H_
