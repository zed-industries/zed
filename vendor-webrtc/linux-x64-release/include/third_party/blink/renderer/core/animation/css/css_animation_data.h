// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_DATA_H_

#include <memory>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/animation/css/css_timing_data.h"
#include "third_party/blink/renderer/core/animation/effect_model.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_name_or_keyword.h"
#include "third_party/blink/renderer/core/style/style_timeline.h"

namespace blink {

class CORE_EXPORT CSSAnimationData final : public CSSTimingData {
 public:
  CSSAnimationData();
  explicit CSSAnimationData(const CSSAnimationData&);

  std::unique_ptr<CSSAnimationData> Clone() const {
    return base::WrapUnique(new CSSAnimationData(*this));
  }

  bool AnimationsMatchForStyleRecalc(const CSSAnimationData& other) const;
  bool operator==(const CSSAnimationData& other) const {
    return AnimationsMatchForStyleRecalc(other);
  }

  Timing ConvertToTiming(size_t index) const;
  const StyleTimeline& GetTimeline(size_t index) const;

  const Vector<AtomicString>& NameList() const { return name_list_; }
  const Vector<StyleTimeline>& TimelineList() const { return timeline_list_; }

  const Vector<double>& IterationCountList() const {
    return iteration_count_list_;
  }
  const Vector<Timing::PlaybackDirection>& DirectionList() const {
    return direction_list_;
  }
  const Vector<Timing::FillMode>& FillModeList() const {
    return fill_mode_list_;
  }
  const Vector<EAnimPlayState>& PlayStateList() const {
    return play_state_list_;
  }
  const Vector<std::optional<TimelineOffset>>& RangeStartList() const {
    return range_start_list_;
  }
  const Vector<std::optional<TimelineOffset>>& RangeEndList() const {
    return range_end_list_;
  }

  const Vector<EffectModel::CompositeOperation>& CompositionList() const {
    return composition_list_;
  }
  const Vector<EAnimationTriggerType>& TriggerTypeList() const {
    return trigger_type_list_;
  }
  const Vector<StyleTimeline>& TriggerTimelineList() const {
    return trigger_timeline_list_;
  }
  const StyleTimeline& GetTriggerTimeline(size_t index) const;
  const Vector<std::optional<TimelineOffset>>& TriggerRangeStartList() const {
    return trigger_range_start_list_;
  }
  const Vector<std::optional<TimelineOffset>>& TriggerRangeEndList() const {
    return trigger_range_end_list_;
  }
  const Vector<TimelineOffsetOrAuto>& TriggerExitRangeStartList() const {
    return trigger_exit_range_start_list_;
  }
  const Vector<TimelineOffsetOrAuto>& TriggerExitRangeEndList() const {
    return trigger_exit_range_end_list_;
  }

  EffectModel::CompositeOperation GetComposition(size_t animation_index) const {
    if (!composition_list_.size()) {
      return EffectModel::kCompositeReplace;
    }
    wtf_size_t index = animation_index % composition_list_.size();
    return composition_list_[index];
  }

  Vector<AtomicString>& NameList() { return name_list_; }
  Vector<StyleTimeline>& TimelineList() { return timeline_list_; }
  Vector<double>& IterationCountList() { return iteration_count_list_; }
  Vector<Timing::PlaybackDirection>& DirectionList() { return direction_list_; }
  Vector<Timing::FillMode>& FillModeList() { return fill_mode_list_; }
  Vector<EAnimPlayState>& PlayStateList() { return play_state_list_; }
  Vector<EAnimationTriggerType>& TriggerTypeList() {
    return trigger_type_list_;
  }
  Vector<StyleTimeline>& TriggerTimelineList() {
    return trigger_timeline_list_;
  }

  Vector<std::optional<TimelineOffset>>& RangeStartList() {
    return range_start_list_;
  }
  Vector<std::optional<TimelineOffset>>& RangeEndList() {
    return range_end_list_;
  }
  Vector<EffectModel::CompositeOperation>& CompositionList() {
    return composition_list_;
  }

  Vector<std::optional<TimelineOffset>>& TriggerRangeStartList() {
    return trigger_range_start_list_;
  }
  Vector<std::optional<TimelineOffset>>& TriggerRangeEndList() {
    return trigger_range_end_list_;
  }
  Vector<TimelineOffsetOrAuto>& TriggerExitRangeStartList() {
    return trigger_exit_range_start_list_;
  }
  Vector<TimelineOffsetOrAuto>& TriggerExitRangeEndList() {
    return trigger_exit_range_end_list_;
  }

  bool HasSingleInitialTimeline() const {
    return timeline_list_.size() == 1u &&
           timeline_list_.front() == InitialTimeline();
  }
  bool HasSingleInitialRangeStart() const {
    return range_start_list_.size() == 1u &&
           range_start_list_.front() == InitialRangeStart();
  }
  bool HasSingleInitialRangeEnd() const {
    return range_end_list_.size() == 1u &&
           range_end_list_.front() == InitialRangeEnd();
  }

  static std::optional<double> InitialDuration();
  static const AtomicString& InitialName();
  static const StyleTimeline& InitialTimeline();
  static Timing::PlaybackDirection InitialDirection() {
    return Timing::PlaybackDirection::NORMAL;
  }
  static Timing::FillMode InitialFillMode() { return Timing::FillMode::NONE; }
  static double InitialIterationCount() { return 1.0; }
  static EAnimPlayState InitialPlayState() { return EAnimPlayState::kPlaying; }
  static std::optional<TimelineOffset> InitialRangeStart() {
    return std::nullopt;
  }
  static std::optional<TimelineOffset> InitialRangeEnd() {
    return std::nullopt;
  }
  static EffectModel::CompositeOperation InitialComposition() {
    return EffectModel::CompositeOperation::kCompositeReplace;
  }
  static const StyleTimeline& InitialTriggerTimeline();
  static EAnimationTriggerType InitialTriggerType() {
    return EAnimationTriggerType::kOnce;
  }
  static std::optional<TimelineOffset> InitialTriggerRangeStart() {
    return std::nullopt;
  }
  static std::optional<TimelineOffset> InitialTriggerRangeEnd() {
    return std::nullopt;
  }
  static TimelineOffsetOrAuto InitialTriggerExitRangeStart() {
    return TimelineOffsetOrAuto();
  }
  static TimelineOffsetOrAuto InitialTriggerExitRangeEnd() {
    return TimelineOffsetOrAuto();
  }

 private:
  Vector<AtomicString> name_list_;
  Vector<StyleTimeline> timeline_list_;
  Vector<std::optional<TimelineOffset>> range_start_list_;
  Vector<std::optional<TimelineOffset>> range_end_list_;
  Vector<double> iteration_count_list_;
  Vector<Timing::PlaybackDirection> direction_list_;
  Vector<Timing::FillMode> fill_mode_list_;
  Vector<EAnimPlayState> play_state_list_;
  Vector<EffectModel::CompositeOperation> composition_list_;
  Vector<EAnimationTriggerType> trigger_type_list_;
  Vector<StyleTimeline> trigger_timeline_list_;
  Vector<std::optional<TimelineOffset>> trigger_range_start_list_;
  Vector<std::optional<TimelineOffset>> trigger_range_end_list_;
  Vector<TimelineOffsetOrAuto> trigger_exit_range_start_list_;
  Vector<TimelineOffsetOrAuto> trigger_exit_range_end_list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_ANIMATION_DATA_H_
