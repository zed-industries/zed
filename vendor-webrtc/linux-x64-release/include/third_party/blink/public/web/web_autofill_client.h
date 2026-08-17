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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AUTOFILL_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AUTOFILL_CLIENT_H_

#include "third_party/blink/public/web/web_element.h"
#include "third_party/blink/public/web/web_form_control_element.h"
#include "third_party/blink/public/web/web_form_related_change_type.h"
#include "third_party/blink/public/web/web_local_frame.h"

namespace blink {

class WebFormElement;
class WebInputElement;
class WebKeyboardEvent;
class WebNode;
class WebString;
class WebElement;

class WebAutofillClient {
 public:
  // These methods are called when the users edits a text-field.
  virtual void TextFieldDidEndEditing(const WebInputElement&) {}
  virtual void TextFieldValueChanged(const WebFormControlElement&) {}
  virtual void TextFieldDidReceiveKeyDown(const WebInputElement&,
                                          const WebKeyboardEvent&) {}
  // Called when a text field is cleared either by simply deleting the text or
  // briefly cleared when the whole text is selected and replaced. The latter
  // would not be conveyed by `TextFieldValueChanged()` and some clients might
  // need that information.
  virtual void TextFieldCleared(const WebFormControlElement&) {}
  // This is called once per-character when a user edits a contenteditable
  // element by typing.
  virtual void ContentEditableDidChange(const WebElement&) {}
  // This is called when a datalist indicator is clicked.
  virtual void OpenTextDataListChooser(const WebInputElement&) {}
  // This is called when the datalist for an input has changed.
  virtual void DataListOptionsChanged(const WebInputElement&) {}

  // Called when the selected option of a <select> control is changed as a
  // result of user activation - see
  // https://html.spec.whatwg.org/multipage/interaction.html#tracking-user-activation
  virtual void SelectControlSelectionChanged(const WebFormControlElement&) {}

  // Called when the options of a select control change.
  virtual void SelectFieldOptionsChanged(const WebFormControlElement&) {}

  // Called when the user interacts with the page after a load.
  virtual void UserGestureObserved() {}

  virtual void DidChangeFormRelatedElementDynamically(
      const WebElement&,
      WebFormRelatedChangeType) {}
  virtual void AjaxSucceeded() {}
  // Called when the value of `element` has been changed by JavaScript.
  // `old_value` contains the value before being changed.
  // `was_autofilled` is the state of the field prior to the JS change.
  // Only called if there is an observable change in the actual value, i.e.
  // JavaScript setting it to the current value will not trigger this.
  virtual void JavaScriptChangedValue(WebFormControlElement element,
                                      const WebString& old_value,
                                      bool was_autofilled) {}

  // Called when the focused node has changed. This is not called if the focus
  // moves outside the frame.
  virtual void DidCompleteFocusChangeInFrame() {}

  virtual void DidReceiveLeftMouseDownOrGestureTapInNode(const WebNode&) {}

  // Called when the given form element is reset.
  virtual void FormElementReset(const WebFormElement&) {}

  // Determines the form-related issues in the WebAutofillClient's document and
  // adds them to the associated frame's DevTools issues.
  virtual void EmitFormIssuesToDevtools() {}

  // Called when the empty value is set for the given input element, which is
  // or has been a password field.
  virtual void PasswordFieldReset(const WebInputElement& element) {}

 protected:
  virtual ~WebAutofillClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AUTOFILL_CLIENT_H_
