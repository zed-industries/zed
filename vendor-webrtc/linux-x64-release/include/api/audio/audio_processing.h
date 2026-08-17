/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_AUDIO_PROCESSING_H_
#define API_AUDIO_AUDIO_PROCESSING_H_

// MSVC++ requires this to be set before any other includes to get M_PI.
#ifndef _USE_MATH_DEFINES
#define _USE_MATH_DEFINES
#endif

#include <math.h>
#include <stddef.h>  // size_t
#include <stdio.h>   // FILE
#include <string.h>

#include <array>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/audio/audio_processing_statistics.h"
#include "api/audio/echo_control.h"
#include "api/environment/environment.h"
#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/arraysize.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class AecDump;
class AudioBuffer;

class StreamConfig;
class ProcessingConfig;

class EchoDetector;

// The Audio Processing Module (APM) provides a collection of voice processing
// components designed for real-time communications software.
//
// APM operates on two audio streams on a frame-by-frame basis. Frames of the
// primary stream, on which all processing is applied, are passed to
// `ProcessStream()`. Frames of the reverse direction stream are passed to
// `ProcessReverseStream()`. On the client-side, this will typically be the
// near-end (capture) and far-end (render) streams, respectively. APM should be
// placed in the signal chain as close to the audio hardware abstraction layer
// (HAL) as possible.
//
// On the server-side, the reverse stream will normally not be used, with
// processing occurring on each incoming stream.
//
// Component interfaces follow a similar pattern and are accessed through
// corresponding getters in APM. All components are disabled at create-time,
// with default settings that are recommended for most situations. New settings
// can be applied without enabling a component. Enabling a component triggers
// memory allocation and initialization to allow it to start processing the
// streams.
//
// Thread safety is provided with the following assumptions to reduce locking
// overhead:
//   1. The stream getters and setters are called from the same thread as
//      ProcessStream(). More precisely, stream functions are never called
//      concurrently with ProcessStream().
//   2. Parameter getters are never called concurrently with the corresponding
//      setter.
//
// APM accepts only linear PCM audio data in chunks of ~10 ms (see
// AudioProcessing::GetFrameSize() for details) and sample rates ranging from
// 8000 Hz to 384000 Hz. The int16 interfaces use interleaved data, while the
// float interfaces use deinterleaved data.
//
// Usage example, omitting error checking:
//
// AudioProcessing::Config config;
// config.echo_canceller.enabled = true;
// config.echo_canceller.mobile_mode = false;
//
// config.gain_controller1.enabled = true;
// config.gain_controller1.mode =
// AudioProcessing::Config::GainController1::kAdaptiveAnalog;
// config.gain_controller1.analog_level_minimum = 0;
// config.gain_controller1.analog_level_maximum = 255;
//
// config.gain_controller2.enabled = true;
//
// config.high_pass_filter.enabled = true;
//
// scoped_refptr<AudioProcessing> apm =
//     BuiltinAudioProcessingBuilder(config).Build(CreateEnvironment());
//
// // Start a voice call...
//
// // ... Render frame arrives bound for the audio HAL ...
// apm->ProcessReverseStream(render_frame);
//
// // ... Capture frame arrives from the audio HAL ...
// // Call required set_stream_ functions.
// apm->set_stream_delay_ms(delay_ms);
// apm->set_stream_analog_level(analog_level);
//
// apm->ProcessStream(capture_frame);
//
// // Call required stream_ functions.
// analog_level = apm->recommended_stream_analog_level();
// has_voice = apm->stream_has_voice();
//
// // Repeat render and capture processing for the duration of the call...
// // Start a new call...
// apm->Initialize();
//
// // Close the application...
// apm.reset();
//
class RTC_EXPORT AudioProcessing : public RefCountInterface {
 public:
  // The struct below constitutes the new parameter scheme for the audio
  // processing. It is being introduced gradually and until it is fully
  // introduced, it is prone to change.
  // TODO(peah): Remove this comment once the new config scheme is fully rolled
  // out.
  //
  // The parameters and behavior of the audio processing module are controlled
  // by changing the default values in the AudioProcessing::Config struct.
  // The config is applied by passing the struct to the ApplyConfig method.
  //
  // This config is intended to be used during setup, and to enable/disable
  // top-level processing effects. Use during processing may cause undesired
  // submodule resets, affecting the audio quality. Use the RuntimeSetting
  // construct for runtime configuration.
  struct RTC_EXPORT Config {
    // Sets the properties of the audio processing pipeline.
    struct RTC_EXPORT Pipeline {
      // Ways to downmix a multi-channel track to mono.
      enum class DownmixMethod {
        kAverageChannels,  // Average across channels.
        kUseFirstChannel   // Use the first channel.
      };

