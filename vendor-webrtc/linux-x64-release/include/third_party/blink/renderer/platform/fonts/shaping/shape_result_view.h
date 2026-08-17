// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_VIEW_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/fonts/shaping/glyph_data.h"
#include "third_party/blink/renderer/platform/fonts/shaping/glyph_data_range.h"
#include "third_party/blink/renderer/platform/fonts/shaping/glyph_offset_array.h"
#include "third_party/blink/renderer/platform/fonts/shaping/shape_result.h"
#include "third_party/blink/renderer/platform/fonts/simple_font_data.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ShapeResult;

// Class representing a read-only composite of views into one or more existing
// shape results.
// Implemented as a list of ref counted RunInfo instances and a start/end
// offset for each, represented using the internal RunInfoPart struct.
// This allows lines to be reference sections of the overall paragraph shape
// results without the memory or computational overhead of a copy.
//
// The example below shows the shape result and the individual lines as
// ShapeResultView instances pointing to the original paragraph results for
// the string "Pack my box with five dozen liquor jugs.":
//  ╔═════════════════════════════════════════════════════╗
//  ║ Paragraph with single run, no re-shaping for lines. ║
//  ╟─────────────────────────────────────────────────────╢
//  ║ runs_ ╭───────────────────────────────────────────╮ ║
//  ║   1:  │ Pack my box with five dozen liquor jugs.  │ ║
//  ║       ╰───────────────────────────────────────────╯ ║
//  ║ lines ╭───────────────────────────────────────────╮ ║
//  ║   1:  │ Pack my box with    -> view, run 1:  0-16 │ ║
//  ║   2:  │ five dozen liquor   -> view, run 1: 17-34 │ ║
//  ║   3:  │ jugs.               -> view, run 1: 35-40 │ ║
//  ║       ╰───────────────────────────────────────────╯ ║
//  ╚═════════════════════════════════════════════════════╝
//
// In cases where a portion of the line needs re-shaping the new results are
// added as separate runs at the beginning and/or end of the runs_ vector with a
// reference to zero or more sub-runs in the middle representing the original
// content that could be reused.
//
// In the example below the end of the first line "Jack!" needs to be re-shaped:
//  ╔═════════════════════════════════════════════════════╗
//  ║ Paragraph with single run, requiring re-shape.      ║
//  ╟─────────────────────────────────────────────────────╢
//  ║ runs_ ╭───────────────────────────────────────────╮ ║
//  ║   1:  │ "Now fax quiz Jack!" my brave ghost pled. │ ║
//  ║       ╰───────────────────────────────────────────╯ ║
//  ║ lines ╭───────────────────────────────────────────╮ ║
//  ║   1:  │ "Now fax quiz     -> view, run 1:  0-14   │ ║
//  ║   1:  │ Jack!             -> new result/run       │ ║
//  ║   2:  │ my brave ghost    -> view, run 1: 21-35   │ ║
//  ║   3:  │ pled.             -> view, run 1: 41-36   │ ║
//  ║       ╰───────────────────────────────────────────╯ ║
//  ╚═════════════════════════════════════════════════════╝
//
// In this case the beginning of the first line would be represented as a part
// referencing a range in the original ShapeResult while the last word
// would be a separate result owned by the ShapeResultView instance. The second
// and third lines would again be represented as parts.
class PLATFORM_EXPORT ShapeResultView final
    : public GarbageCollected<ShapeResultView> {
 public:
  // Create a new ShapeResultView from a pre-defined list of segments.
  // The segments list is assumed to be in logical order.
  struct Segment {
    STACK_ALLOCATED();

   public:
    Segment() = default;
    Segment(const ShapeResult* result, unsigned start_index, unsigned end_index)
        : result(result),
          view(nullptr),
          start_index(start_index),
          end_index(end_index) {}
    Segment(const ShapeResultView* view,
            unsigned start_index,
            unsigned end_index)
        : result(nullptr),
          view(view),
          start_index(start_index),
          end_index(end_index) {}
    const ShapeResult* result;
    const ShapeResultView* view;
    unsigned start_index;
    unsigned end_index;
  };
  static ShapeResultView* Create(base::span<const Segment> segments);

  // Creates a new ShapeResultView from a single segment.
  static ShapeResultView* Create(const ShapeResult*);
  static ShapeResultView* Create(const ShapeResult*,
                                 unsigned start_index,
                                 unsigned end_index);
  static ShapeResultView* Create(const ShapeResultView*,
                                 unsigned start_index,
                                 unsigned end_index);

  struct InitData;
  explicit ShapeResultView(const InitData& data);
  ShapeResultView(const ShapeResultView&) = delete;
  ShapeResultView& operator=(const ShapeResultView&) = delete;
  ~ShapeResultView() = default;

  void Trace(Visitor* visitor) const { visitor->Trace(parts_); }

  ShapeResult* CreateShapeResult() const;

  unsigned StartIndex() const { return start_index_ + char_index_offset_; }
  unsigned EndIndex() const { return StartIndex() + num_characters_; }
  unsigned NumCharacters() const { return num_characters_; }
  float Width() const { return width_; }
  LayoutUnit SnappedWidth() const { return LayoutUnit::FromFloatCeil(width_); }
  TextDirection Direction() const {
    return static_cast<TextDirection>(direction_);
  }
  bool IsLtr() const { return blink::IsLtr(Direction()); }
  bool IsRtl() const { return blink::IsRtl(Direction()); }
  bool HasVerticalOffsets() const { return has_vertical_offsets_; }

  unsigned NumGlyphs() const;
  HeapHashSet<Member<const SimpleFontData>> UsedFonts() const;

  unsigned PreviousSafeToBreakOffset(unsigned index) const;

  float ForEachGlyph(float initial_advance, GlyphCallback, void* context) const;
  float ForEachGlyph(float initial_advance,
                     unsigned from,
                     unsigned to,
                     unsigned index_offset,
                     GlyphCallback,
                     void* context) const;

  float ForEachGraphemeClusters(const StringView& text,
                                float initial_advance,
                                unsigned from,
                                unsigned to,
                                unsigned index_offset,
                                GraphemeClusterCallback,
                                void* context) const;

  // Computes and returns the ink bounds (or visual overflow rect). This is
  // quite expensive and involves measuring each glyph and accumulating the
  // bounds.
  gfx::RectF ComputeInkBounds() const;

  void GetRunFontData(HeapVector<ShapeResult::RunFontData>*) const;

  void ExpandRangeToIncludePartialGlyphs(unsigned* from, unsigned* to) const;

  struct RunInfoPart {
    DISALLOW_NEW();

   public:
    RunInfoPart(const ShapeResultRun* run,
                GlyphDataRange range,
                unsigned start_index,
                unsigned offset,
                unsigned num_characters,
                float width);

    PLATFORM_EXPORT void Trace(Visitor*) const;

    using const_iterator = const HarfBuzzRunGlyphData*;
    const_iterator begin() const { return range_.begin(); }
    const_iterator end() const { return range_.end(); }
    using const_reverse_iterator = std::reverse_iterator<const_iterator>;
    const_reverse_iterator rbegin() const {
      return const_reverse_iterator(end());
    }
    const_reverse_iterator rend() const {
      return const_reverse_iterator(begin());
    }
    const HarfBuzzRunGlyphData& GlyphAt(unsigned index) const {
      return *(UNSAFE_TODO(range_.begin() + index));
    }
    template <bool has_non_zero_glyph_offsets>
    GlyphOffsetArray::iterator<has_non_zero_glyph_offsets> GetGlyphOffsets()
        const {
      return GlyphOffsetArray::iterator<has_non_zero_glyph_offsets>(range_);
    }
    bool HasGlyphOffsets() const { return range_.HasOffsets(); }
    // The end character index of |this| without considering offsets in
    // |ShapeResultView|. This is analogous to:
    //   GlyphAt(IsRtl() ? -1 : NumGlyphs()).character_index
    // if such |HarfBuzzRunGlyphData| is available.
    unsigned CharacterIndexOfEndGlyph() const {
      return num_characters_ + offset_;
    }

    unsigned NumCharacters() const { return num_characters_; }
    unsigned NumGlyphs() const { return range_.size(); }
    float Width() const { return width_; }

    unsigned PreviousSafeToBreakOffset(unsigned offset) const;

    // Common signatures with RunInfo, to templatize algorithms.
    const ShapeResultRun* GetRunInfo() const { return run_.Get(); }
    const GlyphDataRange& GetGlyphDataRange() const { return range_; }
    GlyphDataRange FindGlyphDataRange(unsigned start_character_index,
                                      unsigned end_character_index) const;
    unsigned OffsetToRunStartIndex() const { return offset_; }

    // The helper function for implementing |PopulateRunInfoParts()| for
    // handling iterating over |Vector<scoped_refptr<RunInfo>>| and
    // |base::span<RunInfoPart>|.
    const RunInfoPart* Get() const { return this; }

    template <typename RunType, typename ShapeResultType>
    static unsigned ComputeStart(const RunType& run,
                                 const ShapeResultType& result) {
      const unsigned part_start =
          run.start_index_ + result.StartIndexOffsetForRun();
      if (result.IsLtr()) {
        return part_start;
      }
      // Under RTL and multiple parts, A RunInfoPart may have an
      // offset_ greater than start_index. In this case, run_start
      // would result in an invalid negative value.
      return std::max(part_start, run.OffsetToRunStartIndex());
    }

    template <typename RunType, typename ShapeResultType>
    static std::optional<std::pair<unsigned, unsigned>> ComputeStartEnd(
        const RunType& run,
        const ShapeResultType& result,
        const Segment& segment) {
      if (!run.GetRunInfo()) {
        return std::nullopt;
      }
      const unsigned part_start = ComputeStart(run, result);
      if (segment.end_index <= part_start) {
        return std::nullopt;
      }
      if (!run.num_characters_) {
        return {{part_start, part_start}};
      }
      const unsigned part_end = part_start + run.num_characters_;
      if (segment.start_index >= part_end) {
        return std::nullopt;
      }
      return {{part_start, part_end}};
    }

    Member<const ShapeResultRun> run_;
    GlyphDataRange range_;

    // Start index for partial run, adjusted to ensure that runs are continuous.
    unsigned start_index_;

    // Offset relative to start index for the original run.
    unsigned offset_;

    unsigned num_characters_;
    float width_;
  };

 private:
  void PopulateRunInfoParts(const Segment& segment);

  // Populates `parts_` and accumulates `num_characters_`, and `width_` from
  // runs in `result`.
  template <class ShapeResultType>
  void PopulateRunInfoParts(const ShapeResultType& result,
                            const Segment& segment);

  unsigned CharacterIndexOffsetForGlyphData(const RunInfoPart&) const;

  template <bool is_horizontal_run, bool has_glyph_offsets>
  void ComputePartInkBounds(const ShapeResultView::RunInfoPart&,
                            float run_advance,
                            gfx::RectF* ink_bounds) const;

  template <bool is_horizontal_run, bool has_glyph_offsets>
  void ComputePartInkBoundsScalar(const ShapeResultView::RunInfoPart&,
                                  float run_advance,
                                  gfx::RectF* ink_bounds) const;
#if defined(USE_SIMD_FOR_COMPUTING_GLYPH_BOUNDS)
  template <bool is_horizontal_run, bool has_non_zero_glyph_offsets>
  void ComputePartInkBoundsVectorized(const ShapeResultView::RunInfoPart&,
                                      float run_advance,
                                      gfx::RectF* ink_bounds) const;
#endif  // defined(USE_SIMD_FOR_COMPUTING_GLYPH_BOUNDS)

  // Common signatures with ShapeResult, to templatize algorithms.
  base::span<const RunInfoPart> RunsOrParts() const { return parts_; }

  unsigned StartIndexOffsetForRun() const { return char_index_offset_; }

  HeapVector<RunInfoPart, 1> parts_;

  const unsigned start_index_;

  // Once `parts_` is populated `width_` and `num_characters_` are immutable.
  float width_ = 0;
  unsigned num_characters_ : 30 = 0;

  // Overall direction for the TextRun, dictates which order each individual
  // sub run (represented by RunInfo structs in the m_runs vector) can
  // have a different text direction.
  const unsigned direction_ : 1;

  // Tracks whether any runs contain glyphs with a y-offset != 0.
  const unsigned has_vertical_offsets_ : 1;

  // Offset of the first component added to the view. Used for compatibility
  // with ShapeResult::SubRange
  const unsigned char_index_offset_;

 private:
  friend class ShapeResult;

  template <bool has_glyph_offsets>
  float ForEachGlyphImpl(float initial_advance,
                         GlyphCallback,
                         void* context,
                         const RunInfoPart& part) const;

  template <bool has_glyph_offsets>
  float ForEachGlyphImpl(float initial_advance,
                         unsigned from,
                         unsigned to,
                         unsigned index_offset,
                         GlyphCallback,
                         void* context,
                         const RunInfoPart& part) const;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::ShapeResultView::RunInfoPart)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_SHAPE_RESULT_VIEW_H_
