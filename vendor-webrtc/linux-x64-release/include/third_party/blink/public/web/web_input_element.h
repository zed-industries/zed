/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_INPUT_ELEMENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_INPUT_ELEMENT_H_

#include "build/build_config.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_form_control_element.h"

namespace blink {

class HTMLInputElement;
class WebOptionElement;

// Provides readonly access to some properties of a DOM input element node.
class BLINK_EXPORT WebInputElement final : public WebFormControlElement {
 public:
  WebInputElement() = default;
  WebInputElement(const WebInputElement& element) = default;

  WebInputElement& operator=(const WebInputElement& element) {
    WebFormControlElement::Assign(element);
    return *this;
  }
  void Assign(const WebInputElement& element) {
    WebFormControlElement::Assign(element);
  }

  // Returns true for all of textfield-looking types such as text, password,
  // search, email, url, and number.
  bool IsTextField() const;
  // Makes `FormControlTypeForAutofill()` return
  // `mojom::FormControlType::kInputPassword` as long as the element's type is a
  // text type.
  void MaybeSetHasBeenPasswordField();
  void SetActivatedSubmit(bool);
  int size() const;
  void SetChecked(bool,
                  bool send_events = false,
                  WebAutofillState = WebAutofillState::kNotFilled);
  bool IsValidValue(const WebString&) const;
  bool IsChecked() const;
  bool IsMultiple() const;

  // Associated <datalist> options which match to the current INPUT value.
  std::vector<WebOptionElement> FilteredDataListOptions() const;

  // Return the localized value for this input type.
  WebString LocalizeValue(const WebString&) const;

  // If true, forces the text of the element to be visible.
  void SetShouldRevealPassword(bool value);

  // Returns true if the text of the element should be visible.
  bool ShouldRevealPassword() const;

  // Returns whether this is the last element within its form.
  bool IsLastInputElementInForm();

  // Triggers a form submission.
  void DispatchSimulatedEnter();

#if INSIDE_BLINK
  explicit WebInputElement(HTMLInputElement*);
  WebInputElement& operator=(HTMLInputElement*);
  operator HTMLInputElement*() const;
#endif
};

DECLARE_WEB_NODE_TYPE_CASTS(WebInputElement);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_INPUT_ELEMENT_H_