      // Maximum allowed processing rate used internally. May only be set to
      // 32000 or 48000 and any differing values will be treated as 48000.
      int maximum_internal_processing_rate = 48000;
      // Allow multi-channel processing of render audio.
      bool multi_channel_render = false;
      // Allow multi-channel processing of capture audio when AEC3 is active
      // or a custom AEC is injected..
      bool multi_channel_capture = false;
      // Indicates how to downmix multi-channel capture audio to mono (when
      // needed).
      DownmixMethod capture_downmix_method = DownmixMethod::kAverageChannels;
    } pipeline;

    // Enabled the pre-amplifier. It amplifies the capture signal
    // before any other processing is done.
    // TODO(webrtc:5298): Deprecate and use the pre-gain functionality in
    // capture_level_adjustment instead.
    struct PreAmplifier {
      bool enabled = false;
      float fixed_gain_factor = 1.0f;
    } pre_amplifier;

    // Functionality for general level adjustment in the capture pipeline. This
    // should not be used together with the legacy PreAmplifier functionality.
    struct CaptureLevelAdjustment {
      bool operator==(const CaptureLevelAdjustment& rhs) const;
      bool operator!=(const CaptureLevelAdjustment& rhs) const {
        return !(*this == rhs);
      }
      bool enabled = false;
      // The `pre_gain_factor` scales the signal before any processing is done.
      float pre_gain_factor = 1.0f;
      // The `post_gain_factor` scales the signal after all processing is done.
      float post_gain_factor = 1.0f;
      struct AnalogMicGainEmulation {
        bool operator==(const AnalogMicGainEmulation& rhs) const;
        bool operator!=(const AnalogMicGainEmulation& rhs) const {
          return !(*this == rhs);
        }
        bool enabled = false;
        // Initial analog gain level to use for the emulated analog gain. Must
        // be in the range [0...255].
        int initial_level = 255;
      } analog_mic_gain_emulation;
    } capture_level_adjustment;

    struct HighPassFilter {
      bool enabled = false;
      bool apply_in_full_band = true;
    } high_pass_filter;

    struct EchoCanceller {
      bool enabled = false;
      bool mobile_mode = false;
      bool export_linear_aec_output = false;
      // Enforce the highpass filter to be on (has no effect for the mobile
      // mode).
      bool enforce_high_pass_filtering = true;
    } echo_canceller;

    // Enables background noise suppression.
    struct NoiseSuppression {
      bool enabled = false;
      enum Level { kLow, kModerate, kHigh, kVeryHigh };
      Level level = kModerate;
      bool analyze_linear_aec_output_when_available = false;
    } noise_suppression;

    // TODO(bugs.webrtc.org/357281131): Deprecated. Stop using and remove.
    // Enables transient suppression.
    struct TransientSuppression {
      bool enabled = false;
    } transient_suppression;

    // Enables automatic gain control (AGC) functionality.
    // The automatic gain control (AGC) component brings the signal to an
    // appropriate range. This is done by applying a digital gain directly and,
    // in the analog mode, prescribing an analog gain to be applied at the audio
    // HAL.
    // Recommended to be enabled on the client-side.
    struct RTC_EXPORT GainController1 {
      bool operator==(const GainController1& rhs) const;
      bool operator!=(const GainController1& rhs) const {
        return !(*this == rhs);
      }

