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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EMAIL_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EMAIL_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/base_text_input_type.h"

namespace blink {

class EmailInputType final : public BaseTextInputType {
 public:
  explicit EmailInputType(HTMLInputElement&);

  // They are public for unit testing.
  CORE_EXPORT static String ConvertEmailAddressToASCII(const ScriptRegexp&,
                                                       const String&);
  CORE_EXPORT static bool IsValidEmailAddress(const ScriptRegexp&,
                                              const String&);
  CORE_EXPORT static ScriptRegexp* CreateEmailRegexp(v8::Isolate* isolate);

  static Vector<String> ParseMultipleValues(const String& value);

  bool TypeMismatchFor(const String&) const;

 private:
  void CountUsage() override;
  bool TypeMismatch() const override;
  String TypeMismatchText() const override;
  bool SupportsSelectionAPI() const override;
  String SanitizeValue(const String&) const override;
  String ConvertFromVisibleValue(const String&) const override;
  String VisibleValue() const override;
  void MultipleAttributeChanged() override;

  String ConvertEmailAddressToUnicode(const String&) const;
  String FindInvalidAddress(const String&) const;
};

template <>
struct DowncastTraits<EmailInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsEmailInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EMAIL_INPUT_TYPE_H_
