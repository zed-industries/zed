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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_TEXT_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_TEXT_INPUT_TYPE_H_

namespace blink {

enum WebTextInputType {
  // Input caret is not in an editable node, no input method shall be used.
  kWebTextInputTypeNone,

  // Input caret is in a normal editable node, any input method can be used.
  kWebTextInputTypeText,

  // Input caret is in a specific input field, and input method may be used
  // only if it's suitable for the specific input field.
  kWebTextInputTypePassword,
  kWebTextInputTypeSearch,
  kWebTextInputTypeEmail,
  kWebTextInputTypeNumber,
  kWebTextInputTypeTelephone,
  kWebTextInputTypeURL,

  // These types, though not used in IME, are used by the Android date picker.
  // TODO(dglazkov): They are technically not _text_ input types and likely
  // should be split out into a separate enum.
  kWebTextInputTypeDate,
  kWebTextInputTypeDateTime,
  kWebTextInputTypeDateTimeLocal,
  kWebTextInputTypeMonth,
  kWebTextInputTypeTime,
  kWebTextInputTypeWeek,
  kWebTextInputTypeTextArea,

  // Input caret is in a contenteditable node (not an INPUT field).
  kWebTextInputTypeContentEditable,

  // The focused node is date time field. The date time field does not have
  // input caret but it is necessary to distinguish from WebTextInputTypeNone
  // for on-screen keyboard.
  kWebTextInputTypeDateTimeField,
};

// Separate on/off flags are defined so that the input mechanism can choose
// an appropriate default based on other things (like InputType and direct
// knowledge of the actual input system) if there are no overrides.
//
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.blink_public.web
// GENERATED_JAVA_PREFIX_TO_STRIP: WebTextInputFlag
enum WebTextInputFlags {
  kWebTextInputFlagNone = 0,
  kWebTextInputFlagAutocompleteOn = 1 << 0,
  kWebTextInputFlagAutocompleteOff = 1 << 1,
  kWebTextInputFlagAutocorrectOn = 1 << 2,
  kWebTextInputFlagAutocorrectOff = 1 << 3,
  kWebTextInputFlagSpellcheckOn = 1 << 4,
  kWebTextInputFlagSpellcheckOff = 1 << 5,
  kWebTextInputFlagAutocapitalizeNone = 1 << 6,
  kWebTextInputFlagAutocapitalizeCharacters = 1 << 7,
  kWebTextInputFlagAutocapitalizeWords = 1 << 8,
  kWebTextInputFlagAutocapitalizeSentences = 1 << 9,
  kWebTextInputFlagHaveNextFocusableElement = 1 << 10,
  kWebTextInputFlagHavePreviousFocusableElement = 1 << 11,
  // Whether an input field is or has ever been a password. For such an input
  // type we don't want autocomplete or a keyboard to memorize the content.
  kWebTextInputFlagHasBeenPasswordField = 1 << 12,
  kWebTextInputFlagVertical = 1 << 13,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_TEXT_INPUT_TYPE_H_