      bool enabled = false;
      enum Mode {
        // Adaptive mode intended for use if an analog volume control is
        // available on the capture device. It will require the user to provide
        // coupling between the OS mixer controls and AGC through the
        // stream_analog_level() functions.
        // It consists of an analog gain prescription for the audio device and a
        // digital compression stage.
        kAdaptiveAnalog,
        // Adaptive mode intended for situations in which an analog volume
        // control is unavailable. It operates in a similar fashion to the
        // adaptive analog mode, but with scaling instead applied in the digital
        // domain. As with the analog mode, it additionally uses a digital
        // compression stage.
        kAdaptiveDigital,
        // Fixed mode which enables only the digital compression stage also used
        // by the two adaptive modes.
        // It is distinguished from the adaptive modes by considering only a
        // short time-window of the input signal. It applies a fixed gain
        // through most of the input level range, and compresses (gradually
        // reduces gain with increasing level) the input signal at higher
        // levels. This mode is preferred on embedded devices where the capture
        // signal level is predictable, so that a known gain can be applied.
        kFixedDigital
      };
      Mode mode = kAdaptiveAnalog;
      // Sets the target peak level (or envelope) of the AGC in dBFs (decibels
      // from digital full-scale). The convention is to use positive values. For
      // instance, passing in a value of 3 corresponds to -3 dBFs, or a target
      // level 3 dB below full-scale. Limited to [0, 31].
      int target_level_dbfs = 3;
      // Sets the maximum gain the digital compression stage may apply, in dB. A
      // higher number corresponds to greater compression, while a value of 0
      // will leave the signal uncompressed. Limited to [0, 90].
      // For updates after APM setup, use a RuntimeSetting instead.
      int compression_gain_db = 9;
      // When enabled, the compression stage will hard limit the signal to the
      // target level. Otherwise, the signal will be compressed but not limited
      // above the target level.
      bool enable_limiter = true;

      // Enables the analog gain controller functionality.
      struct AnalogGainController {
        bool enabled = true;
        // TODO(bugs.webrtc.org/7494): Deprecated. Stop using and remove.
        int startup_min_volume = 0;
        // Lowest analog microphone level that will be applied in response to
        // clipping.
        int clipped_level_min = 70;
        // If true, an adaptive digital gain is applied.
        bool enable_digital_adaptive = true;
        // Amount the microphone level is lowered with every clipping event.
        // Limited to (0, 255].
        int clipped_level_step = 15;
        // Proportion of clipped samples required to declare a clipping event.
        // Limited to (0.f, 1.f).
        float clipped_ratio_threshold = 0.1f;
        // Time in frames to wait after a clipping event before checking again.
        // Limited to values higher than 0.
        int clipped_wait_frames = 300;

        // Enables clipping prediction functionality.
        struct ClippingPredictor {
          bool enabled = false;
          enum Mode {
            // Clipping event prediction mode with fixed step estimation.
            kClippingEventPrediction,
            // Clipped peak estimation mode with adaptive step estimation.
            kAdaptiveStepClippingPeakPrediction,
            // Clipped peak estimation mode with fixed step estimation.
            kFixedStepClippingPeakPrediction,
          };
          Mode mode = kClippingEventPrediction;
          // Number of frames in the sliding analysis window.
          int window_length = 5;
          // Number of frames in the sliding reference window.
          int reference_window_length = 5;
          // Reference window delay (unit: number of frames).
          int reference_window_delay = 5;
          // Clipping prediction threshold (dBFS).
          float clipping_threshold = -1.0f;
          // Crest factor drop threshold (dB).
          float crest_factor_margin = 3.0f;
          // If true, the recommended clipped level step is used to modify the
          // analog gain. Otherwise, the predictor runs without affecting the
          // analog gain.
          bool use_predicted_step = true;
        } clipping_predictor;
      } analog_gain_controller;
    } gain_controller1;

    // Parameters for AGC2, an Automatic Gain Control (AGC) sub-module which
    // replaces the AGC sub-module parametrized by `gain_controller1`.
    // AGC2 brings the captured audio signal to the desired level by combining
    // three different controllers (namely, input volume controller, adapative
    // digital controller and fixed digital controller) and a limiter.
    // TODO(bugs.webrtc.org:7494): Name `GainController` when AGC1 removed.
    struct RTC_EXPORT GainController2 {
      bool operator==(const GainController2& rhs) const;
      bool operator!=(const GainController2& rhs) const {
        return !(*this == rhs);
      }

      // AGC2 must be created if and only if `enabled` is true.
      bool enabled = false;

      // Parameters for the input volume controller, which adjusts the input
      // volume applied when the audio is captured (e.g., microphone volume on
      // a soundcard, input volume on HAL).
      struct InputVolumeController {
        bool operator==(const InputVolumeController& rhs) const;
        bool operator!=(const InputVolumeController& rhs) const {
          return !(*this == rhs);
        }
        bool enabled = false;
      } input_volume_controller;

