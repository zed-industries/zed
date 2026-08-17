// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_FULLY_CLIPPED_STATE_STACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_FULLY_CLIPPED_STATE_STACK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/editing_strategy.h"
#include "third_party/blink/renderer/core/editing/iterators/bit_stack.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

template <typename Strategy>
class FullyClippedStateStackAlgorithm final : public BitStack {
  STACK_ALLOCATED();

 public:
  FullyClippedStateStackAlgorithm();
  ~FullyClippedStateStackAlgorithm();

  void PushFullyClippedState(const Node*);
  void SetUpFullyClippedStack(const Node*);
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    FullyClippedStateStackAlgorithm<EditingStrategy>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    FullyClippedStateStackAlgorithm<EditingInFlatTreeStrategy>;

using FullyClippedStateStack = FullyClippedStateStackAlgorithm<EditingStrategy>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_FULLY_CLIPPED_STATE_STACK_H_
