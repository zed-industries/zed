// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_HANDLER_H_

#include <sys/types.h>

#include <atomic>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param_timeline.h"
#include "third_party/blink/renderer/modules/webaudio/audio_summing_junction.h"
#include "third_party/blink/renderer/modules/webaudio/base_audio_context.h"
#include "third_party/blink/renderer/modules/webaudio/inspector_helper_mixin.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

class AudioNodeOutput;

// AudioParamHandler is an actual implementation of web-exposed AudioParam
// interface. Each of AudioParam object creates and owns an AudioParamHandler,
// and it is responsible for all of AudioParam tasks. An AudioParamHandler
// object is owned by the originator AudioParam object, and some audio
// processing classes have additional references. An AudioParamHandler can
// outlive the owner AudioParam, and it never dies before the owner AudioParam
// dies.
//
// Connected to AudioNodeOutput using AudioNodeWiring.
class AudioParamHandler final : public ThreadSafeRefCounted<AudioParamHandler>,
                                public AudioSummingJunction {
 public:
  // Each AudioParam gets an identifier here.  This is mostly for instrospection
  // if warnings or other messages need to be printed. It's useful to know what
  // the AudioParam represents.  The name should include the node type and the
  // name of the AudioParam.
  enum class AudioParamType {
    kParamTypeAudioBufferSourcePlaybackRate,
    kParamTypeAudioBufferSourceDetune,
    kParamTypeBiquadFilterFrequency,
    kParamTypeBiquadFilterQ,
    kParamTypeBiquadFilterGain,
    kParamTypeBiquadFilterDetune,
    kParamTypeDelayDelayTime,
    kParamTypeDynamicsCompressorThreshold,
    kParamTypeDynamicsCompressorKnee,
    kParamTypeDynamicsCompressorRatio,
    kParamTypeDynamicsCompressorAttack,
    kParamTypeDynamicsCompressorRelease,
    kParamTypeGainGain,
    kParamTypeOscillatorFrequency,
    kParamTypeOscillatorDetune,
    kParamTypeStereoPannerPan,
    kParamTypePannerPositionX,
    kParamTypePannerPositionY,
    kParamTypePannerPositionZ,
    kParamTypePannerOrientationX,
    kParamTypePannerOrientationY,
    kParamTypePannerOrientationZ,
    kParamTypeAudioListenerPositionX,
    kParamTypeAudioListenerPositionY,
    kParamTypeAudioListenerPositionZ,
    kParamTypeAudioListenerForwardX,
    kParamTypeAudioListenerForwardY,
    kParamTypeAudioListenerForwardZ,
    kParamTypeAudioListenerUpX,
    kParamTypeAudioListenerUpY,
    kParamTypeAudioListenerUpZ,
    kParamTypeConstantSourceOffset,
    kParamTypeAudioWorklet,
  };

  // Automation rate of the AudioParam
  enum AutomationRate {
    // a-rate
    kAudio,
    // k-rate
    kControl
  };

  // Indicates whether automation rate can be changed.
  enum AutomationRateMode {
    // Rate can't be changed after construction
    kFixed,
    // Rate can be selected
    kVariable
  };

  AudioParamType GetParamType() const { return param_type_; }
  void SetParamType(AudioParamType);
  // Set the parameter name for an AudioWorklet.
  void SetCustomParamName(const String name);
  // Return a nice name for the AudioParam.
  String GetParamName() const;

  static scoped_refptr<AudioParamHandler> Create(BaseAudioContext& context,
                                                 AudioParamType param_type,
                                                 double default_value,
                                                 AutomationRate rate,
                                                 AutomationRateMode rate_mode,
                                                 float min_value,
                                                 float max_value) {
    return base::AdoptRef(new AudioParamHandler(context, param_type,
                                                default_value, rate, rate_mode,
                                                min_value, max_value));
  }

  // This should be used only in audio rendering thread.
  AudioDestinationHandler& DestinationHandler() const;

  // AudioSummingJunction
  void DidUpdate() override {}

  AudioParamTimeline& Timeline() { return timeline_; }

  // Intrinsic value.
  float Value();
  void SetValue(float);

  AutomationRate GetAutomationRate() const {
    base::AutoLock rate_locker(RateLock());
    return automation_rate_;
  }
  void SetAutomationRate(AutomationRate automation_rate) {
    base::AutoLock rate_locker(RateLock());
    automation_rate_ = automation_rate;
  }

  bool IsAutomationRateFixed() const {
    return rate_mode_ == AutomationRateMode::kFixed;
  }

  // Final value for k-rate parameters, otherwise use
  // calculateSampleAccurateValues() for a-rate.
  // Must be called in the audio thread.
  float FinalValue();

  float DefaultValue() const { return static_cast<float>(default_value_); }
  float MinValue() const { return min_value_; }
  float MaxValue() const { return max_value_; }

  // An AudioParam needs sample accurate processing if there are
  // automations scheduled or if there are connections.
  bool HasSampleAccurateValues() {
    bool has_values =
        timeline_.HasValues(destination_handler_->CurrentSampleFrame(),
                            destination_handler_->SampleRate(),
                            GetDeferredTaskHandler().RenderQuantumFrames());

    return has_values || NumberOfRenderingConnections();
  }

  bool IsAudioRate() const { return automation_rate_ == kAudio; }

  // Calculates numberOfValues parameter values starting at the context's
  // current time.
  // Must be called in the context's render thread.
  void CalculateSampleAccurateValues(base::span<float> values);

  float IntrinsicValue() const {
    return intrinsic_value_.load(std::memory_order_relaxed);
  }

  base::Lock& RateLock() const { return rate_lock_; }

 private:
  AudioParamHandler(BaseAudioContext&,
                    AudioParamType,
                    double default_value,
                    AutomationRate rate,
                    AutomationRateMode rate_mode,
                    float min,
                    float max);

  // sampleAccurate corresponds to a-rate (audio rate) vs. k-rate in the Web
  // Audio specification.
  void CalculateFinalValues(float* values,
                            unsigned number_of_values,
                            bool sample_accurate);
  void CalculateTimelineValues(float* values, unsigned number_of_values);

  // The type of AudioParam, indicating what this AudioParam represents and what
  // node it belongs to.  Mostly for informational purposes and doesn't affect
  // implementation.
  AudioParamType param_type_;
  // Name of the AudioParam. This is only used for printing out more informative
  // warnings, and only used for AudioWorklets.  All others have a name derived
  // from the `param_type_`.  Worklets need custom names because they're defined
  // by the user.
  String custom_param_name_;

  // Intrinsic value
  std::atomic<float> intrinsic_value_;
  void SetIntrinsicValue(float new_value);

  float default_value_;

  // Protects `automation_rate_`.
  mutable base::Lock rate_lock_;

  // The automation rate of the AudioParam (k-rate or a-rate)
  AutomationRate automation_rate_;

  // `rate_mode_` determines if the user can change the automation rate to a
  // different value.
  const AutomationRateMode rate_mode_;

  // Nominal range for the value
  float min_value_;
  float max_value_;

  AudioParamTimeline timeline_;

  // The destination node used to get necessary information like the sample rate
  // and context time.
  scoped_refptr<AudioDestinationHandler> destination_handler_;

  // Audio bus to sum in any connections to the AudioParam.
  scoped_refptr<AudioBus> summing_bus_;

  friend class AudioNodeWiring;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_HANDLER_H_