      // Parameters for the adaptive digital controller, which adjusts and
      // applies a digital gain after echo cancellation and after noise
      // suppression.
      struct RTC_EXPORT AdaptiveDigital {
        bool operator==(const AdaptiveDigital& rhs) const;
        bool operator!=(const AdaptiveDigital& rhs) const {
          return !(*this == rhs);
        }
        bool enabled = false;
        float headroom_db = 5.0f;
        float max_gain_db = 50.0f;
        float initial_gain_db = 15.0f;
        float max_gain_change_db_per_second = 6.0f;
        float max_output_noise_level_dbfs = -50.0f;
      } adaptive_digital;

      // Parameters for the fixed digital controller, which applies a fixed
      // digital gain after the adaptive digital controller and before the
      // limiter.
      struct FixedDigital {
        // By setting `gain_db` to a value greater than zero, the limiter can be
        // turned into a compressor that first applies a fixed gain.
        float gain_db = 0.0f;
      } fixed_digital;
    } gain_controller2;

    std::string ToString() const;
  };

  // Specifies the properties of a setting to be passed to AudioProcessing at
  // runtime.
  class RuntimeSetting {
   public:
    enum class Type {
      kNotSpecified,
      kCapturePreGain,
      kCaptureCompressionGain,
      kCaptureFixedPostGain,
      kPlayoutVolumeChange,
      kCustomRenderProcessingRuntimeSetting,
      kPlayoutAudioDeviceChange,
      kCapturePostGain,
      kCaptureOutputUsed
    };

    // Play-out audio device properties.
    struct PlayoutAudioDeviceInfo {
      int id;          // Identifies the audio device.
      int max_volume;  // Maximum play-out volume.
    };

    RuntimeSetting() : type_(Type::kNotSpecified), value_(0.0f) {}
    ~RuntimeSetting() = default;

    static RuntimeSetting CreateCapturePreGain(float gain) {
      return {Type::kCapturePreGain, gain};
    }

    static RuntimeSetting CreateCapturePostGain(float gain) {
      return {Type::kCapturePostGain, gain};
    }

    // Corresponds to Config::GainController1::compression_gain_db, but for
    // runtime configuration.
    static RuntimeSetting CreateCompressionGainDb(int gain_db) {
      RTC_DCHECK_GE(gain_db, 0);
      RTC_DCHECK_LE(gain_db, 90);
      return {Type::kCaptureCompressionGain, static_cast<float>(gain_db)};
    }

    // Corresponds to Config::GainController2::fixed_digital::gain_db, but for
    // runtime configuration.
    static RuntimeSetting CreateCaptureFixedPostGain(float gain_db) {
      RTC_DCHECK_GE(gain_db, 0.0f);
      RTC_DCHECK_LE(gain_db, 90.0f);
      return {Type::kCaptureFixedPostGain, gain_db};
    }

    // Creates a runtime setting to notify play-out (aka render) audio device
    // changes.
    static RuntimeSetting CreatePlayoutAudioDeviceChange(
        PlayoutAudioDeviceInfo audio_device) {
      return {Type::kPlayoutAudioDeviceChange, audio_device};
    }

    // Creates a runtime setting to notify play-out (aka render) volume changes.
    // `volume` is the unnormalized volume, the maximum of which
    static RuntimeSetting CreatePlayoutVolumeChange(int volume) {
      return {Type::kPlayoutVolumeChange, volume};
    }

    static RuntimeSetting CreateCustomRenderSetting(float payload) {
      return {Type::kCustomRenderProcessingRuntimeSetting, payload};
    }

    static RuntimeSetting CreateCaptureOutputUsedSetting(
        bool capture_output_used) {
      return {Type::kCaptureOutputUsed, capture_output_used};
    }

    Type type() const { return type_; }
    // Getters do not return a value but instead modify the argument to protect
    // from implicit casting.
    void GetFloat(float* value) const {
      RTC_DCHECK(value);
      *value = value_.float_value;
    }
    void GetInt(int* value) const {
      RTC_DCHECK(value);
      *value = value_.int_value;
    }
    void GetBool(bool* value) const {
      RTC_DCHECK(value);
      *value = value_.bool_value;
    }
    void GetPlayoutAudioDeviceInfo(PlayoutAudioDeviceInfo* value) const {
      RTC_DCHECK(value);
      *value = value_.playout_audio_device_info;
    }

