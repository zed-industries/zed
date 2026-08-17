// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_NODE_INVALIDATION_SETS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_NODE_INVALIDATION_SETS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_set.h"

namespace blink {

class CORE_EXPORT NodeInvalidationSets final {
 public:
  NodeInvalidationSets() = default;
  NodeInvalidationSets(NodeInvalidationSets&&) = default;
  NodeInvalidationSets& operator=(NodeInvalidationSets&&) = default;
  NodeInvalidationSets(const NodeInvalidationSets&) = delete;
  NodeInvalidationSets& operator=(const NodeInvalidationSets&) = delete;

  InvalidationSetVector& Descendants() { return descendants_; }
  const InvalidationSetVector& Descendants() const { return descendants_; }
  InvalidationSetVector& Siblings() { return siblings_; }
  const InvalidationSetVector& Siblings() const { return siblings_; }

 private:
  InvalidationSetVector descendants_;
  InvalidationSetVector siblings_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_NODE_INVALIDATION_SETS_H_
