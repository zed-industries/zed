// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_DETAILS_SCREEN_DETAILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_DETAILS_SCREEN_DETAILS_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/display/screen_infos.h"

namespace blink {

class LocalDOMWindow;
class ScreenDetailed;

// Interface exposing multi-screen information.
// https://w3c.github.io/window-management/
class MODULES_EXPORT ScreenDetails final
    : public EventTarget,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ScreenDetails(LocalDOMWindow* window);

  // Web-exposed interface:
  const HeapVector<Member<ScreenDetailed>>& screens() const;
  ScreenDetailed* currentScreen() const;
  DEFINE_ATTRIBUTE_EVENT_LISTENER(screenschange, kScreenschange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(currentscreenchange, kCurrentscreenchange)

  // EventTarget:
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContextLifecycleObserver:
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

  // Called on visual property updates with potentially new screen information.
  // Update web-exposed data structures and enqueues events for dispatch.
  void UpdateScreenInfos(LocalDOMWindow* window,
                         const display::ScreenInfos& new_infos);

 private:
  // Update web-exposed data structures on screen information changes.
  // Enqueues events for dispatch if `dispatch_events` is true.
  void UpdateScreenInfosImpl(LocalDOMWindow* window,
                             const display::ScreenInfos& new_infos,
                             bool dispatch_events);

  // The ScreenInfos sent by the previous UpdateScreenInfos call.
  display::ScreenInfos prev_screen_infos_;
  int64_t current_display_id_ = display::ScreenInfo::kInvalidDisplayId;
  HeapVector<Member<ScreenDetailed>> screens_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_DETAILS_SCREEN_DETAILS_H_
