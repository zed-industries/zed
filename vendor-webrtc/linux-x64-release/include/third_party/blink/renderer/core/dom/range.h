/*
 * (C) 1999 Lars Knoll (knoll@kde.org)
 * (C) 2000 Gunnstein Lye (gunnstein@netcom.no)
 * (C) 2000 Frederik Holljen (frederik.holljen@hig.no)
 * (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
 * reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_RANGE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/abstract_range.h"
#include "third_party/blink/renderer/core/dom/range_boundary_point.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/rect_f.h"

namespace gfx {
class QuadF;
}

namespace blink {

class DOMRect;
class DOMRectList;
class ContainerNode;
class Document;
class DocumentFragment;
class ExceptionState;
class Node;
class NodeWithIndex;
class Text;

class CORE_EXPORT Range final : public AbstractRange {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static Range* Create(Document&);

  explicit Range(Document&);
  Range(Document& owner_document, const Position& start, const Position& end);
  Range(Document&,
        Node* start_container,
        unsigned start_offset,
        Node* end_container,
        unsigned end_offset);

  void Dispose();

  Document& OwnerDocument() const override {
    DCHECK(owner_document_);
    return *owner_document_.Get();
  }
  Node* startContainer() const override { return &start_.Container(); }
  unsigned startOffset() const override { return start_.Offset(); }
  Node* endContainer() const override { return &end_.Container(); }
  unsigned endOffset() const override { return end_.Offset(); }

  bool collapsed() const override { return start_ == end_; }
  bool IsConnected() const;

  Node* commonAncestorContainer() const;
  static Node* commonAncestorContainer(const Node* container_a,
                                       const Node* container_b);
  void setStart(Node* container,
                unsigned offset,
                ExceptionState& = ASSERT_NO_EXCEPTION);
  void setEnd(Node* container,
              unsigned offset,
              ExceptionState& = ASSERT_NO_EXCEPTION);
  void collapse(bool to_start);
  bool isPointInRange(Node* ref_node, unsigned offset, ExceptionState&) const;
  int16_t comparePoint(Node* ref_node, unsigned offset, ExceptionState&) const;
  enum CompareHow { kStartToStart, kStartToEnd, kEndToEnd, kEndToStart };
  int16_t compareBoundaryPoints(unsigned how,
                                const Range* source_range,
                                ExceptionState&) const;
  static int16_t compareBoundaryPoints(Node* container_a,
                                       unsigned offset_a,
                                       Node* container_b,
                                       unsigned offset_b,
                                       ExceptionState&);
  static int16_t compareBoundaryPoints(const RangeBoundaryPoint& boundary_a,
                                       const RangeBoundaryPoint& boundary_b,
                                       ExceptionState&);
  bool BoundaryPointsValid() const;
  bool intersectsNode(Node* ref_node, ExceptionState&);
  void deleteContents(ExceptionState&);
  DocumentFragment* extractContents(ExceptionState&);
  DocumentFragment* cloneContents(ExceptionState&);
  void insertNode(Node*, ExceptionState&);
  String toString() const;

  String GetText() const;

  DocumentFragment* createContextualFragment(const String& html,
                                             ExceptionState&);

  void detach();
  Range* cloneRange() const;

  void setStartAfter(Node*, ExceptionState& = ASSERT_NO_EXCEPTION);
  void setEndBefore(Node*, ExceptionState& = ASSERT_NO_EXCEPTION);
  void setEndAfter(Node*, ExceptionState& = ASSERT_NO_EXCEPTION);
  void selectNode(Node*, ExceptionState& = ASSERT_NO_EXCEPTION);
  void selectNodeContents(Node*, ExceptionState&);
  static bool selectNodeContents(Node*, Position&, Position&);
  void surroundContents(Node*, ExceptionState&);
  void setStartBefore(Node*, ExceptionState& = ASSERT_NO_EXCEPTION);

  const Position StartPosition() const { return start_.ToPosition(); }
  const Position EndPosition() const { return end_.ToPosition(); }
  void setStart(const Position&, ExceptionState& = ASSERT_NO_EXCEPTION);
  void setEnd(const Position&, ExceptionState& = ASSERT_NO_EXCEPTION);

  Node* FirstNode() const;
  Node* PastLastNode() const;

  // Not transform-friendly
  gfx::Rect BoundingBox() const;

  // Transform-friendly
  void GetBorderAndTextQuads(Vector<gfx::QuadF>&) const;
  gfx::RectF BoundingRect() const;

  void NodeChildrenWillBeRemoved(ContainerNode&);
  void NodeWillBeRemoved(Node&);

  // They are special fixups only for sequential focus navigation
  // starting point.
  void FixupRemovedChildrenAcrossShadowBoundary(ContainerNode& container);
  void FixupRemovedNodeAcrossShadowBoundary(Node& node);

  void DidInsertText(const CharacterData&, unsigned offset, unsigned length);
  void DidRemoveText(const CharacterData&, unsigned offset, unsigned length);
  void DidMergeTextNodes(const NodeWithIndex& old_node, unsigned offset);
  void DidSplitTextNode(const Text& old_node);
  void UpdateOwnerDocumentIfNeeded();

  // Expand range to a unit (word or sentence or block or document) boundary.
  // Please refer to https://bugs.webkit.org/show_bug.cgi?id=27632 comment #5
  // for details.
  void expand(const String&, ExceptionState&);

  DOMRectList* getClientRects() const;
  DOMRect* getBoundingClientRect() const;

  static Node* CheckNodeWOffset(Node*, unsigned offset, ExceptionState&);

  bool IsStaticRange() const override { return false; }
  void Trace(Visitor*) const override;

 private:
  void SetDocument(Document&);

  void CheckNodeBA(Node*, ExceptionState&) const;
  void CheckExtractPrecondition(ExceptionState&);
  bool HasSameRoot(const Node&) const;

  enum ActionType { kDeleteContents, kExtractContents, kCloneContents };
  DocumentFragment* ProcessContents(ActionType, ExceptionState&);
  static Node* ProcessContentsBetweenOffsets(ActionType,
                                             DocumentFragment*,
                                             Node*,
                                             unsigned start_offset,
                                             unsigned end_offset,
                                             ExceptionState&);
  static void ProcessNodes(ActionType,
                           NodeVector&,
                           Node* old_container,
                           Node* new_container,
                           ExceptionState&);
  enum ContentsProcessDirection {
    kProcessContentsForward,
    kProcessContentsBackward
  };
  static Node* ProcessAncestorsAndTheirSiblings(ActionType,
                                                Node* container,
                                                ContentsProcessDirection,
                                                Node* cloned_container,
                                                Node* common_root,
                                                ExceptionState&);
  void UpdateSelectionIfAddedToSelection();
  void ScheduleVisualUpdateIfInRegisteredHighlight(Document& document);
  void RemoveFromSelectionIfInDifferentRoot(Document& old_document);

  void CollapseIfNeeded(bool did_move_document, bool collapse_to_start);

  Member<Document> owner_document_;  // Cannot be null.
  RangeBoundaryPoint start_;
  RangeBoundaryPoint end_;

  // This tracks how the range updates the selection:
  // If kAll, set selection to have the same start and end as range.
  // If kStartOnly, set selection to have the same start as range.
  // If kEndOnly, set selection to have the same end as range.
  enum class UpdateSelectionBehavior {
    kAll,
    kStartOnly,
    kEndOnly,
  };
  UpdateSelectionBehavior update_selection_behavior_ =
      UpdateSelectionBehavior::kAll;
  void ResetUpdateSelectionBehavior();

  friend class RangeUpdateScope;
};

CORE_EXPORT bool AreRangesEqual(const Range*, const Range*);

using RangeVector = HeapVector<Member<Range>>;

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from gdb.
void ShowTree(const blink::Range*);
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_RANGE_H_