   private:
    RuntimeSetting(Type id, float value) : type_(id), value_(value) {}
    RuntimeSetting(Type id, int value) : type_(id), value_(value) {}
    RuntimeSetting(Type id, PlayoutAudioDeviceInfo value)
        : type_(id), value_(value) {}
    Type type_;
    union U {
      U() {}
      U(int value) : int_value(value) {}
      U(float value) : float_value(value) {}
      U(PlayoutAudioDeviceInfo value) : playout_audio_device_info(value) {}
      float float_value;
      int int_value;
      bool bool_value;
      PlayoutAudioDeviceInfo playout_audio_device_info;
    } value_;
  };

  ~AudioProcessing() override {}

  // Initializes internal states, while retaining all user settings. This
  // should be called before beginning to process a new audio stream. However,
  // it is not necessary to call before processing the first stream after
  // creation.
  //
  // It is also not necessary to call if the audio parameters (sample
  // rate and number of channels) have changed. Passing updated parameters
  // directly to `ProcessStream()` and `ProcessReverseStream()` is permissible.
  // If the parameters are known at init-time though, they may be provided.
  // TODO(webrtc:5298): Change to return void.
  virtual int Initialize() = 0;

  // The int16 interfaces require:
  //   - only `NativeRate`s be used
  //   - that the input, output and reverse rates must match
  //   - that `processing_config.output_stream()` matches
  //     `processing_config.input_stream()`.
  //
  // The float interfaces accept arbitrary rates and support differing input and
  // output layouts, but the output must have either one channel or the same
  // number of channels as the input.
  virtual int Initialize(const ProcessingConfig& processing_config) = 0;

  // TODO(peah): This method is a temporary solution used to take control
  // over the parameters in the audio processing module and is likely to change.
  virtual void ApplyConfig(const Config& config) = 0;

  // TODO(ajm): Only intended for internal use. Make private and friend the
  // necessary classes?
  virtual int proc_sample_rate_hz() const = 0;
  virtual int proc_split_sample_rate_hz() const = 0;
  virtual size_t num_input_channels() const = 0;
  virtual size_t num_proc_channels() const = 0;
  virtual size_t num_output_channels() const = 0;
  virtual size_t num_reverse_channels() const = 0;

  // Set to true when the output of AudioProcessing will be muted or in some
  // other way not used. Ideally, the captured audio would still be processed,
  // but some components may change behavior based on this information.
  // Default false. This method takes a lock. To achieve this in a lock-less
  // manner the PostRuntimeSetting can instead be used.
  virtual void set_output_will_be_muted(bool muted) = 0;
  virtual bool get_output_will_be_muted() = 0;

  // Enqueues a runtime setting.
  virtual void SetRuntimeSetting(RuntimeSetting setting) = 0;

  // Enqueues a runtime setting. Returns a bool indicating whether the
  // enqueueing was successfull.
  virtual bool PostRuntimeSetting(RuntimeSetting setting) = 0;

  // Accepts and produces a ~10 ms frame of interleaved 16 bit integer audio as
  // specified in `input_config` and `output_config`. `src` and `dest` may use
  // the same memory, if desired.
  virtual int ProcessStream(const int16_t* const src,
                            const StreamConfig& input_config,
                            const StreamConfig& output_config,
                            int16_t* const dest) = 0;

  // Accepts deinterleaved float audio with the range [-1, 1]. Each element of
  // `src` points to a channel buffer, arranged according to `input_stream`. At
  // output, the channels will be arranged according to `output_stream` in
  // `dest`.
  //
  // The output must have one channel or as many channels as the input. `src`
  // and `dest` may use the same memory, if desired.
  virtual int ProcessStream(const float* const* src,
                            const StreamConfig& input_config,
                            const StreamConfig& output_config,
                            float* const* dest) = 0;

  // Accepts and produces a ~10 ms frame of interleaved 16 bit integer audio for
  // the reverse direction audio stream as specified in `input_config` and
  // `output_config`. `src` and `dest` may use the same memory, if desired.
  virtual int ProcessReverseStream(const int16_t* const src,
                                   const StreamConfig& input_config,
                                   const StreamConfig& output_config,
                                   int16_t* const dest) = 0;

