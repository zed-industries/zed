// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BASE_LAYOUT_ALGORITHM_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BASE_LAYOUT_ALGORITHM_TEST_H_

#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/testing/core_unit_test_helper.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BlockNode;
class BreakToken;
class LayoutBlockFlow;
class PhysicalBoxFragment;

// Base class for all LayoutNG Algorithms unit test classes.
typedef bool TestParamLayoutNG;
class BaseLayoutAlgorithmTest
    : public testing::WithParamInterface<TestParamLayoutNG>,
      public RenderingTest {
 protected:
  void SetUp() override;

  void AdvanceToLayoutPhase();

  const PhysicalBoxFragment* RunBlockLayoutAlgorithm(
      BlockNode node,
      const ConstraintSpace& space,
      const BreakToken* break_token = nullptr);

  const PhysicalBoxFragment* RunFieldsetLayoutAlgorithm(
      BlockNode node,
      const ConstraintSpace& space,
      const BreakToken* break_token = nullptr);

  const PhysicalBoxFragment* GetBoxFragmentByElementId(const char*);

  static const PhysicalBoxFragment* CurrentFragmentFor(const LayoutBlockFlow*);
};

class FragmentChildIterator {
  STACK_ALLOCATED();

 public:
  explicit FragmentChildIterator(const PhysicalBoxFragment* parent) {
    SetParent(parent);
  }
  void SetParent(const PhysicalBoxFragment* parent) {
    parent_ = parent;
    index_ = 0;
  }

  const PhysicalBoxFragment* NextChild(
      PhysicalOffset* fragment_offset = nullptr);

 private:
  const PhysicalBoxFragment* parent_;
  unsigned index_;
};

ConstraintSpace ConstructBlockLayoutTestConstraintSpace(
    WritingDirectionMode writing_direction,
    LogicalSize size,
    bool stretch_inline_size_if_auto = true,
    bool is_new_formatting_context = false,
    LayoutUnit fragmentainer_space_available = kIndefiniteSize);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BASE_LAYOUT_ALGORITHM_TEST_H_
