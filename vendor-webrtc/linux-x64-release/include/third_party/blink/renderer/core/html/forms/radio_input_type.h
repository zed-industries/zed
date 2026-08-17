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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RADIO_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RADIO_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/base_checkable_input_type.h"

namespace blink {

class RadioInputType final : public BaseCheckableInputType {
 public:
  // This function finds the next radio button to navigate to using the keyboard
  // arrow keys or for the accessibility screen reader.
  CORE_EXPORT static HTMLInputElement* NextRadioButtonInGroup(HTMLInputElement*,
                                                              bool forward);

  RadioInputType(HTMLInputElement& element)
      : BaseCheckableInputType(Type::kRadio, element) {}
  bool ValueMissing(const String&) const;

 private:
  void CountUsage() override;
  AppearanceValue AutoAppearance() const override;
  void WillUpdateCheckedness(bool new_checked) override;
  String ValueMissingText() const override;
  void HandleClickEvent(MouseEvent&) override;
  void HandleKeydownEvent(KeyboardEvent&) override;
  void HandleKeyupEvent(KeyboardEvent&) override;
  bool IsKeyboardFocusableSlow(
      Element::UpdateBehavior update_behavior =
          Element::UpdateBehavior::kStyleAndLayout) const override;
  bool ShouldSendChangeEventAfterCheckedChanged() override;
  ClickHandlingState* WillDispatchClick() override;
  void DidDispatchClick(Event&, const ClickHandlingState&) override;
  bool ShouldAppearIndeterminate() const override;

  HTMLInputElement* FindNextFocusableRadioButtonInGroup(HTMLInputElement*,
                                                        bool);
  HTMLInputElement* CheckedRadioButtonForGroup() const;
};

template <>
struct DowncastTraits<RadioInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsRadioInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RADIO_INPUT_TYPE_H_
