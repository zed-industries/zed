// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CONTAINER_QUERY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CONTAINER_QUERY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/container_selector.h"
#include "third_party/blink/renderer/core/css/media_query_exp.h"
#include "third_party/blink/renderer/core/layout/geometry/axis.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"

namespace blink {

class CORE_EXPORT ContainerQuery final
    : public GarbageCollected<ContainerQuery> {
 public:
  ContainerQuery(ContainerSelector, const MediaQueryExpNode* query);
  ContainerQuery(const ContainerQuery&);

  const ContainerSelector& Selector() const { return selector_; }
  const ContainerQuery* Parent() const { return parent_.Get(); }

  ContainerQuery* CopyWithParent(const ContainerQuery*) const;

  String ToString() const;

  void Trace(Visitor* visitor) const {
    visitor->Trace(query_);
    visitor->Trace(parent_);
  }

 private:
  friend class ContainerQueryTest;
  friend class ContainerQueryEvaluator;
  friend class CSSContainerRule;
  friend class StyleRuleContainer;

  const MediaQueryExpNode& Query() const { return *query_; }

  ContainerSelector selector_;
  Member<const MediaQueryExpNode> query_;
  Member<const ContainerQuery> parent_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CONTAINER_QUERY_H_
