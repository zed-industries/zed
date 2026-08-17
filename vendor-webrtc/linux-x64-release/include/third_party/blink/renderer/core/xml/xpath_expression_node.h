/*
 * Copyright (C) 2005 Frerich Raabe <raabe@kde.org>
 * Copyright (C) 2006, 2009 Apple Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_EXPRESSION_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_EXPRESSION_NODE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/xml/xpath_value.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class UseCounter;

namespace xpath {

struct CORE_EXPORT EvaluationContext {
  STACK_ALLOCATED();

 public:
  // |had_type_conversion_error| must be a reference to a variable of
  // which lifetime is same as this object, or longer than this object.
  EvaluationContext(Node&, bool& had_type_conversion_error);

  Node* node;
  wtf_size_t size;
  wtf_size_t position;
  HashMap<String, String> variable_bindings;
  UseCounter* use_counter = nullptr;

  bool& had_type_conversion_error;
};

class CORE_EXPORT ParseNode : public GarbageCollected<ParseNode> {
 public:
  virtual ~ParseNode() = default;
  virtual void Trace(Visitor* visitor) const {}
};

class CORE_EXPORT Expression : public ParseNode {
 public:
  Expression();
  Expression(const Expression&) = delete;
  Expression& operator=(const Expression&) = delete;
  ~Expression() override;
  void Trace(Visitor*) const override;

  virtual Value Evaluate(EvaluationContext&) const = 0;

  void AddSubExpression(Expression* expr) {
    is_context_node_sensitive_ |= expr->is_context_node_sensitive_;
    is_context_position_sensitive_ |= expr->is_context_position_sensitive_;
    is_context_size_sensitive_ |= expr->is_context_size_sensitive_;
    sub_expressions_.push_back(expr);
  }

  bool IsContextNodeSensitive() const { return is_context_node_sensitive_; }
  bool IsContextPositionSensitive() const {
    return is_context_position_sensitive_;
  }
  bool IsContextSizeSensitive() const { return is_context_size_sensitive_; }
  void SetIsContextNodeSensitive(bool value) {
    is_context_node_sensitive_ = value;
  }
  void SetIsContextPositionSensitive(bool value) {
    is_context_position_sensitive_ = value;
  }
  void SetIsContextSizeSensitive(bool value) {
    is_context_size_sensitive_ = value;
  }

  virtual Value::Type ResultType() const = 0;

 protected:
  unsigned SubExprCount() const { return sub_expressions_.size(); }
  Expression* SubExpr(unsigned i) { return sub_expressions_[i].Get(); }
  const Expression* SubExpr(unsigned i) const {
    return sub_expressions_[i].Get();
  }

 private:
  HeapVector<Member<Expression>> sub_expressions_;

  // Evaluation details that can be used for optimization.
  bool is_context_node_sensitive_;
  bool is_context_position_sensitive_;
  bool is_context_size_sensitive_;
};

}  // namespace xpath

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_EXPRESSION_NODE_H_
