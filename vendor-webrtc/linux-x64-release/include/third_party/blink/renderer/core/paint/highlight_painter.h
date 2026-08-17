// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIGHLIGHT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIGHLIGHT_PAINTER_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/frame_selection.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/editing/markers/highlight_pseudo_marker.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/inline/inline_cursor.h"
#include "third_party/blink/renderer/core/layout/inline/text_offset_range.h"
#include "third_party/blink/renderer/core/layout/selection_state.h"
#include "third_party/blink/renderer/core/paint/highlight_overlay.h"
#include "third_party/blink/renderer/core/paint/line_relative_rect.h"
#include "third_party/blink/renderer/core/paint/text_decoration_info.h"
#include "third_party/blink/renderer/core/paint/text_paint_style.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class ComputedStyle;
class FragmentItem;
class FrameSelection;
class LayoutObject;
class Node;
class TextDecorationPainter;
class TextPainter;
struct LayoutSelectionStatus;
struct PaintInfo;
struct TextFragmentPaintInfo;

using HighlightLayer = HighlightOverlay::HighlightLayer;
using HighlightPart = HighlightOverlay::HighlightPart;

// Highlight overlay painter for LayoutNG. Operates on a FragmentItem that
// IsText(). Delegates to TextPainter to paint the text itself.
class CORE_EXPORT HighlightPainter {
  STACK_ALLOCATED();

 public:
  class CORE_EXPORT SelectionPaintState {
    STACK_ALLOCATED();

   public:
    // ComputeSelectionStyle must be called to finish initializing. Until then,
    // only Status() may be called.
    explicit SelectionPaintState(
        const InlineCursor& containing_block,
        const PhysicalOffset& box_offset,
        const std::optional<AffineTransform> writing_mode_rotation = {});
    explicit SelectionPaintState(
        const InlineCursor& containing_block,
        const PhysicalOffset& box_offset,
        const std::optional<AffineTransform> writing_mode_rotation,
        const FrameSelection&);

    const LayoutSelectionStatus& Status() const { return selection_status_; }

    const TextPaintStyle& GetSelectionStyle() const { return selection_style_.style; }

    SelectionState State() const { return state_; }

    bool ShouldPaintSelectedTextOnly() const {
      return paint_selected_text_only_;
    }

    void ComputeSelectionStyle(const Document& document,
                               const ComputedStyle& style,
                               Node* node,
                               const PaintInfo& paint_info,
                               const TextPaintStyle& text_style);

    // When painting text fragments in a vertical writing-mode, we sometimes
    // need to rotate the canvas into a line-relative coordinate space. Paint
    // ops done while rotated need coordinates in this rotated space, but ops
    // done outside of these rotations need the original physical rect.
    const PhysicalRect& PhysicalSelectionRect();
    const LineRelativeRect& LineRelativeSelectionRect();

    void PaintSelectionBackground(
        GraphicsContext& context,
        Node* node,
        const Document& document,
        const ComputedStyle& style,
        const std::optional<AffineTransform>& rotation);

    void PaintSelectedText(TextPainter& text_painter,
                           const TextFragmentPaintInfo&,
                           const TextPaintStyle& text_style,
                           DOMNodeId node_id,
                           const AutoDarkMode& auto_dark_mode);

    void PaintSuppressingTextProperWhereSelected(
        TextPainter& text_painter,
        const TextFragmentPaintInfo&,
        const TextPaintStyle& text_style,
        DOMNodeId node_id,
        const AutoDarkMode& auto_dark_mode);

   private:
    struct SelectionRect {
      PhysicalRect physical;
      LineRelativeRect rotated;
      STACK_ALLOCATED();
    };

    // Lazy init |selection_rect_| only when needed, such as when we need to
    // record selection bounds or actually paint the selection. There are many
    // subtle conditions where we won’t ever need this field.
    void ComputeSelectionRectIfNeeded();

    const LayoutSelectionStatus selection_status_;
    const SelectionState state_;
    const InlineCursor& containing_block_;
    const PhysicalOffset& box_offset_;
    const std::optional<AffineTransform> writing_mode_rotation_;
    std::optional<SelectionRect> selection_rect_;
    HighlightStyleUtils::HighlightTextPaintStyle selection_style_;
    bool paint_selected_text_only_;
  };

  HighlightPainter(const TextFragmentPaintInfo& fragment_paint_info,
                   TextPainter& text_painter,
                   TextDecorationPainter& decoration_painter,
                   const PaintInfo& paint_info,
                   const InlineCursor& cursor,
                   const FragmentItem& fragment_item,
                   const PhysicalOffset& box_origin,
                   const ComputedStyle& style,
                   const TextPaintStyle& text_style,
                   SelectionPaintState*);

  enum Phase { kBackground, kForeground };

  // Paints backgrounds or foregrounds for markers that are not exposed as CSS
  // highlight pseudos. Note that when text is painted here, that text will be
  // painted two or more times, which will create antialiasing errors.
  void PaintNonCssMarkers(Phase phase);

