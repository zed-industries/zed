/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Apply dynamic range control to an audio signal. This class is vectorized
// using Eigen for fast computation and supports processing multichannel
// inputs.
//
// Common other names for this (to help with code searching):
//  - automatic gain control (AGC)
//  - dynamic range compression (DRC)
//  - limiting, limiter
//  - noise gate
//  - expander
//
// All measures with units in decibels are with reference to unity. This
// is commonly called dB full-scale (dBFS).
//
// Usage:
//
//  /* Get everything setup. */
//  DynamicRangeControl drc(...);
//  /**
//   *  Block size has no effect on output unless changing DRC params
//   * (interpolation happens over a single block).
//   */
//  const int kFramesPerBlock = 512;
//  drc.Init(num_channels, kFramesPerBlock, sample_rate_hz);
//
// /* Get input samples from somewhere and allocate space for output. */
// std::vector<float> input_samples = GetInterleavedAudioSamples();
// std::vector<float> output_samples(input_samples.size());
//
// /**
//  * Note that ProcessBlock(), ProcessBlockWithSidechain(), and
//  * ComputeGainSignalOnly()/ApplyGainToSignal() all have the same effect
//  * on the state of the object. It is expected that you will use only one of
//  * the following configurations for any initialization of
//  * DynamicRangeControl. Below are three examples of how to use this class to
//  * do block processing.
//  */
//
//  /** Option 1: Typical Use. */
//
//  for (int i = 0; i < rounded_block_size; i += kSamplesPerFrame) {
//    Map<const ArrayXXf> input_block(input_samples.data() + i,
//                                    num_channels, kFramesPerBlock);
//    Map<ArrayXXf> output_block(output_samples.data() + i,
//                               num_channels, kFramesPerBlock);
//    drc.ProcessBlock(input_block, &output_block);
//  }
//
//  /** Option 2: Sidechain gain control. */
//  std::vector<float> sidechain_samples = GetInterleavedSidechainAudio();
//  for (int i = 0; i < rounded_block_size; i += kSamplesPerFrame) {
//    Map<const ArrayXXf> input_block(...);  /* Same as above. */
//    Map<const ArrayXXf> sidechain_block(sidechain_samples.data() + i,
//                                        num_channels, kFramesPerBlock);
//    Map<ArrayXXf> output_block(...);       /* Same as above. */
//    drc.ProcessBlockWithSidechain(input_block, sidechain_block,
//                                  &output_block);
//  }
//
//  /**
//   * Option 3: Do additional processing on gain signal before applying.
//   *           Note that this is compatible with sidechaining.
//   */
//  std::vector<float> gain_samples(kSamplesPerFrame);
//  for (int i = 0; i < rounded_block_size; i += kSamplesPerFrame) {
//    Map<const ArrayXXf> input_block(...);  /* Same as above. */
//    Map<ArrayXf> gain_block(gain_samples.data(), 1,
//                            kSamplesPerFrame / num_channels);
//    Map<ArrayXXf> output_block(...);       /* Same as above. */
//    /* To do sidechaining, use sidechain_block instead of input_block. */
//    drc.ComputeGainSignalOnly(input_block, &gain_signal);
//    /** This is a convenient way to implement cross-coupling between
//     *  different filterbank channels in a multiband compressor.
//     */
//    DoSomethingWithGain(&gain_signal);
//    drc.ApplyGainToSignal(input_block, gain_signal, &output_block);
//  }

#ifndef AUDIO_DSP_HIFI_DYNAMIC_RANGE_CONTROL_H_
#define AUDIO_DSP_HIFI_DYNAMIC_RANGE_CONTROL_H_

#include <memory>

#include "audio/dsp/attack_release_envelope.h"
#include "audio/dsp/fixed_delay_line.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Chooses whether to use the peak or the RMS value of the signal when
// computing the envelope.
enum EnvelopeType {
  kPeak,
  kRms,
};

// Chooses the type of gain control that will be applied.
// See http://www.rane.com/note155.html for some explanation of each.
enum DynamicsControlType {
  // Compression reduces dynamic range above a certain level.
  kCompressor,
  // Limiters are a means of gain control to prevent the level from exceeding
  // the threshold (this is a much softer nonlinearity than a hard clipper).
  // For typical signals, a limiter will prevent the level from exceeding
  // the threshold very well, but is easy to come up with a highly impulsive
  // signal causes the output to exceed the threshold.
  kLimiter,
  // The expander increases the dynamic range when the signal is below the
  // threshold.
  kExpander,
  // Noise gates leave signals above the threshold alone and silence sounds that
  // are less than the threshold.
  kNoiseGate,
};