  // Accepts deinterleaved float audio with the range [-1, 1]. Each element of
  // `data` points to a channel buffer, arranged according to `reverse_config`.
  virtual int ProcessReverseStream(const float* const* src,
                                   const StreamConfig& input_config,
                                   const StreamConfig& output_config,
                                   float* const* dest) = 0;

  // Accepts deinterleaved float audio with the range [-1, 1]. Each element
  // of `data` points to a channel buffer, arranged according to
  // `reverse_config`.
  virtual int AnalyzeReverseStream(const float* const* data,
                                   const StreamConfig& reverse_config) = 0;

  // Returns the most recently produced ~10 ms of the linear AEC output at a
  // rate of 16 kHz. If there is more than one capture channel, a mono
  // representation of the input is returned. Returns true/false to indicate
  // whether an output returned.
  virtual bool GetLinearAecOutput(
      ArrayView<std::array<float, 160>> linear_output) const = 0;

  // This must be called prior to ProcessStream() if and only if adaptive analog
  // gain control is enabled, to pass the current analog level from the audio
  // HAL. Must be within the range [0, 255].
  virtual void set_stream_analog_level(int level) = 0;

  // When an analog mode is set, this should be called after
  // `set_stream_analog_level()` and `ProcessStream()` to obtain the recommended
  // new analog level for the audio HAL. It is the user's responsibility to
  // apply this level.
  virtual int recommended_stream_analog_level() const = 0;

  // This must be called if and only if echo processing is enabled.
  //
  // Sets the `delay` in ms between ProcessReverseStream() receiving a far-end
  // frame and ProcessStream() receiving a near-end frame containing the
  // corresponding echo. On the client-side this can be expressed as
  //   delay = (t_render - t_analyze) + (t_process - t_capture)
  // where,
  //   - t_analyze is the time a frame is passed to ProcessReverseStream() and
  //     t_render is the time the first sample of the same frame is rendered by
  //     the audio hardware.
  //   - t_capture is the time the first sample of a frame is captured by the
  //     audio hardware and t_process is the time the same frame is passed to
  //     ProcessStream().
  virtual int set_stream_delay_ms(int delay) = 0;
  virtual int stream_delay_ms() const = 0;

  // Call to signal that a key press occurred (true) or did not occur (false)
  // with this chunk of audio.
  virtual void set_stream_key_pressed(bool key_pressed) = 0;

  // Creates and attaches an webrtc::AecDump for recording debugging
  // information.
  // The `worker_queue` may not be null and must outlive the created
  // AecDump instance. |max_log_size_bytes == -1| means the log size
  // will be unlimited. `handle` may not be null. The AecDump takes
  // responsibility for `handle` and closes it in the destructor. A
  // return value of true indicates that the file has been
  // sucessfully opened, while a value of false indicates that
  // opening the file failed.
  virtual bool CreateAndAttachAecDump(absl::string_view file_name,
                                      int64_t max_log_size_bytes,
                                      TaskQueueBase* absl_nonnull
                                          worker_queue) = 0;
  virtual bool CreateAndAttachAecDump(FILE* absl_nonnull handle,
                                      int64_t max_log_size_bytes,
                                      TaskQueueBase* absl_nonnull
                                          worker_queue) = 0;

  // TODO(webrtc:5298) Deprecated variant.
  // Attaches provided webrtc::AecDump for recording debugging
  // information. Log file and maximum file size logic is supposed to
  // be handled by implementing instance of AecDump. Calling this
  // method when another AecDump is attached resets the active AecDump
  // with a new one. This causes the d-tor of the earlier AecDump to
  // be called. The d-tor call may block until all pending logging
  // tasks are completed.
  virtual void AttachAecDump(std::unique_ptr<AecDump> aec_dump) = 0;

  // If no AecDump is attached, this has no effect. If an AecDump is
  // attached, it's destructor is called. The d-tor may block until
  // all pending logging tasks are completed.
  virtual void DetachAecDump() = 0;

