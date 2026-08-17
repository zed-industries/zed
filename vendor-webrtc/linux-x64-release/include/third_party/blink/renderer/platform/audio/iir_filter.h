// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_IIR_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_IIR_FILTER_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT IIRFilter final {
 public:
  // The maximum IIR filter order.  This also limits the number of feedforward
  // coefficients.  The maximum number of coefficients is 20 according to the
  // spec.
  const static size_t kMaxOrder = 19;
  IIRFilter(const AudioDoubleArray* feedforward_coef,
            const AudioDoubleArray* feedback_coef);
  ~IIRFilter();

  void Process(const float* source_p,
               float* dest_p,
               uint32_t frames_to_process);

  void Reset();

  void GetFrequencyResponse(int n_frequencies,
                            const float* frequency,
                            float* mag_response,
                            float* phase_response);

  // Compute the tail time of the IIR filter
  double TailTime(double sample_rate,
                  bool is_filter_stable,
                  unsigned render_quantum_frames);

  // Reset the internal state of the IIR filter to the initial state.
  void ResetState();

 private:
  // Filter memory
  //
  // For simplicity, we assume |m_xBuffer| and |m_yBuffer| have the same length,
  // and the length is a power of two.  Since the number of coefficients has a
  // fixed upper length, the size of xBuffer and yBuffer is fixed. |m_xBuffer|
  // holds the old input values and |m_yBuffer| holds the old output values
  // needed to compute the new output value.
  //
  // m_yBuffer[m_bufferIndex] holds the most recent output value, say, y[n].
  // Then m_yBuffer[m_bufferIndex - k] is y[n - k].  Similarly for m_xBuffer.
  //
  // To minimize roundoff, these arrays are double's instead of floats.
  AudioDoubleArray x_buffer_;
  AudioDoubleArray y_buffer_;

  // Index into the xBuffer and yBuffer arrays where the most current x and y
  // values should be stored.  xBuffer[bufferIndex] corresponds to x[n], the
  // current x input value and yBuffer[bufferIndex] is where y[n], the current
  // output value.
  int buffer_index_;

  // Coefficients of the IIR filter.  To minimize storage, these point to the
  // arrays given in the constructor.
  raw_ptr<const AudioDoubleArray> feedback_;
  raw_ptr<const AudioDoubleArray> feedforward_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_IIR_FILTER_H_
