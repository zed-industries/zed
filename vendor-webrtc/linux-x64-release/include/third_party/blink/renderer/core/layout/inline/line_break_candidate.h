// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BREAK_CANDIDATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BREAK_CANDIDATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_text_index.h"
#include "third_party/blink/renderer/core/layout/inline/line_break_point.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class InlineItem;
class LineBreaker;
class LineInfo;

//
// Represents a break candidate (break opportunity).
//
struct CORE_EXPORT LineBreakCandidate : public LineBreakPoint {
  DISALLOW_NEW();

 public:
  LineBreakCandidate() = default;
  LineBreakCandidate(const InlineItemTextIndex& offset,
                     const InlineItemTextIndex& end,
                     float pos_no_break,
                     float pos_if_break,
                     float penalty = .0f,
                     bool is_hyphenated = false)
      : LineBreakPoint(offset, end, is_hyphenated),
        pos_no_break(pos_no_break),
        pos_if_break(pos_if_break),
        penalty(penalty) {}
  LineBreakCandidate(const InlineItemTextIndex& offset, float position)
      : LineBreakCandidate(offset, offset, position, position) {}

  bool operator==(const LineBreakCandidate& other) const {
    return LineBreakPoint::operator==(other) &&
           pos_no_break == other.pos_no_break &&
           pos_if_break == other.pos_if_break && penalty == other.penalty;
  }

  // The position when the line doesn't break offset the `offset`.
  float pos_no_break = 0;
  // The position if the line breaks here. This is different from `pos_no_break`
  // if there are trailing spaces, kernings, hyphens, etc.
  float pos_if_break = 0;
  // The line break penalty of this candidate.
  float penalty = 0;

  static constexpr wtf_size_t kInlineCapacity = 128;
};

CORE_EXPORT std::ostream& operator<<(std::ostream& ostream,
                                     const LineBreakCandidate& candidate);

//
// A vector of `LineBreakCandidate`.
//
using LineBreakCandidates =
    Vector<LineBreakCandidate, LineBreakCandidate::kInlineCapacity>;

//
// Provides a context for computing `LineBreakCandidate` from multiple
// `LineInfo` and `InlineItemResult`.
//
class CORE_EXPORT LineBreakCandidateContext {
  STACK_ALLOCATED();

 public:
  explicit LineBreakCandidateContext(LineBreakCandidates& candidates)
      : candidates_(candidates) {}

  float HyphenPenalty() const { return hyphen_penalty_; }
  void SetHyphenPenalty(float penalty) { hyphen_penalty_ = penalty; }

  enum State : uint8_t { kBreak, kMidWord };
  State GetState() const { return state_; }

  float Position() const { return position_no_snap_; }
  LayoutUnit SnappedPosition() const {
    return LayoutUnit::FromFloatCeil(position_no_snap_);
  }

  const LineBreakCandidates& Candidates() const { return candidates_; }

  const InlineItem* LastItem() const { return last_item_; }
  wtf_size_t LastEndOffset() const { return last_end_offset_; }
  void SetLast(const InlineItem* item, wtf_size_t offset) {
    last_item_ = item;
    last_end_offset_ = offset;
  }

  // Append a `LineInfo` to this context.
  bool AppendLine(const LineInfo& line_info, LineBreaker& line_breaker);

  // Append a new `LineBreakCandidate`. This modifies the last candidate if
  // `state` is `kMidWord`, instead of adding a new candidate.
  void Append(State new_state,
              InlineItemTextIndex offset,
              InlineItemTextIndex end,
              float pos_no_break,
              float pos_if_break,
              float penalty = .0f,
              bool is_hyphenated = false);
  void Append(State new_state,
              const InlineItemTextIndex& offset,
              float position);
  void AppendTrailingSpaces(State new_state,
                            const InlineItemTextIndex& offset,
                            float pos_no_break);

  // Append the first/last sentinel. `ScoreLineBreaker` requires these two
  // sentinels.
  void EnsureFirstSentinel(const LineInfo& first_line_info);
  void EnsureLastSentinel(const LineInfo& last_line_info);

 private:
  float position_no_snap_ = .0f;
  State state_ = State::kBreak;
  const InlineItem* last_item_ = nullptr;
  wtf_size_t last_end_offset_ = 0;
  float hyphen_penalty_ = .0f;
  LineBreakCandidates& candidates_;

#if EXPENSIVE_DCHECKS_ARE_ON()
  void CheckConsistency() const;

  InlineItemTextIndex first_offset_;
#endif  // EXPENSIVE_DCHECKS_ARE_ON()
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BREAK_CANDIDATE_H_
