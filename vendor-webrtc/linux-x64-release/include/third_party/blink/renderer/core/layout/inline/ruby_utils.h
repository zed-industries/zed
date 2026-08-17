// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_RUBY_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_RUBY_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/fonts/font_height.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"

namespace blink {

class ComputedStyle;
class InlineItem;
class LineInfo;
class LogicalLineContainer;
class LogicalLineItems;
struct InlineItemResult;
struct LogicalRubyColumn;

struct RubyItemIndexes {
  // Points a kOpenRubyColumn item.
  wtf_size_t column_start;
  // Points a kOpenTag for <rt> item or a kCloseRubyColumn item.
  wtf_size_t base_end;
  // Points a kOpenTag for <rt> item, or WTF::kNotFound.
  wtf_size_t annotation_start;
  // Points a kCloseRubyColumn item.
  wtf_size_t column_end;
};

// Get item indexes for a ruby column starting at `start_item_index`.
// `start_item_index` must point to kOpenRubyColumn item.
RubyItemIndexes ParseRubyInInlineItems(
    const HeapVector<Member<InlineItem>>& items,
    wtf_size_t start_item_index);

struct AnnotationOverhang {
  LayoutUnit start;
  LayoutUnit end;
};

// Returns overhang values of the specified InlineItemResult representing
// a ruby column.
//
// This is used by LineBreaker.
AnnotationOverhang GetOverhang(const InlineItemResult& item);

// Returns overhang values of the specified base/annotation lines.
// These lines should have correct LineStyle.
//
// This is used by LineBreaker.
AnnotationOverhang GetOverhang(
    LayoutUnit ruby_size,
    const LineInfo& base_line,
    const HeapVector<LineInfo, 1> annotation_line_list);

// Returns true if |start_overhang| is applied to a previous item, and
// clamp |start_overhang| to the width of the previous item.
//
// This is used by LineBreaker.
bool CanApplyStartOverhang(const LineInfo& line_info,
                           wtf_size_t ruby_index,
                           const ComputedStyle& ruby_style,
                           LayoutUnit& start_overhang);

// This should be called before a text `InlineItem` is added in
// LineBreaker::HandleText().
//
// This function may update a InlineItemResult representing RubyColumn
// in |line_info|
LayoutUnit CommitPendingEndOverhang(const InlineItem& text_item,
                                    LineInfo* line_info);

// Justify InlineItemResutls of the specified `line_info`.
// Returns a pair of the left and the right insets.  They should be applied
// to LogicalLineItems generated from `line_info` after bidi reorder.
[[nodiscard]] std::pair<LayoutUnit, LayoutUnit> ApplyRubyAlign(
    LayoutUnit available_line_size,
    bool on_start_edge,
    bool on_end_edge,
    LineInfo& line_info);

// Stores ComputeAnnotationOverflow() results.
//
// |overflow_over| and |space_over| are exclusive. Only one of them can be
// non-zero. |overflow_under| and |space_under| are exclusive too.
// All fields never be negative.
struct AnnotationMetrics {
  // The amount of annotation overflow at the line-over side.
  LayoutUnit overflow_over;
  // The amount of annotation overflow at the line-under side.
  LayoutUnit overflow_under;
  // The amount of annotation space which the next line at the line-over
  // side can consume.
  LayoutUnit space_over;
  // The amount of annotation space which the next line at the line-under
  // side can consume.
  LayoutUnit space_under;
};

// Compute over/under annotation overflow/space for the specified line.
AnnotationMetrics ComputeAnnotationOverflow(
    const LogicalLineItems& logical_line,
    const FontHeight& line_box_metrics,
    const ComputedStyle& line_style,
    std::optional<FontHeight> annotation_metrics);

// Update inline positions of LogicalLineItems for all LogicalRubyColumns
// linked from `column_list`.
void UpdateRubyColumnInlinePositions(
    const LogicalLineItems& line_items,
    LayoutUnit inline_size,
    HeapVector<Member<LogicalRubyColumn>>& column_list);

// This class calculates block positions of annotation lines on the base line.
class CORE_EXPORT RubyBlockPositionCalculator {
  STACK_ALLOCATED();

