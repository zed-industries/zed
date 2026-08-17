/*
 * Copyright (C) 2006 Apple Computer, Inc.
 * Copyright (C) 2012 Nokia Corporation and/or its subsidiary(-ies)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_RESULT_H_

#include <tuple>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/hit_test_request.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/quad_f.h"
#include "ui/gfx/geometry/rect_f.h"

namespace cc {
class Region;
}

namespace blink {

class Element;
class HTMLAreaElement;
class HTMLMediaElement;
class HitTestLocation;
class Image;
class KURL;
class LocalFrame;
class MediaSourceHandle;
class MediaStreamDescriptor;
class Node;
class PhysicalBoxFragment;
class Scrollbar;

class CORE_EXPORT HitTestResult {
  DISALLOW_NEW();

 public:
  using NodeSet = GCedHeapLinkedHashSet<Member<Node>>;

  HitTestResult();
  HitTestResult(const HitTestRequest&, const HitTestLocation&);
  HitTestResult(const HitTestResult&);
  ~HitTestResult();
  HitTestResult& operator=(const HitTestResult&);
  void Trace(Visitor*) const;

  bool EqualForCacheability(const HitTestResult&) const;
  void CacheValues(const HitTestResult& other);

  // Populate this object based on another HitTestResult; similar to assignment
  // operator but don't assign any of the request parameters. ie. This method
  // avoids setting |m_hitTestLocation|, |m_hitTestRequest|.
  void PopulateFromCachedResult(const HitTestResult&);

  // For point-based hit tests, these accessors provide information about the
  // node under the point. For rect-based hit tests they are meaningless
  // (reflect the last candidate node observed in the rect).
  // FIXME: Make these less error-prone for rect-based hit tests (center point
  // or fail).
  Node* InnerNode() const { return inner_node_.Get(); }
  Node* InnerPossiblyPseudoNode() const {
    return inner_possibly_pseudo_node_.Get();
  }
  CompositorElementId GetScrollableContainer() const;
  Element* InnerElement() const { return inner_element_.Get(); }

  // If innerNode is an image map or image map area, return the associated image
  // node.
  Node* InnerNodeOrImageMapImage() const;

  Element* URLElement() const { return inner_url_element_.Get(); }
  Scrollbar* GetScrollbar() const { return scrollbar_.Get(); }
  bool IsOverEmbeddedContentView() const {
    return is_over_embedded_content_view_;
  }

  // The hit-tested point in the coordinates of the innerNode frame, the frame
  // containing innerNode.
  const PhysicalOffset& PointInInnerNodeFrame() const {
    return point_in_inner_node_frame_;
  }
  void SetPointInInnerNodeFrame(const PhysicalOffset& point) {
    point_in_inner_node_frame_ = point;
  }
  gfx::Point RoundedPointInInnerNodeFrame() const {
    return ToRoundedPoint(PointInInnerNodeFrame());
  }
  LocalFrame* InnerNodeFrame() const;

  // The hit-tested point in the coordinates of the
  // |inner_possibly_pseudo_node_|.
  const PhysicalOffset& LocalPoint() const { return local_point_; }
  void SetNodeAndPosition(Node* node, const PhysicalOffset& p) {
    local_point_ = p;
    SetInnerNode(node);
  }
  void SetNodeAndPosition(Node*,
                          const PhysicalBoxFragment*,
                          const PhysicalOffset&);

  // Override an inner node previously set. The new node needs to be monolithic
  // (or at least only consist of one fragment).
  //
  // TODO(layout-dev): Figure out if we really need this. Why can't we just
  // hit-test correctly in the first place instead?
  void OverrideNodeAndPosition(Node*, PhysicalOffset);

  PositionWithAffinity GetPosition() const;
  PositionWithAffinity GetPositionForInnerNodeOrImageMapImage() const;

  void SetToShadowHostIfInUAShadowRoot();

  const HitTestRequest& GetHitTestRequest() const { return hit_test_request_; }

  void SetInnerNode(Node*);
  HTMLAreaElement* ImageAreaForImage() const;
  void SetURLElement(Element*);
  void SetScrollbar(Scrollbar*);
  void SetIsOverEmbeddedContentView(bool b) {
    is_over_embedded_content_view_ = b;
  }
  void SetIsOverResizer(bool is_over_resizer) {
    is_over_resizer_ = is_over_resizer;
  }
  bool IsOverResizer() const { return is_over_resizer_; }

  void SetIsOverScrollCorner(bool is_over_scroll_corner) {
    is_over_scroll_corner_ = is_over_scroll_corner;
  }
  bool IsOverScrollCorner() const { return is_over_scroll_corner_; }

  bool IsSelected(const HitTestLocation& location) const;
  String Title(TextDirection&) const;
  const AtomicString& AltDisplayString() const;
  static Image* GetImage(const Node* node);
  Image* GetImage() const;
  gfx::Rect ImageRect() const;
  static KURL AbsoluteImageURL(const Node* node);
  KURL AbsoluteImageURL() const;
  KURL AbsoluteMediaURL() const;
  MediaStreamDescriptor* GetMediaStreamDescriptor() const;
  MediaSourceHandle* GetMediaSourceHandle() const;
  KURL AbsoluteLinkURL() const;
  String TextContent() const;
  bool IsLiveLink() const;
  bool IsContentEditable() const;

  bool IsOverLink() const;

  bool IsCacheable() const { return cacheable_; }
  void SetCacheable(bool cacheable) { cacheable_ = cacheable; }

  // TODO(pdr): When using the default rect argument, this function does not
  // check if the tapped area is entirely contained by the HitTestLocation's
  // bounding box. Callers should pass a PhysicalRect as the third parameter so
  // hit testing can early-out when a tapped area is covered.
  ListBasedHitTestBehavior AddNodeToListBasedTestResult(
      Node*,
      const HitTestLocation&,
      const PhysicalRect& = PhysicalRect());
  ListBasedHitTestBehavior AddNodeToListBasedTestResult(Node*,
                                                        const HitTestLocation&,
                                                        const gfx::QuadF& quad);
  ListBasedHitTestBehavior AddNodeToListBasedTestResult(Node*,
                                                        const HitTestLocation&,
                                                        const cc::Region&);

  void Append(const HitTestResult&);

  bool HasListBasedResult() const {
    return GetHitTestRequest().ListBased() && InnerNode();
  }

  // If m_listBasedTestResult is 0 then set it to a new NodeSet. Return
  // *m_listBasedTestResult. Lazy allocation makes sense because the NodeSet is
  // seldom necessary, and it's somewhat expensive to allocate and initialize.
  // This method does the same thing as mutableListBasedTestResult(), but here
  // the return value is const.
  const NodeSet& ListBasedTestResult() const;

  // Collapse the rect-based test result into a single target at the specified
  // location.
  HitTestLocation ResolveRectBasedTest(
      Node* resolved_inner_node,
      const PhysicalOffset& resolved_point_in_main_frame);

 private:
  NodeSet& MutableListBasedTestResult();  // See above.
  HTMLMediaElement* MediaElement() const;
  std::tuple<bool, ListBasedHitTestBehavior>
  AddNodeToListBasedTestResultInternal(Node* node,
                                       const HitTestLocation& location);

  HitTestRequest hit_test_request_;
  bool cacheable_;

  Member<Node> inner_node_;
  // This gets calculated in the first call to InnerElement function.
  Member<Element> inner_element_;
  Member<Node> inner_possibly_pseudo_node_;
  // FIXME: Nothing changes this to a value different from m_hitTestLocation!
  // The hit-tested point in innerNode frame coordinates.
  PhysicalOffset point_in_inner_node_frame_;
  // A point in the local coordinate space of |inner_possibly_pseudo_node_|'s
  // layoutObject, or its containing block when it is an inline object. Allows
  // us to efficiently determine where inside the layoutObject we hit on
  // subsequent operations.
  PhysicalOffset local_point_;
  // For non-URL, this is the enclosing that triggers navigation.
  Member<Element> inner_url_element_;
  Member<Scrollbar> scrollbar_;
  // Returns true if we are over a EmbeddedContentView (and not in the
  // border/padding area of a LayoutEmbeddedContent for example).
  bool is_over_embedded_content_view_;
  // This is true if the location is over the bottom right of a resizable
  // object, where resize controls are located. See
  // PaintLayerScrollableArea::IsAbsolutePointInResizeControl for how that is
  // tested.
  bool is_over_resizer_ = false;

  // Returns true if we are over custom scroll corner
  bool is_over_scroll_corner_ = false;

  mutable Member<NodeSet> list_based_test_result_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::HitTestResult)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_RESULT_H_
