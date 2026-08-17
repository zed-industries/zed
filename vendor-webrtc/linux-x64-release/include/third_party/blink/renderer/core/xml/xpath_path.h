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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PATH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PATH_H_

#include "third_party/blink/renderer/core/xml/xpath_expression_node.h"
#include "third_party/blink/renderer/core/xml/xpath_node_set.h"

namespace blink {

namespace xpath {

class Predicate;
class Step;

class Filter final : public Expression {
 public:
  Filter(Expression*, GCedHeapVector<Member<Predicate>>&);
  ~Filter() override;
  void Trace(Visitor*) const override;

  Value Evaluate(EvaluationContext&) const override;

 private:
  Value::Type ResultType() const override { return Value::kNodeSetValue; }

  Member<Expression> expr_;
  HeapVector<Member<Predicate>> predicates_;
};

class LocationPath final : public Expression {
 public:
  LocationPath();
  ~LocationPath() override;
  void Trace(Visitor*) const override;

  Value Evaluate(EvaluationContext&) const override;
  void SetAbsolute(bool value) {
    absolute_ = value;
    SetIsContextNodeSensitive(!absolute_);
  }
  void Evaluate(EvaluationContext&,
                NodeSet&) const;  // nodes is an input/output parameter
  void AppendStep(Step*);
  void InsertFirstStep(Step*);

 private:
  Value::Type ResultType() const override { return Value::kNodeSetValue; }

  HeapVector<Member<Step>> steps_;
  bool absolute_;
};

class Path final : public Expression {
 public:
  Path(Expression*, LocationPath*);
  ~Path() override;
  void Trace(Visitor*) const override;

  Value Evaluate(EvaluationContext&) const override;

 private:
  Value::Type ResultType() const override { return Value::kNodeSetValue; }

  Member<Expression> filter_;
  Member<LocationPath> path_;
};

}  // namespace xpath

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PATH_H_
