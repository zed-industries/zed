/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_CLIENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_CLIENT_IMPL_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/page.h"
#include "third_party/blink/renderer/core/page/popup_opening_observer.h"
#include "third_party/blink/renderer/core/page/validation_message_client.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class LocalFrameView;
class FrameOverlay;
class ValidationMessageOverlayDelegate;

class CORE_EXPORT ValidationMessageClientImpl final
    : public GarbageCollected<ValidationMessageClientImpl>,
      public ValidationMessageClient,
      private PopupOpeningObserver {
 public:
  explicit ValidationMessageClientImpl(Page&);
  ~ValidationMessageClientImpl() override;

  void ShowValidationMessage(Element& anchor,
                             const String& message,
                             TextDirection message_dir,
                             const String& sub_message,
                             TextDirection sub_message_dir) override;

  void Trace(Visitor*) const override;

  ValidationMessageOverlayDelegate* GetDelegateForTesting() const {
    return overlay_delegate_;
  }

 private:
  void CheckAnchorStatus(TimerBase*);
  LocalFrameView* CurrentView();
  void HideValidationMessageImmediately(const Element& anchor);
  void Reset(TimerBase*);
  void ValidationMessageVisibilityChanged(Element& anchor);

  void HideValidationMessage(const Element& anchor) override;
  bool IsValidationMessageVisible(const Element& anchor) override;
  void DocumentDetached(const Document&) override;
  void DidChangeFocusTo(const Element* new_element) override;
  void WillBeDestroyed() override;
  void ServiceScriptedAnimations(base::TimeTicks) override;
  void LayoutOverlay() override;
  void UpdatePrePaint() override;
  void PaintOverlay(GraphicsContext&) override;

  // PopupOpeningObserver function
  void WillOpenPopup() override;

  Member<Page> page_;
  Member<Element> current_anchor_;
  String message_;
  Member<DisallowNewWrapper<HeapTaskRunnerTimer<ValidationMessageClientImpl>>>
      timer_;
  Member<FrameOverlay> overlay_;
  // Raw pointer. This pointer is valid unless overlay_ is nullptr.
  ValidationMessageOverlayDelegate* overlay_delegate_ = nullptr;
  bool allow_initial_empty_anchor_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_VALIDATION_MESSAGE_CLIENT_IMPL_H_
