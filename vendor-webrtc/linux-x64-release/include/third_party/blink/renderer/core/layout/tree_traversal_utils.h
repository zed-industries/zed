// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TREE_TRAVERSAL_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TREE_TRAVERSAL_UTILS_H_

#include "base/functional/function_ref.h"

namespace blink {

class LayoutInline;
class PhysicalBoxFragment;

// How to proceed after having processed a fragment (via the callback).
enum class FragmentTraversalNextStep {
  // Continue traversal normally.
  kContinue,
  // Skip any children, then continue traversal normally.
  kSkipChildren,
};

// TODO(bug/406288653): Get rid of the LayoutInline parameter.
using BoxFragmentDescendantsCallback =
    base::FunctionRef<FragmentTraversalNextStep(
        const PhysicalBoxFragment*,
        const LayoutInline* culled_inline,
        bool is_first_for_node)>;

// Visit every box fragment descendant in the subtree, depth-first, from left to
// right, including those inside inline formatting contexts (FragmentItem), and
// invoke the callback for each. Fragments that both have PhysicalBoxFragment
// children and an inline formatting context (rare) will walk the
// PhysicalBoxFragment children first. For each descendant visited, the
// specified callback will be called, and its return value determines how to
// proceed with the traversal afterwards. As an added bonus, mostly thanks to
// crbug.com/406288653 , culled inlines also have to be visited.
void ForAllBoxFragmentDescendants(const PhysicalBoxFragment&,
                                  BoxFragmentDescendantsCallback);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TREE_TRAVERSAL_UTILS_H_
