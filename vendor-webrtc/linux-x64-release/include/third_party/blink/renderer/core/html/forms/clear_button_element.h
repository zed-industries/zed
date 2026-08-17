/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CLEAR_BUTTON_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CLEAR_BUTTON_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ClearButtonElement final : public HTMLDivElement {
 public:
  class ClearButtonOwner : public GarbageCollectedMixin {
   public:
    virtual ~ClearButtonOwner() = default;
    virtual void FocusAndSelectClearButtonOwner() = 0;
    virtual bool ShouldClearButtonRespondToMouseEvents() = 0;
    virtual void ClearValue() = 0;
  };

  ClearButtonElement(Document&, ClearButtonOwner&);

  void RemoveClearButtonOwner() { clear_button_owner_ = nullptr; }

  void Trace(Visitor*) const override;

 private:
  void DetachLayoutTree(bool performing_reattach) override;
  FocusableState IsFocusableState(UpdateBehavior) const override {
    return FocusableState::kNotFocusable;
  }
  void DefaultEventHandler(Event&) override;
  bool IsClearButtonElement() const override;

  Member<ClearButtonOwner> clear_button_owner_;
};

template <>
struct DowncastTraits<ClearButtonElement> {
  static bool AllowFrom(const Element& element) {
    return element.IsClearButtonElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CLEAR_BUTTON_ELEMENT_H_
