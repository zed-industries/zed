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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PREDICATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PREDICATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/xml/xpath_expression_node.h"
#include "third_party/blink/renderer/core/xml/xpath_value.h"

namespace blink {

namespace xpath {

class CORE_EXPORT Number final : public Expression {
 public:
  explicit Number(double);
  void Trace(Visitor*) const override;

 private:
  Value Evaluate(EvaluationContext&) const override;
  Value::Type ResultType() const override { return Value::kNumberValue; }

  Value value_;
};

class CORE_EXPORT StringExpression final : public Expression {
 public:
  explicit StringExpression(const String&);
  void Trace(Visitor*) const override;

 private:
  Value Evaluate(EvaluationContext&) const override;
  Value::Type ResultType() const override { return Value::kStringValue; }

  Value value_;
};

class Negative final : public Expression {
 private:
  Value Evaluate(EvaluationContext&) const override;
  Value::Type ResultType() const override { return Value::kNumberValue; }
};

class NumericOp final : public Expression {
 public:
  enum Opcode { kOP_Add, kOP_Sub, kOP_Mul, kOP_Div, kOP_Mod };
  NumericOp(Opcode, Expression* lhs, Expression* rhs);

 private:
  Value Evaluate(EvaluationContext&) const override;
  Value::Type ResultType() const override { return Value::kNumberValue; }

  Opcode opcode_;
};

class EqTestOp final : public Expression {
 public:
  enum Opcode {
    kOpcodeEqual,
    kOpcodeNotEqual,
    kOpcodeGreaterThan,
    kOpcodeLessThan,
    kOpcodeGreaterOrEqual,
    kOpcodeLessOrEqual
  };
  EqTestOp(Opcode, Expression* lhs, Expression* rhs);
  Value Evaluate(EvaluationContext&) const override;

 private:
  Value::Type ResultType() const override { return Value::kBooleanValue; }
  bool Compare(EvaluationContext&, const Value&, const Value&) const;

  Opcode opcode_;
};

class LogicalOp final : public Expression {
 public:
  enum Opcode { kOP_And, kOP_Or };
  LogicalOp(Opcode, Expression* lhs, Expression* rhs);

 private:
  Value::Type ResultType() const override { return Value::kBooleanValue; }
  bool ShortCircuitOn() const;
  Value Evaluate(EvaluationContext&) const override;

  Opcode opcode_;
};

class Union final : public Expression {
 private:
  Value Evaluate(EvaluationContext&) const override;
  Value::Type ResultType() const override { return Value::kNodeSetValue; }
};

class Predicate final : public GarbageCollected<Predicate> {
 public:
  explicit Predicate(Expression*);
  Predicate(const Predicate&) = delete;
  Predicate& operator=(const Predicate&) = delete;
  void Trace(Visitor*) const;

  bool Evaluate(EvaluationContext&) const;
  bool IsContextPositionSensitive() const {
    return expr_->IsContextPositionSensitive() ||
           expr_->ResultType() == Value::kNumberValue;
  }
  bool IsContextSizeSensitive() const {
    return expr_->IsContextSizeSensitive();
  }

 private:
  Member<Expression> expr_;
};

}  // namespace xpath

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PREDICATE_H_
