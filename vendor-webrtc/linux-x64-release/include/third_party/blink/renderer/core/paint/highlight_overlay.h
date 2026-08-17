// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIGHLIGHT_OVERLAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIGHLIGHT_OVERLAY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/highlight/highlight_registry.h"
#include "third_party/blink/renderer/core/highlight/highlight_style_utils.h"
#include "third_party/blink/renderer/core/layout/inline/text_offset_range.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class HighlightRegistry;
struct LayoutSelectionStatus;
struct TextFragmentPaintInfo;

class CORE_EXPORT HighlightOverlay {
  STATIC_ONLY(HighlightOverlay);

 public:
  enum class HighlightLayerType : uint8_t {
    kOriginating,
    kCustom,
    kGrammar,
    kSpelling,
    kTargetText,
    kSearchText,
    kSearchTextActiveMatch,
    kSelection,
  };

  enum class HighlightEdgeType : uint8_t { kStart, kEnd };

  // Identifies a highlight layer, such as the originating content, one of the
  // highlight pseudos, or a custom highlight (name unique within a registry).
  struct CORE_EXPORT HighlightLayer {
    DISALLOW_NEW();

   public:
    explicit HighlightLayer(HighlightLayerType type,
                            const AtomicString& name = g_null_atom);

    void Trace(Visitor* visitor) const {
      visitor->Trace(style);
      visitor->Trace(text_style);
    }

    String ToString() const;
    enum PseudoId PseudoId() const;
    const AtomicString& PseudoArgument() const;

    int8_t ComparePaintOrder(const HighlightLayer&,
                             const HighlightRegistry*) const;
    bool operator==(const HighlightLayer&) const;
    bool operator!=(const HighlightLayer&) const;

    HighlightLayerType type;
    Member<const ComputedStyle> style;
    HighlightStyleUtils::HighlightTextPaintStyle text_style;
    TextDecorationLine decorations_in_effect;
    // Constructed from the highlight markers name reference, and the
    // marker always outlives the painter that owns the layers.
    AtomicString name;
  };

  // Represents a range of the fragment, as offsets in canonical text space.
  // More details about canonical text offsets: <https://goo.gl/CJbxky>
  struct CORE_EXPORT HighlightRange {
    DISALLOW_NEW();

   public:
    HighlightRange(unsigned from, unsigned to);

    String ToString() const;

    bool operator==(const HighlightRange&) const;
    bool operator!=(const HighlightRange&) const;

    unsigned from;
    unsigned to;
  };

  // Represents the start or end (indicated by |type|) of a highlighted |range|
  // for the given |layer|. Storing both offsets of the range, rather than just
  // the offset of this edge, allows decorations added by highlights to recover
  // the original range for the purposes of decoration phase and wavelength.
  struct CORE_EXPORT HighlightEdge {
    DISALLOW_NEW();

   public:
    HighlightEdge(HighlightRange range,
                  HighlightLayerType layer_type,
                  uint16_t layer_index,
                  HighlightEdgeType edge_type)
        : range(range),
          layer_index(layer_index),
          layer_type(layer_type),
          edge_type(edge_type) {}

    String ToString() const;
    unsigned Offset() const;

    // Order by offset asc, then “end” edges first, then by layer paint order.
    // ComputeParts requires “end” edges first in case two ranges of the same
    // highlight are immediately adjacent. The opposite would be required for
    // empty highlight ranges, but they’re illegal as per DocumentMarker ctor.
    bool LessThan(const HighlightEdge&,
                  const HeapVector<HighlightLayer>& layers,
                  const HighlightRegistry*) const;
    bool operator==(const HighlightEdge&) const;
    bool operator!=(const HighlightEdge&) const;

    HighlightRange range;
    uint16_t layer_index;
    HighlightLayerType layer_type;
    HighlightEdgeType edge_type;
  };

  // Represents a potential decoration for the given layer type and index that
  // would need to be painted over the given |range|.
  //
  // Note that decorations are painted with this range, but clipped to the range
  // in each HighlightPart, ensuring that decoration phase and wavelength are
  // maintained while allowing them to be recolored or split across layers.
  struct CORE_EXPORT HighlightDecoration {
    DISALLOW_NEW();

   public:
    HighlightDecoration(HighlightLayerType type,
                        uint16_t layer_index,
                        HighlightRange range,
                        Color override_color);

