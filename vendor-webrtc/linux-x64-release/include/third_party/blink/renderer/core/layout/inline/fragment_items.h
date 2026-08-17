// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_FRAGMENT_ITEMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_FRAGMENT_ITEMS_H_

#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_item.h"

namespace blink {

class FragmentItemsBuilder;
class LayoutInline;

// Represents the inside of an inline formatting context.
//
// During the layout phase, descendants of the inline formatting context is
// transformed to a flat list of |FragmentItem| and stored in this class.
class CORE_EXPORT FragmentItems final {
  DISALLOW_NEW();

 public:
  FragmentItems(const FragmentItems& other);
  explicit FragmentItems(FragmentItemsBuilder* builder);

  wtf_size_t Size() const { return items_.size(); }

  using Span = base::span<const FragmentItem>;
  Span Items() const { return base::span(items_); }
  bool Equals(const Span& span) const {
    return ItemsData() == span.data() && Size() == span.size();
  }
  bool IsSubSpan(const Span& span) const;

  const FragmentItem& operator[](wtf_size_t index) const {
    return items_[index];
  }
  const FragmentItem& front() const {
    CHECK_GE(items_.size(), 1u);
    return items_[0];
  }

  // Text content for |this| inline formatting context.
  const String& NormalText() const { return text_content_; }
  // Text content for `::first-line`. Available only if `::first-line` has
  // different style than non-first-line style.
  const String& FirstLineText() const { return first_line_text_content_; }
  // Returns |FirstLineText()| if it is available and |first_line| is |true|.
  // Otherwise returns |NormalText()|.
  const String& Text(bool first_line) const {
    if (first_line && first_line_text_content_) [[unlikely]] {
      return first_line_text_content_;
    }
    return text_content_;
  }

  // When block-fragmented, returns the number of |FragmentItem| in earlier
  // fragments for this box. 0 for the first fragment.
  wtf_size_t SizeOfEarlierFragments() const {
    return size_of_earlier_fragments_;
  }
  wtf_size_t EndItemIndex() const {
    return size_of_earlier_fragments_ + items_.size();
  }
  bool HasItemIndex(wtf_size_t index) const {
    return index >= SizeOfEarlierFragments() && index < EndItemIndex();
  }

  // Associate |FragmentItem|s with |LayoutObject|s and finalize the items
  // (set which ones are the first / last for the LayoutObject).
  static void FinalizeAfterLayout(
      const HeapVector<Member<const LayoutResult>, 1>& results,
      LayoutBlockFlow& container);

  // Disassociate |FragmentItem|s with |LayoutObject|s. And more.
  static void ClearAssociatedFragments(LayoutObject* container);

  // Notify when |LayoutObject| will be destroyed/moved.
  static void LayoutObjectWillBeDestroyed(const LayoutObject& layout_object);
  static void LayoutObjectWillBeMoved(const LayoutObject& layout_object);

  // Returns the end (next of the last) item that are reusable. If no items are
  // reusable, it is the first item.
  const FragmentItem* EndOfReusableItems(
      const PhysicalBoxFragment& container) const;

  // Return true if any items inside the culled inline occur here. In that case,
  // |is_first_container| and |is_last_container| will also be set to indicate
  // whether this is the first/last container for the culled inline. If false is
  // returned, the |is_first_container| and |is_last_container| out-parameters
  // must be ignored. |child_has_any_child_items| will be set to true if there
  // are any items inside the LayoutInline at all, regardless of return value.
  bool IsContainerForCulledInline(const LayoutInline&,
                                  bool* is_first_container,
                                  bool* is_last_container,
                                  bool* child_has_any_child_items) const;

  // Mark items dirty when |child| is removed from the tree.
  static void DirtyLinesFromChangedChild(const LayoutObject& child,
                                         const LayoutBlockFlow& container);

  // Mark items dirty from |LayoutObject::NeedsLayout| flags.
  static void DirtyLinesFromNeedsLayout(const LayoutBlockFlow& block_flow);

  // Search for |old_fragment| among the fragment items inside
  // |containing_fragment|, and replace it with |new_fragment| if found. Return
  // true if found and replaced, otherwise false.
  static bool ReplaceBoxFragment(
      const PhysicalBoxFragment& old_fragment,
      const PhysicalBoxFragment& new_fragment,
      const PhysicalBoxFragment& containing_fragment);

#if DCHECK_IS_ON()
  void CheckAllItemsAreValid() const;
#endif

  void Trace(Visitor*) const;

 private:
  const FragmentItem* ItemsData() const { return items_.data(); }

  static bool CanReuseAll(InlineCursor* cursor);
  static bool TryDirtyFirstLineFor(const LayoutObject& layout_object,
                                   const LayoutBlockFlow& container);
  static bool TryDirtyLastLineFor(const LayoutObject& layout_object,
                                  const LayoutBlockFlow& container);
  static void DirtyFirstItem(const LayoutBlockFlow& container);

  String text_content_;
  String first_line_text_content_;

  // Total size of |FragmentItem| in earlier fragments when block fragmented.
  // 0 for the first |FragmentItems|.
  mutable wtf_size_t size_of_earlier_fragments_ = 0u;

  HeapVector<FragmentItem> items_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_FRAGMENT_ITEMS_H_
