// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGE_CONTAINER_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGE_CONTAINER_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/counters_attachment_context.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"

namespace blink {

class BlockBreakToken;
struct PageAreaLayoutParams;

// Algorithm that generates a fragment for a page container, which is
// essentially the containing block of a page (we could refer to it the "margin
// box" of the page, but that would be confusing, since the spec defines up to
// 16 "margin boxes" per page, to hold things like author-generated headers and
// footers).
//
// Inside a page container there are @page margins, borders and padding, and the
// "content box" inside defines the page area, into which fragmented document
// content flows.
//
// See https://drafts.csswg.org/css-page-3/#page-model
//
// The spec has the concept of a "page box". To implement this concept, we
// create two fragments. The page container is the outermost one. In addition to
// any "page margin boxes", the page container contains the other part that
// comprisies the "page box", namely the page border box.
//
// If the destination is an actual printer (and not PDF), The size of the page
// container will always match the selected paper size (whatever @page size
// dictates will be honored by layout, but then scaled down and centered to fit
// on paper).
class CORE_EXPORT PageContainerLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  PageContainerLayoutAlgorithm(
      const LayoutAlgorithmParams& params,
      wtf_size_t page_index,
      wtf_size_t total_page_count,
      const AtomicString& page_name,
      const BlockNode& content_node,
      const CountersAttachmentContext&,
      const PageAreaLayoutParams&,
      bool ignore_author_page_style,
      const PhysicalBoxFragment* existing_page_container);

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&) {
    NOTREACHED();
  }

  // Return the outgoing break token from the fragmentainer (page area).
  const BlockBreakToken* FragmentainerBreakToken() const {
    return fragmentainer_break_token_;
  }

  const CountersAttachmentContext& GetCountersContext() const {
    return counters_context_;
  }

  bool NeedsTotalPageCount() const { return needs_total_page_count_; }

 private:
  enum ProgressionDirection {
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop
  };
  static bool IsHorizontal(ProgressionDirection dir) {
    return dir == LeftToRight || dir == RightToLeft;
  }
  static bool IsReverse(ProgressionDirection dir) {
    return dir == RightToLeft || dir == BottomToTop;
  }

  enum EdgeMarginType {
    StartMarginBox = 0,
    CenterMarginBox = 1,
    EndMarginBox = 2,

    // Used when resolving auto-sized center/middle boxes.
    FirstResolvee = 0,
    NonResolvee = 1,
    SecondResolvee = 2
  };

  class PreferredSizeInfo {
   public:
    PreferredSizeInfo() = default;
    PreferredSizeInfo(MinMaxSizes min_max, LayoutUnit margin_sum, bool is_auto)
        : min_max(min_max), margin_sum(margin_sum), is_auto(is_auto) {}

    LayoutUnit MinLength() const { return min_max.min_size + margin_sum; }
    LayoutUnit MaxLength() const { return min_max.max_size + margin_sum; }
    LayoutUnit Length() const {
      DCHECK_EQ(min_max.min_size, min_max.max_size);
      return MinLength();
    }
    LayoutUnit MarginSum() const { return margin_sum; }
    bool IsAuto() const { return is_auto; }

    PreferredSizeInfo Doubled() const {
      PreferredSizeInfo doubled(*this);
      doubled.min_max.min_size *= 2;
      doubled.min_max.max_size *= 2;
      doubled.margin_sum *= 2;
      return doubled;
    }

   private:
    MinMaxSizes min_max;
    LayoutUnit margin_sum;
    bool is_auto = false;
  };

  void LayoutPageBorderBox(LogicalSize containing_block_size,
                           LogicalOffset target_offset);

  void LayoutAllMarginBoxes(const BoxStrut& logical_margins);

  // Specify which page box edges a page margin box (or group of 3 page margin
  // boxes on one side) is adjacent to. This matters when it comes to resolving
  // auto margins, and also overconstrainedness.
  enum EdgeAdjacencyFlag {
    LeftEdge = 1,
    RightEdge = 2,
    TopEdge = 4,
    BottomEdge = 8,
  };
  typedef int EdgeAdjacency;
  bool IsAtTopEdge(EdgeAdjacency mask) const { return mask & TopEdge; }
  bool IsAtLeftEdge(EdgeAdjacency mask) const { return mask & LeftEdge; }
  bool IsAtHorizontalEdge(EdgeAdjacency mask) const {
    return mask & (LeftEdge | RightEdge);
  }
  bool IsAtVerticalEdge(EdgeAdjacency mask) const {
    return mask & (TopEdge | BottomEdge);
  }

  // Lay out a page margin box in one of the four corners.
  //
  // EdgeAdjacency defines which page edges we're adjacent to. Since this is a
  // corner, two of LeftEdge, RightEdge, TopEdge, and BottomEdge will be set.
  // (e.g. for bottom-right-corner it will be RightEdge | BottomEdge)
  void LayoutCornerMarginNode(const ComputedStyle* corner_style,
                              const PhysicalRect&,
                              EdgeAdjacency);

  // Lay out page margin boxes at one of the four edges.
  //
  // They will be positioned within `edge_rect`. This rectangle also represents
  // the available size as far as percentage resolution and cross axis margins
  // are concerned.
  //
  // EdgeAdjacency defines which page edges we're adjacent to. Since we're
  // laying out edge boxes, one, and only one, of LeftEdge, RightEdge, TopEdge,
  // and BottomEdge will be set.
  void LayoutEdgeMarginNodes(const ComputedStyle* start_box_style,
                             const ComputedStyle* center_box_style,
                             const ComputedStyle* end_box_style,
                             const PhysicalRect& edge_rect,
                             EdgeAdjacency);

  // Create a block node based on computed style for an @page margin box. This
  // includes creating children for the content property. Will return
  // BlockNode(nullptr) if there's nothing to lay out for this margin box,
  // e.g. when there's no content property.
  BlockNode CreateBlockNodeIfNeeded(const ComputedStyle*);

  // Calculate the preferred main size for one of the page margin boxes along
  // one of the four edges.
  PreferredSizeInfo EdgeMarginNodePreferredSize(
      const BlockNode& child,
      LogicalSize containing_block_size,
      ProgressionDirection) const;

  // Based on available size, size properties and intrinsic sizes, calculate the
  // main-axis size for each of the (up to) three page margin boxes along one of
  // the four edges, and place the result in `final_main_axis_sizes`.
  void CalculateEdgeMarginBoxSizes(
      PhysicalSize available_physical_size,
      const std::array<BlockNode, 3>& nodes,
      ProgressionDirection,
      std::array<LayoutUnit, 3>& final_main_axis_sizes) const;

  // Resolve at most two auto size values. The first and last entry in
  // preferred_main_axis_sizes may be auto. The one in the middle is required to
  // be non-auto.
  static void ResolveTwoEdgeMarginBoxLengths(
      const std::array<PreferredSizeInfo, 3>& preferred_main_axis_sizes,
      LayoutUnit available_main_axis_size,
      LayoutUnit* first_main_axis_size,
      LayoutUnit* second_main_axis_size);

  // Lay out one of the (up to) three page margin box nodes at one of the four
  // edges. It will be positioned within `edge_rect`. This rectangle also
  // represents the available size as far as percentage resolution and cross
  // axis margins are concerned. The main axis size (width for margins at
  // top/bottom edges, and height for left/right edges) has already been
  // calculated at this point.
  void LayoutEdgeMarginNode(const BlockNode& child,
                            const PhysicalRect& edge_rect,
                            LayoutUnit main_axis_size,
                            EdgeMarginType,
                            EdgeAdjacency,
                            ProgressionDirection);

  // Resolve margins after layout. This includes adding auto margins to resolve
  // any overconstrainedness.
  PhysicalBoxStrut ResolveMargins(const ConstraintSpace& child_space,
                                  const ComputedStyle& child_style,
                                  PhysicalSize child_size,
                                  PhysicalSize available_size,
                                  EdgeAdjacency) const;

  // The current page being laid out.
  wtf_size_t page_index_;

  // The total number of pages in the document. This number isn't known until
  // all pages have been laid out, and will be 0 in the first pass. If anyone
  // actually needs to know the total page count, we need to come back for
  // another layout pass, with this value set correctly.
  wtf_size_t total_page_count_;

  const AtomicString& page_name_;
  const BlockNode& content_node_;
  CountersAttachmentContext counters_context_;
  const PageAreaLayoutParams& page_area_params_;
  bool ignore_author_page_style_;
  bool needs_total_page_count_ = false;

  // Page container fragment from a previous layout pass. It's possible to
  // re-use parts of it if we get a second layout pass, which happens if we need
  // to output the total number of pages on at least one of the pages.
  const PhysicalBoxFragment* existing_page_container_ = nullptr;

  const BlockBreakToken* fragmentainer_break_token_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGE_CONTAINER_LAYOUT_ALGORITHM_H_
