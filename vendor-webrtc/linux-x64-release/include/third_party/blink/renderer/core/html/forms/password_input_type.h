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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_PASSWORD_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_PASSWORD_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/html/forms/base_text_input_type.h"

namespace blink {

class KeyboardEvent;

class PasswordInputType final : public BaseTextInputType {
 public:
  explicit PasswordInputType(HTMLInputElement& element)
      : BaseTextInputType(Type::kPassword, element) {}

 private:
  void CountUsage() override;
  bool ShouldSaveAndRestoreFormControlState() const override;
  FormControlState SaveFormControlState() const override;
  void RestoreFormControlState(const FormControlState&) override;
  bool ShouldRespectListAttribute() override;

  bool NeedsContainer() const override;
  void CreateShadowSubtree() override;

  void UpdateView() override;
  void CapsLockStateMayHaveChanged() override;
  bool ShouldDrawCapsLockIndicator() const override;
  void UpdatePasswordRevealButton();
  void DidSetValueByUserEdit() override;
  void DidSetValue(const String&, bool value_changed) override;

  void ForwardEvent(Event& event) override;
  void HandleKeydownEvent(KeyboardEvent&) override;
  void HandleBeforeTextInsertedEvent(BeforeTextInsertedEvent&) override;

  void HandleBlurEvent() override;
  bool SupportsInputModeAttribute() const override;

  bool should_draw_caps_lock_indicator_ = false;
  bool should_show_reveal_button_ = false;
};

template <>
struct DowncastTraits<PasswordInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsPasswordInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_PASSWORD_INPUT_TYPE_H_
