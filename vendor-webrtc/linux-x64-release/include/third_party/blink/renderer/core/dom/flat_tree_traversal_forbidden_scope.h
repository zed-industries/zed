// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_TRAVERSAL_FORBIDDEN_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_TRAVERSAL_FORBIDDEN_SCOPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/dom/document.h"

namespace blink {

class FlatTreeTraversalForbiddenScope {
  STACK_ALLOCATED();

 public:
  explicit FlatTreeTraversalForbiddenScope(Document& document)
      : count_(document.FlatTreeTraversalForbiddenRecursionDepth()) {
    ++count_;
  }
  FlatTreeTraversalForbiddenScope(const FlatTreeTraversalForbiddenScope&) =
      delete;
  FlatTreeTraversalForbiddenScope& operator=(
      const FlatTreeTraversalForbiddenScope&) = delete;

  ~FlatTreeTraversalForbiddenScope() {
    DCHECK_GT(count_, 0u);
    --count_;
  }

 private:
  unsigned& count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FLAT_TREE_TRAVERSAL_FORBIDDEN_SCOPE_H_
