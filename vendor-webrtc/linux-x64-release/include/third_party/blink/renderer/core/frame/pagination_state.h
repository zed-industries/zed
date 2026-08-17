// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGINATION_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGINATION_STATE_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class ClipPaintPropertyNodeOrAlias;
class ComputedStyle;
class Document;
class LayoutBlockFlow;
class LayoutObject;
class LayoutView;
class ObjectPaintProperties;
class PropertyTreeState;
class TransformPaintPropertyNodeOrAlias;

// Holds structures and state needed during pagination (layout and painting).
//
// Created before layout for pagination, and destroyed when leaving paginated
// layout.
//
// For each page box that is being laid out, layout objects will be created for
// each part of a page box that may need to paint something (page container,
// page border box, and any @page margins).
//
// PaginationState also keeps track of the current page to print, and the paint
// properties (translation, scaling and clipping) that are to be applied to the
// document contents. Nothing outside the page area (@page margin, border and
// padding) is affected by these properties.
//
// The paint properties are placed in the tree between the properties of the
// LayoutView and its children. They are created during pre-paint, and updated
// for each page printed. They affect all paginated content, but not things
// outside the page area, such as @page decorations / margins.
class PaginationState : public GarbageCollected<PaginationState> {
 public:
  PaginationState();

  void Trace(Visitor*) const;

  // Create an anonymous layout object to measure, lay out and paint some part
  // of a page box. A page box (and its margins) does not exist in the DOM, but
  // is created by layout. Such layout objects will not be attached to the
  // layout object tree. PaginationState "owns" the layout objects returned from
  // this function, so that they are destroyed in a controlled manner (layout
  // objects cannot simply be garbage collected).
  LayoutBlockFlow* CreateAnonymousPageLayoutObject(Document&,
                                                   const ComputedStyle&);

  // Destroy all layout objects created and returned from
  // CreateAnonymousPageLayoutObject().
  void DestroyAnonymousPageLayoutObjects();

  void SetCurrentPageIndex(wtf_size_t n) { current_page_index_ = n; }
  wtf_size_t CurrentPageIndex() const { return current_page_index_; }

  ObjectPaintProperties& ContentAreaProperties() {
    return *content_area_paint_properties_;
  }

  // Make sure that the content area properties have been created, so that they
  // can be inserted into the tree.
  ObjectPaintProperties& EnsureContentAreaProperties(
      const TransformPaintPropertyNodeOrAlias& parent_transform,
      const ClipPaintPropertyNodeOrAlias& parent_clip);

  // Update transform (translation + scaling) and clipping for the current page
  // (see CurrentPageIndex()). This will update the translation into the
  // stitched coordinate system for paginated content (where all pages are
  // imagined to be laid out stacked after oneanother in the block direction, to
  // accommodate for overflow from one page into another). Furthermore, all
  // pages don't necessarily have the same size or margins, so the clip
  // rectangle needs to be updated as well. These properties will be applied to
  // everything inside the page area [1] (but not to e.g. page margins).
  //
  // [1] https://drafts.csswg.org/css-page-3/#page-model
  void UpdateContentAreaPropertiesForCurrentPage(const LayoutView&);

  PropertyTreeState ContentAreaPropertyTreeStateForCurrentPage(
      const LayoutView&) const;

 private:
  HeapVector<Member<LayoutObject>> anonymous_page_objects_;
  Member<ObjectPaintProperties> content_area_paint_properties_;
  wtf_size_t current_page_index_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGINATION_STATE_H_