  // Indicates the way this painter should be used by the caller, aside from
  // the Paint method, which should always be used.
  //
  // The full overlay painting algorithm (kOverlay) is not needed when there
  // are no highlights that change the text color, add backgrounds, or add
  // decorations that are required to paint under decorations from earlier
  // layers (e.g. ::target-text underline with originating overline).
  enum Case {
    // Caller should not use this painter. This happens if nothing is
    // highlighted.
    kNoHighlights,
    // Caller should use PaintOriginatingText and PaintHighlightOverlays.
    // This happens if there are highlights that may involve the text
    // fragment, except in some situations with only spelling/grammar
    // (kFastSpellingGrammar) or selection (kFastSelection).
    kOverlay,
    // Caller should use PaintSelectedText only.
    // This happens if ShouldPaintSelectedTextOnly is true, such as when
    // painting the ::selection drag image.
    kSelectionOnly,
    // Caller should use PaintSuppressingTextProperWhereSelected,
    // PaintSelectionBackground, and PaintSelectedText.
    // This happens if the only highlight that may involve the text fragment
    // is a selection, and neither the selection nor the originating content
    // has any decorations.
    kFastSelection,
    // Caller should use FastPaintSpellingGrammarDecorations.
    // This happens if the only highlights that may involve the text fragment
    // are spelling and/or grammar errors, they are completely unstyled (since
    // the default style only adds a spelling or grammar decoration), and the
    // originating content has no decorations.
    kFastSpellingGrammar
  };
  Case PaintCase() const;

  static TextOffsetRange GetFragmentDOMOffsets(const Text& text,
                                               unsigned from,
                                               unsigned to);

  // PaintCase() == kFastSpellingGrammar only
  void FastPaintSpellingGrammarDecorations();

  void PaintOriginatingShadow(const TextPaintStyle&, DOMNodeId);
  void PaintHighlightOverlays(const TextPaintStyle&,
                              DOMNodeId,
                              bool paint_marker_backgrounds,
                              std::optional<AffineTransform> rotation);

  static void PaintHighlightBackground(
      GraphicsContext& context,
      const ComputedStyle& style,
      Color color,
      const PhysicalRect& rect,
      const std::optional<AffineTransform>& rotation);

  // Query various style pieces for the given marker type
  static PseudoId PseudoFor(DocumentMarker::MarkerType type);
  static TextDecorationLine LineFor(DocumentMarker::MarkerType type);
  static Color ColorFor(DocumentMarker::MarkerType type);

  SelectionPaintState* Selection() { return selection_; }

 private:
  struct HighlightEdgeInfo {
    unsigned offset;
    float x;
  };

  Case ComputePaintCase() const;

  const PhysicalRect ComputeBackgroundRect(StringView text,
                                           unsigned start_offset,
                                           unsigned end_offset);
  const PhysicalRect ComputeBackgroundRectForSelection(unsigned start_offset,
                                                       unsigned end_offset);
  void FastPaintSpellingGrammarDecorations(const Text& text_node,
                                           const StringView& text,
                                           const DocumentMarkerVector& markers);
  void PaintOneSpellingGrammarDecoration(DocumentMarker::MarkerType,
                                         const StringView& text,
                                         unsigned paint_start_offset,
                                         unsigned paint_end_offset);
  void PaintOneSpellingGrammarDecoration(
      DocumentMarker::MarkerType,
      const StringView& text,
      unsigned paint_start_offset,
      unsigned paint_end_offset,
      const ComputedStyle& style,
      const TextPaintStyle& text_style,
      const AppliedTextDecoration* decoration_override);
  LineRelativeRect LineRelativeWorldRect(
      const HighlightOverlay::HighlightRange&);
  LineRelativeRect LocalRectInWritingModeSpace(unsigned from,
                                               unsigned to) const;
  void ClipToPartRect(const LineRelativeRect& part_rect);
  void PaintDecorationsExceptLineThrough(const HighlightOverlay::HighlightPart&,
                                         const LineRelativeRect&);
  void PaintDecorationsExceptLineThrough(const HighlightOverlay::HighlightPart&,
                                         const LineRelativeRect&,
                                         TextDecorationLine lines_to_paint);
  void PaintDecorationsOnlyLineThrough(const HighlightOverlay::HighlightPart&,
                                       const LineRelativeRect&);

  // Paints originating text and decorations (again) with the given color.
  // Used for composition markers only.
  void PaintTextForCompositionMarker(const StringView& text,
                                     const Color& text_color,
                                     unsigned paint_start_offset,
                                     unsigned paint_end_offset);

  void PaintBackgroundForGlicMarker(const DocumentMarker* marker,
                                    const StringView& text,
                                    unsigned paint_start_offset,
                                    unsigned paint_end_offset);

  const TextFragmentPaintInfo& fragment_paint_info_;

  // Offsets of the fragment in DOM space, or nullopt if |node_| is not Text or
  // the fragment is generated text (or there are no markers). Used to reject
  // markers outside the target range in dom space, without converting the
  // marker's offsets to the fragment space.
  std::optional<TextOffsetRange> fragment_dom_offsets_{};

  TextPainter& text_painter_;
  TextDecorationPainter& decoration_painter_;
  const PaintInfo& paint_info_;
  const InlineCursor& cursor_;
  InlineCursor root_inline_cursor_;
  const FragmentItem& fragment_item_;
  const PhysicalOffset& box_origin_;
  const ComputedStyle& originating_style_;
  const TextPaintStyle& originating_text_style_;
  SelectionPaintState* selection_;
  const LayoutObject* layout_object_;
  Node* node_;
  const AutoDarkMode foreground_auto_dark_mode_;
  const AutoDarkMode background_auto_dark_mode_;
  DocumentMarkerVector markers_;
  DocumentMarkerVector search_;
  DocumentMarkerVector target_;
  DocumentMarkerVector spelling_;
  DocumentMarkerVector grammar_;
  DocumentMarkerVector custom_;
  HeapVector<HighlightLayer> layers_;
  HeapVector<HighlightPart> parts_;
  Vector<HighlightEdgeInfo> edges_info_;
  Case paint_case_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIGHLIGHT_PAINTER_H_
