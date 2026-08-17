/*
 * Copyright (C) 2009 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2009 Antonio Gomes <tonikitoo@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SPATIAL_NAVIGATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SPATIAL_NAVIGATION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"

#include <limits>

namespace blink {

class LocalFrame;
class HTMLAreaElement;
class HTMLFrameOwnerElement;

enum class SpatialNavigationDirection { kNone, kUp, kRight, kDown, kLeft };

constexpr double kMaxDistance = std::numeric_limits<double>::max();

CORE_EXPORT bool IsSpatialNavigationEnabled(const LocalFrame*);

struct CORE_EXPORT FocusCandidate {
  STACK_ALLOCATED();

 public:
  FocusCandidate()
      : visible_node(nullptr), focusable_node(nullptr), is_offscreen(true) {}

  FocusCandidate(Node*, SpatialNavigationDirection);
  explicit FocusCandidate(HTMLAreaElement*, SpatialNavigationDirection);
  bool IsNull() const { return !visible_node; }
  Document* GetDocument() const {
    return visible_node ? &visible_node->GetDocument() : nullptr;
  }

  // We handle differently visibleNode and FocusableNode to properly handle the
  // areas of imagemaps, where visibleNode would represent the image element and
  // focusableNode would represent the area element.  In all other cases,
  // visibleNode and focusableNode are one and the same.
  Node* visible_node;
  Node* focusable_node;
  PhysicalRect rect_in_root_frame;
  bool is_offscreen;
};

CORE_EXPORT bool HasRemoteFrame(const Node*);
CORE_EXPORT int LineBoxes(const LayoutObject& layout_object);
CORE_EXPORT
bool IsFragmentedInline(const LayoutObject& layout_object);
CORE_EXPORT gfx::RectF RectInViewport(const Node&);
CORE_EXPORT bool IsOffscreen(const Node*);
CORE_EXPORT bool IsUnobscured(const FocusCandidate&);
bool ScrollInDirection(Node* container, SpatialNavigationDirection);
// Note this function might trigger UpdateStyleAndLayout.
CORE_EXPORT bool IsScrollableNode(const Node* node);
CORE_EXPORT bool IsScrollableAreaOrDocument(const Node*);
CORE_EXPORT Node* ScrollableAreaOrDocumentOf(Node*);
bool CanScrollInDirection(const Node* container, SpatialNavigationDirection);
bool CanScrollInDirection(const LocalFrame*, SpatialNavigationDirection);

double ComputeDistanceDataForNode(SpatialNavigationDirection,
                                  const FocusCandidate& current_interest,
                                  const FocusCandidate& candidate);
CORE_EXPORT PhysicalRect NodeRectInRootFrame(const Node*);
CORE_EXPORT PhysicalRect OppositeEdge(SpatialNavigationDirection side,
                                      const PhysicalRect& box,
                                      LayoutUnit thickness = LayoutUnit());
CORE_EXPORT PhysicalRect RootViewport(const LocalFrame*);
PhysicalRect StartEdgeForAreaElement(const HTMLAreaElement&,
                                     SpatialNavigationDirection);
HTMLFrameOwnerElement* FrameOwnerElement(const FocusCandidate&);

CORE_EXPORT PhysicalRect
ShrinkInlineBoxToLineBox(const LayoutObject& layout_object,
                         PhysicalRect visible_part,
                         int line_boxes = -1);

CORE_EXPORT PhysicalRect
SearchOriginFragment(const PhysicalRect& visible_part,
                     const LayoutObject& fragmented,
                     const SpatialNavigationDirection direction);
CORE_EXPORT PhysicalRect SearchOrigin(const PhysicalRect&,
                                      Node*,
                                      const SpatialNavigationDirection);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SPATIAL_NAVIGATION_H_
