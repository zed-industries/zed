/*
 * Copyright 2005 Frerich Raabe <raabe@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_VARIABLE_REFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_VARIABLE_REFERENCE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/xml/xpath_expression_node.h"

namespace blink {

namespace xpath {

// Variable references are not used with XPathEvaluator.
class VariableReference final : public Expression {
 public:
  explicit VariableReference(const String& name);

 private:
  Value Evaluate(EvaluationContext&) const override;
  Value::Type ResultType() const override { NOTREACHED(); }

  String name_;
};

}  // namespace xpath

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_VARIABLE_REFERENCE_H_
