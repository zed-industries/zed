// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_FRAGMENT_H_

#include "third_party/blink/renderer/core/layout/custom/custom_layout_scope.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CustomLayoutChild;
class LayoutBox;
class LayoutInputNode;
class LayoutResult;
class ScriptState;
class ScriptValue;
struct LogicalSize;

// This represents the result of a layout (on a LayoutChild).
//
// The result is stuck in time, e.g. performing another layout of the same
// LayoutChild will not live update the values of this fragment.
//
// The web developer can position this child fragment (setting inlineOffset,
// and blockOffset), which are relative to its parent.
//
// This should eventually mirror the information in a LogicalFragment, it has
// the additional capability that it is exposed to web developers.
class CustomLayoutFragment : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CustomLayoutFragment(CustomLayoutChild*,
                       CustomLayoutToken*,
                       const LayoutResult*,
                       const LogicalSize& size,
                       const std::optional<LayoutUnit> baseline,
                       v8::Isolate*);
  CustomLayoutFragment(const CustomLayoutFragment&) = delete;
  CustomLayoutFragment& operator=(const CustomLayoutFragment&) = delete;
  ~CustomLayoutFragment() override = default;

  double inlineSize() const { return inline_size_; }
  double blockSize() const { return block_size_; }

  double inlineOffset() const { return inline_offset_; }
  double blockOffset() const { return block_offset_; }

  void setInlineOffset(double inline_offset) { inline_offset_ = inline_offset; }
  void setBlockOffset(double block_offset) { block_offset_ = block_offset; }

  std::optional<double> baseline() const { return baseline_; }

  ScriptValue data(ScriptState*) const;

  const LayoutResult& GetLayoutResult() const;
  const LayoutInputNode& GetLayoutNode() const;

  bool IsValid() const { return token_->IsValid(); }

  void Trace(Visitor*) const override;

 private:
  Member<CustomLayoutChild> child_;
  Member<CustomLayoutToken> token_;

  // TODO(ikilpatrick): Store the constraint space here so that we can relayout.
  //
  // There is complexity around state in the layout tree, e.g. from the web
  // developers perspective:
  //
  // const fragment1 = yield child.layoutNextFragment(options1);
  // const fragment2 = yield child.layoutNextFragment(options2);
  // return { childFragments: [fragment1] };
  //
  // In the above example the LayoutBox representing "child" has the incorrect
  // layout state. As we are processing the returned childFragments we detect
  // that the last layout on the child wasn't with the same inputs, and force a
  // layout again.

  Member<const LayoutResult> layout_result_;

  // The inline and block size on this object should never change.
  const double inline_size_;
  const double block_size_;

  // The offset is relative to our parent, and in the parent's writing mode.
  double inline_offset_ = 0;
  double block_offset_ = 0;

  // The first-line baseline.
  const std::optional<double> baseline_;

  TraceWrapperV8Reference<v8::Value> layout_worklet_world_v8_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_LAYOUT_FRAGMENT_H_
