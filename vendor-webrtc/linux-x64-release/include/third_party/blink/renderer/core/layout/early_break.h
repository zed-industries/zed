// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EARLY_BREAK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EARLY_BREAK_H_

#include "base/check_op.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/break_appeal.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

// Possible early unforced breakpoint. This represents a possible (and good)
// location to break. In cases where we run out of space at an unideal location,
// we may want to go back and break here instead.
class EarlyBreak : public GarbageCollected<EarlyBreak> {
 public:
  enum BreakType {
    kLine,  // Break before a specified line number.
    kBlock  // Break before or inside a specified child block.
  };

  explicit EarlyBreak(BlockNode block,
                      BreakAppeal break_appeal,
                      const EarlyBreak* break_inside_child = nullptr)
      : box_(block.GetLayoutBox()),
        break_inside_child_(break_inside_child),
        const_type_(kBlock),
        break_appeal_(break_appeal) {}
  explicit EarlyBreak(int line_number, BreakAppeal break_appeal)
      : line_number_(line_number),
        const_type_(kLine),
        break_appeal_(break_appeal) {}

  BreakType Type() const { return static_cast<BreakType>(const_type_); }
  bool IsBreakBefore() const { return !break_inside_child_; }
  BlockNode GetBlockNode() const {
    CHECK_EQ(const_type_, kBlock);
    return BlockNode(box_);
  }
  int LineNumber() const {
    DCHECK_EQ(const_type_, kLine);
    return line_number_;
  }
  const EarlyBreak* BreakInside() const { return break_inside_child_.Get(); }

  BreakAppeal GetBreakAppeal() const {
    return static_cast<BreakAppeal>(break_appeal_);
  }

  void Trace(Visitor* visitor) const {
    // It is safe to check |const_type_| here because it is a const value.
    if (const_type_ == kBlock)
      visitor->Trace(box_);
    visitor->Trace(break_inside_child_);
  }

 private:
  union {
    GC_PLUGIN_IGNORE("https://crbug.com/1146383")
    Member<LayoutBox> box_;  // Set if const_type_ == kBlock

    int line_number_;  // Set if const_type_ == kLine
  };
  Member<const EarlyBreak> break_inside_child_;
  const unsigned const_type_ : 1;                     // BreakType
  unsigned break_appeal_ : kBreakAppealBitsNeeded;    // BreakAppeal
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EARLY_BREAK_H_
