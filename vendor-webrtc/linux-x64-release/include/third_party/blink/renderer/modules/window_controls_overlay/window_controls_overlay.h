// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WINDOW_CONTROLS_OVERLAY_WINDOW_CONTROLS_OVERLAY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WINDOW_CONTROLS_OVERLAY_WINDOW_CONTROLS_OVERLAY_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/frame/window_controls_overlay_changed_delegate.h"
#include "third_party/blink/renderer/core/geometry/dom_rect.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Navigator;

class WindowControlsOverlay final
    : public EventTarget,
      public Supplement<Navigator>,
      public WindowControlsOverlayChangedDelegate {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  // Web Exposed as navigator.windowControlsOverlay
  static WindowControlsOverlay* windowControlsOverlay(Navigator& navigator);

  static WindowControlsOverlay& From(Navigator& navigator);
  static WindowControlsOverlay* FromIfExists(Navigator& navigator);

  explicit WindowControlsOverlay(Navigator& navigator);
  WindowControlsOverlay(const WindowControlsOverlay&) = delete;
  ~WindowControlsOverlay() override;

  WindowControlsOverlay& operator=(const WindowControlsOverlay&) = delete;

  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(geometrychange, kGeometrychange)

  bool visible() const;
  DOMRect* getTitlebarAreaRect() const;

  void WindowControlsOverlayChanged(const gfx::Rect&) final;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WINDOW_CONTROLS_OVERLAY_WINDOW_CONTROLS_OVERLAY_H_
