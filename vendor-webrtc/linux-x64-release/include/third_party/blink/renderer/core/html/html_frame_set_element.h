/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2004, 2006, 2009, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_FRAME_SET_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_FRAME_SET_ELEMENT_H_

#include <optional>

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/frame/window_event_handlers.h"
#include "third_party/blink/renderer/core/html/html_dimension.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class FrameEdgeInfo;
class MouseEvent;

class HTMLFrameSetElement final : public HTMLElement,
                                  public WindowEventHandlers {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLFrameSetElement(Document&);

  // HTMLElement override
  bool IsHTMLFrameSetElement() const override { return true; }

  bool HasFrameBorder() const;
  bool NoResize() const;

  wtf_size_t TotalRows() const {
    return std::max<wtf_size_t>(1, row_lengths_.size());
  }
  wtf_size_t TotalCols() const {
    return std::max<wtf_size_t>(1, col_lengths_.size());
  }
  int Border(const ComputedStyle& style) const;
  FrameEdgeInfo EdgeInfo() const;
  void DirtyEdgeInfo();
  void DirtyEdgeInfoAndFullPaintInvalidation();

  bool HasBorderColor() const;

  const Vector<HTMLDimension>& RowLengths() const { return row_lengths_; }
  const Vector<HTMLDimension>& ColLengths() const { return col_lengths_; }
  const Vector<int>& RowDeltas() const { return resize_rows_.deltas_; }
  const Vector<int>& ColDeltas() const { return resize_cols_.deltas_; }
  const Vector<bool>& AllowBorderRows() const;
  const Vector<bool>& AllowBorderColumns() const;

  bool HasNonInBodyInsertionMode() const override { return true; }

  bool CanResizeRow(const gfx::Point& p) const;
  bool CanResizeColumn(const gfx::Point& p) const;

  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(blur, kBlur)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(focus, kFocus)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(load, kLoad)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(resize, kResize)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(scroll, kScroll)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(orientationchange, kOrientationchange)

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;

  void AttachLayoutTree(AttachContext&) override;
  bool LayoutObjectIsNeeded(const DisplayStyle&) const override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  void DefaultEventHandler(Event&) override;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void WillRecalcStyle(const StyleRecalcChange) override;

  Document& GetDocumentForWindowEventHandler() const override {
    return GetDocument();
  }

  void ResizeChildrenData();

  class ResizeAxis {
    DISALLOW_NEW();

   public:
    ResizeAxis() = default;
    ResizeAxis(const ResizeAxis&) = delete;
    ResizeAxis& operator=(const ResizeAxis&) = delete;

    void Resize(wtf_size_t number_of_frames);
    // Returns true if a split is being resized now.
    bool IsResizingSplit() const { return split_being_resized_ != kNoSplit; }
    // Returns true if a split is being resized now.
    bool CanResizeSplitAt(int split_index) const;

    static constexpr int kNoSplit = -1;

    Vector<int> deltas_;
    Vector<bool> prevent_resize_;
    int split_being_resized_ = kNoSplit;
    int split_resize_offset_;
  };

  bool UserResize(const MouseEvent& event);
  void SetIsResizing(bool is_resizing);
  void StartResizing(const Vector<LayoutUnit>& sizes,
                     int position,
                     ResizeAxis& resize_axis);
  void ContinueResizing(const Vector<LayoutUnit>& sizes,
                        int position,
                        ResizeAxis& resize_axis);
  int SplitPosition(const Vector<LayoutUnit>& sizes, int split) const;
  int HitTestSplit(const Vector<LayoutUnit>& sizes, int position) const;

  void CollectEdgeInfoIfDirty();
  void FillFromEdgeInfo(const FrameEdgeInfo& edge_info,
                        wtf_size_t r,
                        wtf_size_t c);

  Vector<HTMLDimension> row_lengths_;
  Vector<HTMLDimension> col_lengths_;
  ResizeAxis resize_rows_;
  ResizeAxis resize_cols_;
  Vector<bool> allow_border_rows_;
  Vector<bool> allow_border_cols_;

  std::optional<int> border_;
  std::optional<bool> frameborder_;
  bool is_edge_info_dirty_ = true;
  bool is_resizing_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_FRAME_SET_ELEMENT_H_
