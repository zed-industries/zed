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

// A filter based on ITU-R BS.1771-1: Requirements for
// loudness and true-peak indicating meters. For a detailed description of the
// algorithm, please refer to Annex 2 of the reccommendation
// (https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1771-1-201201-I!!PDF-E.pdf)
//
// In contrast to loudness measurements made with ITU-R BS.1770, this filter
// produces statistics about the audio on a per-sample basis instead of for a
// block of audio at a time. This makes it more useful for applications that
// want to have fully IIR-based streaming processing or that want to have
// loudness at higher sample rates, such as dynamic range control.
//
// NOTE: About the underlying loudness measurement algorithm described in ITU
// 1770, it is stated that "while this algorithm has been shown to be effective
// for use on audio programmes that are typical of broadcast content, the
// algorithm is not, in general, suitable for use to estimate the subjective
// loudness of pure tones."

#ifndef AUDIO_DSP_LOUDNESS_STREAMING_LOUDNESS_1771_H_
#define AUDIO_DSP_LOUDNESS_STREAMING_LOUDNESS_1771_H_

#include <vector>

#include "audio/dsp/decibels.h"
#include "audio/linear_filters/biquad_filter.h"
#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/discretization.h"

namespace audio_dsp {

struct StreamingLoudnessParams {
  StreamingLoudnessParams()
      : channel_weights({}),
        averaging_coefficients(linear_filters::BiquadFilterCoefficients()) {}

  // The time constant in seconds for the averaging filter used in the ITU 1771
  // algorithm for measuring momentary loudness.
  constexpr static double kITU1771TimeConstantSeconds = 0.4;

  // Generates parameters to measure loudness according to the ITU 1771
  // algorithm for a single channel of audio. time_constant_s is a time
  // constant, in seconds, for the corresponding averaging filter, which in the
  // 1771 algorithm is a first-order IIR lowpass filter. For strict compliance
  // with ITU 1771 for measuring momentary loudness, the time constant should be
  // 400ms.
  static StreamingLoudnessParams Mono1771Params(float time_constant_s,
                                                float sample_rate_hz) {
    StreamingLoudnessParams params;
    params.averaging_coefficients =
        StreamingLoudnessParams::FirstOrderLowPassBiquadCoefficients(
            time_constant_s, sample_rate_hz);
    params.channel_weights = {1.0f};
    return params;
  }

  // Generates parameters to measure loudness according to the ITU 1771
  // algorithm for a single channel of audio using a time constant of 400 ms for
  // strict compliance with the specification.
  static StreamingLoudnessParams Mono1771ParamsStrict(float sample_rate_hz) {
    return Mono1771Params(kITU1771TimeConstantSeconds, sample_rate_hz);
  }

  // Generates parameters to measure loudness according to the ITU 1771
  // algorithm for stereo audio. For strict compliance with ITU 1771 for
  // measuring momentary loudness, the time constant should be 400ms.
  static StreamingLoudnessParams Stereo1771Params(float time_constant_s,
                                                  float sample_rate_hz) {
    StreamingLoudnessParams params;
    params.averaging_coefficients =
        StreamingLoudnessParams::FirstOrderLowPassBiquadCoefficients(
            time_constant_s, sample_rate_hz);
    // Channel layout: left channel, right channel.
    params.channel_weights = {1.0f, 1.0f};
    return params;
  }

  // Generates parameters to measure loudness according to the ITU 1771
  // algorithm for stereo audio using a time constant of 400 ms for strict
  // compliance with the specification.
  static StreamingLoudnessParams Stereo1771ParamsStrict(float sample_rate_hz) {
    return Stereo1771Params(kITU1771TimeConstantSeconds, sample_rate_hz);
  }

  // Generates parameters to measure loudness according to the ITU 1771
  // algorithm for measuring momentary loudness. The LFE
  // channel is not used for loudness measurement. For strict compliance with
  // ITU 1771 for measuring momentary loudness, the time constant should be
  // 400ms.
  //
  // Note: the ordering of the channels here is assumed to be the typical
  // ordering that ffmpeg produces.
  static StreamingLoudnessParams Surround51Channel1771Params(
      float time_constant_s, float sample_rate_hz) {
    StreamingLoudnessParams params;
    params.averaging_coefficients =
        StreamingLoudnessParams::FirstOrderLowPassBiquadCoefficients(
            time_constant_s, sample_rate_hz);
    // Channel layout: left channel, right channel, center channel, LFE channel,
    // left surround channel, right surround channel.
    params.channel_weights = {1.0f, 1.0f, 1.0f, 0.0f, 1.41f, 1.41f};
    return params;
  }

  // Generates parameters to measure loudness according to the ITU 1771
  // algorithm for measuring momentary loudness using a time constant of 400
  // ms for strict compliance with the specification.
  static StreamingLoudnessParams Surround51Channel1771ParamsStrict(
      float sample_rate_hz) {
    return Surround51Channel1771Params(kITU1771TimeConstantSeconds,
                                       sample_rate_hz);
  }