// NOTE: There is no one-size-fits-all solution for dynamic range control.
// The type of signal and the input level/desired output level will have a lot
// of impact over how the threshold, ratio, and gains should be set.
struct DynamicRangeControlParams {
  DynamicRangeControlParams()  // Mostly for putting params in a defined state.
      : envelope_type(kRms),
        dynamics_type(kCompressor),
        input_gain_db(0),
        output_gain_db(0),
        threshold_db(0),
        ratio(1),
        knee_width_db(0),
        attack_s(0.001f),
        release_s(0.05f),
        lookahead_s(0) {}

  // See NOTE above.
  static DynamicRangeControlParams ReasonableCompressorParams() {
    DynamicRangeControlParams params;
    params.envelope_type = kRms;
    params.dynamics_type = kCompressor;
    params.input_gain_db = 0.0f;
    params.output_gain_db = 30.0f;
    params.threshold_db = -37.0f;
    params.ratio = 4.6f;
    params.knee_width_db = 4.0f;
    // See documentation on these time constants below.
    params.attack_s = 0.001f;
    params.release_s = 0.08f;
    params.lookahead_s = 0;
    return params;
  }

  // See NOTE above.
  static DynamicRangeControlParams ReasonableLimiterParams() {
    DynamicRangeControlParams params;
    params.envelope_type = kPeak;
    params.dynamics_type = kLimiter;
    params.input_gain_db = 0.0f;
    params.output_gain_db = 0.0f;
    params.threshold_db = -3.0f;
    params.knee_width_db = 1.0f;
    // See documentation on these time constants below.
    params.attack_s = 0.0004f;
    params.release_s = 0.004f;
    params.lookahead_s = 0;
    return params;
  }

  // Choose whether to use the peak value of the signal or the RMS when
  // computing the gain.
  EnvelopeType envelope_type;

  // Describes the type of gain control.
  DynamicsControlType dynamics_type;

  // Applied before the nonlinearity.
  float input_gain_db;

  // Applied at the output to make up for lost signal energy.
  float output_gain_db;

  // The amplitude in decibels (dBFS) at which the range control kicks in.
  // For compression and limiting, a signal below this threshold is not
  // scaled (ignoring the knee).
  // For a noise gate or an expander, a signal above the threshold is not
  // scaled (ignoring the knee).
  float threshold_db;

  // Except in a transitional knee around threshold_db, the input/output
  // relationship of a compressor is
  //   output_db = input_db, for input_db < threshold_db
  //   output_db = threshold_db + (input_db - threshold_db) / ratio,
  //     for input_db > threshold_db.
  // Likewise, for an expander
  //   output_db = input_db, for input_db > threshold_db
  //   output_db = threshold_db + (input_db - threshold_db) * ratio,
  //     for input_db < threshold_db.
  float ratio;  // Ignored for limiter and noise gate.

  // A gentle transition between into range control around the threshold.
  float knee_width_db;  // Two-sided (diameter).

  // Exponential time constants associated with the gain estimator. Depending
  // on the application, these may range from 1ms to over 1s.
  //
  // If you are trying to compare this DRC to another commercial DRC, you may
  // find that the same time constants produce different results. We use the
  // definition of time constant that is typical in signal processing:
  // a filter's step response reaches (1 - 1/e) of its final value after one
  // time constant. Some commercial DRCs will define their time constants such
  // that the filter is within some small tolerance of its final value after
  // one time constant (meaning that to get similar results, time constants for
  // this DRC should be set about a factor of 4 smaller).
  float attack_s;
  float release_s;

  // The following parameters must not change after initialization:

  // Delay the incoming audio to make the gain control more predictive.
  // This has the effect of delaying the audio output (advancing the compression
  // action relative to the signal).
  float lookahead_s;
};

// Multichannel, feed-forward dynamic range control. Note that the gain
// adjustment is the same on each channel.
//
// This class is not thread safe.
class DynamicRangeControl {
 public:
  explicit DynamicRangeControl(const DynamicRangeControlParams& initial_params);

  // sample_rate_hz is the audio sample rate.
  void Init(int num_channels, int max_block_size_samples, float sample_rate_hz);

  int num_channels() const;

  int max_block_size_samples() const;

  void Reset();

  // This should be called in between calls to ProcessBlock. If called multiple
  // times, only the last call will be used.
  void SetDynamicRangeControlParams(
      const DynamicRangeControlParams& params);

