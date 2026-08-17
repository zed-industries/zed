/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ADAPTIVE_FIR_FILTER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ADAPTIVE_FIR_FILTER_H_

#include <stddef.h>

#include <array>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/aec3_fft.h"
#include "modules/audio_processing/aec3/fft_data.h"
#include "modules/audio_processing/aec3/render_buffer.h"
#include "modules/audio_processing/logging/apm_data_dumper.h"
#include "rtc_base/system/arch.h"

namespace webrtc {
namespace aec3 {
// Computes and stores the frequency response of the filter.
void ComputeFrequencyResponse(
    size_t num_partitions,
    const std::vector<std::vector<FftData>>& H,
    std::vector<std::array<float, kFftLengthBy2Plus1>>* H2);
#if defined(WEBRTC_HAS_NEON)
void ComputeFrequencyResponse_Neon(
    size_t num_partitions,
    const std::vector<std::vector<FftData>>& H,
    std::vector<std::array<float, kFftLengthBy2Plus1>>* H2);
#endif
#if defined(WEBRTC_ARCH_X86_FAMILY)
void ComputeFrequencyResponse_Sse2(
    size_t num_partitions,
    const std::vector<std::vector<FftData>>& H,
    std::vector<std::array<float, kFftLengthBy2Plus1>>* H2);

void ComputeFrequencyResponse_Avx2(
    size_t num_partitions,
    const std::vector<std::vector<FftData>>& H,
    std::vector<std::array<float, kFftLengthBy2Plus1>>* H2);
#endif

// Adapts the filter partitions.
void AdaptPartitions(const RenderBuffer& render_buffer,
                     const FftData& G,
                     size_t num_partitions,
                     std::vector<std::vector<FftData>>* H);
#if defined(WEBRTC_HAS_NEON)
void AdaptPartitions_Neon(const RenderBuffer& render_buffer,
                          const FftData& G,
                          size_t num_partitions,
                          std::vector<std::vector<FftData>>* H);
#endif
#if defined(WEBRTC_ARCH_X86_FAMILY)
void AdaptPartitions_Sse2(const RenderBuffer& render_buffer,
                          const FftData& G,
                          size_t num_partitions,
                          std::vector<std::vector<FftData>>* H);

void AdaptPartitions_Avx2(const RenderBuffer& render_buffer,
                          const FftData& G,
                          size_t num_partitions,
                          std::vector<std::vector<FftData>>* H);
#endif

// Produces the filter output.
void ApplyFilter(const RenderBuffer& render_buffer,
                 size_t num_partitions,
                 const std::vector<std::vector<FftData>>& H,
                 FftData* S);
#if defined(WEBRTC_HAS_NEON)
void ApplyFilter_Neon(const RenderBuffer& render_buffer,
                      size_t num_partitions,
                      const std::vector<std::vector<FftData>>& H,
                      FftData* S);
#endif
#if defined(WEBRTC_ARCH_X86_FAMILY)
void ApplyFilter_Sse2(const RenderBuffer& render_buffer,
                      size_t num_partitions,
                      const std::vector<std::vector<FftData>>& H,
                      FftData* S);

void ApplyFilter_Avx2(const RenderBuffer& render_buffer,
                      size_t num_partitions,
                      const std::vector<std::vector<FftData>>& H,
                      FftData* S);
#endif

}  // namespace aec3

// Provides a frequency domain adaptive filter functionality.
class AdaptiveFirFilter {
 public:
  AdaptiveFirFilter(size_t max_size_partitions,
                    size_t initial_size_partitions,
                    size_t size_change_duration_blocks,
                    size_t num_render_channels,
                    Aec3Optimization optimization,
                    ApmDataDumper* data_dumper);

  ~AdaptiveFirFilter();

  AdaptiveFirFilter(const AdaptiveFirFilter&) = delete;
  AdaptiveFirFilter& operator=(const AdaptiveFirFilter&) = delete;

  // Produces the output of the filter.
  void Filter(const RenderBuffer& render_buffer, FftData* S) const;

  // Adapts the filter and updates an externally stored impulse response
  // estimate.
  void Adapt(const RenderBuffer& render_buffer,
             const FftData& G,
             std::vector<float>* impulse_response);

  // Adapts the filter.
  void Adapt(const RenderBuffer& render_buffer, const FftData& G);

  // Receives reports that known echo path changes have occured and adjusts
  // the filter adaptation accordingly.
  void HandleEchoPathChange();

  // Returns the filter size.
  size_t SizePartitions() const { return current_size_partitions_; }

  // Sets the filter size.
  void SetSizePartitions(size_t size, bool immediate_effect);

  // Computes the frequency responses for the filter partitions.
  void ComputeFrequencyResponse(
      std::vector<std::array<float, kFftLengthBy2Plus1>>* H2) const;

  // Returns the maximum number of partitions for the filter.
  size_t max_filter_size_partitions() const { return max_size_partitions_; }

  void DumpFilter(absl::string_view name_frequency_domain) {
    for (size_t p = 0; p < max_size_partitions_; ++p) {
      data_dumper_->DumpRaw(name_frequency_domain, H_[p][0].re);
      data_dumper_->DumpRaw(name_frequency_domain, H_[p][0].im);
    }
  }

  // Scale the filter impulse response and spectrum by a factor.
  void ScaleFilter(float factor);

  // Set the filter coefficients.
  void SetFilter(size_t num_partitions,
                 const std::vector<std::vector<FftData>>& H);

  // Gets the filter coefficients.
  const std::vector<std::vector<FftData>>& GetFilter() const { return H_; }

 private:
  // Adapts the filter and updates the filter size.
  void AdaptAndUpdateSize(const RenderBuffer& render_buffer, const FftData& G);

  // Constrain the filter partitions in a cyclic manner.
  void Constrain();
  // Constrains the filter in a cyclic manner and updates the corresponding
  // values in the supplied impulse response.
  void ConstrainAndUpdateImpulseResponse(std::vector<float>* impulse_response);

  // Gradually Updates the current filter size towards the target size.
  void UpdateSize();

  ApmDataDumper* const data_dumper_;
  const Aec3Fft fft_;
  const Aec3Optimization optimization_;
  const size_t num_render_channels_;
  const size_t max_size_partitions_;
  const int size_change_duration_blocks_;
  float one_by_size_change_duration_blocks_;
  size_t current_size_partitions_;
  size_t target_size_partitions_;
  size_t old_target_size_partitions_;
  int size_change_counter_ = 0;
  std::vector<std::vector<FftData>> H_;
  size_t partition_to_constrain_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ADAPTIVE_FIR_FILTER_H_
