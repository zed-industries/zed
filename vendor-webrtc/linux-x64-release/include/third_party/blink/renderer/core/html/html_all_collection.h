/*
 * Copyright (C) 2009, 2011 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ALL_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ALL_COLLECTION_H_

#include "third_party/blink/renderer/core/html/html_collection.h"

namespace blink {

class Element;
class V8UnionElementOrHTMLCollection;

class HTMLAllCollection final : public HTMLCollection {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLAllCollection(ContainerNode&);
  HTMLAllCollection(ContainerNode&, CollectionType);
  ~HTMLAllCollection() override;

  V8UnionElementOrHTMLCollection* item(v8::Isolate*, ExceptionState&) {
    return nullptr;
  }
  V8UnionElementOrHTMLCollection* item(v8::Isolate*,
                                       v8::Local<v8::Value>,
                                       ExceptionState&);
  Element* AnonymousIndexedGetter(unsigned index);
  V8UnionElementOrHTMLCollection* NamedGetter(const AtomicString& name);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_ALL_COLLECTION_H_