  // InputType and OutputType are Eigen 2D blocks (ArrayXXf or MatrixXf) or
  // a similar mapped type containing audio samples in column-major format.
  // The number of rows must be equal to num_channels, and the number of cols
  // must be less than or equal to max_block_size_samples.
  //
  // Templating is necessary for supporting Eigen::Map output types.
  //
  // NOTE: When you change params by calling SetDynamicRangeControlParams,
  // there is a crossfade between a signal generated with the old & new
  // parameters that happens over the next block (input.cols() samples).
  // As long as the params changes aren't really large, any block size greater
  // than 100 or so should not produce significant artifacts.
  template <typename InputType, typename OutputType>
  void ProcessBlock(const InputType& input, OutputType* output) {
    ProcessBlockWithSidechain(input, input, output);
  }

  // Process samples using a sidechain signal to determine the applied gain.
  // See header comments. The sidechain signal is used to generate the
  // gain envelope and compute the gain that will be applied to the input
  // signal.
  template <typename InputType, typename SidechainType, typename OutputType>
  void ProcessBlockWithSidechain(const InputType& input,
                                 const SidechainType& sidechain,
                                 OutputType* output) {
    static_assert(std::is_same<typename InputType::Scalar, float>::value,
                  "Scalar type must be float.");
    static_assert(std::is_same<typename SidechainType::Scalar, float>::value,
                  "Scalar type must be float.");
    static_assert(std::is_same<typename OutputType::Scalar, float>::value,
                  "Scalar type must be float.");

    ABSL_DCHECK_GE(input.cols(), 1);
    ABSL_DCHECK_LE(input.cols(), max_block_size_samples_);
    ABSL_DCHECK_EQ(input.rows(), num_channels_);
    ABSL_DCHECK_EQ(input.cols(), output->cols());
    ABSL_DCHECK_EQ(input.rows(), output->rows());
    ABSL_DCHECK_EQ(input.cols(), sidechain.cols());
    ABSL_DCHECK_EQ(input.rows(), sidechain.rows());

    // Map the needed amount of space for computations.
    VectorType workmap = workspace_.head(sidechain.cols());

    ComputeGainSignalOnly(sidechain, &workmap);
    ApplyGainToSignal(input, workmap, output);
  }

  // Compute the gains that would be applied to the signal so that more
  // processing can be done prior to applying them. See header comments.
  //
  // gain is a monaural signal, with length input_or_sidechain.cols(). Note that
  // if GainType is not resizable, it must be presized to the right size.
  template <typename InputType, typename GainType>
  void ComputeGainSignalOnly(const InputType& input_or_sidechain,
                             GainType* gain) {
    ABSL_DCHECK_LE(input_or_sidechain.cols(), max_block_size_samples_);
    gain->resize(input_or_sidechain.cols(), 1);
    // Compute the average power/amplitude across channels. The signal envelope
    // is monaural.
    if (params_.envelope_type == kRms) {
      *gain = input_or_sidechain.square().colwise().mean();
    } else {
      *gain = input_or_sidechain.abs().colwise().mean();
    }
    ComputeGainFromDetectedSignal(gain);
  }

  // To be used with ComputeGainSignalOnly above. See header comments.
  // In-place processing is supported (&input = output).
  template <typename GainType, typename InputType, typename OutputType>
  void ApplyGainToSignal(const InputType& input,
                         const GainType& gain,
                         OutputType* output) {
    ABSL_DCHECK_EQ(input.cols(), gain.size());
    // Scale the input sample-wise by the gains.
    Eigen::Map<const Eigen::ArrayXXf> delayed =
        lookahead_delay_.ProcessBlock(input);
    *output = delayed.rowwise() * gain.transpose();
  }

 private:
  using VectorType = Eigen::VectorBlock<Eigen::ArrayXf, Eigen::Dynamic>;

  // Compute the linear gain to apply to the input. data_ptr should contain
  // a rectified signal envelope. After calling this function it will contain
  // a linear gain to apply to the signal.
  void ComputeGainFromDetectedSignal(VectorType* data_ptr);
  // Defer gain computation to specific types of dynamic range control.
  void ComputeGainForSpecificDynamicRangeControlType(
      const VectorType& input_level, VectorType* output_gain);

  int num_channels_;
  float sample_rate_hz_;
  int max_block_size_samples_;

  Eigen::ArrayXf workspace_;
  Eigen::ArrayXf workspace_drc_output_;
  DynamicRangeControlParams params_;
  // When parameters change, we need to smoothly transition to them.
  bool params_change_needed_;
  DynamicRangeControlParams next_params_;

  FixedDelayLine lookahead_delay_;
  std::unique_ptr<AttackReleaseEnvelope> envelope_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_HIFI_DYNAMIC_RANGE_CONTROL_H_