 public:
  // RubyLevel represents a ruby base/annotation level.  e.g.
  //  The base -> []
  //  The first annotation over the base -> [1]
  //  The second annotation under the base -> [-2]
  //  The first annotation under the first annotation over the base -> [1, -1]
  //   <ruby>[]<rt><ruby ruby-position:under>[1]<rt>[1, -1]</ruby></ruby>
  //   <ruby ruby-position:under><ruby
  //   ruby-position:under>[]<rt>[-1]</ruby><rt>[-2]</ruby>
  using RubyLevel = Vector<int32_t>;

  // A RubyLine instance represents a list of ruby annotations in a single
  // annotation level.
  class RubyLine : public GarbageCollected<RubyLine> {
   public:
    explicit RubyLine(const RubyLevel& level);
    void Trace(Visitor* visitor) const;

    const RubyLevel& Level() const { return level_; }
    bool IsBaseLevel() const { return level_.empty(); }
    bool IsFirstOverLevel() const {
      return level_.size() == 1u && level_[0] == 1;
    }
    bool IsFirstUnderLevel() const {
      return level_.size() == 1u && level_[0] == -1;
    }
    const Vector<wtf_size_t>& BaseIndexList() const { return base_index_list_; }
    // This operator defines lines below are smaller than lines above.
    bool operator<(const RubyLine& another) const;

    void Append(LogicalRubyColumn& logical_column);
    void MaybeRecordBaseIndexes(const LogicalRubyColumn& logical_column);
    FontHeight UpdateMetrics();
    void MoveInBlockDirection(LayoutUnit offset);
    void AddLinesTo(LogicalLineContainer& line_container) const;

    const HeapVector<Member<LogicalRubyColumn>>& ColumnListForTesting() const {
      return column_list_;
    }

   private:
    RubyLevel level_;

    HeapVector<Member<LogicalRubyColumn>> column_list_;

    // Store base item indexes.  This is available only for the first level
    // annotations of the base level because other levels don't touch the base
    // level.
    Vector<wtf_size_t> base_index_list_;

    FontHeight metrics_ = FontHeight::Empty();
  };

  // Represents the maximum number of over/under annotations attached to the
  // current level. Only HandleRubyLine() uses this class.
  struct AnnotationDepth {
    DISALLOW_NEW();

    Member<LogicalRubyColumn> column;
    // Nesting level on the "over" side. The value is zero or positive.
    int32_t over_depth = 0;
    // Nesting level on the "under" side. The value is zero or negative.
    int32_t under_depth = 0;

    void Trace(Visitor* visitor) const;
  };

  RubyBlockPositionCalculator();

  // Group all LogicalRubyColumn instances linked from `column_list` with the
  // same level. The result is stored in `this`.
  RubyBlockPositionCalculator& GroupLines(
      const HeapVector<Member<LogicalRubyColumn>>& column_list);

  // Update block offset values of annotation LogicalRubyColumns. This must be
  // called after GroupLines().
  RubyBlockPositionCalculator& PlaceLines(
      const LogicalLineItems& base_line_items,
      const FontHeight& line_box_metrics);

  // Associate annotation lines to the specified line container. This must be
  // called after PlaceLines().
  RubyBlockPositionCalculator& AddLinesTo(LogicalLineContainer& line_container);

  // Returns a metrics including all annotation lines. This must be called
  // after PlaceLines().
  FontHeight AnnotationMetrics() const;

  const HeapVector<Member<RubyLine>, 2>& RubyLineListForTesting() const {
    return ruby_lines_;
  }

 private:
  void HandleRubyLine(const RubyLine& current_ruby_line,
                      const HeapVector<Member<LogicalRubyColumn>>& column_list);
  RubyLine& EnsureRubyLine(const RubyLevel& level);

  HeapVector<Member<RubyLine>, 2> ruby_lines_;

  // Annotation distance from the base baseline.  `.ascent` is the top offset
  // of the highest annotation from the base baseline, and `.descent` is the
  // bottom offset of the lowest annotation from the base baseline.  They are
  // zero if there are no higher/lower annotations.  This is available after
  // PlaceLines().
  FontHeight annotation_metrics_ = FontHeight::Empty();
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::RubyBlockPositionCalculator::AnnotationDepth)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_RUBY_UTILS_H_
