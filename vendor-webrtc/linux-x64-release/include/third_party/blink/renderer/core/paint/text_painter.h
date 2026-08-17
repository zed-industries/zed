// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_PAINTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/style_variant.h"
#include "third_party/blink/renderer/core/paint/line_relative_rect.h"
#include "third_party/blink/renderer/core/paint/paint_flags.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"

namespace blink {

class ComputedStyle;
class Document;
class Font;
class GraphicsContext;
class LayoutObject;
class LayoutSVGInlineText;
class TextDecorationInfo;
enum class TextEmphasisPosition : unsigned;
struct AutoDarkMode;
struct PaintInfo;
struct SvgContextPaints;
struct TextFragmentPaintInfo;
struct TextPaintStyle;

// Base class for text painting. Operates on PhysicalTextFragments and only
// paints text and decorations. Border painting etc is handled by the
// TextFragmentPainter class.
// TODO(layout-dev): Does this distinction make sense?
class CORE_EXPORT TextPainter {
  STACK_ALLOCATED();

 public:
  class SvgTextPaintState final {
    STACK_ALLOCATED();

   public:
    SvgTextPaintState(const LayoutSVGInlineText&,
                      const ComputedStyle&,
                      StyleVariant style_variant,
                      PaintFlags paint_flags);
    SvgTextPaintState(const LayoutSVGInlineText&,
                      const ComputedStyle&,
                      Color text_match_color);

    const LayoutSVGInlineText& InlineText() const;
    const LayoutObject& TextDecorationObject() const;
    const ComputedStyle& Style() const;
    bool IsPaintingSelection() const;
    PaintFlags GetPaintFlags() const;
    bool IsRenderingClipPathAsMaskImage() const;
    bool IsPaintingTextMatch() const;
    // This is callable only if IsPaintingTextMatch().
    Color TextMatchColor() const;

    AffineTransform& EnsureShaderTransform();
    const AffineTransform* GetShaderTransform() const;

   private:
    const LayoutSVGInlineText& layout_svg_inline_text_;
    const ComputedStyle& style_;
    std::optional<AffineTransform> shader_transform_;
    std::optional<Color> text_match_color_;
    StyleVariant style_variant_ = StyleVariant::kStandard;
    PaintFlags paint_flags_ = PaintFlag::kNoFlag;
    bool is_painting_selection_ = false;
    friend class TextPainter;
    friend class HighlightPainter;
  };

  TextPainter(GraphicsContext& context,
              const SvgContextPaints* svg_context_paints,
              const Font& font,
              const gfx::Rect& visual_rect,
              const LineRelativeOffset& text_origin)
      : graphics_context_(context),
        svg_context_paints_(svg_context_paints),
        font_(font),
        visual_rect_(visual_rect),
        text_origin_(text_origin) {}
  ~TextPainter() = default;

  enum ShadowMode { kBothShadowsAndTextProper, kShadowsOnly, kTextProperOnly };
  void Paint(const TextFragmentPaintInfo& fragment_paint_info,
             const TextPaintStyle&,
             DOMNodeId,
             const AutoDarkMode& auto_dark_mode,
             ShadowMode = kBothShadowsAndTextProper);

  void PaintSelectedText(const TextFragmentPaintInfo& fragment_paint_info,
                         unsigned selection_start,
                         unsigned selection_end,
                         const TextPaintStyle& text_style,
                         const TextPaintStyle& selection_style,
                         const LineRelativeRect& selection_rect,
                         DOMNodeId node_id,
                         const AutoDarkMode& auto_dark_mode);

  void PaintDecorationLine(const TextDecorationInfo& decoration_info,
                           const Color& line_color,
                           const TextFragmentPaintInfo* fragment_paint_info);

  SvgTextPaintState& SetSvgState(const LayoutSVGInlineText&,
                                 const ComputedStyle&,
                                 StyleVariant style_variant,
                                 PaintFlags paint_flags);
  SvgTextPaintState& SetSvgState(const LayoutSVGInlineText& svg_inline_text,
                                 const ComputedStyle& style,
                                 Color text_match_color);
  SvgTextPaintState* GetSvgState();

  static Color TextColorForWhiteBackground(Color);

  static TextPaintStyle TextPaintingStyle(const Document&,
                                          const ComputedStyle&,
                                          const PaintInfo&);

  void SetEmphasisMark(const AtomicString&, LineLogicalSide);

 protected:
  const Font& font() const { return font_; }
  const LineRelativeOffset& text_origin() const { return text_origin_; }
  const AtomicString& emphasis_mark() const { return emphasis_mark_; }
  int emphasis_mark_offset() const { return emphasis_mark_offset_; }
  GraphicsContext& graphics_context() const { return graphics_context_; }

 private:
  void PaintSvgTextFragment(const TextFragmentPaintInfo&,
                            DOMNodeId node_id,
                            const AutoDarkMode& auto_dark_mode);

  virtual void ClipDecorationsStripe(const TextFragmentPaintInfo&,
                                     float upper,
                                     float stripe_width,
                                     float dilation);

  GraphicsContext& graphics_context_;
  const SvgContextPaints* svg_context_paints_;
  const Font& font_;
  const gfx::Rect visual_rect_;
  const LineRelativeOffset text_origin_;
  std::optional<SvgTextPaintState> svg_text_paint_state_;
  AtomicString emphasis_mark_;
  int emphasis_mark_offset_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_PAINTER_H_
