// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_STYLE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_STYLE_UTILS_H_

#include <optional>

#include "base/containers/enum_set.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/paint/paint_flags.h"
#include "third_party/blink/renderer/core/paint/text_paint_style.h"
#include "third_party/blink/renderer/core/style/applied_text_decoration.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Color;
class CSSProperty;
class Document;
class ComputedStyle;
class LayoutObject;
class Node;
class Text;
struct PaintInfo;

enum class SearchTextIsActiveMatch : bool {
  kNo,
  kYes,
};

class CORE_EXPORT HighlightStyleUtils {
  STATIC_ONLY(HighlightStyleUtils);

 public:
  // A property that may need to be resolved by ResolveColorsFromPreviousLayer.
  //
  // Note that ‘text-shadow’ is excluded here, because we already have a way to
  // resolve ‘currentColor’ at highlight painting time for it: TextPainter uses
  // TextPaintStyle.current_color to resolve each StyleColor in its ShadowList,
  // and we can wrap any ShadowList in a temporary TextPaintStyle for painting.
  //
  // When adding another property, update HighlightColorPropertySet below.
  enum class HighlightColorProperty : unsigned {
    kCurrentColor,
    kFillColor,
    kEmphasisColor,
    kSelectionDecorationColor,
    kTextDecorationColor,
    kBackgroundColor,
  };
  using HighlightColorPropertySet =
      base::EnumSet<HighlightColorProperty,
                    HighlightColorProperty::kCurrentColor,
                    HighlightColorProperty::kBackgroundColor>;
  struct HighlightTextPaintStyle {
    DISALLOW_NEW();

   public:
    TextPaintStyle style;
    Color text_decoration_color;
    Color background_color;
    HighlightColorPropertySet properties_using_current_color;

    void Trace(Visitor* visitor) const { visitor->Trace(style); }
  };

  static Color ResolveColor(const Document&,
                            const ComputedStyle& originating_style,
                            const ComputedStyle* pseudo_style,
                            PseudoId pseudo,
                            const CSSProperty& property,
                            std::optional<Color> current_color,
                            SearchTextIsActiveMatch);
  static std::optional<Color> MaybeResolveColor(
      const Document&,
      const ComputedStyle& originating_style,
      const ComputedStyle* pseudo_style,
      PseudoId pseudo,
      const CSSProperty& property,
      SearchTextIsActiveMatch);
  static std::optional<AppliedTextDecoration> SelectionTextDecoration(
      const Document& document,
      const ComputedStyle& style,
      const ComputedStyle& pseudo_style);
  static Color HighlightBackgroundColor(const Document&,
                                        const ComputedStyle&,
                                        Node*,
                                        std::optional<Color>,
                                        PseudoId,
                                        SearchTextIsActiveMatch);
  static HighlightTextPaintStyle HighlightPaintingStyle(
      const Document&,
      const ComputedStyle& originating_style,
      const ComputedStyle* pseudo_style,
      Node*,
      PseudoId,
      const TextPaintStyle&,
      const PaintInfo&,
      SearchTextIsActiveMatch);
  static const ComputedStyle* HighlightPseudoStyle(
      Node* node,
      const ComputedStyle& style,
      PseudoId pseudo,
      const AtomicString& pseudo_argument = g_null_atom);

  static void ResolveColorsFromPreviousLayer(
      HighlightTextPaintStyle& text_style,
      const HighlightTextPaintStyle& previous_layer_style);

  static bool ShouldInvalidateVisualOverflow(const LayoutObject& layout_object,
                                             DocumentMarker::MarkerType type);

  static bool CustomHighlightHasVisualOverflow(
      const Text& text_node,
      const AtomicString& pseudo_argument = g_null_atom);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_STYLE_UTILS_H_
