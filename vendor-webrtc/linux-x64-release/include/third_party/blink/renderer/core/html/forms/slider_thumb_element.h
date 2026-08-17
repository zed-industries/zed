/*
 * Copyright (C) 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SLIDER_THUMB_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SLIDER_THUMB_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class HTMLInputElement;
class Event;
class TouchEvent;

class SliderThumbElement final : public HTMLDivElement {
 public:
  SliderThumbElement(Document&);

  void SetPositionFromValue();

  void DragFrom(const PhysicalOffset&);
  void DefaultEventHandler(Event&) override;
  bool WillRespondToMouseMoveEvents() const override;
  bool WillRespondToMouseClickEvents() override;
  void DetachLayoutTree(bool performing_reattach) override;
  const AtomicString& ShadowPseudoId() const override;
  HTMLInputElement* HostInput() const;
  void SetPositionFromPoint(const PhysicalOffset&);
  void StopDragging();
  bool IsSliderThumbElement() const override { return true; }

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void AdjustStyle(ComputedStyleBuilder&) final;
  Element& CloneWithoutAttributesAndChildren(Document&) const override;
  bool IsDisabledFormControl() const override;
  bool MatchesReadOnlyPseudoClass() const override;
  bool MatchesReadWritePseudoClass() const override;
  void StartDragging();

  // Mouse only. Touch is handled by SliderContainerElement.
  bool in_drag_mode_;
};

inline Element& SliderThumbElement::CloneWithoutAttributesAndChildren(
    Document& factory) const {
  return *MakeGarbageCollected<SliderThumbElement>(factory);
}

template <>
struct DowncastTraits<SliderThumbElement> {
  static bool AllowFrom(const Element& element) {
    return element.IsSliderThumbElement();
  }
};

class SliderContainerElement final : public HTMLDivElement {
 public:
  enum class Direction {
    kHorizontal,
    kVertical,
    kNoMove,
  };

  explicit SliderContainerElement(Document&);

  HTMLInputElement* HostInput() const;
  void DefaultEventHandler(Event&) override;
  void HandleTouchEvent(TouchEvent*);
  void UpdateTouchEventHandlerRegistry();
  void DidMoveToNewDocument(Document&) override;
  void RemoveAllEventListeners() override;

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  const AtomicString& ShadowPseudoId() const override;
  static Direction GetDirection(const PhysicalOffset&, const PhysicalOffset&);
  bool CanSlide();

  bool has_touch_event_handler_ = false;
  bool touch_started_ = false;
  Direction sliding_direction_ = Direction::kNoMove;
  PhysicalOffset start_point_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_SLIDER_THUMB_ELEMENT_H_
