/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2007 David Smith (catfish.man@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc.
 *               All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BLOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BLOCK_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/linked_hash_set.h"

namespace blink {

struct PaintInfo;

using TrackedLayoutBoxLinkedHashSet = GCedHeapLinkedHashSet<Member<LayoutBox>>;
using TrackedDescendantsMap =
    GCedHeapHashMap<WeakMember<const LayoutBlock>,
                    Member<TrackedLayoutBoxLinkedHashSet>>;

// LayoutBlock is the class that is used by any LayoutObject
// that is a containing block.
// http://www.w3.org/TR/CSS2/visuren.html#containing-block
// See also LayoutObject::ContainingBlock() that is the function
// used to get the containing block of a LayoutObject.
//
// CSS is inconsistent and allows inline elements (LayoutInline) to be
// containing blocks, even though they are not blocks. Our
// implementation is as confused with inlines. See e.g.
// LayoutObject::ContainingBlock() vs LayoutObject::Container().
//
// Containing blocks are a central concept for layout, in
// particular to the layout of out-of-flow positioned
// elements. They are used to determine the sizing as well
// as the positioning of the LayoutObjects.
//
// LayoutBlock is the class that handles out-of-flow positioned elements in
// Blink, in particular for layout (see LayoutPositionedObjects()). That's why
// LayoutBlock keeps track of them through |GetPositionedDescendantsMap()| (see
// layout_block.cc).
// Note that this is a design decision made in Blink that doesn't reflect CSS:
// CSS allows relatively positioned inlines (LayoutInline) to be containing
// blocks, but they don't have the logic to handle out-of-flow positioned
// objects. This induces some complexity around choosing an enclosing
// LayoutBlock (for inserting out-of-flow objects during layout) vs the CSS
// containing block (for sizing, invalidation).
//
//
// ***** WHO LAYS OUT OUT-OF-FLOW POSITIONED OBJECTS? *****
// A positioned object gets inserted into an enclosing LayoutBlock's positioned
// map. This is determined by LayoutObject::ContainingBlock().
//
//
// ***** HANDLING OUT-OF-FLOW POSITIONED OBJECTS *****
// Care should be taken to handle out-of-flow positioned objects during
// certain tree walks (e.g. Layout()). The rule is that anything that
// cares about containing blocks should skip the out-of-flow elements
// in the normal tree walk and do an optional follow-up pass for them
// using LayoutBlock::PositionedObjects().
// Not doing so will result in passing the wrong containing
// block as tree walks will always pass the parent as the
// containing block.
//
// Sample code of how to handle positioned objects in LayoutBlock:
//
// for (LayoutObject* child = FirstChild(); child; child = child->NextSibling())
// {
//     if (child->IsOutOfFlowPositioned())
//         continue;
//
//     // Handle normal flow children.
//     ...
// }
// for (LayoutBox* positioned_object : PositionedObjects()) {
//     // Handle out-of-flow positioned objects.
//     ...
// }
class CORE_EXPORT LayoutBlock : public LayoutBox {
 protected:
  explicit LayoutBlock(ContainerNode*);

 public:
  void Trace(Visitor*) const override;

  bool IsLayoutNGObject() const override;

  LayoutObject* FirstChild() const {
    NOT_DESTROYED();
    DCHECK_EQ(Children(), VirtualChildren());
    return Children()->FirstChild();
  }
  LayoutObject* LastChild() const {
    NOT_DESTROYED();
    DCHECK_EQ(Children(), VirtualChildren());
    return Children()->LastChild();
  }

  // If you have a LayoutBlock, use FirstChild or LastChild instead.
  void SlowFirstChild() const = delete;
  void SlowLastChild() const = delete;

  const LayoutObjectChildList* Children() const {
    NOT_DESTROYED();
    return &children_;
  }
  LayoutObjectChildList* Children() {
    NOT_DESTROYED();
    return &children_;
  }

  // These two functions are overridden for inline-block.
  LayoutUnit FirstLineHeight() const override;

  const char* GetName() const override;

 protected:
  // Insert a child correctly into the tree when |before_descendant| isn't a
  // direct child of |this|. This happens e.g. when there's an anonymous block
  // child of |this| and |before_descendant| has been reparented into that one.
  // Such things are invisible to the DOM, and addChild() is typically called
  // with the DOM tree (and not the layout tree) in mind.
  void AddChildBeforeDescendant(LayoutObject* new_child,
                                LayoutObject* before_descendant);

 public:
  void AddChild(LayoutObject* new_child,
                LayoutObject* before_child = nullptr) override;

  void RemovePositionedObjects(LayoutObject*);

  void AddSvgTextDescendant(LayoutBox& svg_text);
  void RemoveSvgTextDescendant(LayoutBox& svg_text);

  LayoutUnit TextIndentOffset() const;

  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  static LayoutBlock* CreateAnonymousWithParentAndDisplay(
      const LayoutObject*,
      EDisplay = EDisplay::kBlock);
  LayoutBlock* CreateAnonymousBlock(EDisplay display = EDisplay::kBlock) const {
    NOT_DESTROYED();
    return CreateAnonymousWithParentAndDisplay(this, display);
  }

  LayoutBox* CreateAnonymousBoxWithSameTypeAs(
      const LayoutObject* parent) const override;

 public:
  RecalcScrollableOverflowResult RecalcScrollableOverflow() override;

  void RecalcVisualOverflow() override;

  // An example explaining layout tree structure about first-line style:
  // <style>
  //   #enclosingFirstLineStyleBlock::first-line { ... }
  // </style>
  // <div id="enclosingFirstLineStyleBlock">
  //   <div>
  //     <div id="nearestInnerBlockWithFirstLine">
  //       [<span>]first line text[</span>]
  //     </div>
  //   </div>
  // </div>

  // Return the parent LayoutObject if it can contribute to our ::first-line
  // style.
  const LayoutBlock* FirstLineStyleParentBlock() const;

  // Returns this block or the nearest inner block containing the actual first
  // line.
  LayoutBlockFlow* NearestInnerBlockWithFirstLine();

 protected:
  void WillBeDestroyed() override;

 public:
  void Paint(const PaintInfo&) const override;

  virtual bool HasLineIfEmpty() const;
  // Returns baseline offset if we can get |SimpleFontData| from primary font.
  // Or returns no value if we can't get font data.
  std::optional<LayoutUnit> BaselineForEmptyLine() const;

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) override;

 protected:
  bool HitTestChildren(HitTestResult&,
                       const HitTestLocation&,
                       const PhysicalOffset& accumulated_offset,
                       HitTestPhase) override;

  void StyleWillChange(StyleDifference,
                       const ComputedStyle& new_style) override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  bool RespectsCSSOverflow() const override;

 protected:
  void AddOutlineRects(OutlineRectCollector&,
                       OutlineInfo*,
                       const PhysicalOffset& additional_offset,
                       OutlineType) const override;

  bool IsInSelfHitTestingPhase(HitTestPhase phase) const final {
    NOT_DESTROYED();
    return phase == HitTestPhase::kSelfBlockBackground;
  }

 private:
  LayoutObjectChildList* VirtualChildren() final {
    NOT_DESTROYED();
    return Children();
  }
  const LayoutObjectChildList* VirtualChildren() const final {
    NOT_DESTROYED();
    return Children();
  }

  bool IsLayoutBlock() const final {
    NOT_DESTROYED();
    return true;
  }

  virtual void RemoveLeftoverAnonymousBlock(LayoutBlock* child);

 protected:
  void InvalidatePaint(const PaintInvalidatorContext&) const override;

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;

 private:
  PhysicalRect LocalCaretRect(int caret_offset) const final;
  bool IsInlineBoxWrapperActuallyChild() const;

  // End helper functions and structs used by layoutBlockChildren.

  void RemoveFromGlobalMaps();

 protected:
  PositionWithAffinity PositionForPointIfOutsideAtomicInlineLevel(
      const PhysicalOffset&) const;

  LayoutObjectChildList children_;

  // FIXME: This is temporary as we move code that accesses block flow
  // member variables out of LayoutBlock and into LayoutBlockFlow.
  friend class LayoutBlockFlow;
};

template <>
struct DowncastTraits<LayoutBlock> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutBlock();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BLOCK_H_
