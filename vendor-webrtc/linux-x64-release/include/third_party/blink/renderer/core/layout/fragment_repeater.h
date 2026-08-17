// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAGMENT_REPEATER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAGMENT_REPEATER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutBox;
class LayoutResult;
class PhysicalBoxFragment;

// Fragment tree mutator / cloner / repeater.
//
// This is needed in order to implement repeated content in block fragmentation
// (repeated table headers / footers, and also fixed-positioned elements when
// printing).
//
// On the layout side, we only lay out the element once, but pre-paint and paint
// require one unique fragment for each time it repeats, since we need one
// FragmentData object for each, each with its own global-ish paint offset.
class FragmentRepeater {
  STACK_ALLOCATED();

 public:
  // When regular layout is complete, we will have the right number of cloned
  // root fragments, but its child fragments have not yet been cloned. Do that
  // now, and update break tokens accordingly.
  static void DeepCloneRepeatableRoot(LayoutBox&);

 private:
  FragmentRepeater(bool is_first_clone, bool is_last_fragment)
      : is_first_clone_(is_first_clone), is_last_fragment_(is_last_fragment) {}

  // Deep-clone the subtree of an already shallowly cloned fragment. This will
  // also create new break tokens inside, in order to set unique sequence
  // numbers. The result is only usable by pre-paint / painting, not by actual
  // layout.
  void CloneChildFragments(const PhysicalBoxFragment& cloned_fragment);

  const LayoutResult* Repeat(const LayoutResult& other);

  const LayoutResult* GetClonableLayoutResult(
      const LayoutBox& layout_box,
      const PhysicalBoxFragment& fragment) const;

  // True when at the first cloned fragment.
  bool is_first_clone_;

  // True when at the last container fragment. No outgoing "repeat" break tokens
  // should be created then.
  bool is_last_fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAGMENT_REPEATER_H_
