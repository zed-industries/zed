// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_EVAL_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_EVAL_UTILS_H_

#include "third_party/blink/renderer/core/css/kleene_value.h"
#include "third_party/blink/renderer/core/css/media_query_exp.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

// https://drafts.csswg.org/mediaqueries-4/#evaluating
template <typename FeatureHandler>
KleeneValue MediaEval(const MediaQueryExpNode& node,
                      FeatureHandler feature_handler) {
  if (auto* n = DynamicTo<MediaQueryNestedExpNode>(node)) {
    return MediaEval(n->Operand(), feature_handler);
  }
  if (auto* n = DynamicTo<MediaQueryFunctionExpNode>(node)) {
    return MediaEval(n->Operand(), feature_handler);
  }
  if (auto* n = DynamicTo<MediaQueryNotExpNode>(node)) {
    return MediaEvalNot(n->Operand(), feature_handler);
  }
  if (auto* n = DynamicTo<MediaQueryAndExpNode>(node)) {
    return MediaEvalAnd(n->Left(), n->Right(), feature_handler);
  }
  if (auto* n = DynamicTo<MediaQueryOrExpNode>(node)) {
    return MediaEvalOr(n->Left(), n->Right(), feature_handler);
  }
  if (IsA<MediaQueryUnknownExpNode>(node)) {
    return KleeneValue::kUnknown;
  }
  return feature_handler(To<MediaQueryFeatureExpNode>(node));
}

template <typename FeatureHandler>
KleeneValue MediaEvalNot(const MediaQueryExpNode& operand_node,
                         FeatureHandler feature_handler) {
  return KleeneNot(MediaEval(operand_node, feature_handler));
}

template <typename FeatureHandler>
KleeneValue MediaEvalAnd(const MediaQueryExpNode& left_node,
                         const MediaQueryExpNode& right_node,
                         FeatureHandler feature_handler) {
  KleeneValue left = MediaEval(left_node, feature_handler);
  if (left == KleeneValue::kFalse) {
    return left;
  }
  return KleeneAnd(left, MediaEval(right_node, feature_handler));
}

template <typename FeatureHandler>
KleeneValue MediaEvalOr(const MediaQueryExpNode& left_node,
                        const MediaQueryExpNode& right_node,
                        FeatureHandler feature_handler) {
  KleeneValue left = MediaEval(left_node, feature_handler);
  if (left == KleeneValue::kTrue) {
    return left;
  }
  return KleeneOr(left, MediaEval(right_node, feature_handler));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_EVAL_UTILS_H_
