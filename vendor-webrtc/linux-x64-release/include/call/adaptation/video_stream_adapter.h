/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_VIDEO_STREAM_ADAPTER_H_
#define CALL_ADAPTATION_VIDEO_STREAM_ADAPTER_H_

#include <cstdint>
#include <optional>
#include <variant>
#include <vector>

#include "api/adaptation/resource.h"
#include "api/field_trials_view.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/video/video_adaptation_counters.h"
#include "api/video_codecs/video_codec.h"
#include "call/adaptation/adaptation_constraint.h"
#include "call/adaptation/video_source_restrictions.h"
#include "call/adaptation/video_stream_input_state.h"
#include "call/adaptation/video_stream_input_state_provider.h"
#include "rtc_base/experiments/balanced_degradation_settings.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "video/video_stream_encoder_observer.h"

namespace webrtc {

// The listener is responsible for carrying out the reconfiguration of the video
// source such that the VideoSourceRestrictions are fulfilled.
class VideoSourceRestrictionsListener {
 public:
  virtual ~VideoSourceRestrictionsListener();

  // The `restrictions` are filtered by degradation preference but not the
  // `adaptation_counters`, which are currently only reported for legacy stats
  // calculation purposes.
  virtual void OnVideoSourceRestrictionsUpdated(
      VideoSourceRestrictions restrictions,
      const VideoAdaptationCounters& adaptation_counters,
      scoped_refptr<Resource> reason,
      const VideoSourceRestrictions& unfiltered_restrictions) = 0;
};

class VideoStreamAdapter;

extern const int kMinFrameRateFps;

VideoSourceRestrictions FilterRestrictionsByDegradationPreference(
    VideoSourceRestrictions source_restrictions,
    DegradationPreference degradation_preference);

int GetLowerResolutionThan(int pixel_count);
int GetHigherResolutionThan(int pixel_count);

// Either represents the next VideoSourceRestrictions the VideoStreamAdapter
// will take, or provides a Status code indicating the reason for not adapting
// if the adaptation is not valid.
class Adaptation final {
 public:
  enum class Status {
    // Applying this adaptation will have an effect. All other Status codes
    // indicate that adaptation is not possible and why.
    kValid,
    // Cannot adapt. The minimum or maximum adaptation has already been reached.
    // There are no more steps to take.
    kLimitReached,
    // Cannot adapt. The resolution or frame rate requested by a recent
    // adaptation has not yet been reflected in the input resolution or frame
    // rate; adaptation is refused to avoid "double-adapting".
    kAwaitingPreviousAdaptation,
    // Not enough input.
    kInsufficientInput,
    // Adaptation disabled via degradation preference.
    kAdaptationDisabled,
    // Adaptation up was rejected by a VideoAdaptationConstraint.
    kRejectedByConstraint,
  };

  static const char* StatusToString(Status status);

  Status status() const;
  const VideoStreamInputState& input_state() const;
  const VideoSourceRestrictions& restrictions() const;
  const VideoAdaptationCounters& counters() const;

 private:
  friend class VideoStreamAdapter;

  // Constructs with a valid adaptation. Status is kValid.
  Adaptation(int validation_id,
             VideoSourceRestrictions restrictions,
             VideoAdaptationCounters counters,
             VideoStreamInputState input_state);
  // Constructor when adaptation is not valid. Status MUST NOT be kValid.
  Adaptation(int validation_id, Status invalid_status);

  // An Adaptation can become invalidated if the state of VideoStreamAdapter is
  // modified before the Adaptation is applied. To guard against this, this ID
  // has to match VideoStreamAdapter::adaptation_validation_id_ when applied.
  // TODO(https://crbug.com/webrtc/11700): Remove the validation_id_.
  const int validation_id_;
  const Status status_;
  // Input state when adaptation was made.
  const VideoStreamInputState input_state_;
  const VideoSourceRestrictions restrictions_;
  const VideoAdaptationCounters counters_;
};

// Owns the VideoSourceRestriction for a single stream and is responsible for
// adapting it up or down when told to do so. This class serves the following
// purposes:
// 1. Keep track of a stream's restrictions.
// 2. Provide valid ways to adapt up or down the stream's restrictions.
// 3. Modify the stream's restrictions in one of the valid ways.
class VideoStreamAdapter {
 public:
  VideoStreamAdapter(VideoStreamInputStateProvider* input_state_provider,
                     VideoStreamEncoderObserver* encoder_stats_observer,
                     const FieldTrialsView& field_trials);
  ~VideoStreamAdapter();

  VideoSourceRestrictions source_restrictions() const;
  const VideoAdaptationCounters& adaptation_counters() const;
  void ClearRestrictions();

  void AddRestrictionsListener(
      VideoSourceRestrictionsListener* restrictions_listener);
  void RemoveRestrictionsListener(
      VideoSourceRestrictionsListener* restrictions_listener);
  void AddAdaptationConstraint(AdaptationConstraint* adaptation_constraint);
  void RemoveAdaptationConstraint(AdaptationConstraint* adaptation_constraint);

  // TODO(hbos): Setting the degradation preference should not clear
  // restrictions! This is not defined in the spec and is unexpected, there is a
  // tiny risk that people would discover and rely on this behavior.
  void SetDegradationPreference(DegradationPreference degradation_preference);