  // Get audio processing statistics.
  virtual AudioProcessingStats GetStatistics() = 0;
  // TODO(webrtc:5298) Deprecated variant. The `has_remote_tracks` argument
  // should be set if there are active remote tracks (this would usually be true
  // during a call). If there are no remote tracks some of the stats will not be
  // set by AudioProcessing, because they only make sense if there is at least
  // one remote track.
  virtual AudioProcessingStats GetStatistics(bool has_remote_tracks) = 0;

  // Returns the last applied configuration.
  virtual AudioProcessing::Config GetConfig() const = 0;

  enum Error {
    // Fatal errors.
    kNoError = 0,
    kUnspecifiedError = -1,
    kCreationFailedError = -2,
    kUnsupportedComponentError = -3,
    kUnsupportedFunctionError = -4,
    kNullPointerError = -5,
    kBadParameterError = -6,
    kBadSampleRateError = -7,
    kBadDataLengthError = -8,
    kBadNumberChannelsError = -9,
    kFileError = -10,
    kStreamParameterNotSetError = -11,
    kNotEnabledError = -12,

    // Warnings are non-fatal.
    // This results when a set_stream_ parameter is out of range. Processing
    // will continue, but the parameter may have been truncated.
    kBadStreamParameterWarning = -13
  };

  // Native rates supported by the integer interfaces.
  enum NativeRate {
    kSampleRate8kHz = 8000,
    kSampleRate16kHz = 16000,
    kSampleRate32kHz = 32000,
    kSampleRate48kHz = 48000
  };

  // TODO(kwiberg): We currently need to support a compiler (Visual C++) that
  // complains if we don't explicitly state the size of the array here. Remove
  // the size when that's no longer the case.
  static constexpr int kNativeSampleRatesHz[4] = {
      kSampleRate8kHz, kSampleRate16kHz, kSampleRate32kHz, kSampleRate48kHz};
  static constexpr size_t kNumNativeSampleRates =
      arraysize(kNativeSampleRatesHz);
  static constexpr int kMaxNativeSampleRateHz =
      kNativeSampleRatesHz[kNumNativeSampleRates - 1];

  // APM processes audio in chunks of about 10 ms. See GetFrameSize() for
  // details.
  static constexpr int kChunkSizeMs = 10;

  // Returns floor(sample_rate_hz/100): the number of samples per channel used
  // as input and output to the audio processing module in calls to
  // ProcessStream, ProcessReverseStream, AnalyzeReverseStream, and
  // GetLinearAecOutput.
  //
  // This is exactly 10 ms for sample rates divisible by 100. For example:
  //  - 48000 Hz (480 samples per channel),
  //  - 44100 Hz (441 samples per channel),
  //  - 16000 Hz (160 samples per channel).
  //
  // Sample rates not divisible by 100 are received/produced in frames of
  // approximately 10 ms. For example:
  //  - 22050 Hz (220 samples per channel, or ~9.98 ms per frame),
  //  - 11025 Hz (110 samples per channel, or ~9.98 ms per frame).
  // These nondivisible sample rates yield lower audio quality compared to
  // multiples of 100. Internal resampling to 10 ms frames causes a simulated
  // clock drift effect which impacts the performance of (for example) echo
  // cancellation.
  static int GetFrameSize(int sample_rate_hz) { return sample_rate_hz / 100; }
};

class AudioProcessingBuilderInterface {
 public:
  virtual ~AudioProcessingBuilderInterface() = default;

  virtual absl_nullable scoped_refptr<AudioProcessing> Build(
      const Environment& env) = 0;
};

// Returns builder that returns the `audio_processing` ignoring the extra
// construction parameter `env`.
// nullptr `audio_processing` is not supported as in some scenarios that imply
// no audio processing, while in others - default builtin audio processing.
// Callers should be explicit which of these two behaviors they want.
absl_nonnull std::unique_ptr<AudioProcessingBuilderInterface>
CustomAudioProcessing(
    absl_nonnull scoped_refptr<AudioProcessing> audio_processing);

// Experimental interface for a custom analysis submodule.
class CustomAudioAnalyzer {
 public:
  // (Re-) Initializes the submodule.
  virtual void Initialize(int sample_rate_hz, int num_channels) = 0;
  // Analyzes the given capture or render signal.
  virtual void Analyze(const AudioBuffer* audio) = 0;
  // Returns a string representation of the module state.
  virtual std::string ToString() const = 0;

  virtual ~CustomAudioAnalyzer() {}
};

