// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_SCORE_LINE_BREAKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_SCORE_LINE_BREAKER_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node.h"
#include "third_party/blink/renderer/core/layout/inline/line_break_candidate.h"
#include "third_party/blink/renderer/core/layout/inline/score_line_break_context.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ConstraintSpace;
class InlineBreakToken;
class InlineNode;
class LineInfoList;
class LineWidths;
struct LeadingFloats;

//
// This class computes line break points using penalties and scores, similar to
// the Knuth's TeX algorithm.
//
// In short, the algorithm works in following steps:
// 1. It runs `LineBreaker` to compute line break points greedy.
// 2. If the result doesn't meet the criteria to apply this score-based line
//    breaking, it returns the result without applying the algorithm.
// 3. It then computes all break candidates (a.k.a., break opportunities) with
//    penalties from the greedy results.
// 4. It then computes the scores for all break candidates.
// 5. The break candidates of the highest score is determined as the line break
//    points.
//
// This algorithm is based on Android's `LineBreak.Strategy.HighQuality`:
// https://cs.android.com/android/platform/superproject/+/master:frameworks/minikin/libs/minikin/OptimalLineBreaker.cpp
//
class CORE_EXPORT ScoreLineBreaker {
  STACK_ALLOCATED();

 public:
  ScoreLineBreaker(const InlineNode& node,
                   const ConstraintSpace& space,
                   const LineWidths& line_widths,
                   const InlineBreakToken* break_token,
                   ExclusionSpace* exclusion_space)
      : node_(node),
        space_(space),
        line_widths_(line_widths),
        exclusion_space_(exclusion_space),
        break_token_(break_token) {
    DCHECK(!node.IsScoreLineBreakDisabled());
  }

  wtf_size_t MaxLines() const {
    return is_balanced_ ? kMaxLinesForBalance : kMaxLinesForOptimal;
  }

  const ConstraintSpace& GetConstraintSpace() const { return space_; }
  const InlineBreakToken* BreakToken() const { return break_token_; }

  // The primary entry point of doing all the work described in the class
  // comment.
  void OptimalBreakPoints(const LeadingFloats& leading_floats,
                          ScoreLineBreakContext& context);

  // Makes the length of all lines balanced, by running the `OptimalBreakPoints`
  // with a higher penalty for the end of the paragraph.
  void BalanceBreakPoints(const LeadingFloats& leading_floats,
                          ScoreLineBreakContext& context);

  void SetScoresOutForTesting(Vector<float>* scores_out);

 private:
  struct LineBreakScore {
    float score = 0;            // best score found for this break
    wtf_size_t prev_index = 0;  // index to previous break
    wtf_size_t line_index = 0;  // the computed line number of the candidate
  };
  using LineBreakScores =
      Vector<LineBreakScore, LineBreakCandidate::kInlineCapacity>;

  const InlineNode& Node() const { return node_; }
  LayoutUnit AvailableWidth(wtf_size_t line_index) const;
  LayoutUnit AvailableWidthToFit(wtf_size_t line_index) const {
    return AvailableWidth(line_index).AddEpsilon();  // Match `LineBreaker`.
  }

  bool Optimize(const LineInfoList& line_info_list,
                LineBreaker& line_breaker,
                LineBreakPoints& break_points);
  bool ComputeCandidates(const LineInfoList& line_info_list,
                         LineBreaker& line_breaker,
                         LineBreakCandidates& candidates);
  void SetupParameters();
  void ComputeLineWidths(const LineInfoList& line_info_list);
  void ComputeScores(const LineBreakCandidates& candidates,
                     LineBreakScores& scores);
  void ComputeBreakPoints(const LineBreakCandidates& candidates,
                          const LineBreakScores& scores,
                          LineBreakPoints& break_points);

  static constexpr float kScoreInfinity = std::numeric_limits<float>::max();
  static constexpr float kScoreOverfull = 1e12f;
  static constexpr float kLastLinePenaltyMultiplier = 4.0f;

  const InlineNode node_;
  const ConstraintSpace& space_;
  const LineWidths& line_widths_;
  ExclusionSpace* exclusion_space_;
  const InlineBreakToken* break_token_;
  LayoutUnit first_line_indent_;
  float hyphen_penalty_ = .0f;
  float line_penalty_ = .0f;
  float zoom_ = .0f;
  bool is_balanced_ = false;
  bool is_justified_ = false;

  Vector<float>* scores_out_for_testing_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_SCORE_LINE_BREAKER_H_
