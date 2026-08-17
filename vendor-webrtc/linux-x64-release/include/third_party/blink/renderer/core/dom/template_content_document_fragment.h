/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEMPLATE_CONTENT_DOCUMENT_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEMPLATE_CONTENT_DOCUMENT_FRAGMENT_H_

#include "third_party/blink/renderer/core/dom/document_fragment.h"

namespace blink {

class TemplateContentDocumentFragment final : public DocumentFragment {
 public:
  TemplateContentDocumentFragment(Document& document, Element* host)
      : DocumentFragment(&document, kCreateDocumentFragment), host_(host) {}

  Element* Host() const { return host_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(host_);
    DocumentFragment::Trace(visitor);
  }

 private:
  bool IsTemplateContent() const override { return true; }

  Member<Element> host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEMPLATE_CONTENT_DOCUMENT_FRAGMENT_H_
