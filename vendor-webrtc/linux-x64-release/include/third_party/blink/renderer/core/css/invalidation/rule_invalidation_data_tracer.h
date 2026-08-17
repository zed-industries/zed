// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_TRACER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_TRACER_H_

#include "third_party/blink/renderer/core/css/invalidation/rule_invalidation_data_visitor.h"

namespace blink {

class RuleInvalidationDataTracer
    : public RuleInvalidationDataVisitor<
          RuleInvalidationDataVisitorType::kTracer> {
 public:
  explicit RuleInvalidationDataTracer(const RuleInvalidationData&);

  void TraceInvalidationSetsForSelector(const CSSSelector& selector);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_TRACER_H_