// Interface for a custom processing submodule.
class CustomProcessing {
 public:
  // (Re-)Initializes the submodule.
  virtual void Initialize(int sample_rate_hz, int num_channels) = 0;
  // Processes the given capture or render signal.
  virtual void Process(AudioBuffer* audio) = 0;
  // Returns a string representation of the module state.
  virtual std::string ToString() const = 0;
  // Handles RuntimeSettings. TODO(webrtc:9262): make pure virtual
  // after updating dependencies.
  virtual void SetRuntimeSetting(AudioProcessing::RuntimeSetting setting);

  virtual ~CustomProcessing() {}
};

class StreamConfig {
 public:
  // sample_rate_hz: The sampling rate of the stream.
  // num_channels: The number of audio channels in the stream.
  StreamConfig(int sample_rate_hz = 0,
               size_t num_channels = 0)  // NOLINT(runtime/explicit)
      : sample_rate_hz_(sample_rate_hz),
        num_channels_(num_channels),
        num_frames_(calculate_frames(sample_rate_hz)) {}

  void set_sample_rate_hz(int value) {
    sample_rate_hz_ = value;
    num_frames_ = calculate_frames(value);
  }
  void set_num_channels(size_t value) { num_channels_ = value; }

  int sample_rate_hz() const { return sample_rate_hz_; }

  // The number of channels in the stream.
  size_t num_channels() const { return num_channels_; }

  size_t num_frames() const { return num_frames_; }
  size_t num_samples() const { return num_channels_ * num_frames_; }

  bool operator==(const StreamConfig& other) const {
    return sample_rate_hz_ == other.sample_rate_hz_ &&
           num_channels_ == other.num_channels_;
  }

  bool operator!=(const StreamConfig& other) const { return !(*this == other); }

 private:
  static size_t calculate_frames(int sample_rate_hz) {
    return static_cast<size_t>(AudioProcessing::GetFrameSize(sample_rate_hz));
  }

  int sample_rate_hz_;
  size_t num_channels_;
  size_t num_frames_;
};

class ProcessingConfig {
 public:
  enum StreamName {
    kInputStream,
    kOutputStream,
    kReverseInputStream,
    kReverseOutputStream,
    kNumStreamNames,
  };

  const StreamConfig& input_stream() const {
    return streams[StreamName::kInputStream];
  }
  const StreamConfig& output_stream() const {
    return streams[StreamName::kOutputStream];
  }
  const StreamConfig& reverse_input_stream() const {
    return streams[StreamName::kReverseInputStream];
  }
  const StreamConfig& reverse_output_stream() const {
    return streams[StreamName::kReverseOutputStream];
  }

  StreamConfig& input_stream() { return streams[StreamName::kInputStream]; }
  StreamConfig& output_stream() { return streams[StreamName::kOutputStream]; }
  StreamConfig& reverse_input_stream() {
    return streams[StreamName::kReverseInputStream];
  }
  StreamConfig& reverse_output_stream() {
    return streams[StreamName::kReverseOutputStream];
  }

  bool operator==(const ProcessingConfig& other) const {
    for (int i = 0; i < StreamName::kNumStreamNames; ++i) {
      if (this->streams[i] != other.streams[i]) {
        return false;
      }
    }
    return true;
  }

  bool operator!=(const ProcessingConfig& other) const {
    return !(*this == other);
  }

  StreamConfig streams[StreamName::kNumStreamNames];
};

// Interface for an echo detector submodule.
class EchoDetector : public RefCountInterface {
 public:
  // (Re-)Initializes the submodule.
  virtual void Initialize(int capture_sample_rate_hz,
                          int num_capture_channels,
                          int render_sample_rate_hz,
                          int num_render_channels) = 0;

  // Analysis (not changing) of the first channel of the render signal.
  virtual void AnalyzeRenderAudio(ArrayView<const float> render_audio) = 0;

  // Analysis (not changing) of the capture signal.
  virtual void AnalyzeCaptureAudio(ArrayView<const float> capture_audio) = 0;

  struct Metrics {
    std::optional<double> echo_likelihood;
    std::optional<double> echo_likelihood_recent_max;
  };

  // Collect current metrics from the echo detector.
  virtual Metrics GetMetrics() const = 0;
};

}  // namespace webrtc

#endif  // API_AUDIO_AUDIO_PROCESSING_H_
