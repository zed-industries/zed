// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CREATE_ELEMENT_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CREATE_ELEMENT_FLAGS_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;

class CreateElementFlags {
  STACK_ALLOCATED();

 public:
  bool IsCreatedByParser() const { return created_by_parser_; }
  Document* ParserDocument() const { return parser_document_; }
  bool IsAsyncCustomElements() const { return async_custom_elements_; }
  bool IsCustomElements() const { return custom_elements_; }
  bool WasAlreadyStarted() const { return already_started_; }

  // https://html.spec.whatwg.org/C/#create-an-element-for-the-token
  static CreateElementFlags ByParser(Document* document) {
    return CreateElementFlags().SetCreatedByParser(true, document);
  }

  // https://dom.spec.whatwg.org/#concept-node-clone
  static CreateElementFlags ByCloneNode() {
    return CreateElementFlags().SetAsyncCustomElements();
  }

  // https://dom.spec.whatwg.org/#dom-document-createelement
  static CreateElementFlags ByCreateElement() { return CreateElementFlags(); }

  // https://wicg.github.io/webcomponents/proposals/Scoped-Custom-Element-Registries
  static CreateElementFlags ByShadowRootCreateElement() {
    return CreateElementFlags().SetAsyncCustomElements();
  }

  // https://html.spec.whatwg.org/C/#create-an-element-for-the-token
  static CreateElementFlags ByFragmentParser(Document* document) {
    return CreateElementFlags()
        .SetCreatedByParser(true, document)
        .SetAsyncCustomElements();
  }

  // Construct an instance indicating default behavior.
  CreateElementFlags()
      : created_by_parser_(false),
        async_custom_elements_(false),
        custom_elements_(true),
        already_started_(false) {}

  CreateElementFlags& SetCreatedByParser(bool flag, Document* document) {
    DCHECK(flag || !document);
    created_by_parser_ = flag;
    parser_document_ = document;
    return *this;
  }

  // For <script>.
  CreateElementFlags& SetAlreadyStarted(bool flag) {
    already_started_ = flag;
    return *this;
  }

 private:
  CreateElementFlags& SetAsyncCustomElements() {
    async_custom_elements_ = true;
    return *this;
  }

  bool created_by_parser_ : 1;
  bool async_custom_elements_ : 1;
  bool custom_elements_ : 1;

  bool already_started_ : 1;

  // This implements the HTML Standard concept of a "parser document" [1].
  // Contrary to the spec, this member can be null even when
  // |created_by_parser_| is true. This can happen in rare cases where the
  // parser creates an element after it detaches from its document. The element
  // will be constructed with |created_by_parser_| = true, but the parser's
  // document used for |parser_document_| is null. If the parser is ever changed
  // such that elements created after detachment are constructed with
  // |created_by_parser_| = false, we can get rid of that flag and simply query
  // |parser_document_| for this information. See crbug.com/1086507.
  // [1]: https://html.spec.whatwg.org/C/#parser-document
  Document* parser_document_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_CREATE_ELEMENT_FLAGS_H_
