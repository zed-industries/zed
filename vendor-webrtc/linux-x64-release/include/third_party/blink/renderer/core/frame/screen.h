/*
 * Copyright (C) 2007 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCREEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCREEN_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "ui/gfx/geometry/rect.h"

namespace display {
struct ScreenInfo;
}

namespace blink {

class LocalDOMWindow;

class CORE_EXPORT Screen : public EventTarget,
                           public ExecutionContextClient,
                           public Supplementable<Screen> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Screen(LocalDOMWindow*, int64_t display_id);

  static bool AreWebExposedScreenPropertiesEqual(
      const display::ScreenInfo& prev,
      const display::ScreenInfo& current);

  int height() const;
  int width() const;
  unsigned colorDepth() const;
  unsigned pixelDepth() const;
  int availLeft() const;
  int availTop() const;
  int availHeight() const;
  int availWidth() const;

  void Trace(Visitor*) const override;

  // EventTarget:
  const WTF::AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // True if information about the device's screen size should be reduced in
  // this context.
  bool ShouldReduceScreenSize() const;

  // Whether the device’s visual output extends over multiple screens.
  // https://w3c.github.io/window-management/
  bool isExtended() const;
  // Fired when the window’s screen or that screen's attributes change.
  // https://w3c.github.io/window-management/
  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  // Not web-exposed; for internal usage only.
  static constexpr int64_t kInvalidDisplayId = -1;
  int64_t DisplayId() const { return display_id_; }
  void UpdateDisplayId(int64_t display_id) { display_id_ = display_id; }

 protected:
  // Helpers to access screen information.
  gfx::Rect GetRect(bool available) const;
  const display::ScreenInfo& GetScreenInfo() const;

  // The internal id of the underlying display, to support multi-screen devices.
  int64_t display_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCREEN_H_
