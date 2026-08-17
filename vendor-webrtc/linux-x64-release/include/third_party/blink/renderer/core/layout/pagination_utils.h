// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGINATION_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGINATION_UTILS_H_

#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class BlockBreakToken;
class BlockNode;
class ConstraintSpaceBuilder;
class ComputedStyle;
class Document;
class LayoutView;
class PhysicalBoxFragment;
struct BoxStrut;
struct FragmentGeometry;
struct LogicalRect;
struct PhysicalFragmentLink;
struct PhysicalRect;
struct WebPrintPageDescription;

void SetUpSpaceBuilderForPageBox(LogicalSize available_size,
                                 ConstraintSpaceBuilder*);

// Return the @page containing block size. If no size is specified in @page, the
// size passed from the print settings will be used.
LogicalSize DesiredPageContainingBlockSize(const Document&,
                                           const ComputedStyle&);

// Calculate the FragmentGeometry for the given page container or page border
// box, and optionally its margins. page_containing_block_size is typically the
// @page size returned from DesiredPageContainingBlockSize().
void ResolvePageBoxGeometry(const BlockNode& page_box,
                            LogicalSize page_containing_block_size,
                            FragmentGeometry*,
                            BoxStrut* margins = nullptr);

// Calculate the initial containing block size to use when paginating (to be
// used by viewport units, out-of-flow positioning, etc.). This is defined as
// the page area of the first page:
// https://drafts.csswg.org/css-page-3/#page-model
PhysicalSize CalculateInitialContainingBlockSizeForPagination(Document&);

// Return the scale factor to use when scaling paginated content (and the page
// border box) from layout to target to the target output. Layout may use a
// different viewport size than the requested page size because of a scale
// factor in the print parameters, or in order to fit more unbreakable content
// in the inline direction. Additionally, if the target is actual paper, it may
// be necessary to scale everything down to fit within the given paper size.
float TargetScaleForPage(const PhysicalBoxFragment& page_container);

// Fit the page margin-box size to paper / printable area, if needed. The input
// size is the desired page box size (from print parameters and @page
// properties). The output size will be the same if not fitting to paper. If
// fitting to paper, it will be paper size, but still honoring the orientation
// of the desired page box size.
LogicalSize FittedPageContainerSize(const Document& document,
                                    const ComputedStyle& style,
                                    LogicalSize source_margin_box_size);

// Calculate the page border-box rectangle in the target coordinate system
// (which fits on paper, if needed).
LogicalRect TargetPageBorderBoxLogicalRect(
    const Document& document,
    const ComputedStyle& page_style,
    const LogicalSize& source_margin_box_size,
    const BoxStrut& margins);

// Return the total number of pages. Only to be called on a document that has
// been laid out for pagination.
wtf_size_t PageCount(const LayoutView& view);

// Get the page container (BoxType::kPageContainer) for a given page.
const PhysicalBoxFragment* GetPageContainer(const LayoutView&,
                                            wtf_size_t page_number);

// Get the page area (BoxType::kPageArea) for a given page.
const PhysicalBoxFragment* GetPageArea(const LayoutView&,
                                       wtf_size_t page_number);

// Get the page border box (BoxType::kPageBorderBox) child of a page container.
const PhysicalFragmentLink& GetPageBorderBoxLink(
    const PhysicalBoxFragment& page_container);
const PhysicalBoxFragment& GetPageBorderBox(
    const PhysicalBoxFragment& page_container);

// Get the page area (BoxType::kPageArea) child of a page border box.
const PhysicalBoxFragment& GetPageArea(
    const PhysicalBoxFragment& page_border_box);

// Return the page rectangle at the specified index in the stitched coordinate
// system.
PhysicalRect StitchedPageContentRect(const LayoutView&, wtf_size_t page_number);

// Return the page rectangle of the page area inside the specified container in
// the stitched coordinate system.
PhysicalRect StitchedPageContentRect(const PhysicalBoxFragment& page_container);

const BlockBreakToken* FindPreviousBreakTokenForPageArea(
    const PhysicalBoxFragment& page_area);

float CalculateOverflowShrinkForPrinting(const LayoutView&,
                                         float maximum_shrink_factor);

// Populate and return a WebPrintPageDescription structure for a given page
// based on layout and style.
///
// If fitting to paper size is enabled (i.e. when using a printer and not just
// generating a PDF), the page box size returned will be the paper size (not
// whatever @page says), and the returned margins may be scaled down, if the
// page box was scaled down as part of the fitting. Such scaling and fitting may
// introduce additional spacing between the page box (container) edge and the
// page border box. This will be included in the margin values.
WebPrintPageDescription GetPageDescriptionFromLayout(const Document&,
                                                     wtf_size_t page_number);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGINATION_UTILS_H_
