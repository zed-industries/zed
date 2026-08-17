// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_FIELDSET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_FIELDSET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

class CORE_EXPORT LayoutFieldset final : public LayoutBlockFlow {
 public:
  explicit LayoutFieldset(Element*);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutFieldset";
  }

  void AddChild(LayoutObject* new_child,
                LayoutObject* before_child = nullptr) override;

  bool CreatesNewFormattingContext() const final {
    NOT_DESTROYED();
    return true;
  }

  LayoutBox* ContentLayoutBox() final {
    NOT_DESTROYED();
    return FindAnonymousFieldsetContentBox();
  }

  LayoutBlock* FindAnonymousFieldsetContentBox() const;

  static LayoutBox* FindInFlowLegend(const LayoutBlock& fieldset);
  LayoutBox* FindInFlowLegend() const {
    NOT_DESTROYED();
    return FindInFlowLegend(*this);
  }

 protected:
  bool IsFieldset() const final {
    NOT_DESTROYED();
    return true;
  }
  void InsertedIntoTree() override;
  void UpdateAnonymousChildStyle(
      const LayoutObject* child,
      ComputedStyleBuilder& child_style_builder) const override;
  void InvalidatePaint(const PaintInvalidatorContext& context) const final;
  bool BackgroundIsKnownToBeOpaqueInRect(const PhysicalRect&) const override;

  // Fieldset paints background specially.
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override {
    NOT_DESTROYED();
    return false;
  }

  bool RespectsCSSOverflow() const override {
    NOT_DESTROYED();
    return false;
  }
  // Override to forward to the anonymous fieldset content box.
  LayoutUnit ScrollWidth() const override;
  LayoutUnit ScrollHeight() const override;
};

template <>
struct DowncastTraits<LayoutFieldset> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsFieldset();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_LAYOUT_FIELDSET_H_