  // Returns an adaptation that we are guaranteed to be able to apply, or a
  // status code indicating the reason why we cannot adapt.
  Adaptation GetAdaptationUp();
  Adaptation GetAdaptationDown();
  Adaptation GetAdaptationTo(const VideoAdaptationCounters& counters,
                             const VideoSourceRestrictions& restrictions);
  // Tries to adapt the resolution one step. This is used for initial frame
  // dropping. Does nothing if the degradation preference is not BALANCED or
  // MAINTAIN_FRAMERATE. In the case of BALANCED, it will try twice to reduce
  // the resolution. If it fails twice it gives up.
  Adaptation GetAdaptDownResolution();

  // Updates source_restrictions() the Adaptation.
  void ApplyAdaptation(const Adaptation& adaptation,
                       scoped_refptr<Resource> resource);

  struct RestrictionsWithCounters {
    VideoSourceRestrictions restrictions;
    VideoAdaptationCounters counters;
  };

  static std::optional<uint32_t> GetSingleActiveLayerPixels(
      const VideoCodec& codec);

 private:
  void BroadcastVideoRestrictionsUpdate(
      const VideoStreamInputState& input_state,
      const scoped_refptr<Resource>& resource);

  bool HasSufficientInputForAdaptation(const VideoStreamInputState& input_state)
      const RTC_RUN_ON(&sequence_checker_);

  using RestrictionsOrState =
      std::variant<RestrictionsWithCounters, Adaptation::Status>;
  RestrictionsOrState GetAdaptationUpStep(
      const VideoStreamInputState& input_state) const
      RTC_RUN_ON(&sequence_checker_);
  RestrictionsOrState GetAdaptationDownStep(
      const VideoStreamInputState& input_state,
      const RestrictionsWithCounters& current_restrictions) const
      RTC_RUN_ON(&sequence_checker_);
  RestrictionsOrState GetAdaptDownResolutionStepForBalanced(
      const VideoStreamInputState& input_state) const
      RTC_RUN_ON(&sequence_checker_);
  RestrictionsOrState AdaptIfFpsDiffInsufficient(
      const VideoStreamInputState& input_state,
      const RestrictionsWithCounters& restrictions) const
      RTC_RUN_ON(&sequence_checker_);

  Adaptation GetAdaptationUp(const VideoStreamInputState& input_state) const
      RTC_RUN_ON(&sequence_checker_);
  Adaptation GetAdaptationDown(const VideoStreamInputState& input_state) const
      RTC_RUN_ON(&sequence_checker_);

  static RestrictionsOrState DecreaseResolution(
      const VideoStreamInputState& input_state,
      const RestrictionsWithCounters& current_restrictions);
  static RestrictionsOrState IncreaseResolution(
      const VideoStreamInputState& input_state,
      const RestrictionsWithCounters& current_restrictions);
  // Framerate methods are member functions because they need internal state
  // if the degradation preference is BALANCED.
  RestrictionsOrState DecreaseFramerate(
      const VideoStreamInputState& input_state,
      const RestrictionsWithCounters& current_restrictions) const
      RTC_RUN_ON(&sequence_checker_);
  RestrictionsOrState IncreaseFramerate(
      const VideoStreamInputState& input_state,
      const RestrictionsWithCounters& current_restrictions) const
      RTC_RUN_ON(&sequence_checker_);

  struct RestrictionsOrStateVisitor;
  Adaptation RestrictionsOrStateToAdaptation(
      RestrictionsOrState step_or_state,
      const VideoStreamInputState& input_state) const
      RTC_RUN_ON(&sequence_checker_);

  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  // Gets the input state which is the basis of all adaptations.
  // Thread safe.
  VideoStreamInputStateProvider* input_state_provider_;
  // Used to signal when min pixel limit has been reached.
  VideoStreamEncoderObserver* const encoder_stats_observer_;
  // Decides the next adaptation target in DegradationPreference::BALANCED.
  const BalancedDegradationSettings balanced_settings_;
  // To guard against applying adaptations that have become invalidated, an
  // Adaptation that is applied has to have a matching validation ID.
  int adaptation_validation_id_ RTC_GUARDED_BY(&sequence_checker_);
  // When deciding the next target up or down, different strategies are used
  // depending on the DegradationPreference.
  // https://w3c.github.io/mst-content-hint/#dom-rtcdegradationpreference
  DegradationPreference degradation_preference_
      RTC_GUARDED_BY(&sequence_checker_);
  // Used to avoid adapting twice. Stores the resolution at the time of the last
  // adaptation.
  // TODO(hbos): Can we implement a more general "cooldown" mechanism of
  // resources intead? If we already have adapted it seems like we should wait
  // a while before adapting again, so that we are not acting on usage
  // measurements that are made obsolete/unreliable by an "ongoing" adaptation.
  struct AwaitingFrameSizeChange {
    AwaitingFrameSizeChange(bool pixels_increased, int frame_size);
    const bool pixels_increased;
    const int frame_size_pixels;
  };
  std::optional<AwaitingFrameSizeChange> awaiting_frame_size_change_
      RTC_GUARDED_BY(&sequence_checker_);
  // The previous restrictions value. Starts as unrestricted.
  VideoSourceRestrictions last_video_source_restrictions_
      RTC_GUARDED_BY(&sequence_checker_);
  VideoSourceRestrictions last_filtered_restrictions_
      RTC_GUARDED_BY(&sequence_checker_);

  std::vector<VideoSourceRestrictionsListener*> restrictions_listeners_
      RTC_GUARDED_BY(&sequence_checker_);
  std::vector<AdaptationConstraint*> adaptation_constraints_
      RTC_GUARDED_BY(&sequence_checker_);

  RestrictionsWithCounters current_restrictions_
      RTC_GUARDED_BY(&sequence_checker_);
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_VIDEO_STREAM_ADAPTER_H_
