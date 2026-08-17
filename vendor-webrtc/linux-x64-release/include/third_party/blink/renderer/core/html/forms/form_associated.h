// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_FORM_ASSOCIATED_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_FORM_ASSOCIATED_H_

namespace blink {

class HTMLFormElement;

// Contains code to associate form with a form associated element
// https://html.spec.whatwg.org/C/#form-associated-element
class FormAssociated {
 public:
  // HTMLFormElement can be null
  virtual void AssociateWith(HTMLFormElement*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_FORM_ASSOCIATED_H_