  // TODO: Move this function to biquad_filter_design.
  // Calculate the coefficients to configure a biquad filter as a first-order
  // lowpass filter.
  //
  // Note: Because ITU-1771 specifies using a first-order filter, we
  // provide this function for calculating the coefficients for one, but
  // it is the same expense to configure and use the averaging filter as a
  // second-order biquad filter instead.
  static linear_filters::BiquadFilterCoefficients
    FirstOrderLowPassBiquadCoefficients(float time_constant_s,
                                        float sample_rate_hz) {
    float first_order_coefficient =
        linear_filters::FirstOrderCoefficientFromTimeConstant(time_constant_s,
                                                              sample_rate_hz);
    return linear_filters::BiquadFilterCoefficients(
        {first_order_coefficient, 0, 0}, {1, first_order_coefficient - 1, 0});
  }

  // The weights to be applied to each channel before summation. The ordering of
  // the weights must correspond to the layout of the input audio data. Helper
  // functions are provided to generate the channel weights that would be used
  // with conventional ffmpeg-style channel ordering and canonical BS.1771-1
  // channel layouts. However, these weights may also be manually customized to
  // fit the needs of the application.
  std::vector<float> channel_weights;

  // Coefficients to configure the averaging filter. The ITU 1771 algorithm
  // specifies that the averaging filter is a first-order lowpass filter, but
  // coefficients for a second-order biquad may also be provided depending on
  // the needs of the user.
  linear_filters::BiquadFilterCoefficients averaging_coefficients;
};

// Measures the perceptual loudness of an audio signal and provides
// loudness levels at the sample rate of the audio.
class StreamingLoudness1771 {
 public:
  StreamingLoudness1771() {}

  // sample_rate_hz is the audio sample rate. A return value of false indicates
  // an error in inialization.
  bool Init(const StreamingLoudnessParams& params, int num_channels,
            float sample_rate_hz);

  void Reset();

  // The input is a 2D Eigen Array of planar audio data. The data is arranged
  // such that each row of the array is one channel of audio.
  //
  // The output is a 1D Eigen Array of loudness measurements, at the sample rate
  // of the input audio.
  //
  // For a block diagram of this algorithm, please refer to the Annex 2 of ITU-R
  // BS.1771-1
  // https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1771-1-201201-I!!PDF-E.pdf
  template <typename InputEigenType, typename OutputEigenType>
  void ProcessBlock(const InputEigenType& input, OutputEigenType* output) {
    Eigen::ArrayXXf multichannel_signal =
        Eigen::ArrayXXf::Zero(input.rows(), input.cols());
    Eigen::ArrayXXf single_channel_signal =
        Eigen::ArrayXXf::Zero(1, input.cols());

    // Process audio with the k-weighting filter.
    k_weighting_filter_.ProcessBlock(input, &multichannel_signal);

    multichannel_signal = multichannel_signal.square();

    // Sum the weighted channels.
    for (int i = 0; i < input.rows(); ++i) {
      multichannel_signal.row(i) *= params_.channel_weights[i];
    }

    single_channel_signal = multichannel_signal.colwise().sum();

    // Final filtering stage.
    averaging_filter_.ProcessBlock(single_channel_signal,
                                   &single_channel_signal);

    PowerRatioToDecibels(single_channel_signal.row(0), output);
  }

  // Convenience wrapper around ProcessBlock, where the signal in input is
  // assumed to be interleaved multi-channel audio samples where the number of
  // channels is specified by the call to Init.
  // See ProcessBlock for details.
  std::vector<float> ProcessVector(const std::vector<float>& input) {
    const size_t num_channels = params_.channel_weights.size();
    const size_t num_frames = input.size() / num_channels;
    Eigen::Map<const Eigen::ArrayXXf> input_block(
        input.data(), num_channels, num_frames);
    std::vector<float> output_block(num_frames);
    Eigen::Map<Eigen::ArrayXf> output_map(output_block.data(), num_frames);
    ProcessBlock(input_block, &output_map);
    return output_block;
  }

 private:
  StreamingLoudnessParams params_;
  linear_filters::BiquadFilterCascade<Eigen::ArrayXf> k_weighting_filter_;

  // The filter that performs averaging on the weighted sum of the channels.
  // For strict adherence to ITU-R BS.1771-1, the filter should be configured to
  // be a first-order IIR lowpass filter with a time constant of 400ms.
  // Helper functions are provided in StreamingLoudnessParams to configure the
  // parameters with coefficients for a first-order lowpass filter given a time
  // constant.
  linear_filters::BiquadFilter<Eigen::ArrayXf> averaging_filter_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_LOUDNESS_STREAMING_LOUDNESS_1771_H_
