/*
 * Copyright (C) 2006, 2008, 2010 Apple Inc. All rights reserved.
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SPIN_BUTTON_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SPIN_BUTTON_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/core/page/popup_opening_observer.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class CORE_EXPORT SpinButtonElement final : public HTMLDivElement,
                                            public PopupOpeningObserver {
 public:
  enum UpDownState {
    kDown,
    kUp,
  };
  enum EventDispatch {
    kEventDispatchAllowed,
    kEventDispatchDisallowed,
  };
  class SpinButtonOwner : public GarbageCollectedMixin {
   public:
    virtual ~SpinButtonOwner() = default;
    virtual void FocusAndSelectSpinButtonOwner() = 0;
    virtual bool ShouldSpinButtonRespondToMouseEvents() = 0;
    virtual bool ShouldSpinButtonRespondToWheelEvents() = 0;
    virtual void SpinButtonDidReleaseMouseCapture(EventDispatch) = 0;
    virtual void SpinButtonStepDown() = 0;
    virtual void SpinButtonStepUp() = 0;
  };

  // The owner of SpinButtonElement must call removeSpinButtonOwner
  // because SpinButtonElement can be outlive SpinButtonOwner
  // implementation, e.g. during event handling.
  SpinButtonElement(Document&, SpinButtonOwner&);

  UpDownState GetUpDownState() const { return up_down_state_; }
  void ReleaseCapture(EventDispatch = kEventDispatchAllowed);
  void RemoveSpinButtonOwner() { spin_button_owner_ = nullptr; }

  void Step(int amount);

  bool WillRespondToMouseMoveEvents() const override;
  bool WillRespondToMouseClickEvents() override;

  void ForwardEvent(Event&);

  void Trace(Visitor*) const override;

 private:
  void DetachLayoutTree(bool performing_reattach) override;
  bool IsSpinButtonElement() const override { return true; }
  bool IsDisabledFormControl() const override {
    return OwnerShadowHost() && OwnerShadowHost()->IsDisabledFormControl();
  }
  bool MatchesReadOnlyPseudoClass() const override;
  bool MatchesReadWritePseudoClass() const override;
  void DefaultEventHandler(Event&) override;
  void WillOpenPopup() override;
  void DoStepAction(int);
  void StartRepeatingTimer();
  void StopRepeatingTimer();
  void RepeatingTimerFired(TimerBase*);
  bool ShouldRespondToMouseEvents() const;
  FocusableState IsFocusableState(UpdateBehavior) const override {
    return FocusableState::kNotFocusable;
  }
  void CalculateUpDownStateByMouseLocation(Event&);

  Member<SpinButtonOwner> spin_button_owner_;
  bool capturing_;
  UpDownState up_down_state_;
  UpDownState press_starting_state_;
  bool should_recalc_up_down_state_;
  HeapTaskRunnerTimer<SpinButtonElement> repeating_timer_;
};

template <>
struct DowncastTraits<SpinButtonElement> {
  static bool AllowFrom(const Node& node) {
    auto* element = DynamicTo<Element>(node);
    return element && element->IsSpinButtonElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SPIN_BUTTON_ELEMENT_H_
