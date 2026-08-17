/*
 * Copyright (c) 2012, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TEMPLATE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TEMPLATE_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/shadow_root.h"
#include "third_party/blink/renderer/core/dom/template_content_document_fragment.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class DocumentFragment;
class TemplateContentDocumentFragment;

class CORE_EXPORT HTMLTemplateElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLTemplateElement(Document&);
  ~HTMLTemplateElement() override;

  bool HasNonInBodyInsertionMode() const override { return true; }

  void Trace(Visitor*) const override;

  DocumentFragment* content() const;

  // This just retrieves existing content, and will not construct a content
  // DocumentFragment if one does not exist.
  DocumentFragment* getContent() const {
    CHECK(!declarative_shadow_root_ || !content_);
    return content_;
  }

  // This retrieves either a currently-being-parsed declarative shadow root,
  // or the content fragment for a "regular" template element. This should only
  // be used by HTMLConstructionSite.
  DocumentFragment* TemplateContentOrDeclarativeShadowRoot() const {
    return declarative_shadow_root_ ? declarative_shadow_root_.Get()
                                    : content();
  }

  void SetDeclarativeShadowRoot(ShadowRoot& shadow) {
    declarative_shadow_root_ = &shadow;
  }

 private:
  void CloneNonAttributePropertiesFrom(const Element&,
                                       NodeCloningData&) override;
  void DidMoveToNewDocument(Document& old_document) override;

  mutable Member<TemplateContentDocumentFragment> content_;

  Member<ShadowRoot> declarative_shadow_root_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TEMPLATE_ELEMENT_H_
