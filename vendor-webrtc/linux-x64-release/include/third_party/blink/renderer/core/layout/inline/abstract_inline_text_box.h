// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_ABSTRACT_INLINE_TEXT_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_ABSTRACT_INLINE_TEXT_BOX_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_text.h"
#include "third_party/blink/renderer/platform/geometry/physical_direction.h"

namespace blink {

class InlineCursor;
class PhysicalBoxFragment;

// High-level abstraction of a text box fragment, to allow the accessibility
// module to get information without tight coupling.
class CORE_EXPORT AbstractInlineTextBox final
    : public GarbageCollected<AbstractInlineTextBox> {
 private:
  static void WillDestroy(const InlineCursor& cursor);

  friend class LayoutText;

 public:
  // Returns existing or newly created |AbstractInlineTextBox|.
  // * |cursor| should be attached to a text item.
  static AbstractInlineTextBox* GetOrCreate(const InlineCursor& cursor);
  explicit AbstractInlineTextBox(const InlineCursor& cursor);
  ~AbstractInlineTextBox();
  void Trace(Visitor* visitor) const;

  struct WordBoundaries {
    DISALLOW_NEW();
    WordBoundaries(int start_index, int end_index)
        : start_index(start_index), end_index(end_index) {}
    int start_index;
    int end_index;
  };
  static void GetWordBoundariesForText(Vector<WordBoundaries>&, const String&);

  void Detach();
  AbstractInlineTextBox* NextInlineTextBox() const;
  PhysicalRect LocalBounds() const;
  unsigned Len() const;
  // Given a text offset in this inline text box, returns the equivalent text
  // offset in this box's formatting context. The formatting context is the
  // deepest block flow ancestor, e.g. the enclosing paragraph. A "text offset",
  // in contrast to a "DOM offset", is an offset in the box's text after any
  // collapsible white space in the DOM has been collapsed.
  unsigned TextOffsetInFormattingContext(unsigned offset) const;
  PhysicalDirection GetDirection() const;
  Node* GetNode() const;
  LayoutText* GetLayoutText() const { return layout_text_.Get(); }
  AXObjectCache* ExistingAXObjectCache() const;
  void GetCharacterLayoutPixelOffsets(Vector<int>&) const;
  void GetWordBoundaries(Vector<WordBoundaries>&) const;
  String GetText() const;

  // Returns true if the AbstractInlineTextBox is the first/last in the
  // LayoutObject. Note that this **is different** from
  // NextOnLine/PreviousOnLine, where they are allowed to cross boundaries
  // between layoutObjects.
  bool IsFirstForLayoutObject() const;

  bool IsLineBreak() const;
  bool NeedsTrailingSpace() const;
  InlineCursor GetCursor() const;
  InlineCursor GetCursorOnLine() const;

 private:
  LayoutText* GetFirstLetterPseudoLayoutText() const;
  String GetTextContent() const;

  // FragmentItem index in root_box_fragment_'s FragmentItems.
  // It's an index instead of an FragmentItem pointer because FragmentItem
  // instances are stored in HeapVector instances, and Oilpan heap compaction
  // changes addresses of FragmentItem instances.
  std::optional<wtf_size_t> fragment_item_index_;
  Member<LayoutText> layout_text_;
  // |root_box_fragment_| owns |fragment_item_|. Persistent is used here to keep
  // |AbstractInlineTextBoxCache| off-heap.
  Member<const PhysicalBoxFragment> root_box_fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_ABSTRACT_INLINE_TEXT_BOX_H_
