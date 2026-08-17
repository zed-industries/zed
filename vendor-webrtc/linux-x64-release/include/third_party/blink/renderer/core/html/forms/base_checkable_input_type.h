/*
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_BASE_CHECKABLE_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_BASE_CHECKABLE_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/forms/input_type.h"
#include "third_party/blink/renderer/core/html/forms/input_type_view.h"

namespace blink {

// Base of checkbox and radio types.
class BaseCheckableInputType : public InputType, public InputTypeView {
 public:
  void Trace(Visitor*) const override;
  using InputType::GetElement;

  void HandleBlurEvent() override;
  bool CanSetStringValue() const;

 protected:
  BaseCheckableInputType(Type type, HTMLInputElement& element)
      : InputType(type, element),
        InputTypeView(element),
        is_in_click_handler_(false) {}
  void HandleKeydownEvent(KeyboardEvent&) override;
  bool NeedsShadowSubtree() const override { return false; }

  bool is_in_click_handler_;

 private:
  InputTypeView* CreateView() override;
  FormControlState SaveFormControlState() const final;
  void RestoreFormControlState(const FormControlState&) final;
  void AppendToFormData(FormData&) const final;
  void HandleKeypressEvent(KeyboardEvent&) final;
  void AccessKeyAction(SimulatedClickCreationScope creation_scope) final;
  bool MatchesDefaultPseudoClass() override;
  ValueMode GetValueMode() const override;
  void SetValue(const String&,
                bool,
                TextFieldEventBehavior,
                TextControlSetValueSelection) final;
  void ReadingChecked() const final;
  bool IsCheckable() final;
};

template <>
struct DowncastTraits<BaseCheckableInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsBaseCheckableInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_BASE_CHECKABLE_INPUT_TYPE_H_