    String ToString() const;

    bool operator==(const HighlightDecoration&) const;
    bool operator!=(const HighlightDecoration&) const;

    HighlightLayerType type;
    uint16_t layer_index;
    HighlightRange range;
    Color highlight_override_color;
  };

  struct CORE_EXPORT HighlightBackground {
    DISALLOW_NEW();

   public:
    String ToString() const;

    bool operator==(const HighlightBackground&) const;
    bool operator!=(const HighlightBackground&) const;

    HighlightLayerType type;
    uint16_t layer_index;
    Color color;
  };

  struct CORE_EXPORT HighlightTextShadow {
    DISALLOW_NEW();

   public:
    String ToString() const;

    bool operator==(const HighlightTextShadow&) const;
    bool operator!=(const HighlightTextShadow&) const;

    HighlightLayerType type;
    uint16_t layer_index;
    Color current_color;
  };

  // Represents a |range| of the fragment that needs its text proper painted in
  // the style of the given topmost layer with the given |decorations|.
  //
  // Note that decorations are clipped to this range, but painted with the range
  // in each HighlightDecoration, ensuring that decoration phase and wavelength
  // are maintained while allowing them to be recolored or split across layers.
  struct CORE_EXPORT HighlightPart {
    DISALLOW_NEW();

   public:
    HighlightPart(HighlightLayerType,
                  uint16_t,
                  HighlightRange,
                  TextPaintStyle,
                  float,
                  Vector<HighlightDecoration>,
                  Vector<HighlightBackground>,
                  Vector<HighlightTextShadow>);
    HighlightPart(HighlightLayerType,
                  uint16_t,
                  HighlightRange,
                  TextPaintStyle,
                  float,
                  Vector<HighlightDecoration>);

    void Trace(Visitor* visitor) const { visitor->Trace(style); }

    String ToString() const;

    bool operator==(const HighlightPart&) const;
    bool operator!=(const HighlightPart&) const;

    HighlightLayerType type;
    uint16_t layer_index;
    HighlightRange range;
    TextPaintStyle style;
    float stroke_width;
    Vector<HighlightDecoration> decorations;
    Vector<HighlightBackground> backgrounds;
    Vector<HighlightTextShadow> text_shadows;
  };

  // Given details of a fragment and how it is highlighted, returns the layers
  // that need to be painted, in overlay painting order.
  static HeapVector<HighlightLayer> ComputeLayers(
      const Document& document,
      Node* node,
      const ComputedStyle& originating_style,
      const TextPaintStyle& originating_text_style,
      const PaintInfo& paint_info,
      const LayoutSelectionStatus* selection,
      const DocumentMarkerVector& custom,
      const DocumentMarkerVector& grammar,
      const DocumentMarkerVector& spelling,
      const DocumentMarkerVector& target,
      const DocumentMarkerVector& search);

  // Given details of a fragment and how it is highlighted, returns the start
  // and end transitions (edges) of the layers, in offset and layer order.
  static Vector<HighlightEdge> ComputeEdges(
      const Node*,
      bool is_generated_text_fragment,
      std::optional<TextOffsetRange> dom_offsets,
      const HeapVector<HighlightLayer>& layers,
      const LayoutSelectionStatus* selection,
      const DocumentMarkerVector& custom,
      const DocumentMarkerVector& grammar,
      const DocumentMarkerVector& spelling,
      const DocumentMarkerVector& target,
      const DocumentMarkerVector& search);

  // Given highlight |layers| and |edges|, returns the ranges of text that can
  // be painted in the same layer with the same decorations, clamping the result
  // to the given |originating| fragment.
  //
  // The edges must not represent overlapping ranges. If the highlight is active
  // in overlapping ranges, those ranges must be merged before ComputeEdges.
  static HeapVector<HighlightPart> ComputeParts(
      const TextFragmentPaintInfo& originating,
      const HeapVector<HighlightLayer>& layers,
      const Vector<HighlightEdge>& edges);
};

CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const HighlightOverlay::HighlightLayer&);
CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const HighlightOverlay::HighlightEdge&);
CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const HighlightOverlay::HighlightPart&);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::HighlightOverlay::HighlightLayer)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::HighlightOverlay::HighlightPart)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIGHLIGHT_OVERLAY_H_
