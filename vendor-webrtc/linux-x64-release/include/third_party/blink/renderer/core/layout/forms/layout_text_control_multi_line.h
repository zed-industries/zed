// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_TEXT_CONTROL_MULTI_LINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_TEXT_CONTROL_MULTI_LINE_H_

#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

// LayoutTextControlMultiLine is a LayoutObject for <textarea>.
class LayoutTextControlMultiLine final : public LayoutBlockFlow {
 public:
  explicit LayoutTextControlMultiLine(Element* element);

 private:
  HTMLElement* InnerEditorElement() const;

  bool IsTextArea() const final {
    NOT_DESTROYED();
    return true;
  }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTextControlMultiLine";
  }

  bool CreatesNewFormattingContext() const override {
    NOT_DESTROYED();
    return true;
  }

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  bool NodeAtPoint(HitTestResult& result,
                   const HitTestLocation& hit_test_location,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase phase) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_TEXT_CONTROL_MULTI_LINE_H_
