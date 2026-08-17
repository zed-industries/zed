/*
 * Copyright 2005 Frerich Raabe <raabe@kde.org>
 * Copyright (C) 2006 Apple Computer, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_EVALUATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_EVALUATOR_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExceptionState;
class ExecutionContext;
class Node;
class ScriptValue;
class V8XPathNSResolver;
class XPathExpression;
class XPathResult;

class XPathEvaluator final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XPathEvaluator* Create() {
    return MakeGarbageCollected<XPathEvaluator>();
  }

  XPathEvaluator() = default;

  XPathExpression* createExpression(ExecutionContext* execution_context,
                                    const WTF::String& expression,
                                    V8XPathNSResolver*,
                                    ExceptionState&);
  Node* createNSResolver(Node* node_resolver);
  XPathResult* evaluate(ExecutionContext* execustin_context,
                        const WTF::String& expression,
                        Node* context_node,
                        V8XPathNSResolver*,
                        uint16_t type,
                        const ScriptValue&,
                        ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_EVALUATOR_H_
