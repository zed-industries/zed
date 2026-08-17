/*
 * Copyright (C) 2013, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_DOCUMENT_XPATH_EVALUATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_DOCUMENT_XPATH_EVALUATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/xml/xpath_evaluator.h"

namespace blink {

class ExceptionState;
class V8XPathNSResolver;
class XPathExpression;
class XPathResult;

class CORE_EXPORT DocumentXPathEvaluator final
    : public GarbageCollected<DocumentXPathEvaluator>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];

  static DocumentXPathEvaluator& From(Document&);

  static XPathExpression* createExpression(Document&,
                                           const String& expression,
                                           V8XPathNSResolver*,
                                           ExceptionState&);
  static Node* createNSResolver(Document&, Node* node_resolver);
  static XPathResult* evaluate(Document&,
                               const String& expression,
                               Node* context_node,
                               V8XPathNSResolver*,
                               uint16_t type,
                               const ScriptValue&,
                               ExceptionState&);

  explicit DocumentXPathEvaluator(Document&);
  void Trace(Visitor*) const override;

 private:
  Member<XPathEvaluator> xpath_evaluator_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_DOCUMENT_XPATH_EVALUATOR_H_
