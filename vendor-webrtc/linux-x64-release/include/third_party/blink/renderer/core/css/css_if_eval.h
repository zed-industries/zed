// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IF_EVAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IF_EVAL_H_

#include "third_party/blink/renderer/core/css/if_condition.h"
#include "third_party/blink/renderer/core/css/kleene_value.h"

namespace blink {

template <typename IfTestEvaluator>
KleeneValue IfEval(const IfCondition& node, IfTestEvaluator evaluator) {
  if (IsA<IfConditionElse>(node)) {
    return KleeneValue::kTrue;
  }
  if (auto* n = DynamicTo<IfConditionNot>(node)) {
    return IfEvalNot(n->Operand(), evaluator);
  }
  if (auto* n = DynamicTo<IfConditionAnd>(node)) {
    return IfEvalAnd(n->Left(), n->Right(), evaluator);
  }
  if (auto* n = DynamicTo<IfConditionOr>(node)) {
    return IfEvalOr(n->Left(), n->Right(), evaluator);
  }
  if (IsA<IfConditionUnknown>(node)) {
    return KleeneValue::kUnknown;
  }
  // style() and media()
  return evaluator(node);
}

template <typename IfTestEvaluator>
KleeneValue IfEvalNot(const IfCondition& operand_node,
                      IfTestEvaluator evaluator) {
  return KleeneNot(IfEval(operand_node, evaluator));
}

template <typename IfTestEvaluator>
KleeneValue IfEvalAnd(const IfCondition& left_node,
                      const IfCondition& right_node,
                      IfTestEvaluator evaluator) {
  KleeneValue left = IfEval(left_node, evaluator);
  if (left == KleeneValue::kFalse) {
    return left;
  }
  return KleeneAnd(left, IfEval(right_node, evaluator));
}

template <typename IfTestEvaluator>
KleeneValue IfEvalOr(const IfCondition& left_node,
                     const IfCondition& right_node,
                     IfTestEvaluator evaluator) {
  KleeneValue left = IfEval(left_node, evaluator);
  if (left == KleeneValue::kTrue) {
    return left;
  }
  return KleeneOr(left, IfEval(right_node, evaluator));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IF_EVAL_H_
