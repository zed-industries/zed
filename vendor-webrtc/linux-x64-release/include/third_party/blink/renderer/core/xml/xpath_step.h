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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_STEP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_STEP_H_

#include "third_party/blink/renderer/core/xml/xpath_expression_node.h"
#include "third_party/blink/renderer/core/xml/xpath_node_set.h"

namespace blink {

class Node;

namespace xpath {

class Predicate;

class Step final : public ParseNode {
 public:
  enum Axis {
    kAncestorAxis,
    kAncestorOrSelfAxis,
    kAttributeAxis,
    kChildAxis,
    kDescendantAxis,
    kDescendantOrSelfAxis,
    kFollowingAxis,
    kFollowingSiblingAxis,
    kNamespaceAxis,
    kParentAxis,
    kPrecedingAxis,
    kPrecedingSiblingAxis,
    kSelfAxis
  };

  class NodeTest final : public GarbageCollected<NodeTest> {
   public:
    enum Kind {
      kTextNodeTest,
      kCommentNodeTest,
      kProcessingInstructionNodeTest,
      kAnyNodeTest,
      kNameTest
    };

    NodeTest(Kind kind) : kind_(kind) {}
    NodeTest(Kind kind, const String& data) : kind_(kind), data_(data) {}
    NodeTest(Kind kind,
             const AtomicString& data,
             const AtomicString& namespace_uri)
        : kind_(kind), data_(data), namespace_uri_(namespace_uri) {}

    NodeTest(const NodeTest& o)
        : kind_(o.kind_), data_(o.data_), namespace_uri_(o.namespace_uri_) {
      DCHECK(o.merged_predicates_.empty());
    }
    NodeTest& operator=(const NodeTest& o) {
      kind_ = o.kind_;
      data_ = o.data_;
      namespace_uri_ = o.namespace_uri_;
      DCHECK(o.merged_predicates_.empty());
      return *this;
    }
    void Trace(Visitor* visitor) const { visitor->Trace(merged_predicates_); }

    Kind GetKind() const { return kind_; }
    const AtomicString& Data() const { return data_; }
    const AtomicString& NamespaceURI() const { return namespace_uri_; }
    HeapVector<Member<Predicate>>& MergedPredicates() {
      return merged_predicates_;
    }
    const HeapVector<Member<Predicate>>& MergedPredicates() const {
      return merged_predicates_;
    }

   private:
    Kind kind_;
    AtomicString data_;
    AtomicString namespace_uri_;

    // When possible, we merge some or all predicates with node test for better
    // performance.
    HeapVector<Member<Predicate>> merged_predicates_;
  };

  Step(Axis, const NodeTest&);
  Step(Axis, const NodeTest&, GCedHeapVector<Member<Predicate>>&);
  Step(const Step&) = delete;
  Step& operator=(const Step&) = delete;
  ~Step() override;
  void Trace(Visitor*) const override;

  void Optimize();

  void Evaluate(EvaluationContext&, Node* context, NodeSet&) const;

  Axis GetAxis() const { return axis_; }
  const NodeTest& GetNodeTest() const { return *node_test_; }

 private:
  friend bool OptimizeStepPair(Step*, Step*);
  bool PredicatesAreContextListInsensitive() const;
  NodeTest& GetNodeTest() { return *node_test_; }

  void ParseNodeTest(const String&);
  void NodesInAxis(EvaluationContext&, Node* context, NodeSet&) const;
  String NamespaceFromNodetest(const String& node_test) const;

  Axis axis_;
  Member<NodeTest> node_test_;
  HeapVector<Member<Predicate>> predicates_;
};

bool OptimizeStepPair(Step*, Step*);

}  // namespace xpath

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_STEP_H_
