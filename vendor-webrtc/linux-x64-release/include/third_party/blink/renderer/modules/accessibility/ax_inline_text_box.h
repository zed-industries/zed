/*
 * Copyright (C) 2013, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_INLINE_TEXT_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_INLINE_TEXT_BOX_H_

#include <variant>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/layout/inline/abstract_inline_text_box.h"
#include "third_party/blink/renderer/modules/accessibility/ax_block_flow_iterator.h"
#include "third_party/blink/renderer/modules/accessibility/ax_object.h"

namespace blink {

class AXObjectCacheImpl;

// Encapsulates an AbstractInlineTextBox and adapts it for use in Blink's
// accessibility tree.
class AXInlineTextBox final : public AXObject {
 public:
  AXInlineTextBox(AbstractInlineTextBox*, AXObjectCacheImpl&);
  AXInlineTextBox(AXBlockFlowIterator::FragmentIndex, AXObjectCacheImpl&);
  void Trace(Visitor* visitor) const override;

  AXInlineTextBox(const AXInlineTextBox&) = delete;
  AXInlineTextBox& operator=(const AXInlineTextBox&) = delete;

  // AXObject overrides.
  RGBA32 GetColor() const override { return Color::kTransparent.Rgb(); }
  String GetName(ax::mojom::blink::NameFrom&,
                 AXObject::AXObjectVector* name_objects) const override;
  void TextCharacterOffsets(Vector<int>&) const override;
  void GetWordBoundaries(Vector<int>& word_starts,
                         Vector<int>& word_ends) const override;
  int TextOffsetInFormattingContext(int offset) const override;
  int TextOffsetInContainer(int offset) const override;
  void GetRelativeBounds(AXObject** out_container,
                         gfx::RectF& out_bounds_in_container,
                         gfx::Transform& out_container_transform,
                         bool* clips_children = nullptr) const override;
  ax::mojom::blink::WritingDirection GetTextDirection() const override;
  Document* GetDocument() const override;
  AXObject* NextOnLine() const override;
  AXObject* PreviousOnLine() const override;
  void SerializeMarkerAttributes(ui::AXNodeData* node_data) const override;
  ax::mojom::blink::Role NativeRoleIgnoringAria() const override {
    NOTREACHED();
  }
  void ClearChildren() override;
  AbstractInlineTextBox* GetInlineTextBox() const override;
  std::optional<AXBlockFlowIterator::FragmentIndex> GetFragmentIndex()
      const override;

 protected:
  void Init(AXObject* parent) override;
  void Detach() override;
  bool IsAXInlineTextBox() const override;
  bool IsLineBreakingObject() const override;
  int TextLength() const override;

  // Always a leaf.
  bool CanHaveChildren() const override { return false; }
  void AddChildren() override {}

 private:
  enum class Direction { kNext, kPrevious };
  AXObject* NeighboringOnLine(const Direction direction) const;

  // TODO(accessibility): rename this to the one above once we remove the AITB
  // algorithm.
  AXObject* NeighboringOnLineWithAXBlockFlowIterator(
      const Direction direction) const;

  // TODO(a11y): Remove default argument on virtual override.
  // See https://google.github.io/styleguide/cppguide.html#Default_Arguments
  bool ComputeIsIgnored(IgnoredReasons* = nullptr) const override;
  bool IsPartOfAListItem() const;

  // Holds one of two possible values that identify this AXInlineTextBox,
  // depending on the algorithm being used.
  std::variant<Member<AbstractInlineTextBox>,
               AXBlockFlowIterator::FragmentIndex>
      data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_INLINE_TEXT_BOX_H_
