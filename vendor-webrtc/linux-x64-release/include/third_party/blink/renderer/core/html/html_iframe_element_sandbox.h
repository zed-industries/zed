// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_IFRAME_ELEMENT_SANDBOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_IFRAME_ELEMENT_SANDBOX_H_

#include "third_party/blink/renderer/core/dom/dom_token_list.h"

namespace blink {

class HTMLFrameOwnerElement;

class HTMLIFrameElementSandbox final : public DOMTokenList {
 public:
  explicit HTMLIFrameElementSandbox(HTMLFrameOwnerElement*);

 private:
  bool ValidateTokenValue(const AtomicString&, ExceptionState&) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_IFRAME_ELEMENT_SANDBOX_H_
